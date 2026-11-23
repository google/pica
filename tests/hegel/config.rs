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

use pdl_runtime::Packet;
use pica::packets::uci::{
    AppConfigTlv, AppConfigTlvType, ChannelNumber, DataMessageSnd, DataPacket, DataPacketFormat,
    DeviceRole, DeviceType, HEADER_SIZE, MAX_DATA_PACKET_PAYLOAD_SIZE, MacAddressMode, MessageType,
    MultiNodeMode, PacketBoundaryFlag, RangingRoundUsage, ScheduleMode,
};
use pica::{Handle, MacAddress, RangingEstimator, RangingMeasurement};

pub const SAMPLE_MEASUREMENT: RangingMeasurement = RangingMeasurement {
    range: 175,
    azimuth: 15,
    elevation: -6,
};

#[derive(Clone, Default)]
pub struct PairwiseEstimator {
    pairs: Arc<RwLock<HashMap<(Handle, Handle), RangingMeasurement>>>,
}

impl PairwiseEstimator {
    pub fn insert(&self, left: Handle, right: Handle, measurement: RangingMeasurement) {
        self.pairs
            .write()
            .expect("lock pairwise estimator")
            .insert((left, right), measurement);
    }

    pub fn get(&self, left: Handle, right: Handle) -> Option<RangingMeasurement> {
        self.pairs
            .read()
            .expect("lock pairwise estimator")
            .get(&(left, right))
            .copied()
    }

    pub fn remove(&self, left: Handle, right: Handle) {
        self.pairs
            .write()
            .expect("lock pairwise estimator")
            .remove(&(left, right));
    }
}

impl RangingEstimator for PairwiseEstimator {
    fn estimate(&self, left: &Handle, right: &Handle) -> Option<RangingMeasurement> {
        self.pairs
            .read()
            .expect("lock pairwise estimator")
            .get(&(*left, *right))
            .copied()
    }
}

pub struct TwrSessionSpec {
    pub role: DeviceRole,
    pub dev_type: DeviceType,
    pub mac_mode: MacAddressMode,
    pub multi_node_mode: MultiNodeMode,
    pub ranging_round_usage: RangingRoundUsage,
    pub channel: ChannelNumber,
    pub local_mac: MacAddress,
    pub dst_macs: Vec<MacAddress>,
    pub duration_ms: u32,
    pub max_measurements: u16,
    pub block_stride: u8,
}

pub fn mac_bytes(mac: MacAddress) -> Vec<u8> {
    Vec::from(mac)
}

pub fn twr_mac_pair(mac_mode: MacAddressMode, c0: u8, c1: u8) -> (MacAddress, MacAddress) {
    match mac_mode {
        MacAddressMode::Mode2 => (
            MacAddress::Extended([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, c0, c1]),
            MacAddress::Extended([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, c0.wrapping_add(1), c1]),
        ),
        _ => (
            MacAddress::Short([c0, c1]),
            MacAddress::Short([c0.wrapping_add(1), c1.wrapping_add(1)]),
        ),
    }
}

pub fn twr_spec_to_tlvs(spec: &TwrSessionSpec) -> Vec<AppConfigTlv> {
    let dst_bytes: Vec<u8> = spec
        .dst_macs
        .iter()
        .flat_map(|&mac| mac_bytes(mac))
        .collect();
    [
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceRole,
            v: vec![spec.role as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceType,
            v: vec![spec.dev_type as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MacAddressMode,
            v: vec![spec.mac_mode as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceMacAddress,
            v: mac_bytes(spec.local_mac),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MultiNodeMode,
            v: vec![spec.multi_node_mode as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::ScheduleMode,
            v: vec![ScheduleMode::ContentionBased as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RangingRoundUsage,
            v: vec![spec.ranging_round_usage as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::ChannelNumber,
            v: vec![spec.channel as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RangingDuration,
            v: spec.duration_ms.to_le_bytes().to_vec(),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MaxNumberOfMeasurements,
            v: spec.max_measurements.to_le_bytes().to_vec(),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::BlockStrideLength,
            v: vec![spec.block_stride],
        },
    ]
    .into_iter()
    .chain(
        (!spec.dst_macs.is_empty())
            .then(|| {
                [
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::NumberOfControlees,
                        v: vec![spec.dst_macs.len() as u8],
                    },
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::DstMacAddress,
                        v: dst_bytes,
                    },
                ]
            })
            .into_iter()
            .flatten(),
    )
    .collect()
}

pub fn dl_tdoa_tlvs(
    role: DeviceRole,
    mac_mode: MacAddressMode,
    channel: ChannelNumber,
    local_mac: MacAddress,
    duration_ms: u32,
    max_measurements: u16,
    block_skipping: u8,
) -> Vec<AppConfigTlv> {
    vec![
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceRole,
            v: vec![role as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceType,
            v: vec![DeviceType::Controller as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MacAddressMode,
            v: vec![mac_mode as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DeviceMacAddress,
            v: mac_bytes(local_mac),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MultiNodeMode,
            v: vec![MultiNodeMode::OneToMany as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::ScheduleMode,
            v: vec![ScheduleMode::TimeScheduled as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RangingRoundUsage,
            v: vec![RangingRoundUsage::OneWayRangingDlTdoa as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::ChannelNumber,
            v: vec![channel as u8],
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::RangingDuration,
            v: duration_ms.to_le_bytes().to_vec(),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::MaxNumberOfMeasurements,
            v: max_measurements.to_le_bytes().to_vec(),
        },
        AppConfigTlv {
            cfg_id: AppConfigTlvType::DlTdoaBlockSkipping,
            v: vec![block_skipping],
        },
    ]
}

pub fn twr_session_tlvs(
    role: DeviceRole,
    dev_type: DeviceType,
    mac_mode: MacAddressMode,
    local_mac: MacAddress,
    dst_mac: MacAddress,
    duration_ms: u32,
    extra: Vec<AppConfigTlv>,
) -> Vec<AppConfigTlv> {
    twr_spec_to_tlvs(&TwrSessionSpec {
        role,
        dev_type,
        mac_mode,
        multi_node_mode: MultiNodeMode::OneToOne,
        ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
        channel: ChannelNumber::ChannelNumber9,
        local_mac,
        dst_macs: vec![dst_mac],
        duration_ms,
        max_measurements: 0,
        block_stride: 0,
    })
    .into_iter()
    .chain(extra)
    .collect()
}

/// Encodes and segments a UCI control command into wire fragments bounded by `max_payload`.
pub fn encode_control_fragments(packet: &impl Packet, max_payload: usize) -> Vec<Vec<u8>> {
    let raw = packet.encode_to_vec().expect("encode control packet");
    let (header, payload) = raw
        .split_at_checked(HEADER_SIZE)
        .expect("packet shorter than UCI header");
    let &[b0, b1, b2, _] = header.first_chunk::<HEADER_SIZE>().expect("4-byte header");
    let chunks: Vec<&[u8]> = if payload.is_empty() {
        vec![b""]
    } else {
        payload.chunks(max_payload.max(1)).collect()
    };

    chunks
        .iter()
        .enumerate()
        .map(|(idx, chunk)| {
            let pbf = if idx + 1 == chunks.len() { 0x00 } else { 0x10 };
            [&[(b0 & !0x10) | pbf, b1, b2, chunk.len() as u8], *chunk].concat()
        })
        .collect()
}

/// Encodes `DataMessageSnd` and splits it into UCI `DataPacket` fragments.
pub fn encode_data_snd_fragments(
    session_handle: u32,
    destination_address: u64,
    data_sequence_number: u16,
    application_data: Vec<u8>,
) -> Vec<Vec<u8>> {
    let raw = DataMessageSnd {
        pbf: PacketBoundaryFlag::Complete,
        session_handle,
        destination_address,
        data_sequence_number,
        application_data,
    }
    .encode_to_vec()
    .expect("encode DataMessageSnd");
    let payload = raw
        .get(HEADER_SIZE..)
        .expect("missing DataMessageSnd header");
    let chunks: Vec<&[u8]> = payload.chunks(MAX_DATA_PACKET_PAYLOAD_SIZE).collect();

    chunks
        .iter()
        .enumerate()
        .map(|(idx, chunk)| {
            let pbf = if idx + 1 == chunks.len() {
                PacketBoundaryFlag::Complete
            } else {
                PacketBoundaryFlag::NotComplete
            };
            DataPacket {
                dpf: DataPacketFormat::DataSnd,
                pbf,
                mt: MessageType::Data,
                payload: chunk.to_vec(),
            }
            .encode_to_vec()
            .expect("encode DataPacket")
        })
        .collect()
}
