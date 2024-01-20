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

//! ## Specifications
//! - [MAC] FiRa Consortium UWB MAC Technical Requirements
//! - [UCI] FiRa Consortium UWB Command Interface Generic Technical specification

use crate::packets::uci::{self, *};
use crate::{MacAddress, PicaCommand};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

pub const MAX_SESSION: usize = 255;
pub const DEFAULT_RANGING_INTERVAL: Duration = time::Duration::from_millis(200);
pub const DEFAULT_SLOT_DURATION: u16 = 2400; // RTSU unit
/// cf. [UCI] 8.3 Table 29
pub const MAX_NUMBER_OF_CONTROLEES: usize = 8;
pub const FIRA_1_1_INITIATION_TIME_SIZE: usize = 4;
pub const FIRA_2_0_INITIATION_TIME_SIZE: usize = 8;

#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq)]
pub enum DeviceType {
    /// [MAC] 5.1.2 Device utilizing the ranging features set through Control Messages
    Controlee = 0x00,
    /// [MAC] 5.1.1 Device controlling the ranging features through Control Messages
    Controller = 0x01,
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq)]
pub enum DeviceRole {
    /// [MAC] 5.1.3 Device initiating a ranging exchange with a ranging initiation message
    Initiator,
    /// [MAC] 5.1.4 Device responding to ranging initiation messages
    Responder,
}

/// cf. [UCI] 8.4 Table 29
#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum MacAddressMode {
    /// MAC address is 2 bytes and 2 bytes to be used in MAC header
    AddressMode0 = 0x00,
    /// Not Supported: MAC address is 8 bytes and 2 bytes to be used in MAC header
    AddressMode1 = 0x01,
    /// MAC address is 8 bytes and 8 bytes to be used in MAC header
    AddressMode2 = 0x02,
}

/// cf. [UCI] 8.3 Table 29
#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum ChannelNumber {
    ChannelNumber5 = 0x05,
    ChannelNumber6 = 0x06,
    ChannelNumber8 = 0x08,
    ChannelNumber9 = 0x09,
    ChannelNumber10 = 0x0a,
    ChannelNumber12 = 0x0c,
    ChannelNumber13 = 0x0d,
    ChannelNumber14 = 0x0e,
}

const DEFAULT_CHANNEL_NUMBER: ChannelNumber = ChannelNumber::ChannelNumber9;

/// cf. [UCI] 8.3 Table 29
#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum MultiNodeMode {
    /// Single device to single device
    Unicast = 0x00,
    OneToMany = 0x01,
    ManyToMany = 0x02,
}

/// cf. [UCI] 7.7
#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum UpdateMulticastListAction {
    Add = 0x00,
    Delete = 0x01,
    AddWithShortSubSessionKey = 0x02,
    AddwithExtendedSubSessionKey = 0x03,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum RangingRoundUsage {
    UlTdoa = 0x00,
    SsTwrDeferredMode = 0x01,
    DsTwrDeferredMode = 0x02,
    SsTwrNonDeferredMode = 0x03,
    DsTwrNonDeferredMode = 0x04,
    DlTdoa = 0x05,
    OwrAoaMeasurement = 0x06,
    DataTransferMode = 0x09,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum StsConfig {
    Static = 0x00,
    Dynamic = 0x01,
    DynamicForControleeIndividualKey = 0x02,
    Provisioned = 0x03,
    ProvisionedForControleeIndividualKey = 0x04,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum MacFcsType {
    MacFcsTypeCrc16 = 0x00,
    MacFcsTypeCrc32 = 0x01,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum AoaResultReq {
    NoAoaResult = 0x00,
    ReqAoaResults = 0x01,
    ReqAoaResultsAzimuthOnly = 0x02,
    ReqAoaResultsElevationOnly = 0x03,
    ReqAoaResultsInterleaved = 0x04,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum RframeConfig {
    Sp0 = 0x00,
    Sp1 = 0x01,
    Sp3 = 0x03,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum PsduDataRate {
    Rate6M81 = 0x00,
    Rate7M80 = 0x01,
    Rate27M2 = 0x02,
    Rate31M2 = 0x03,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum PreambleDuration {
    PreambleDurationT32Symbols = 0x00,
    PreambleDurationT64Symbols = 0x01,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum RangingTimeStruct {
    IntervalBasedScheduling = 0x00,
    BlockBasedScheduling = 0x01,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum PrfMode {
    PrfModeBprf = 0x00,
    PrfModeHprf = 0x01,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum SchedulingMode {
    ContentionBased = 0x00,
    TimeScheduled = 0x01,
    HybridScheduled = 0x02,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum HoppingMode {
    Disable = 0x00,
    FiraEnable = 0x01,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum StsLength {
    StsLength32 = 0x00,
    StsLength64 = 0x01,
    StsLength128 = 0x02,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum BprfPhrDataRate {
    BprfPhrDataRate850K = 0x00,
    BprfPhrDataRate6M81 = 0x01,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum SfdIdValue {
    SfdIdValue0 = 0x00,
    SfdIdValue1 = 0x01,
    SfdIdValue2 = 0x02,
    SfdIdValue3 = 0x03,
    SfdIdValue4 = 0x04,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
enum StsSegmentCountValue {
    StsSegmentCountValue0 = 0x00,
    StsSegmentCountValue1 = 0x01,
    StsSegmentCountValue2 = 0x02,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
pub enum RangeDataNtfConfig {
    Disable = 0x00,
    Enable = 0x01,
    EnableProximityLevelTrig = 0x02,
    EnableAoaLevelTrig = 0x03,
    EnableProximityAoaLevelTrig = 0x04,
    EnableProximityEdgeTrig = 0x05,
    EnableAoaEdgeTrig = 0x06,
    EnableProximityAoaEdgeTrig = 0x07,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
pub enum LinkLayerMode {
    Bypass = 0x00,
    Assigned = 0x01,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
pub enum DataRepetitionCount {
    NoRepetition = 0x00,
    Infinite = 0xFF,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, PartialEq)]
#[repr(u8)]
pub enum SessionDataTransferStatusNtfConfig {
    Disable = 0x00,
    Enable = 0x01,
}
/// cf. [UCI] 8.3 Table 29
#[derive(Clone)]
pub struct AppConfig {
    /// Copy of the valid App Configuration parameters provided by host
    raw: HashMap<AppConfigTlvType, Vec<u8>>,

    device_type: DeviceType,
    device_role: DeviceRole,
    mac_address_mode: MacAddressMode,
    pub device_mac_address: MacAddress,
    number_of_controlees: usize,
    dst_mac_addresses: Vec<MacAddress>,
    ranging_interval: time::Duration,
    slot_duration: u16,
    channel_number: ChannelNumber,
    multi_node_mode: MultiNodeMode,
    ranging_round_usage: RangingRoundUsage,
    sts_config: StsConfig,
    mac_fcs_type: MacFcsType,
    ranging_round_control: u8,
    aoa_result_req: AoaResultReq,
    rng_data_ntf: RangeDataNtfConfig,
    rng_data_ntf_proximity_near: u16,
    rng_data_ntf_proximity_far: u16,
    r_frame_config: RframeConfig,
    rssi_reporting: bool,
    preamble_code_index: u8,
    sfd_id: SfdIdValue,
    psdu_data_rate: PsduDataRate,
    preamble_duration: PreambleDuration,
    ranging_time_struct: RangingTimeStruct,
    slots_per_rr: u8,
    tx_adaptive_payload_power: bool,
    prf_mode: PrfMode,
    schedule_mode: SchedulingMode,
    key_rotation: bool,
    key_rotation_rate: u8,
    session_priority: u8,
    number_of_sts_segments: StsSegmentCountValue,
    max_rr_retry: u16,
    hopping_mode: HoppingMode,
    block_stride_length: u8,
    result_report_config: bool,
    in_band_termination_attempt_count: u8,
    bprf_phr_data_rate: BprfPhrDataRate,
    max_number_of_measurements: u8,
    sts_length: StsLength,
    uwb_initiation_time: u64,
    vendor_id: Option<Vec<u8>>,
    static_sts_iv: Option<Vec<u8>>,
    session_key: Option<Vec<u8>>,
    sub_session_key: Option<Vec<u8>>,
    sub_session_id: u32,
    link_layer_mode: LinkLayerMode,
    data_repetition_count: DataRepetitionCount,
    session_data_transfer_status_ntf_config: SessionDataTransferStatusNtfConfig,
    application_data_endpoint: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            raw: HashMap::new(),
            mac_address_mode: MacAddressMode::AddressMode0,
            device_role: DeviceRole::Responder,
            device_type: DeviceType::Controlee,
            ranging_interval: DEFAULT_RANGING_INTERVAL,
            slot_duration: DEFAULT_SLOT_DURATION,
            channel_number: DEFAULT_CHANNEL_NUMBER,
            device_mac_address: MacAddress::Short([0x00, 0x00]),
            number_of_controlees: 0,
            dst_mac_addresses: Vec::new(),
            multi_node_mode: MultiNodeMode::Unicast,
            ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
            sts_config: StsConfig::Static,
            mac_fcs_type: MacFcsType::MacFcsTypeCrc16,
            ranging_round_control: 6_u8,
            aoa_result_req: AoaResultReq::ReqAoaResults,
            rng_data_ntf: RangeDataNtfConfig::Enable,
            rng_data_ntf_proximity_near: 0,
            rng_data_ntf_proximity_far: 0,
            r_frame_config: RframeConfig::Sp3,
            rssi_reporting: false,
            preamble_code_index: 10,
            sfd_id: SfdIdValue::SfdIdValue2,
            psdu_data_rate: PsduDataRate::Rate6M81,
            preamble_duration: PreambleDuration::PreambleDurationT64Symbols,
            ranging_time_struct: RangingTimeStruct::IntervalBasedScheduling,
            slots_per_rr: 25,
            tx_adaptive_payload_power: false,
            prf_mode: PrfMode::PrfModeBprf,
            schedule_mode: SchedulingMode::TimeScheduled,
            key_rotation: false,
            key_rotation_rate: 0,
            session_priority: 50,
            number_of_sts_segments: StsSegmentCountValue::StsSegmentCountValue1,
            max_rr_retry: 0,
            hopping_mode: HoppingMode::Disable,
            block_stride_length: 0,
            result_report_config: true,
            in_band_termination_attempt_count: 1,
            bprf_phr_data_rate: BprfPhrDataRate::BprfPhrDataRate850K,
            max_number_of_measurements: 0,
            sts_length: StsLength::StsLength64,
            uwb_initiation_time: 0,
            vendor_id: None,
            static_sts_iv: None,
            session_key: None,
            sub_session_key: None,
            sub_session_id: 0,
            link_layer_mode: LinkLayerMode::Bypass,
            data_repetition_count: DataRepetitionCount::NoRepetition,
            session_data_transfer_status_ntf_config: SessionDataTransferStatusNtfConfig::Disable,
            application_data_endpoint: 0,
        }
    }
}

impl PartialEq for AppConfig {
    fn eq(&self, other: &Self) -> bool {
        self.mac_address_mode == other.mac_address_mode
            && self.ranging_interval == other.ranging_interval
            && self.slot_duration == other.slot_duration
            && self.channel_number == other.channel_number
            && self.multi_node_mode == other.multi_node_mode
            && self.ranging_round_usage == other.ranging_round_usage
            && self.sts_config == other.sts_config
            && self.mac_fcs_type == other.mac_fcs_type
            && self.ranging_round_control == other.ranging_round_control
            && self.aoa_result_req == other.aoa_result_req
            && self.rng_data_ntf == other.rng_data_ntf
            && self.rng_data_ntf_proximity_near == other.rng_data_ntf_proximity_near
            && self.rng_data_ntf_proximity_far == other.rng_data_ntf_proximity_far
            && self.r_frame_config == other.r_frame_config
            && self.rssi_reporting == other.rssi_reporting
            && self.preamble_code_index == other.preamble_code_index
            && self.sfd_id == other.sfd_id
            && self.psdu_data_rate == other.psdu_data_rate
            && self.preamble_duration == other.preamble_duration
            && self.ranging_time_struct == other.ranging_time_struct
            && self.slots_per_rr == other.slots_per_rr
            && self.tx_adaptive_payload_power == other.tx_adaptive_payload_power
            && self.prf_mode == other.prf_mode
            && self.schedule_mode == other.schedule_mode
            && self.key_rotation == other.key_rotation
            && self.key_rotation_rate == other.key_rotation_rate
            && self.session_priority == other.session_priority
            && self.number_of_sts_segments == other.number_of_sts_segments
            && self.max_rr_retry == other.max_rr_retry
            && self.result_report_config == other.result_report_config
            && self.bprf_phr_data_rate == other.bprf_phr_data_rate
            && self.max_number_of_measurements == other.max_number_of_measurements
            && self.sts_length == other.sts_length
            && self.uwb_initiation_time == other.uwb_initiation_time
            && self.vendor_id == other.vendor_id
            && self.static_sts_iv == other.static_sts_iv
    }
}

fn app_config_has_mandatory_parameters(configs: &[AppConfigTlv]) -> bool {
    const MANDATORY_PARAMETERS: [AppConfigTlvType; 6] = [
        AppConfigTlvType::DeviceRole,
        AppConfigTlvType::MultiNodeMode,
        AppConfigTlvType::NoOfControlee,
        AppConfigTlvType::DeviceMacAddress,
        AppConfigTlvType::DstMacAddress,
        AppConfigTlvType::DeviceType,
    ];

    MANDATORY_PARAMETERS
        .iter()
        .all(|&mparam| configs.iter().any(|param| mparam == param.cfg_id))
}

impl AppConfig {
    fn set_config(
        &mut self,
        id: AppConfigTlvType,
        value: &[u8],
    ) -> std::result::Result<(), StatusCode> {
        match id {
            AppConfigTlvType::MacAddressMode => {
                let mode = MacAddressMode::from_u8(value[0]).unwrap();
                if mode == MacAddressMode::AddressMode1 {
                    return Err(StatusCode::UciStatusInvalidParam);
                }
                self.mac_address_mode = mode;
            }
            AppConfigTlvType::RangingDuration => {
                let interval = u32::from_le_bytes(value[..].try_into().unwrap());
                self.ranging_interval = time::Duration::from_millis(interval as u64)
            }
            AppConfigTlvType::SlotDuration => {
                self.slot_duration = u16::from_le_bytes(value[..].try_into().unwrap())
            }
            AppConfigTlvType::ChannelNumber => {
                self.channel_number = ChannelNumber::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::DeviceMacAddress => {
                self.device_mac_address = match self.mac_address_mode {
                    MacAddressMode::AddressMode0 => {
                        MacAddress::Short(value[..].try_into().unwrap())
                    }
                    MacAddressMode::AddressMode2 => {
                        MacAddress::Extend(value[..].try_into().unwrap())
                    }
                    _ => panic!("Unexpected MAC Address Mode"),
                };
            }
            AppConfigTlvType::NoOfControlee => {
                assert!(value[0] as usize <= MAX_NUMBER_OF_CONTROLEES);
                self.number_of_controlees = value[0] as usize;
            }
            AppConfigTlvType::DstMacAddress => {
                let mac_address_size = match self.mac_address_mode {
                    MacAddressMode::AddressMode0 => 2,
                    MacAddressMode::AddressMode2 => 8,
                    _ => panic!("Unexpected MAC Address Mode"),
                };
                if value.len() % mac_address_size != 0
                    || (value.len() / mac_address_size) != self.number_of_controlees
                {
                    return Err(StatusCode::UciStatusInvalidParam);
                }
                self.dst_mac_addresses = value
                    .chunks(mac_address_size)
                    .map(|c| match self.mac_address_mode {
                        MacAddressMode::AddressMode0 => MacAddress::Short(c.try_into().unwrap()),
                        MacAddressMode::AddressMode2 => MacAddress::Extend(c.try_into().unwrap()),
                        _ => panic!("Unexpected MAC Address Mode"),
                    })
                    .collect();
                assert_eq!(self.dst_mac_addresses.len(), self.number_of_controlees);
            }
            AppConfigTlvType::MultiNodeMode => {
                self.multi_node_mode = MultiNodeMode::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::DeviceType => {
                self.device_type = DeviceType::from_u8(value[0]).unwrap();
            }
            AppConfigTlvType::RangingRoundUsage => {
                self.ranging_round_usage = RangingRoundUsage::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::StsConfig => self.sts_config = StsConfig::from_u8(value[0]).unwrap(),
            AppConfigTlvType::MacFcsType => {
                self.mac_fcs_type = MacFcsType::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::RangingRoundControl => self.ranging_round_control = value[0],
            AppConfigTlvType::AoaResultReq => {
                self.aoa_result_req = AoaResultReq::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::RngDataNtf => {
                self.rng_data_ntf = RangeDataNtfConfig::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::RngDataNtfProximityNear => {
                self.rng_data_ntf_proximity_near = u16::from_le_bytes(value[..].try_into().unwrap())
            }
            AppConfigTlvType::RngDataNtfProximityFar => {
                self.rng_data_ntf_proximity_far = u16::from_le_bytes(value[..].try_into().unwrap())
            }
            AppConfigTlvType::DeviceRole => {
                self.device_role = DeviceRole::from_u8(value[0]).unwrap();
            }
            AppConfigTlvType::RframeConfig => {
                self.r_frame_config = RframeConfig::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::RssiReporting => {
                self.rssi_reporting = match value[0] {
                    0 => false,
                    1 => true,
                    _ => panic!("Invalid rssi reporting value!"),
                }
            }
            AppConfigTlvType::PreambleCodeIndex => self.preamble_code_index = value[0],
            AppConfigTlvType::SfdId => self.sfd_id = SfdIdValue::from_u8(value[0]).unwrap(),
            AppConfigTlvType::PsduDataRate => {
                self.psdu_data_rate = PsduDataRate::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::PreambleDuration => {
                self.preamble_duration = PreambleDuration::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::RangingTimeStruct => {
                self.ranging_time_struct = RangingTimeStruct::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::SlotsPerRr => self.slots_per_rr = value[0],
            AppConfigTlvType::TxAdaptivePayloadPower => {
                self.tx_adaptive_payload_power = match value[0] {
                    0 => false,
                    1 => true,
                    _ => panic!("Invalid tx adaptive payload power value!"),
                }
            }
            AppConfigTlvType::PrfMode => self.prf_mode = PrfMode::from_u8(value[0]).unwrap(),
            AppConfigTlvType::ScheduledMode => {
                self.schedule_mode = SchedulingMode::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::KeyRotation => {
                self.key_rotation = match value[0] {
                    0 => false,
                    1 => true,
                    _ => panic!("Invalid key rotation value!"),
                }
            }
            AppConfigTlvType::KeyRotationRate => self.key_rotation_rate = value[0],
            AppConfigTlvType::SessionPriority => self.session_priority = value[0],
            AppConfigTlvType::VendorId => {
                self.vendor_id = Some(value.to_vec());
            }
            AppConfigTlvType::StaticStsIv => {
                self.static_sts_iv = Some(value.to_vec());
            }
            AppConfigTlvType::NumberOfStsSegments => {
                self.number_of_sts_segments = StsSegmentCountValue::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::MaxRrRetry => {
                self.max_rr_retry = u16::from_le_bytes(value[..].try_into().unwrap())
            }
            AppConfigTlvType::UwbInitiationTime => {
                self.uwb_initiation_time = match value.len() {
                    // Backward compatible with Fira 1.1 Version UCI host.
                    FIRA_1_1_INITIATION_TIME_SIZE => {
                        u32::from_le_bytes(value[..].try_into().unwrap()) as u64
                    }
                    FIRA_2_0_INITIATION_TIME_SIZE => {
                        u64::from_le_bytes(value[..].try_into().unwrap())
                    }
                    _ => panic!("Invalid initiation time!"),
                }
            }
            AppConfigTlvType::HoppingMode => {
                self.hopping_mode = HoppingMode::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::BlockStrideLength => self.block_stride_length = value[0],
            AppConfigTlvType::ResultReportConfig => {
                self.result_report_config = match value[0] {
                    0 => false,
                    1 => true,
                    _ => panic!("Invalid result report config value!"),
                }
            }
            AppConfigTlvType::BprfPhrDataRate => {
                self.bprf_phr_data_rate = BprfPhrDataRate::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::MaxNumberOfMeasurements => self.max_number_of_measurements = value[0],
            AppConfigTlvType::StsLength => self.sts_length = StsLength::from_u8(value[0]).unwrap(),
            AppConfigTlvType::InBandTerminationAttemptCount => {
                self.in_band_termination_attempt_count = value[0]
            }
            AppConfigTlvType::SessionKey => self.session_key = Some(value.to_vec()),
            AppConfigTlvType::SubSessionId => {
                self.sub_session_id = u32::from_le_bytes(value[..].try_into().unwrap())
            }
            AppConfigTlvType::SubsessionKey => self.sub_session_key = Some(value.to_vec()),
            AppConfigTlvType::LinkLayerMode => {
                self.link_layer_mode = LinkLayerMode::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::DataRepetitionCount => {
                self.data_repetition_count = DataRepetitionCount::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::SessionDataTransferStatusNtfConfig => {
                self.session_data_transfer_status_ntf_config =
                    SessionDataTransferStatusNtfConfig::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::ApplicationDataEndpoint => self.application_data_endpoint = value[0],
            id => {
                println!("Ignored AppConfig parameter {:?}", id);
                return Err(StatusCode::UciStatusInvalidParam);
            }
        };

        self.raw.insert(id, value.to_vec());

        Ok(())
    }

    fn get_config(&self, id: AppConfigTlvType) -> Option<Vec<u8>> {
        self.raw.get(&id).cloned()
    }

    pub fn is_compatible_for_ranging(&self, peer_config: &Self) -> bool {
        self == peer_config
            && self.device_role != peer_config.device_role
            && self.device_type != peer_config.device_type
            && peer_config
                .dst_mac_addresses
                .contains(&self.device_mac_address)
            && self
                .dst_mac_addresses
                .contains(&peer_config.device_mac_address)
    }

    fn extend(&mut self, configs: &[AppConfigTlv]) -> Vec<AppConfigStatus> {
        if !app_config_has_mandatory_parameters(configs) {
            // TODO: What shall we do in this situation?
        }

        configs
            .iter()
            .fold(Vec::new(), |mut invalid_parameters, config| {
                match self.set_config(config.cfg_id, &config.v) {
                    Ok(_) => (),
                    Err(status) => invalid_parameters.push(AppConfigStatus {
                        cfg_id: config.cfg_id,
                        status,
                    }),
                };
                invalid_parameters
            })
    }
}

enum SubSessionKey {
    None,
    Short([u8; 16]),
    Extended([u8; 32]),
}
struct Controlee {
    short_address: MacAddress,
    sub_session_id: u32,
    #[allow(dead_code)]
    session_key: SubSessionKey,
}

impl From<&uci::Controlee> for Controlee {
    fn from(value: &uci::Controlee) -> Self {
        Controlee {
            short_address: MacAddress::Short(value.short_address),
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::None,
        }
    }
}

impl From<&uci::Controlee_V2_0_16_Byte_Version> for Controlee {
    fn from(value: &uci::Controlee_V2_0_16_Byte_Version) -> Self {
        Controlee {
            short_address: MacAddress::Short(value.short_address),
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::Short(value.subsession_key),
        }
    }
}

impl From<&uci::Controlee_V2_0_32_Byte_Version> for Controlee {
    fn from(value: &uci::Controlee_V2_0_32_Byte_Version) -> Self {
        Controlee {
            short_address: MacAddress::Short(value.short_address),
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::Extended(value.subsession_key),
        }
    }
}

pub struct Session {
    /// cf. [UCI] 7.1
    pub state: SessionState,
    /// cf. [UCI] 7.2 Table 13: 4 octets unique random number generated by application
    id: u32,
    device_handle: usize,

    session_type: SessionType,
    pub sequence_number: u32,
    pub app_config: AppConfig,
    ranging_task: Option<JoinHandle<()>>,
    tx: mpsc::Sender<ControlPacket>,
    pica_tx: mpsc::Sender<PicaCommand>,
}

impl Session {
    pub fn new(
        id: u32,
        session_type: SessionType,
        device_handle: usize,
        tx: mpsc::Sender<ControlPacket>,
        pica_tx: mpsc::Sender<PicaCommand>,
    ) -> Self {
        Self {
            state: SessionState::SessionStateDeinit,
            id,
            device_handle,
            session_type,
            sequence_number: 0,
            app_config: AppConfig::default(),
            ranging_task: None,
            tx,
            pica_tx,
        }
    }

    pub fn set_state(&mut self, session_state: SessionState, reason_code: ReasonCode) {
        // No transition: ignore
        if session_state == self.state {
            return;
        }

        // Send status notification
        self.state = session_state;
        let tx = self.tx.clone();
        let session_id = self.id;
        tokio::spawn(async move {
            time::sleep(Duration::from_millis(1)).await;
            tx.send(
                SessionStatusNtfBuilder {
                    session_token: session_id,
                    session_state,
                    reason_code: reason_code.into(),
                }
                .build()
                .into(),
            )
            .await
            .unwrap()
        });
    }

    pub fn get_dst_mac_addresses(&self) -> &Vec<MacAddress> {
        &self.app_config.dst_mac_addresses
    }

    pub fn is_ranging_data_ntf_enabled(&self) -> RangeDataNtfConfig {
        self.app_config.rng_data_ntf
    }

    pub fn session_state(&self) -> SessionState {
        self.state
    }

    pub fn init(&mut self) {
        self.set_state(
            SessionState::SessionStateInit,
            ReasonCode::StateChangeWithSessionManagementCommands,
        );
    }

    fn command_set_app_config(&mut self, cmd: SessionSetAppConfigCmd) -> SessionSetAppConfigRsp {
        // TODO properly handle these asserts
        println!(
            "[{}:0x{:x}] Session Set App Config",
            self.device_handle, self.id
        );
        assert_eq!(self.id, cmd.get_session_token());
        assert!(
            self.session_type.eq(&SessionType::FiraRangingSession)
                || self
                    .session_type
                    .eq(&SessionType::FiraRangingAndInBandDataSession)
        );

        if self.state == SessionState::SessionStateActive {
            const IMMUTABLE_PARAMETERS: &[AppConfigTlvType] = &[AppConfigTlvType::AoaResultReq];
            if cmd
                .get_tlvs()
                .iter()
                .any(|cfg| IMMUTABLE_PARAMETERS.contains(&cfg.cfg_id))
            {
                return SessionSetAppConfigRspBuilder {
                    status: StatusCode::UciStatusSessionActive,
                    cfg_status: vec![],
                }
                .build();
            }
        }

        let (status, invalid_parameters) = if self.state != SessionState::SessionStateInit
            && self.state != SessionState::SessionStateActive
        {
            (StatusCode::UciStatusRejected, Vec::new())
        } else {
            let mut app_config = self.app_config.clone();
            let invalid_parameters = app_config.extend(cmd.get_tlvs());
            if invalid_parameters.is_empty() {
                self.app_config = app_config;
                if self.state == SessionState::SessionStateInit {
                    self.set_state(
                        SessionState::SessionStateIdle,
                        ReasonCode::StateChangeWithSessionManagementCommands,
                    );
                }
                (StatusCode::UciStatusOk, invalid_parameters)
            } else {
                (StatusCode::UciStatusInvalidParam, invalid_parameters)
            }
        };

        SessionSetAppConfigRspBuilder {
            status,
            cfg_status: invalid_parameters,
        }
        .build()
    }

    fn command_get_app_config(&self, cmd: SessionGetAppConfigCmd) -> SessionGetAppConfigRsp {
        println!(
            "[{}:0x{:x}] Session Get App Config",
            self.device_handle, self.id
        );
        assert_eq!(self.id, cmd.get_session_token());

        let (status, valid_parameters) = {
            let (valid_parameters, invalid_parameters) = cmd.get_app_cfg().iter().fold(
                (Vec::new(), Vec::new()),
                |(mut valid_parameters, mut invalid_parameters), config_id| {
                    match AppConfigTlvType::try_from(*config_id) {
                        Ok(id) => match self.app_config.get_config(id) {
                            Some(value) => valid_parameters.push(AppConfigTlv {
                                cfg_id: id,
                                v: value,
                            }),
                            None => invalid_parameters.push(AppConfigTlv {
                                cfg_id: id,
                                v: Vec::new(),
                            }),
                        },
                        Err(_) => println!("Failed to parse AppConfigTlv: {:?}", *config_id),
                    }
                    (valid_parameters, invalid_parameters)
                },
            );
            if invalid_parameters.is_empty() {
                (StatusCode::UciStatusOk, valid_parameters)
            } else {
                (StatusCode::UciStatusFailed, Vec::new())
            }
        };
        SessionGetAppConfigRspBuilder {
            status,
            tlvs: valid_parameters,
        }
        .build()
    }

    fn command_get_state(&self, cmd: SessionGetStateCmd) -> SessionGetStateRsp {
        println!("[{}:0x{:x}] Session Get State", self.device_handle, self.id);
        assert_eq!(self.id, cmd.get_session_token());
        SessionGetStateRspBuilder {
            status: StatusCode::UciStatusOk,
            session_state: self.state,
        }
        .build()
    }

    fn command_update_controller_multicast_list(
        &mut self,
        cmd: SessionUpdateControllerMulticastListCmd,
    ) -> SessionUpdateControllerMulticastListRsp {
        println!(
            "[{}:0x{:x}] Session Update Controller Multicast List",
            self.device_handle, self.id
        );
        assert_eq!(self.id, cmd.get_session_token());
        if (self.state != SessionState::SessionStateActive
            && self.state != SessionState::SessionStateIdle)
            || self.app_config.device_type != DeviceType::Controller
            || (self.app_config.multi_node_mode != MultiNodeMode::OneToMany
                && self.app_config.multi_node_mode != MultiNodeMode::ManyToMany)
        {
            return SessionUpdateControllerMulticastListRspBuilder {
                status: StatusCode::UciStatusRejected,
            }
            .build();
        }
        let action = UpdateMulticastListAction::from_u8(cmd.get_action().into()).unwrap();
        let mut dst_addresses = self.app_config.dst_mac_addresses.clone();
        let new_controlees: Vec<Controlee> = match action {
            UpdateMulticastListAction::Add | UpdateMulticastListAction::Delete => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmdPayload::parse(cmd.get_payload())
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRspBuilder {
                        status: StatusCode::UciStatusSyntaxError,
                    }
                    .build();
                }
            }
            UpdateMulticastListAction::AddWithShortSubSessionKey => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload::parse(
                        cmd.get_payload(),
                    )
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRspBuilder {
                        status: StatusCode::UciStatusSyntaxError,
                    }
                    .build();
                }
            }
            UpdateMulticastListAction::AddwithExtendedSubSessionKey => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload::parse(
                        cmd.get_payload(),
                    )
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRspBuilder {
                        status: StatusCode::UciStatusSyntaxError,
                    }
                    .build();
                }
            }
        };
        let mut controlee_status = Vec::new();

        let session_id = self.id;
        let mut status = StatusCode::UciStatusOk;

        match action {
            UpdateMulticastListAction::Add
            | UpdateMulticastListAction::AddWithShortSubSessionKey
            | UpdateMulticastListAction::AddwithExtendedSubSessionKey => {
                new_controlees.iter().for_each(|controlee| {
                    let mut update_status = MulticastUpdateStatusCode::StatusOkMulticastListUpdate;
                    if !dst_addresses.contains(&controlee.short_address) {
                        if dst_addresses.len() == MAX_NUMBER_OF_CONTROLEES {
                            status = StatusCode::UciStatusMulticastListFull;
                            update_status = MulticastUpdateStatusCode::StatusErrorMulticastListFull;
                        } else if (action == UpdateMulticastListAction::AddWithShortSubSessionKey
                            || action == UpdateMulticastListAction::AddwithExtendedSubSessionKey)
                            && self.app_config.sts_config
                                != StsConfig::ProvisionedForControleeIndividualKey
                        {
                            // If Action is 0x02 or 0x03 for STS_CONFIG values other than
                            // 0x04, the UWBS shall return SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF
                            // with Status set to STATUS_ERROR_SUB_SESSION_KEY_NOT_APPLICABLE for each
                            // Controlee in the Controlee List.
                            status = StatusCode::UciStatusFailed;
                            update_status =
                                MulticastUpdateStatusCode::StatusErrorSubSessionKeyNotApplicable;
                        } else {
                            dst_addresses.push(controlee.short_address);
                        };
                    }
                    controlee_status.push(ControleeStatus {
                        mac_address: match controlee.short_address {
                            MacAddress::Short(address) => address,
                            MacAddress::Extend(_) => panic!("Extended address is not supported!"),
                        },
                        subsession_id: controlee.sub_session_id,
                        status: update_status,
                    });
                });
            }
            UpdateMulticastListAction::Delete => {
                new_controlees.iter().for_each(|controlee: &Controlee| {
                    let pica_tx = self.pica_tx.clone();
                    let address = controlee.short_address;
                    let attempt_count = self.app_config.in_band_termination_attempt_count;
                    let mut update_status = MulticastUpdateStatusCode::StatusOkMulticastListUpdate;
                    if !dst_addresses.contains(&address) {
                        status = StatusCode::UciStatusAddressNotFound;
                        update_status = MulticastUpdateStatusCode::StatusErrorKeyFetchFail;
                    } else {
                        dst_addresses.retain(|value| *value != address);
                        // If IN_BAND_TERMINATION_ATTEMPT_COUNT is not equal to 0x00, then the
                        // UWBS shall transmit the RCM with the “Stop Ranging” bit set to ‘1’
                        // for IN_BAND_TERMINATION_ATTEMPT_COUNT times to the corresponding
                        // Controlee.
                        if attempt_count != 0 {
                            tokio::spawn(async move {
                                for _ in 0..attempt_count {
                                    pica_tx
                                        .send(PicaCommand::StopRanging(address, session_id))
                                        .await
                                        .unwrap()
                                }
                            });
                        }
                    }
                    controlee_status.push(ControleeStatus {
                        mac_address: match address {
                            MacAddress::Short(addr) => addr,
                            MacAddress::Extend(_) => panic!("Extended address is not supported!"),
                        },
                        subsession_id: controlee.sub_session_id,
                        status: update_status,
                    });
                });
            }
        }
        self.app_config.number_of_controlees = dst_addresses.len();
        self.app_config.dst_mac_addresses = dst_addresses.clone();
        // If the multicast list becomes empty, the UWBS shall move the session to
        // SESSION_STATE_IDLE by sending the SESSION_STATUS_NTF with Reason Code
        // set to ERROR_INVALID_NUM_OF_CONTROLEES.
        if self.app_config.dst_mac_addresses.is_empty() {
            self.set_state(
                SessionState::SessionStateIdle,
                ReasonCode::ErrorInvalidNumOfControlees,
            )
        }
        let tx = self.tx.clone();
        tokio::spawn(async move {
            tx.send(
                SessionUpdateControllerMulticastListNtfBuilder {
                    controlee_status,
                    remaining_multicast_list_size: dst_addresses.len() as u8,
                    session_token: session_id,
                }
                .build()
                .into(),
            )
            .await
            .unwrap()
        });
        SessionUpdateControllerMulticastListRspBuilder { status }.build()
    }

    fn command_range_start(&mut self, cmd: SessionStartCmd) -> SessionStartRsp {
        println!("[{}:0x{:x}] Range Start", self.device_handle, self.id);
        assert_eq!(self.id, cmd.get_session_id());

        let status = if self.state != SessionState::SessionStateIdle {
            StatusCode::UciStatusSessionNotConfigured
        } else {
            assert!(self.ranging_task.is_none());
            assert_eq!(self.state, SessionState::SessionStateIdle);

            let session_id = self.id;
            let ranging_interval = self.app_config.ranging_interval;
            let device_handle = self.device_handle;
            let tx = self.pica_tx.clone();
            self.ranging_task = Some(tokio::spawn(async move {
                loop {
                    time::sleep(ranging_interval).await;
                    tx.send(PicaCommand::Ranging(device_handle, session_id))
                        .await
                        .unwrap();
                }
            }));
            self.set_state(
                SessionState::SessionStateActive,
                ReasonCode::StateChangeWithSessionManagementCommands,
            );
            StatusCode::UciStatusOk
        };
        SessionStartRspBuilder { status }.build()
    }

    pub fn stop_ranging_task(&mut self) {
        if let Some(handle) = &self.ranging_task {
            handle.abort();
            self.ranging_task = None;
        }
    }
    fn command_range_stop(&mut self, cmd: SessionStopCmd) -> SessionStopRsp {
        println!("[{}:0x{:x}] Range Stop", self.device_handle, self.id);
        assert_eq!(self.id, cmd.get_session_id());

        let status = if self.state != SessionState::SessionStateActive {
            StatusCode::UciStatusSessionActive
        } else {
            self.stop_ranging_task();
            self.set_state(
                SessionState::SessionStateIdle,
                ReasonCode::StateChangeWithSessionManagementCommands,
            );
            StatusCode::UciStatusOk
        };
        SessionStopRspBuilder { status }.build()
    }

    fn command_get_ranging_count(
        &self,
        cmd: SessionGetRangingCountCmd,
    ) -> SessionGetRangingCountRsp {
        println!(
            "[{}:0x{:x}] Range Get Ranging Count",
            self.device_handle, self.id
        );
        assert_eq!(self.id, cmd.get_session_id());

        SessionGetRangingCountRspBuilder {
            status: StatusCode::UciStatusOk,
            count: self.sequence_number,
        }
        .build()
    }

    pub fn session_command(&mut self, cmd: SessionConfigCommand) -> SessionConfigResponse {
        match cmd.specialize() {
            SessionConfigCommandChild::SessionSetAppConfigCmd(cmd) => {
                self.command_set_app_config(cmd).into()
            }
            SessionConfigCommandChild::SessionGetAppConfigCmd(cmd) => {
                self.command_get_app_config(cmd).into()
            }
            SessionConfigCommandChild::SessionGetStateCmd(cmd) => {
                self.command_get_state(cmd).into()
            }
            SessionConfigCommandChild::SessionUpdateControllerMulticastListCmd(cmd) => {
                self.command_update_controller_multicast_list(cmd).into()
            }
            _ => panic!("Unsupported session command"),
        }
    }

    pub fn ranging_command(&mut self, cmd: SessionControlCommand) -> SessionControlResponse {
        match cmd.specialize() {
            SessionControlCommandChild::SessionStartCmd(cmd) => {
                self.command_range_start(cmd).into()
            }
            SessionControlCommandChild::SessionStopCmd(cmd) => self.command_range_stop(cmd).into(),
            SessionControlCommandChild::SessionGetRangingCountCmd(cmd) => {
                self.command_get_ranging_count(cmd).into()
            }
            _ => panic!("Unsupported ranging command"),
        }
    }

    pub fn data_message_snd(&mut self, data: DataMessageSnd) -> SessionControlNotification {
        let session_token = data.get_session_handle();
        let uci_sequence_number = data.get_data_sequence_number() as u8;

        if self.session_type != SessionType::FiraRangingAndInBandDataSession {
            return DataTransferStatusNtfBuilder {
                session_token,
                status: DataTransferNtfStatusCode::UciDataTransferStatusSessionTypeNotSupported,
                tx_count: 1, // TODO: support for retries?
                uci_sequence_number,
            }
            .build()
            .into();
        }

        assert_eq!(self.id, session_token);

        // TODO: perform actual data transfer across devices
        println!(
            "Data packet received, payload bytes: {:?}",
            data.get_application_data()
        );

        DataCreditNtfBuilder {
            credit_availability: CreditAvailability::CreditAvailable,
            session_token,
        }
        .build()
        .into()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Make sure to abort the ranging task when dropping the session,
        // the default behaviour when dropping a task handle is to detach
        // the task, which is undesirable.
        self.stop_ranging_task();
        self.set_state(
            SessionState::SessionStateDeinit,
            ReasonCode::StateChangeWithSessionManagementCommands,
        );
    }
}
