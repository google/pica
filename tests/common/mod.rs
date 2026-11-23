// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use pdl_runtime::Packet;
use pica::packets::uci::{self, *};
use pica::{Handle, Pica, RangingEstimator, RangingMeasurement};

pub struct FixedEstimator(pub RangingMeasurement);

impl RangingEstimator for FixedEstimator {
    fn estimate(&self, _left: &Handle, _right: &Handle) -> Option<RangingMeasurement> {
        Some(self.0)
    }
}

pub struct PicaHarness {
    pub pica: Pica,
    pub now: Instant,
    rx_buffers: HashMap<Handle, Vec<u8>>,
    packets: HashMap<Handle, VecDeque<Vec<u8>>>,
}

impl PicaHarness {
    pub fn new(estimator: Box<dyn RangingEstimator>) -> Self {
        Self {
            pica: Pica::new(estimator),
            now: Instant::now(),
            rx_buffers: HashMap::new(),
            packets: HashMap::new(),
        }
    }

    fn drain(&mut self) {
        for (handle, fragment) in self.pica.drain_packets() {
            let header_bytes = fragment
                .get(..COMMON_HEADER_SIZE)
                .expect("fragment shorter than common header");
            let header = CommonPacketHeader::decode_full(header_bytes).expect("invalid header");
            let complete = match self.rx_buffers.remove(&handle) {
                Some(mut buf) => {
                    let payload = fragment
                        .get(HEADER_SIZE..)
                        .expect("fragment shorter than header");
                    buf.extend_from_slice(payload);
                    buf
                }
                None => fragment,
            };
            if header.pbf == PacketBoundaryFlag::Complete {
                self.packets.entry(handle).or_default().push_back(complete);
            } else {
                self.rx_buffers.insert(handle, complete);
            }
        }
    }

    pub fn add_reset_device(&mut self) -> Handle {
        let handle = self.pica.add_device();
        let ready = self.pop::<CoreDeviceStatusNtf>(handle);
        assert_eq!(ready.device_state, DeviceState::DeviceStateReady);

        self.send_control(
            handle,
            CoreDeviceResetCmd {
                reset_config: ResetConfig::UwbsReset,
            },
        );

        let rsp = self.pop::<CoreDeviceResetRsp>(handle);
        assert_eq!(rsp.status, uci::Status::Ok);
        let ready = self.pop::<CoreDeviceStatusNtf>(handle);
        assert_eq!(ready.device_state, DeviceState::DeviceStateReady);
        handle
    }

    pub fn send_control(&mut self, handle: Handle, packet: impl Packet) {
        let raw = packet.encode_to_vec().expect("encode control packet");
        let (header, payload) = raw
            .split_at_checked(HEADER_SIZE)
            .expect("packet shorter than UCI header");
        let [b0, b1, b2, _]: [u8; 4] = header.try_into().expect("4-byte UCI header");
        let chunks: Vec<&[u8]> = if payload.is_empty() {
            vec![b""]
        } else {
            payload.chunks(MAX_CTRL_PACKET_PAYLOAD_SIZE).collect()
        };

        for (idx, chunk) in chunks.iter().enumerate() {
            let pbf = if idx + 1 == chunks.len() { 0x00 } else { 0x10 };
            let mut frag = Vec::with_capacity(HEADER_SIZE + chunk.len());
            frag.push((b0 & !0x10) | pbf);
            frag.push(b1);
            frag.push(b2);
            frag.push(chunk.len() as u8);
            frag.extend_from_slice(chunk);
            self.pica.handle_packet(handle, &frag, self.now);
        }
    }

    pub fn pop<T: Packet>(&mut self, handle: Handle) -> T {
        self.drain();
        let queue = self
            .packets
            .get_mut(&handle)
            .expect("expected outbound packets for handle");
        let idx = queue
            .iter()
            .position(|bytes| T::decode_full(bytes).is_ok())
            .expect("expected matching outbound packet");
        let bytes = queue.remove(idx).expect("checked index");
        T::decode_full(&bytes).expect("decode UCI packet")
    }

    pub fn tick_until<T: Packet>(&mut self, handle: Handle) -> T {
        self.drain();
        while !self
            .packets
            .get(&handle)
            .is_some_and(|queue| queue.iter().any(|bytes| T::decode_full(bytes).is_ok()))
        {
            let deadline = self
                .pica
                .next_deadline()
                .expect("expected active ranging tick");
            self.now = self.now.max(deadline);
            self.pica.tick(self.now);
            self.drain();
        }
        self.pop::<T>(handle)
    }

    pub fn start_session(&mut self, handle: Handle, session_id: u32) {
        self.send_control(handle, SessionStartCmd { session_id });
        let rsp = self.pop::<SessionStartRsp>(handle);
        assert_eq!(rsp.status, uci::Status::Ok);
        let s_ntf = self.pop::<SessionStatusNtf>(handle);
        assert_eq!(s_ntf.session_token, session_id);
        assert_eq!(s_ntf.session_state, SessionState::SessionStateActive);
        let d_ntf = self.pop::<CoreDeviceStatusNtf>(handle);
        assert_eq!(d_ntf.device_state, DeviceState::DeviceStateActive);
    }

    pub fn stop_and_deinit_session(&mut self, handle: Handle, session_id: u32) {
        self.send_control(handle, SessionStopCmd { session_id });
        let rsp = self.pop::<SessionStopRsp>(handle);
        assert_eq!(rsp.status, uci::Status::Ok);
        let s_ntf = self.pop::<SessionStatusNtf>(handle);
        assert_eq!(s_ntf.session_state, SessionState::SessionStateIdle);
        let d_ntf = self.pop::<CoreDeviceStatusNtf>(handle);
        assert_eq!(d_ntf.device_state, DeviceState::DeviceStateReady);

        self.send_control(
            handle,
            SessionDeinitCmd {
                session_token: session_id,
            },
        );
        let rsp = self.pop::<SessionDeinitRsp>(handle);
        assert_eq!(rsp.status, uci::Status::Ok);
        let s_ntf = self.pop::<SessionStatusNtf>(handle);
        assert_eq!(s_ntf.session_state, SessionState::SessionStateDeinit);
    }

    pub fn init_and_configure(
        &mut self,
        handle: Handle,
        session_id: u32,
        session_type: SessionType,
        tlvs: Vec<AppConfigTlv>,
    ) {
        self.send_control(
            handle,
            SessionInitCmd {
                session_id,
                session_type,
            },
        );
        let rsp = self.pop::<SessionInitRsp>(handle);
        assert_eq!(rsp.status, uci::Status::Ok);
        let s_ntf = self.pop::<SessionStatusNtf>(handle);
        assert_eq!(s_ntf.session_state, SessionState::SessionStateInit);

        self.send_control(
            handle,
            SessionSetAppConfigCmd {
                session_token: session_id,
                tlvs,
            },
        );
        let rsp = self.pop::<SessionSetAppConfigRsp>(handle);
        assert_eq!(rsp.status, uci::Status::Ok);
        let s_ntf = self.pop::<SessionStatusNtf>(handle);
        assert_eq!(s_ntf.session_state, SessionState::SessionStateIdle);
    }
}
