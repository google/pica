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

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Instant;

use pdl_runtime::Packet;
use pica::packets::uci::{
    self, COMMON_HEADER_SIZE, CommonPacketHeader, ControlPacket, ControlPacketChild,
    CorePacketChild, DataMessageRcv, DeviceState, GroupId, HEADER_SIZE,
    MAX_CTRL_PACKET_PAYLOAD_SIZE, MAX_DATA_PACKET_PAYLOAD_SIZE, MessageType, PacketBoundaryFlag,
    ReasonCode, SessionConfigPacketChild, SessionControlPacketChild, SessionDataTransferStatusNtf,
    SessionInfoNtfChild, SessionState, SessionUpdateControllerMulticastListNtf, Status,
};
use pica::{Handle, MacAddress, Pica, UciPacket};

use crate::config::encode_control_fragments;

#[derive(Clone, Debug)]
pub struct ObservedSession {
    pub state: SessionState,
    pub last_sequence_number: Option<u32>,
    pub max_measurements: u16,
    pub ranging_reports: u16,
}

#[derive(Clone, Debug, Default)]
pub struct ObservedDevice {
    pub is_reset: bool,
    pub device_state: Option<DeviceState>,
    pub sessions: BTreeMap<u32, ObservedSession>,
}

/// Decoded two-way ranging measurement normalized across short and extended MAC modes.
#[derive(Clone, Copy, Debug)]
pub struct ObservedMeasurement {
    pub mac: MacAddress,
    pub status: Status,
    pub distance: u16,
}

/// Summary of a decoded two-way `SessionInfoNtf`.
#[derive(Clone, Debug)]
pub struct TwrNtfSummary {
    pub session_token: u32,
    pub current_ranging_interval: u32,
    pub measurements: Vec<ObservedMeasurement>,
}

#[derive(Default)]
pub struct UciMonitor {
    ctrl_rx_buffers: HashMap<Handle, Vec<u8>>,
    data_rx_buffers: HashMap<Handle, Vec<u8>>,
    devices: HashMap<Handle, ObservedDevice>,
}

impl UciMonitor {
    pub fn register_device(&mut self, handle: Handle) {
        self.devices.insert(handle, ObservedDevice::default());
    }

    pub fn remove_device(&mut self, handle: Handle) {
        self.devices.remove(&handle);
        self.ctrl_rx_buffers.remove(&handle);
        self.data_rx_buffers.remove(&handle);
    }

    pub fn device(&self, handle: Handle) -> Option<&ObservedDevice> {
        self.devices.get(&handle)
    }

    pub fn set_session_max_measurements(
        &mut self,
        handle: Handle,
        session_id: u32,
        max_measurements: u16,
    ) {
        if let Some(sess) = self
            .devices
            .get_mut(&handle)
            .and_then(|d| d.sessions.get_mut(&session_id))
        {
            sess.max_measurements = max_measurements;
        }
    }

    pub fn any_active(&self) -> bool {
        self.devices.values().any(|dev| {
            dev.sessions
                .values()
                .any(|sess| sess.state == SessionState::SessionStateActive)
        })
    }

    pub fn send_control(
        &mut self,
        pica: &mut Pica,
        handle: Handle,
        packet: impl Packet,
        now: Instant,
    ) -> Vec<(Handle, UciPacket)> {
        let raw = packet.encode_to_vec().expect("encode control packet");
        let sent_ctrl = ControlPacket::decode_full(&raw).expect("valid control command");
        let sent_gid = sent_ctrl.gid;
        let sent_oid = sent_ctrl.payload.first().copied().unwrap_or(0);
        let is_reset_cmd = sent_ctrl.is_core_device_reset_cmd();

        for frag in encode_control_fragments(&sent_ctrl, MAX_CTRL_PACKET_PAYLOAD_SIZE) {
            pica.handle_packet(handle, &frag, now);
        }

        let packets = self.drain_reassembled(pica);
        let mut saw_response = false;
        let mut saw_pre_reset_err = false;
        for &(pkt_handle, ref bytes) in &packets {
            if pkt_handle == handle
                && let Ok(ctrl) = ControlPacket::decode_full(bytes)
            {
                if ctrl.mt == MessageType::Response {
                    assert!(!saw_response, "at most one response per command");
                    saw_response = true;
                    assert_eq!(ctrl.gid, sent_gid, "response GID mismatch");
                    let rsp_oid = ctrl.payload.first().copied().unwrap_or(0);
                    assert_eq!(rsp_oid, sent_oid, "response OID mismatch");
                    if is_reset_cmd
                        && ctrl.payload.get(3).copied() == Some(u8::from(Status::Ok))
                        && let Some(dev) = self.devices.get_mut(&handle)
                    {
                        dev.is_reset = true;
                        dev.device_state = None;
                        dev.sessions.clear();
                    }
                }
                if let Ok(ControlPacketChild::CorePacket(core)) = ctrl.specialize()
                    && let Ok(CorePacketChild::CoreDeviceStatusNtf(ntf)) = core.specialize()
                    && ntf.device_state == DeviceState::DeviceStateError
                {
                    saw_pre_reset_err = true;
                }
            }
            self.observe_packet(pkt_handle, bytes);
        }
        assert!(
            saw_response || saw_pre_reset_err,
            "command on handle {handle} emitted neither response nor DeviceStateError"
        );
        self.check_device_state(handle);
        packets
    }

    pub fn observe_drain(&mut self, pica: &mut Pica) -> Vec<(Handle, UciPacket)> {
        let packets = self.drain_reassembled(pica);
        let touched: BTreeSet<Handle> = packets
            .iter()
            .map(|(handle, bytes)| {
                self.observe_packet(*handle, bytes);
                *handle
            })
            .collect();
        for handle in touched {
            self.check_device_state(handle);
        }
        packets
    }

    fn drain_reassembled(&mut self, pica: &mut Pica) -> Vec<(Handle, UciPacket)> {
        pica.drain_packets()
            .filter_map(|(handle, fragment)| {
                let &[_, _, len_lo, len_hi] = fragment
                    .first_chunk::<HEADER_SIZE>()
                    .expect("fragment shorter than UCI header");
                let header = CommonPacketHeader::decode_full(
                    fragment
                        .get(..COMMON_HEADER_SIZE)
                        .expect("fragment shorter than common header"),
                )
                .expect("invalid header");
                let actual_payload_len = fragment.len() - HEADER_SIZE;

                let (buffers, encoded_len, max_len) = if header.mt == MessageType::Data {
                    (
                        &mut self.data_rx_buffers,
                        usize::from(u16::from_le_bytes([len_lo, len_hi])),
                        MAX_DATA_PACKET_PAYLOAD_SIZE,
                    )
                } else {
                    (
                        &mut self.ctrl_rx_buffers,
                        usize::from(len_hi),
                        MAX_CTRL_PACKET_PAYLOAD_SIZE,
                    )
                };
                assert_eq!(encoded_len, actual_payload_len, "header length mismatch");
                assert!(
                    actual_payload_len <= max_len,
                    "outbound fragment payload {actual_payload_len} exceeds max {max_len}"
                );

                let complete = reassemble_fragment(buffers, handle, fragment);
                if header.pbf == PacketBoundaryFlag::Complete {
                    Some((handle, complete))
                } else {
                    buffers.insert(handle, complete);
                    None
                }
            })
            .collect()
    }

    fn check_device_state(&self, handle: Handle) {
        let Some(dev) = self.devices.get(&handle) else {
            return;
        };
        if !dev.is_reset && dev.device_state == Some(DeviceState::DeviceStateError) {
            return;
        }
        let has_active = dev
            .sessions
            .values()
            .any(|s| s.state == SessionState::SessionStateActive);
        let expected = if has_active {
            DeviceState::DeviceStateActive
        } else {
            DeviceState::DeviceStateReady
        };
        assert_eq!(
            dev.device_state,
            Some(expected),
            "device {handle} state mismatch: sessions={:?}",
            dev.sessions
        );
    }

    fn observe_packet(&mut self, handle: Handle, bytes: &[u8]) {
        let header = CommonPacketHeader::decode_full(
            bytes
                .get(..COMMON_HEADER_SIZE)
                .expect("packet shorter than common header"),
        )
        .expect("valid common header");
        if header.mt == MessageType::Data {
            let rcv = DataMessageRcv::decode_full(bytes).expect("valid outbound DataMessageRcv");
            if let Some(dev) = self.devices.get(&handle) {
                assert!(
                    dev.sessions.contains_key(&rcv.session_handle),
                    "DataMessageRcv for unknown session {} on handle {handle}",
                    rcv.session_handle
                );
            }
            return;
        }
        let ctrl = ControlPacket::decode_full(bytes).expect("valid outbound ControlPacket");
        let Some(dev) = self.devices.get_mut(&handle) else {
            return;
        };
        let child = ctrl
            .specialize()
            .expect("valid ControlPacket specialization");
        match child {
            ControlPacketChild::CorePacket(core) => {
                if let Ok(CorePacketChild::CoreDeviceStatusNtf(ntf)) = core.specialize() {
                    if ntf.device_state != DeviceState::DeviceStateError {
                        assert_ne!(
                            dev.device_state,
                            Some(ntf.device_state),
                            "redundant CoreDeviceStatusNtf({:?}) on handle {handle}",
                            ntf.device_state
                        );
                    }
                    dev.device_state = Some(ntf.device_state);
                } else if let Ok(CorePacketChild::CoreGetConfigRsp(rsp)) = core.specialize()
                    && rsp.status == Status::Ok
                {
                    for param in rsp.parameters {
                        if param.id == uci::ConfigParameterId::DeviceState {
                            assert_eq!(
                                param.value.first().copied(),
                                dev.device_state.map(u8::from),
                                "CoreGetConfig DeviceState disagreed with monitor"
                            );
                        }
                    }
                }
            }
            ControlPacketChild::SessionConfigPacket(sc) => match sc.specialize() {
                Ok(SessionConfigPacketChild::SessionStatusNtf(ntf)) => {
                    Self::observe_session_status(
                        dev,
                        ntf.session_token,
                        ntf.session_state,
                        ntf.reason_code,
                    );
                }
                Ok(SessionConfigPacketChild::SessionGetCountRsp(rsp)) => {
                    assert_eq!(rsp.status, Status::Ok);
                    assert_eq!(usize::from(rsp.session_count), dev.sessions.len());
                }
                Ok(SessionConfigPacketChild::SessionUpdateControllerMulticastListNtf(ntf)) => {
                    assert!(
                        dev.sessions.contains_key(&ntf.session_token),
                        "multicast list NTF for unknown session {}",
                        ntf.session_token
                    );
                }
                _ => {}
            },
            ControlPacketChild::SessionControlPacket(ctl) => match ctl.specialize() {
                Ok(SessionControlPacketChild::SessionInfoNtf(ntf)) => {
                    let token = ntf.session_token;
                    let seq = ntf.sequence_number;
                    let _ = ntf.specialize().expect("valid SessionInfoNtf subtype");
                    let sess = dev
                        .sessions
                        .get_mut(&token)
                        .expect("SessionInfoNtf emitted for unknown session");
                    assert_eq!(
                        sess.state,
                        SessionState::SessionStateActive,
                        "SessionInfoNtf emitted for non-active session {token}"
                    );
                    if let Some(prev_seq) = sess.last_sequence_number {
                        assert!(
                            seq > prev_seq,
                            "SessionInfoNtf sequence_number did not advance: {seq} <= {prev_seq}"
                        );
                    }
                    sess.last_sequence_number = Some(seq);
                    sess.ranging_reports = sess.ranging_reports.saturating_add(1);
                    if sess.max_measurements > 0 {
                        assert!(
                            sess.ranging_reports <= sess.max_measurements,
                            "session {token} exceeded max_measurements={}",
                            sess.max_measurements
                        );
                    }
                }
                Ok(SessionControlPacketChild::SessionDataCreditNtf(ntf)) => {
                    assert!(
                        dev.sessions.contains_key(&ntf.session_token),
                        "SessionDataCreditNtf for unknown session {}",
                        ntf.session_token
                    );
                }
                Ok(SessionControlPacketChild::SessionDataTransferStatusNtf(ntf)) => {
                    assert!(
                        dev.sessions.contains_key(&ntf.session_token),
                        "SessionDataTransferStatusNtf for unknown session {}",
                        ntf.session_token
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn observe_session_query(
        &self,
        handle: Handle,
        session_id: u32,
        rsp: &uci::SessionGetStateRsp,
    ) {
        let Some(dev) = self.devices.get(&handle) else {
            return;
        };
        match dev.sessions.get(&session_id) {
            Some(sess) => {
                assert_eq!(rsp.status, Status::Ok);
                assert_eq!(rsp.session_state, sess.state);
            }
            None => assert_eq!(rsp.status, Status::ErrorSessionNotExist),
        }
    }

    pub fn observe_ranging_count(
        &self,
        handle: Handle,
        session_id: u32,
        rsp: &uci::SessionGetRangingCountRsp,
    ) {
        let Some(dev) = self.devices.get(&handle) else {
            return;
        };
        match dev.sessions.get(&session_id) {
            Some(sess) => {
                assert_eq!(rsp.status, Status::Ok);
                assert!(
                    u32::from(sess.ranging_reports) <= rsp.count,
                    "session {session_id} emitted {} notifications but ranging_count={}",
                    sess.ranging_reports,
                    rsp.count
                );
            }
            None => assert_eq!(rsp.status, Status::ErrorSessionNotExist),
        }
    }

    pub fn session_info_ntf(bytes: &[u8]) -> Option<(u32, bool)> {
        let ctrl = ControlPacket::decode_full(bytes).ok()?;
        let ControlPacketChild::SessionControlPacket(ctl) = ctrl.specialize().ok()? else {
            return None;
        };
        let SessionControlPacketChild::SessionInfoNtf(ntf) = ctl.specialize().ok()? else {
            return None;
        };
        let token = ntf.session_token;
        let is_dl_tdoa = matches!(
            ntf.specialize(),
            Ok(SessionInfoNtfChild::ShortMacDlTDoASessionInfoNtf(_))
                | Ok(SessionInfoNtfChild::ExtendedMacDlTDoASessionInfoNtf(_))
        );
        Some((token, is_dl_tdoa))
    }

    /// Decodes a two-way ranging `SessionInfoNtf` into normalized measurements.
    pub fn decode_two_way_ntf(bytes: &[u8]) -> Option<TwrNtfSummary> {
        let ctrl = ControlPacket::decode_full(bytes).ok()?;
        let ControlPacketChild::SessionControlPacket(ctl) = ctrl.specialize().ok()? else {
            return None;
        };
        let SessionControlPacketChild::SessionInfoNtf(ntf) = ctl.specialize().ok()? else {
            return None;
        };
        let session_token = ntf.session_token;
        let current_ranging_interval = ntf.current_ranging_interval;
        let measurements = match ntf.specialize().ok()? {
            SessionInfoNtfChild::ShortMacTwoWaySessionInfoNtf(s) => s
                .two_way_ranging_measurements
                .into_iter()
                .map(|m| ObservedMeasurement {
                    mac: MacAddress::Short(m.mac_address.to_le_bytes()),
                    status: m.status,
                    distance: m.distance,
                })
                .collect(),
            SessionInfoNtfChild::ExtendedMacTwoWaySessionInfoNtf(e) => e
                .two_way_ranging_measurements
                .into_iter()
                .map(|m| ObservedMeasurement {
                    mac: MacAddress::Extended(m.mac_address.to_le_bytes()),
                    status: m.status,
                    distance: m.distance,
                })
                .collect(),
            _ => return None,
        };
        Some(TwrNtfSummary {
            session_token,
            current_ranging_interval,
            measurements,
        })
    }

    /// Decodes a reassembled outbound `DataMessageRcv` packet.
    pub fn decode_data_rcv(bytes: &[u8]) -> Option<DataMessageRcv> {
        let header = CommonPacketHeader::decode_full(bytes.get(..COMMON_HEADER_SIZE)?).ok()?;
        (header.mt == MessageType::Data)
            .then(|| DataMessageRcv::decode_full(bytes).ok())
            .flatten()
    }

    /// Decodes a `SessionDataTransferStatusNtf` packet.
    pub fn decode_data_status_ntf(bytes: &[u8]) -> Option<SessionDataTransferStatusNtf> {
        SessionDataTransferStatusNtf::decode_full(bytes).ok()
    }

    /// Decodes a `SessionUpdateControllerMulticastListNtf` packet.
    pub fn decode_multicast_ntf(bytes: &[u8]) -> Option<SessionUpdateControllerMulticastListNtf> {
        SessionUpdateControllerMulticastListNtf::decode_full(bytes).ok()
    }

    fn observe_session_status(
        dev: &mut ObservedDevice,
        token: u32,
        next_state: SessionState,
        reason: ReasonCode,
    ) {
        use SessionState::*;
        let prev_state = dev.sessions.get(&token).map(|s| s.state);
        let is_cmd = reason == ReasonCode::StateChangeWithSessionManagementCommands;
        let legal = match (prev_state, next_state) {
            (None, SessionStateInit)
            | (Some(SessionStateInit), SessionStateIdle)
            | (Some(SessionStateIdle), SessionStateActive)
            | (Some(_), SessionStateDeinit) => is_cmd,
            (Some(SessionStateIdle), SessionStateIdle) => !is_cmd,
            (Some(SessionStateActive), SessionStateActive) => matches!(
                reason,
                ReasonCode::SessionSuspendedDueToInbandSignal
                    | ReasonCode::SessionResumedDueToInbandSignal
            ),
            (Some(SessionStateActive), SessionStateIdle) => true,
            _ => false,
        };
        assert!(
            legal,
            "illegal session state edge for session {token}: {prev_state:?} -> {next_state:?} ({reason:?})"
        );

        if next_state == SessionStateDeinit {
            dev.sessions.remove(&token);
        } else {
            let sess = dev.sessions.entry(token).or_insert(ObservedSession {
                state: next_state,
                last_sequence_number: None,
                max_measurements: 0,
                ranging_reports: 0,
            });
            sess.state = next_state;
            if next_state == SessionStateIdle {
                sess.last_sequence_number = None;
                sess.ranging_reports = 0;
            }
        }
    }

    pub fn active_session_ids(&self, handle: Handle) -> Vec<u32> {
        self.devices
            .get(&handle)
            .map(|d| {
                d.sessions
                    .iter()
                    .filter_map(|(&id, s)| {
                        (s.state == SessionState::SessionStateActive).then_some(id)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn has_session(&self, handle: Handle, session_id: u32) -> bool {
        self.devices
            .get(&handle)
            .is_some_and(|d| d.sessions.contains_key(&session_id))
    }

    pub fn decode_response<T: Packet>(packets: &[(Handle, UciPacket)], handle: Handle) -> T {
        packets
            .iter()
            .filter_map(|(h, bytes)| (*h == handle).then(|| T::decode_full(bytes).ok()).flatten())
            .next()
            .expect("expected matching UCI response")
    }

    pub fn contains_group_id(bytes: &[u8], expected_gid: GroupId) -> bool {
        ControlPacket::decode_full(bytes).is_ok_and(|c| c.gid == expected_gid)
    }
}

fn reassemble_fragment(
    buffers: &mut HashMap<Handle, Vec<u8>>,
    handle: Handle,
    fragment: Vec<u8>,
) -> Vec<u8> {
    match buffers.remove(&handle) {
        Some(mut buf) => {
            let payload = fragment
                .get(HEADER_SIZE..)
                .expect("fragment shorter than header");
            buf.extend_from_slice(payload);
            buf
        }
        None => fragment,
    }
}
