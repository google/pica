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

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use pdl_runtime::Packet;

use crate::packets::uci::{self, *};
use crate::{Handle, Pica, RangingEstimator, RangingMeasurement};

pub struct FixedEstimator(pub RangingMeasurement);

impl RangingEstimator for FixedEstimator {
    fn estimate(&self, _left: &Handle, _right: &Handle) -> Option<RangingMeasurement> {
        Some(self.0)
    }
}

#[derive(Clone, Default)]
pub struct PairwiseEstimator {
    pairs: Arc<RwLock<HashMap<(Handle, Handle), RangingMeasurement>>>,
}

impl PairwiseEstimator {
    pub fn insert(&self, left: Handle, right: Handle, measurement: RangingMeasurement) {
        self.pairs
            .write()
            .unwrap()
            .insert((left, right), measurement);
    }
}

impl RangingEstimator for PairwiseEstimator {
    fn estimate(&self, left: &Handle, right: &Handle) -> Option<RangingMeasurement> {
        self.pairs.read().unwrap().get(&(*left, *right)).copied()
    }
}

pub fn send_cmd(pica: &mut Pica, handle: Handle, packet: impl Packet) -> Vec<Vec<u8>> {
    pica.handle_packet(handle, &packet.encode_to_vec().unwrap(), Instant::now());
    pica.drain_packets()
        .filter_map(|(h, bytes)| (h == handle).then_some(bytes))
        .collect()
}

pub fn drain_reassembled_packet(pica: &mut Pica, handle: Handle) -> Vec<u8> {
    let mut complete = Vec::new();
    for (h, fragment) in pica.drain_packets() {
        if h != handle {
            continue;
        }
        let header = CommonPacketHeader::decode_full(&fragment[..COMMON_HEADER_SIZE]).unwrap();
        if complete.is_empty() {
            complete.extend_from_slice(&fragment);
        } else {
            complete.extend_from_slice(&fragment[HEADER_SIZE..]);
        }
        if header.pbf == PacketBoundaryFlag::Complete {
            return complete;
        }
    }
    complete
}

pub fn add_reset_device(pica: &mut Pica) -> Handle {
    let handle = pica.add_device();
    let _ = pica.drain_packets().count();
    let packets = send_cmd(
        pica,
        handle,
        CoreDeviceResetCmd {
            reset_config: ResetConfig::UwbsReset,
        },
    );
    assert_eq!(
        CoreDeviceResetRsp::decode_full(&packets[0]).unwrap().status,
        uci::Status::Ok
    );
    handle
}

pub struct FiraSessionConfig<'a> {
    pub session_id: u32,
    pub session_type: SessionType,
    pub role: DeviceRole,
    pub dev_type: DeviceType,
    pub mac_mode: MacAddressMode,
    pub multi_node_mode: MultiNodeMode,
    pub local_mac: &'a [u8],
    pub dst_mac: Option<&'a [u8]>,
    pub ranging_round_usage: RangingRoundUsage,
    pub ranging_duration_ms: u32,
    pub extra_tlvs: Vec<AppConfigTlv>,
}

pub fn fira_session_tlvs(cfg: &FiraSessionConfig<'_>) -> Vec<AppConfigTlv> {
    let mut tlvs = vec![
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceRole,
            v: vec![cfg.role as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceType,
            v: vec![cfg.dev_type as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MacAddressMode,
            v: vec![cfg.mac_mode as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceMacAddress,
            v: cfg.local_mac.to_vec(),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MultiNodeMode,
            v: vec![cfg.multi_node_mode as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::ScheduleMode,
            v: vec![ScheduleMode::ContentionBased as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RangingRoundUsage,
            v: vec![cfg.ranging_round_usage as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RangingDuration,
            v: cfg.ranging_duration_ms.to_le_bytes().to_vec(),
        },
    ];
    if let Some(dst_mac) = cfg.dst_mac {
        tlvs.push(AppConfigTlv {
            cfg_id: AppConfigTlvType::NumberOfControlees,
            v: vec![1],
        });
        tlvs.push(AppConfigTlv {
            cfg_id: AppConfigTlvType::DstMacAddress,
            v: dst_mac.to_vec(),
        });
    }
    tlvs.extend(cfg.extra_tlvs.iter().cloned());
    tlvs
}

pub fn configure_fira_session(pica: &mut Pica, handle: Handle, cfg: &FiraSessionConfig<'_>) {
    let _ = send_cmd(
        pica,
        handle,
        SessionInitCmd {
            session_id: cfg.session_id,
            session_type: cfg.session_type,
        },
    );
    let packets = send_cmd(
        pica,
        handle,
        SessionSetAppConfigCmd {
            session_token: cfg.session_id,
            tlvs: fira_session_tlvs(cfg),
        },
    );
    assert_eq!(
        SessionSetAppConfigRsp::decode_full(&packets[0])
            .unwrap()
            .status,
        uci::Status::Ok
    );
}

pub fn start_fira_session(pica: &mut Pica, handle: Handle, cfg: &FiraSessionConfig<'_>) {
    let session_id = cfg.session_id;
    configure_fira_session(pica, handle, cfg);
    let packets = send_cmd(pica, handle, SessionStartCmd { session_id });
    assert_eq!(
        SessionStartRsp::decode_full(&packets[0]).unwrap().status,
        uci::Status::Ok
    );
}
