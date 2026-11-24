// Copyright 2022 Google LLC
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

use crate::dl_tdoa::MAX_ACTIVE_RANGING_ROUNDS;
use crate::packets::uci::{self, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl Version {
    pub const fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }

    pub const fn to_bytes(self) -> [u8; 2] {
        [self.major, self.minor]
    }
}

pub const FIRA_VERSION_1_1: Version = Version::new(1, 1);
pub const FIRA_VERSION_2_0: Version = Version::new(2, 0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersionRange {
    pub min: Version,
    pub max: Version,
}

impl VersionRange {
    pub const fn new(min: Version, max: Version) -> Self {
        Self { min, max }
    }

    pub const fn to_bytes(self) -> [u8; 4] {
        [
            self.min.major,
            self.min.minor,
            self.max.major,
            self.max.minor,
        ]
    }
}

const fn bit_range(start: u8, end: u8) -> u64 {
    let mut mask = 0u64;
    let mut set = start;
    while set <= end {
        mask |= 1u64 << (set - 1);
        set += 1;
    }
    mask
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HprfParameterSets {
    start: u8,
    end: u8,
}

impl HprfParameterSets {
    pub const fn range(start: u8, end: u8) -> Self {
        assert!(
            start >= 1 && end <= 35 && start <= end,
            "HPRF parameter sets must be within 1..=35"
        );
        Self { start, end }
    }

    pub const fn to_bytes(self) -> [u8; 5] {
        let [sets_1_8, sets_9_16, sets_17_24, sets_25_32, sets_33_40, ..] =
            bit_range(self.start, self.end).to_le_bytes();
        [sets_1_8, sets_9_16, sets_17_24, sets_25_32, sets_33_40]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BprfParameterSets {
    start: u8,
    end: u8,
}

impl BprfParameterSets {
    pub const fn range(start: u8, end: u8) -> Self {
        assert!(
            start >= 1 && end <= 6 && start <= end,
            "BPRF parameter sets must be within 1..=6"
        );
        Self { start, end }
    }

    pub const fn to_byte(self) -> u8 {
        let [sets_1_6, ..] = bit_range(self.start, self.end).to_le_bytes();
        sets_1_6
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportedAoa {
    pub azimuth_90: bool,
    pub azimuth_180: bool,
    pub elevation: bool,
    pub fom: bool,
}

impl SupportedAoa {
    pub const fn to_byte(self) -> u8 {
        (self.azimuth_90 as u8)
            | ((self.azimuth_180 as u8) << 1)
            | ((self.elevation as u8) << 2)
            | ((self.fom as u8) << 3)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportedSessionKeyLengths {
    pub dynamic_sts_256: bool,
    pub provisioned_sts_256: bool,
}

impl SupportedSessionKeyLengths {
    pub const fn to_byte(self) -> u8 {
        (self.dynamic_sts_256 as u8) | ((self.provisioned_sts_256 as u8) << 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportedCcConstraintLength {
    pub k3: bool,
    pub k7: bool,
}

impl SupportedCcConstraintLength {
    pub const fn to_byte(self) -> u8 {
        (self.k3 as u8) | ((self.k7 as u8) << 1)
    }
}

/// CCC supported sync codes (codes 1..=32, bit (code - 1)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CccSyncCodes {
    start: u8,
    end: u8,
}

impl CccSyncCodes {
    pub const fn range(start: u8, end: u8) -> Self {
        assert!(
            start >= 1 && end <= 32 && start <= end,
            "CCC sync codes must be within 1..=32"
        );
        Self { start, end }
    }

    pub const fn to_bytes(self) -> [u8; 4] {
        let [b0, b1, b2, b3, ..] = bit_range(self.start, self.end).to_le_bytes();
        [b0, b1, b2, b3]
    }
}

pub const CCC_ALL_CHAPS_PER_SLOT: u8 = 0xFF;
pub const CCC_ALL_HOPPING_CONFIGS: u8 = 0xFF;
pub const CCC_ALL_CHANNELS: u8 = 0xFF;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CccCapabilities {
    pub chaps_per_slot: u8,
    pub sync_codes: CccSyncCodes,
    pub hopping_config_modes_and_sequences: u8,
    pub channels: u8,
    pub versions: &'static [Version],
    pub uwb_configs: u8,
    pub pulse_shape_combos: u8,
    pub ran_multiplier: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AliroMacModes {
    pub round_1: bool,
    pub round_2: bool,
}

impl AliroMacModes {
    pub const fn to_byte(self) -> u8 {
        (self.round_1 as u8) | ((self.round_2 as u8) << 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AliroCapabilities {
    pub mac_modes: AliroMacModes,
    pub protocol_version: Version,
}

fn bitmask_u16<T: Copy + Into<u16>>(items: &[T]) -> u16 {
    items.iter().fold(0u16, |mask, &item| {
        let discriminant: u16 = item.into();
        let bit = 1u16
            .checked_shl(u32::from(discriminant))
            .expect("capability discriminant exceeds u16 bitmask width");
        mask | bit
    })
}

fn bitmask_u8<T: Copy + Into<u16>>(items: &[T]) -> u8 {
    u8::try_from(bitmask_u16(items)).expect("capability bitmask exceeds 1 byte")
}

fn encode_channels(channels: &[uci::ChannelNumber]) -> u8 {
    channels.iter().fold(0u8, |mask, &ch| {
        let bit = match ch {
            uci::ChannelNumber::ChannelNumber5 => 0,
            uci::ChannelNumber::ChannelNumber6 => 1,
            uci::ChannelNumber::ChannelNumber8 => 2,
            uci::ChannelNumber::ChannelNumber9 => 3,
            uci::ChannelNumber::ChannelNumber10 => 4,
            uci::ChannelNumber::ChannelNumber12 => 5,
            uci::ChannelNumber::ChannelNumber13 => 6,
            uci::ChannelNumber::ChannelNumber14 => 7,
        };
        mask | (1 << bit)
    })
}

/// [UCI] Device capability parameters (CORE_GET_CAPS_INFO).
pub struct Capabilities {
    pub max_message_size: u16,
    pub max_data_packet_payload_size: u16,
    pub fira_phy_version_range: VersionRange,
    pub fira_mac_version_range: VersionRange,
    pub device_types: &'static [uci::DeviceType],
    pub device_roles: &'static [uci::DeviceRole],
    pub ranging_methods: &'static [uci::RangingRoundUsage],
    pub sts_configs: &'static [uci::StsConfig],
    pub multi_node_modes: &'static [uci::MultiNodeMode],
    pub ranging_time_structs: &'static [uci::RangingTimeStruct],
    pub scheduled_modes: &'static [uci::ScheduleMode],
    pub hopping: bool,
    pub block_striding: bool,
    pub uwb_initiation_time: bool,
    pub channels: &'static [uci::ChannelNumber],
    pub rframe_configs: &'static [uci::RframeConfig],
    pub cc_constraint_length: SupportedCcConstraintLength,
    pub bprf_parameter_sets: BprfParameterSets,
    pub hprf_parameter_sets: HprfParameterSets,
    pub aoa: SupportedAoa,
    pub extended_mac_address: bool,
    pub suspend_ranging_support: bool,
    pub session_key_lengths: SupportedSessionKeyLengths,
    pub dt_anchor_max_active_rr: u8,
    pub dt_tag_max_active_rr: u8,
    pub dt_tag_block_skipping: bool,
    pub ccc: CccCapabilities,
    pub aliro: AliroCapabilities,
    pub aoa_result_req_antenna_interleaving: bool,
    pub rssi_reporting: bool,
}

impl Capabilities {
    pub fn to_tlvs(&self) -> [CapTlv; 38] {
        let device_types = [bitmask_u8(self.device_types)];
        let device_roles = bitmask_u16(self.device_roles).to_le_bytes();
        let ranging_methods = bitmask_u16(self.ranging_methods).to_le_bytes();
        let sts_configs = [bitmask_u8(self.sts_configs)];
        let multi_node_modes = [bitmask_u8(self.multi_node_modes)];
        let ranging_time_struct = [bitmask_u8(self.ranging_time_structs)];
        let scheduled_modes = [bitmask_u8(self.scheduled_modes)];
        let channels = [encode_channels(self.channels)];
        let rframe_configs = [bitmask_u8(self.rframe_configs)];
        let cc_constraint_length = [self.cc_constraint_length.to_byte()];
        let bprf_parameter_sets = [self.bprf_parameter_sets.to_byte()];
        let hprf_parameter_sets = self.hprf_parameter_sets.to_bytes();
        let aoa = [self.aoa.to_byte()];
        let session_key_lengths = [self.session_key_lengths.to_byte()];

        let ccc_versions: Vec<u8> = self
            .ccc
            .versions
            .iter()
            .flat_map(|v| v.to_bytes())
            .collect();
        let ccc_ran_multiplier = self.ccc.ran_multiplier.to_le_bytes();
        let ccc_sync_codes = self.ccc.sync_codes.to_bytes();
        let aliro_mac_modes = [self.aliro.mac_modes.to_byte()];
        let aliro_protocol_version = self.aliro.protocol_version.to_bytes();

        let tlvs: [(CapTlvType, &[u8]); 38] = [
            (
                CapTlvType::SupportedMaxMessageSize,
                &self.max_message_size.to_le_bytes(),
            ),
            (
                CapTlvType::SupportedMaxDataPacketPayloadSize,
                &self.max_data_packet_payload_size.to_le_bytes(),
            ),
            (
                CapTlvType::SupportedFiraPhyVersionRange,
                &self.fira_phy_version_range.to_bytes(),
            ),
            (
                CapTlvType::SupportedFiraMacVersionRange,
                &self.fira_mac_version_range.to_bytes(),
            ),
            (CapTlvType::SupportedDeviceType, &device_types),
            (CapTlvType::SupportedDeviceRoles, &device_roles),
            (CapTlvType::SupportedRangingMethod, &ranging_methods),
            (CapTlvType::SupportedStsConfig, &sts_configs),
            (CapTlvType::SupportedMultiNodeModes, &multi_node_modes),
            (CapTlvType::SupportedRangingTimeStruct, &ranging_time_struct),
            (CapTlvType::SupportedScheduledMode, &scheduled_modes),
            (CapTlvType::SupportedHoppingMode, &[u8::from(self.hopping)]),
            (
                CapTlvType::SupportedBlockStriding,
                &[u8::from(self.block_striding)],
            ),
            (
                CapTlvType::SupportedUwbInitiationTime,
                &[u8::from(self.uwb_initiation_time)],
            ),
            (CapTlvType::SupportedChannels, &channels),
            (CapTlvType::SupportedRframeConfig, &rframe_configs),
            (
                CapTlvType::SupportedCcConstraintLength,
                &cc_constraint_length,
            ),
            (CapTlvType::SupportedBprfParameterSets, &bprf_parameter_sets),
            (CapTlvType::SupportedHprfParameterSets, &hprf_parameter_sets),
            (CapTlvType::SupportedAoa, &aoa),
            (
                CapTlvType::SupportedExtendedMacAddress,
                &[u8::from(self.extended_mac_address)],
            ),
            (
                CapTlvType::SupportedSuspendRangingSupport,
                &[u8::from(self.suspend_ranging_support)],
            ),
            (CapTlvType::SupportedSessionKeyLength, &session_key_lengths),
            (
                CapTlvType::SupportedDtAnchorMaxActiveRr,
                &[self.dt_anchor_max_active_rr],
            ),
            (
                CapTlvType::SupportedDtTagMaxActiveRr,
                &[self.dt_tag_max_active_rr],
            ),
            (
                CapTlvType::SupportedDtTagBlockSkipping,
                &[u8::from(self.dt_tag_block_skipping)],
            ),
            (
                CapTlvType::CccSupportedChapsPerSlot,
                &[self.ccc.chaps_per_slot],
            ),
            (CapTlvType::CccSupportedSyncCodes, &ccc_sync_codes),
            (
                CapTlvType::CccSupportedHoppingConfigModesAndSequences,
                &[self.ccc.hopping_config_modes_and_sequences],
            ),
            (CapTlvType::CccSupportedChannels, &[self.ccc.channels]),
            (CapTlvType::CccSupportedVersions, &ccc_versions),
            (CapTlvType::CccSupportedUwbConfigs, &[self.ccc.uwb_configs]),
            (
                CapTlvType::CccSupportedPulseShapeCombos,
                &[self.ccc.pulse_shape_combos],
            ),
            (CapTlvType::CccSupportedRanMultiplier, &ccc_ran_multiplier),
            (CapTlvType::AliroSupportedMacModes, &aliro_mac_modes),
            (
                CapTlvType::AliroSupportedProtocolVersion,
                &aliro_protocol_version,
            ),
            (
                CapTlvType::SupportedAoaResultReqAntennaInterleaving,
                &[u8::from(self.aoa_result_req_antenna_interleaving)],
            ),
            (
                CapTlvType::SupportedRssiReporting,
                &[u8::from(self.rssi_reporting)],
            ),
        ];

        tlvs.map(|(t, v)| CapTlv { t, v: v.to_vec() })
    }
}

pub const DEFAULT_CAPS_INFO: Capabilities = Capabilities {
    max_message_size: MAX_DATA_MESSAGE_SIZE as u16,
    max_data_packet_payload_size: MAX_DATA_PACKET_PAYLOAD_SIZE as u16,
    fira_phy_version_range: VersionRange::new(FIRA_VERSION_1_1, FIRA_VERSION_2_0),
    fira_mac_version_range: VersionRange::new(FIRA_VERSION_1_1, FIRA_VERSION_2_0),
    device_types: &[uci::DeviceType::Controlee, uci::DeviceType::Controller],
    device_roles: &[
        uci::DeviceRole::Responder,
        uci::DeviceRole::Initiator,
        uci::DeviceRole::DtAnchor,
        uci::DeviceRole::DtTag,
    ],
    ranging_methods: &[
        uci::RangingRoundUsage::SsTwrDeferredMode,
        uci::RangingRoundUsage::DsTwrDeferredMode,
        uci::RangingRoundUsage::SsTwrNonDeferredMode,
        uci::RangingRoundUsage::DsTwrNonDeferredMode,
        uci::RangingRoundUsage::OneWayRangingDlTdoa,
    ],
    sts_configs: &[
        uci::StsConfig::Static,
        uci::StsConfig::Dynamic,
        uci::StsConfig::DynamicForResponderSubSessionKey,
        uci::StsConfig::Provisioned,
        uci::StsConfig::ProvisionedForResponderSubSessionKey,
    ],
    multi_node_modes: &[uci::MultiNodeMode::OneToOne, uci::MultiNodeMode::OneToMany],
    ranging_time_structs: &[uci::RangingTimeStruct::BlockBasedScheduling],
    scheduled_modes: &[
        uci::ScheduleMode::ContentionBased,
        uci::ScheduleMode::TimeScheduled,
    ],
    hopping: false,
    block_striding: true,
    uwb_initiation_time: true,
    channels: &[
        uci::ChannelNumber::ChannelNumber5,
        uci::ChannelNumber::ChannelNumber6,
        uci::ChannelNumber::ChannelNumber8,
        uci::ChannelNumber::ChannelNumber9,
        uci::ChannelNumber::ChannelNumber10,
        uci::ChannelNumber::ChannelNumber12,
        uci::ChannelNumber::ChannelNumber13,
        uci::ChannelNumber::ChannelNumber14,
    ],
    rframe_configs: &[
        uci::RframeConfig::Sp0,
        uci::RframeConfig::Sp1,
        uci::RframeConfig::Sp3,
    ],
    cc_constraint_length: SupportedCcConstraintLength { k3: true, k7: true },
    bprf_parameter_sets: BprfParameterSets::range(1, 6),
    hprf_parameter_sets: HprfParameterSets::range(1, 35),
    aoa: SupportedAoa {
        azimuth_90: true,
        azimuth_180: true,
        elevation: true,
        fom: true,
    },
    extended_mac_address: true,
    suspend_ranging_support: true,
    session_key_lengths: SupportedSessionKeyLengths {
        dynamic_sts_256: true,
        provisioned_sts_256: true,
    },
    dt_anchor_max_active_rr: MAX_ACTIVE_RANGING_ROUNDS as u8,
    dt_tag_max_active_rr: MAX_ACTIVE_RANGING_ROUNDS as u8,
    dt_tag_block_skipping: true,
    ccc: CccCapabilities {
        chaps_per_slot: CCC_ALL_CHAPS_PER_SLOT,
        sync_codes: CccSyncCodes::range(1, 32),
        hopping_config_modes_and_sequences: CCC_ALL_HOPPING_CONFIGS,
        channels: CCC_ALL_CHANNELS,
        versions: &[Version::new(1, 0)],
        uwb_configs: 0,
        pulse_shape_combos: 0,
        ran_multiplier: 1,
    },
    aliro: AliroCapabilities {
        mac_modes: AliroMacModes {
            round_1: true,
            round_2: true,
        },
        protocol_version: Version::new(1, 0),
    },
    aoa_result_req_antenna_interleaving: true,
    rssi_reporting: true,
};
