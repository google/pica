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
// @generated rust packets from uci_packets.pdl

#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(missing_docs)]

use bytes::{BufMut, Bytes, BytesMut};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Packet parsing failed")]
    InvalidPacketError,
    #[error("{field} was {value:x}, which is not known")]
    ConstraintOutOfBounds { field: String, value: u64 },
    #[error("when parsing {obj}.{field} needed length of {wanted} but got {got}")]
    InvalidLengthError {
        obj: String,
        field: String,
        wanted: usize,
        got: usize,
    },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError {
        obj: String,
        field: String,
        value: u64,
        type_: String,
    },
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct TryFromError(&'static str);

pub trait Packet {
    fn to_bytes(self) -> Bytes;
    fn to_vec(self) -> Vec<u8>;
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum PacketBoundaryFlag {
    Complete = 0x0,
    NotComplete = 0x1,
}
impl fmt::Display for PacketBoundaryFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketBoundaryFlag::Complete => write!(f, "{:#04X} (COMPLETE)", self.to_u8().unwrap()),
            PacketBoundaryFlag::NotComplete => {
                write!(f, "{:#04X} (NOT_COMPLETE)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum GroupId {
    Core = 0x0,
    SessionConfig = 0x1,
    RangingSessionControl = 0x2,
    DataControl = 0x3,
    VendorReservedA = 0xa,
    VendorReservedB = 0xb,
    VendorAndroid = 0xc,
    Test = 0xd,
    VendorReservedE = 0xe,
    VendorReservedF = 0xf,
}
impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupId::Core => write!(f, "{:#04X} (CORE)", self.to_u8().unwrap()),
            GroupId::SessionConfig => write!(f, "{:#04X} (SESSION_CONFIG)", self.to_u8().unwrap()),
            GroupId::RangingSessionControl => write!(
                f,
                "{:#04X} (RANGING_SESSION_CONTROL)",
                self.to_u8().unwrap()
            ),
            GroupId::DataControl => write!(f, "{:#04X} (DATA_CONTROL)", self.to_u8().unwrap()),
            GroupId::VendorReservedA => {
                write!(f, "{:#04X} (VENDOR_RESERVED_A)", self.to_u8().unwrap())
            }
            GroupId::VendorReservedB => {
                write!(f, "{:#04X} (VENDOR_RESERVED_B)", self.to_u8().unwrap())
            }
            GroupId::VendorAndroid => write!(f, "{:#04X} (VENDOR_ANDROID)", self.to_u8().unwrap()),
            GroupId::Test => write!(f, "{:#04X} (TEST)", self.to_u8().unwrap()),
            GroupId::VendorReservedE => {
                write!(f, "{:#04X} (VENDOR_RESERVED_E)", self.to_u8().unwrap())
            }
            GroupId::VendorReservedF => {
                write!(f, "{:#04X} (VENDOR_RESERVED_F)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum CoreOpCode {
    CoreDeviceReset = 0x0,
    CoreDeviceStatusNtf = 0x1,
    CoreGetDeviceInfo = 0x2,
    CoreGetCapsInfo = 0x3,
    CoreSetConfig = 0x4,
    CoreGetConfig = 0x5,
    CoreDeviceSuspend = 0x6,
    CoreGenericErrorNtf = 0x7,
}
impl fmt::Display for CoreOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoreOpCode::CoreDeviceReset => {
                write!(f, "{:#04X} (CORE_DEVICE_RESET)", self.to_u8().unwrap())
            }
            CoreOpCode::CoreDeviceStatusNtf => {
                write!(f, "{:#04X} (CORE_DEVICE_STATUS_NTF)", self.to_u8().unwrap())
            }
            CoreOpCode::CoreGetDeviceInfo => {
                write!(f, "{:#04X} (CORE_GET_DEVICE_INFO)", self.to_u8().unwrap())
            }
            CoreOpCode::CoreGetCapsInfo => {
                write!(f, "{:#04X} (CORE_GET_CAPS_INFO)", self.to_u8().unwrap())
            }
            CoreOpCode::CoreSetConfig => {
                write!(f, "{:#04X} (CORE_SET_CONFIG)", self.to_u8().unwrap())
            }
            CoreOpCode::CoreGetConfig => {
                write!(f, "{:#04X} (CORE_GET_CONFIG)", self.to_u8().unwrap())
            }
            CoreOpCode::CoreDeviceSuspend => {
                write!(f, "{:#04X} (CORE_DEVICE_SUSPEND)", self.to_u8().unwrap())
            }
            CoreOpCode::CoreGenericErrorNtf => {
                write!(f, "{:#04X} (CORE_GENERIC_ERROR_NTF)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum SessionOpCode {
    SessionInit = 0x0,
    SessionDeinit = 0x1,
    SessionStatusNtf = 0x2,
    SessionSetAppConfig = 0x3,
    SessionGetAppConfig = 0x4,
    SessionGetCount = 0x5,
    SessionGetState = 0x6,
    SessionUpdateControllerMulticastList = 0x7,
}
impl fmt::Display for SessionOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SessionOpCode::SessionInit => {
                write!(f, "{:#04X} (SESSION_INIT)", self.to_u8().unwrap())
            }
            SessionOpCode::SessionDeinit => {
                write!(f, "{:#04X} (SESSION_DEINIT)", self.to_u8().unwrap())
            }
            SessionOpCode::SessionStatusNtf => {
                write!(f, "{:#04X} (SESSION_STATUS_NTF)", self.to_u8().unwrap())
            }
            SessionOpCode::SessionSetAppConfig => {
                write!(f, "{:#04X} (SESSION_SET_APP_CONFIG)", self.to_u8().unwrap())
            }
            SessionOpCode::SessionGetAppConfig => {
                write!(f, "{:#04X} (SESSION_GET_APP_CONFIG)", self.to_u8().unwrap())
            }
            SessionOpCode::SessionGetCount => {
                write!(f, "{:#04X} (SESSION_GET_COUNT)", self.to_u8().unwrap())
            }
            SessionOpCode::SessionGetState => {
                write!(f, "{:#04X} (SESSION_GET_STATE)", self.to_u8().unwrap())
            }
            SessionOpCode::SessionUpdateControllerMulticastList => write!(
                f,
                "{:#04X} (SESSION_UPDATE_CONTROLLER_MULTICAST_LIST)",
                self.to_u8().unwrap()
            ),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum RangeOpCode {
    RangeStart = 0x0,
    RangeStop = 0x1,
    RangeIntervalUpdateReq = 0x2,
    RangeGetRangingCount = 0x3,
}
impl fmt::Display for RangeOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RangeOpCode::RangeStart => write!(f, "{:#04X} (RANGE_START)", self.to_u8().unwrap()),
            RangeOpCode::RangeStop => write!(f, "{:#04X} (RANGE_STOP)", self.to_u8().unwrap()),
            RangeOpCode::RangeIntervalUpdateReq => write!(
                f,
                "{:#04X} (RANGE_INTERVAL_UPDATE_REQ)",
                self.to_u8().unwrap()
            ),
            RangeOpCode::RangeGetRangingCount => write!(
                f,
                "{:#04X} (RANGE_GET_RANGING_COUNT)",
                self.to_u8().unwrap()
            ),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum AppDataOpCode {
    AppDataTx = 0x0,
    AppDataRx = 0x1,
}
impl fmt::Display for AppDataOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppDataOpCode::AppDataTx => write!(f, "{:#04X} (APP_DATA_TX)", self.to_u8().unwrap()),
            AppDataOpCode::AppDataRx => write!(f, "{:#04X} (APP_DATA_RX)", self.to_u8().unwrap()),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum PicaOpCode {
    PicaInitDevice = 0x0,
    PicaSetDevicePosition = 0x1,
    PicaCreateBeacon = 0x2,
    PicaSetBeaconPosition = 0x3,
    PicaDestroyBeacon = 0x4,
}
impl fmt::Display for PicaOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PicaOpCode::PicaInitDevice => {
                write!(f, "{:#04X} (PICA_INIT_DEVICE)", self.to_u8().unwrap())
            }
            PicaOpCode::PicaSetDevicePosition => write!(
                f,
                "{:#04X} (PICA_SET_DEVICE_POSITION)",
                self.to_u8().unwrap()
            ),
            PicaOpCode::PicaCreateBeacon => {
                write!(f, "{:#04X} (PICA_CREATE_BEACON)", self.to_u8().unwrap())
            }
            PicaOpCode::PicaSetBeaconPosition => write!(
                f,
                "{:#04X} (PICA_SET_BEACON_POSITION)",
                self.to_u8().unwrap()
            ),
            PicaOpCode::PicaDestroyBeacon => {
                write!(f, "{:#04X} (PICA_DESTROY_BEACON)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum AndroidOpCode {
    AndroidGetPowerStats = 0x0,
    AndroidSetCountryCode = 0x1,
}
impl fmt::Display for AndroidOpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AndroidOpCode::AndroidGetPowerStats => write!(
                f,
                "{:#04X} (ANDROID_GET_POWER_STATS)",
                self.to_u8().unwrap()
            ),
            AndroidOpCode::AndroidSetCountryCode => write!(
                f,
                "{:#04X} (ANDROID_SET_COUNTRY_CODE)",
                self.to_u8().unwrap()
            ),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum StatusCode {
    UciStatusOk = 0x0,
    UciStatusRejected = 0x1,
    UciStatusFailed = 0x2,
    UciStatusSyntaxError = 0x3,
    UciStatusInvalidParam = 0x4,
    UciStatusInvalidRange = 0x5,
    UciStatusInvalidMsgSize = 0x6,
    UciStatusUnknownGid = 0x7,
    UciStatusUnknownOid = 0x8,
    UciStatusReadOnly = 0x9,
    UciStatusCommandRetry = 0xa,
    UciStatusSessionNotExist = 0x11,
    UciStatusSessionDuplicate = 0x12,
    UciStatusSessionActive = 0x13,
    UciStatusMaxSessionsExceeded = 0x14,
    UciStatusSessionNotConfigured = 0x15,
    UciStatusActiveSessionOngoing = 0x16,
    UciStatusMulticastListFull = 0x17,
    UciStatusAddressNotFound = 0x18,
    UciStatusAddressAlreadyPresent = 0x19,
    UciStatusRangingTxFailed = 0x20,
    UciStatusRangingRxTimeout = 0x21,
    UciStatusRangingRxPhyDecFailed = 0x22,
    UciStatusRangingRxPhyToaFailed = 0x23,
    UciStatusRangingRxPhyStsFailed = 0x24,
    UciStatusRangingRxMacDecFailed = 0x25,
    UciStatusRangingRxMacIeDecFailed = 0x26,
    UciStatusRangingRxMacIeMissing = 0x27,
    UciStatusDataMaxTxPsduSizeExceeded = 0x30,
    UciStatusDataRxCrcError = 0x31,
    UciStatusErrorCccSeBusy = 0x50,
    UciStatusErrorCccLifecycle = 0x51,
}
impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusCode::UciStatusOk => write!(f, "{:#04X} (UCI_STATUS_OK)", self.to_u8().unwrap()),
            StatusCode::UciStatusRejected => {
                write!(f, "{:#04X} (UCI_STATUS_REJECTED)", self.to_u8().unwrap())
            }
            StatusCode::UciStatusFailed => {
                write!(f, "{:#04X} (UCI_STATUS_FAILED)", self.to_u8().unwrap())
            }
            StatusCode::UciStatusSyntaxError => write!(
                f,
                "{:#04X} (UCI_STATUS_SYNTAX_ERROR)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusInvalidParam => write!(
                f,
                "{:#04X} (UCI_STATUS_INVALID_PARAM)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusInvalidRange => write!(
                f,
                "{:#04X} (UCI_STATUS_INVALID_RANGE)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusInvalidMsgSize => write!(
                f,
                "{:#04X} (UCI_STATUS_INVALID_MSG_SIZE)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusUnknownGid => {
                write!(f, "{:#04X} (UCI_STATUS_UNKNOWN_GID)", self.to_u8().unwrap())
            }
            StatusCode::UciStatusUnknownOid => {
                write!(f, "{:#04X} (UCI_STATUS_UNKNOWN_OID)", self.to_u8().unwrap())
            }
            StatusCode::UciStatusReadOnly => {
                write!(f, "{:#04X} (UCI_STATUS_READ_ONLY)", self.to_u8().unwrap())
            }
            StatusCode::UciStatusCommandRetry => write!(
                f,
                "{:#04X} (UCI_STATUS_COMMAND_RETRY)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusSessionNotExist => write!(
                f,
                "{:#04X} (UCI_STATUS_SESSION_NOT_EXIST)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusSessionDuplicate => write!(
                f,
                "{:#04X} (UCI_STATUS_SESSION_DUPLICATE)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusSessionActive => write!(
                f,
                "{:#04X} (UCI_STATUS_SESSION_ACTIVE)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusMaxSessionsExceeded => write!(
                f,
                "{:#04X} (UCI_STATUS_MAX_SESSIONS_EXCEEDED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusSessionNotConfigured => write!(
                f,
                "{:#04X} (UCI_STATUS_SESSION_NOT_CONFIGURED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusActiveSessionOngoing => write!(
                f,
                "{:#04X} (UCI_STATUS_ACTIVE_SESSION_ONGOING)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusMulticastListFull => write!(
                f,
                "{:#04X} (UCI_STATUS_MULTICAST_LIST_FULL)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusAddressNotFound => write!(
                f,
                "{:#04X} (UCI_STATUS_ADDRESS_NOT_FOUND)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusAddressAlreadyPresent => write!(
                f,
                "{:#04X} (UCI_STATUS_ADDRESS_ALREADY_PRESENT)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusRangingTxFailed => write!(
                f,
                "{:#04X} (UCI_STATUS_RANGING_TX_FAILED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusRangingRxTimeout => write!(
                f,
                "{:#04X} (UCI_STATUS_RANGING_RX_TIMEOUT)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusRangingRxPhyDecFailed => write!(
                f,
                "{:#04X} (UCI_STATUS_RANGING_RX_PHY_DEC_FAILED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusRangingRxPhyToaFailed => write!(
                f,
                "{:#04X} (UCI_STATUS_RANGING_RX_PHY_TOA_FAILED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusRangingRxPhyStsFailed => write!(
                f,
                "{:#04X} (UCI_STATUS_RANGING_RX_PHY_STS_FAILED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusRangingRxMacDecFailed => write!(
                f,
                "{:#04X} (UCI_STATUS_RANGING_RX_MAC_DEC_FAILED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusRangingRxMacIeDecFailed => write!(
                f,
                "{:#04X} (UCI_STATUS_RANGING_RX_MAC_IE_DEC_FAILED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusRangingRxMacIeMissing => write!(
                f,
                "{:#04X} (UCI_STATUS_RANGING_RX_MAC_IE_MISSING)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusDataMaxTxPsduSizeExceeded => write!(
                f,
                "{:#04X} (UCI_STATUS_DATA_MAX_TX_PSDU_SIZE_EXCEEDED)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusDataRxCrcError => write!(
                f,
                "{:#04X} (UCI_STATUS_DATA_RX_CRC_ERROR)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusErrorCccSeBusy => write!(
                f,
                "{:#04X} (UCI_STATUS_ERROR_CCC_SE_BUSY)",
                self.to_u8().unwrap()
            ),
            StatusCode::UciStatusErrorCccLifecycle => write!(
                f,
                "{:#04X} (UCI_STATUS_ERROR_CCC_LIFECYCLE)",
                self.to_u8().unwrap()
            ),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum ResetConfig {
    UwbsReset = 0x0,
}
impl fmt::Display for ResetConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResetConfig::UwbsReset => write!(f, "{:#04X} (UWBS_RESET)", self.to_u8().unwrap()),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum DeviceConfigId {
    DeviceState = 0x0,
    LowPowerMode = 0x1,
}
impl fmt::Display for DeviceConfigId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceConfigId::DeviceState => {
                write!(f, "{:#04X} (DEVICE_STATE)", self.to_u8().unwrap())
            }
            DeviceConfigId::LowPowerMode => {
                write!(f, "{:#04X} (LOW_POWER_MODE)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum AppConfigTlvType {
    DeviceType = 0x0,
    RangingRoundUsage = 0x1,
    StsConfig = 0x2,
    MultiNodeMode = 0x3,
    ChannelNumber = 0x4,
    NoOfControlee = 0x5,
    DeviceMacAddress = 0x6,
    DstMacAddress = 0x7,
    SlotDuration = 0x8,
    RangingInterval = 0x9,
    StsIndex = 0xa,
    MacFcsType = 0xb,
    RangingRoundControl = 0xc,
    AoaResultReq = 0xd,
    RngDataNtf = 0xe,
    RngDataNtfProximityNear = 0xf,
    RngDataNtfProximityFar = 0x10,
    DeviceRole = 0x11,
    RframeConfig = 0x12,
    PreambleCodeIndex = 0x14,
    SfdId = 0x15,
    PsduDataRate = 0x16,
    PreambleDuration = 0x17,
    RangingTimeStruct = 0x1a,
    SlotsPerRr = 0x1b,
    TxAdaptivePayloadPower = 0x1c,
    ResponderSlotIndex = 0x1e,
    PrfMode = 0x1f,
    ScheduledMode = 0x22,
    KeyRotation = 0x23,
    KeyRotationRate = 0x24,
    SessionPriority = 0x25,
    MacAddressMode = 0x26,
    VendorId = 0x27,
    StaticStsIv = 0x28,
    NumberOfStsSegments = 0x29,
    MaxRrRetry = 0x2a,
    UwbInitiationTime = 0x2b,
    HoppingMode = 0x2c,
    BlockStrideLength = 0x2d,
    ResultReportConfig = 0x2e,
    InBandTerminationAttemptCount = 0x2f,
    SubSessionId = 0x30,
    BprfPhrDataRate = 0x31,
    MaxNumberOfMeasurements = 0x32,
    StsLength = 0x35,
    CccHopModeKey = 0xa0,
    CccUwbTime0 = 0xa1,
    CccRangingProtocolVer = 0xa3,
    CccUwbConfigId = 0xa4,
    CccPulseshapeCombo = 0xa5,
    CccUrskTtl = 0xa6,
    NbOfRangeMeasurements = 0xe3,
    NbOfAzimuthMeasurements = 0xe4,
    NbOfElevationMeasurements = 0xe5,
}
impl fmt::Display for AppConfigTlvType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppConfigTlvType::DeviceType => {
                write!(f, "{:#04X} (DEVICE_TYPE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::RangingRoundUsage => {
                write!(f, "{:#04X} (RANGING_ROUND_USAGE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::StsConfig => write!(f, "{:#04X} (STS_CONFIG)", self.to_u8().unwrap()),
            AppConfigTlvType::MultiNodeMode => {
                write!(f, "{:#04X} (MULTI_NODE_MODE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::ChannelNumber => {
                write!(f, "{:#04X} (CHANNEL_NUMBER)", self.to_u8().unwrap())
            }
            AppConfigTlvType::NoOfControlee => {
                write!(f, "{:#04X} (NO_OF_CONTROLEE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::DeviceMacAddress => {
                write!(f, "{:#04X} (DEVICE_MAC_ADDRESS)", self.to_u8().unwrap())
            }
            AppConfigTlvType::DstMacAddress => {
                write!(f, "{:#04X} (DST_MAC_ADDRESS)", self.to_u8().unwrap())
            }
            AppConfigTlvType::SlotDuration => {
                write!(f, "{:#04X} (SLOT_DURATION)", self.to_u8().unwrap())
            }
            AppConfigTlvType::RangingInterval => {
                write!(f, "{:#04X} (RANGING_INTERVAL)", self.to_u8().unwrap())
            }
            AppConfigTlvType::StsIndex => write!(f, "{:#04X} (STS_INDEX)", self.to_u8().unwrap()),
            AppConfigTlvType::MacFcsType => {
                write!(f, "{:#04X} (MAC_FCS_TYPE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::RangingRoundControl => {
                write!(f, "{:#04X} (RANGING_ROUND_CONTROL)", self.to_u8().unwrap())
            }
            AppConfigTlvType::AoaResultReq => {
                write!(f, "{:#04X} (AOA_RESULT_REQ)", self.to_u8().unwrap())
            }
            AppConfigTlvType::RngDataNtf => {
                write!(f, "{:#04X} (RNG_DATA_NTF)", self.to_u8().unwrap())
            }
            AppConfigTlvType::RngDataNtfProximityNear => write!(
                f,
                "{:#04X} (RNG_DATA_NTF_PROXIMITY_NEAR)",
                self.to_u8().unwrap()
            ),
            AppConfigTlvType::RngDataNtfProximityFar => write!(
                f,
                "{:#04X} (RNG_DATA_NTF_PROXIMITY_FAR)",
                self.to_u8().unwrap()
            ),
            AppConfigTlvType::DeviceRole => {
                write!(f, "{:#04X} (DEVICE_ROLE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::RframeConfig => {
                write!(f, "{:#04X} (RFRAME_CONFIG)", self.to_u8().unwrap())
            }
            AppConfigTlvType::PreambleCodeIndex => {
                write!(f, "{:#04X} (PREAMBLE_CODE_INDEX)", self.to_u8().unwrap())
            }
            AppConfigTlvType::SfdId => write!(f, "{:#04X} (SFD_ID)", self.to_u8().unwrap()),
            AppConfigTlvType::PsduDataRate => {
                write!(f, "{:#04X} (PSDU_DATA_RATE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::PreambleDuration => {
                write!(f, "{:#04X} (PREAMBLE_DURATION)", self.to_u8().unwrap())
            }
            AppConfigTlvType::RangingTimeStruct => {
                write!(f, "{:#04X} (RANGING_TIME_STRUCT)", self.to_u8().unwrap())
            }
            AppConfigTlvType::SlotsPerRr => {
                write!(f, "{:#04X} (SLOTS_PER_RR)", self.to_u8().unwrap())
            }
            AppConfigTlvType::TxAdaptivePayloadPower => write!(
                f,
                "{:#04X} (TX_ADAPTIVE_PAYLOAD_POWER)",
                self.to_u8().unwrap()
            ),
            AppConfigTlvType::ResponderSlotIndex => {
                write!(f, "{:#04X} (RESPONDER_SLOT_INDEX)", self.to_u8().unwrap())
            }
            AppConfigTlvType::PrfMode => write!(f, "{:#04X} (PRF_MODE)", self.to_u8().unwrap()),
            AppConfigTlvType::ScheduledMode => {
                write!(f, "{:#04X} (SCHEDULED_MODE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::KeyRotation => {
                write!(f, "{:#04X} (KEY_ROTATION)", self.to_u8().unwrap())
            }
            AppConfigTlvType::KeyRotationRate => {
                write!(f, "{:#04X} (KEY_ROTATION_RATE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::SessionPriority => {
                write!(f, "{:#04X} (SESSION_PRIORITY)", self.to_u8().unwrap())
            }
            AppConfigTlvType::MacAddressMode => {
                write!(f, "{:#04X} (MAC_ADDRESS_MODE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::VendorId => write!(f, "{:#04X} (VENDOR_ID)", self.to_u8().unwrap()),
            AppConfigTlvType::StaticStsIv => {
                write!(f, "{:#04X} (STATIC_STS_IV)", self.to_u8().unwrap())
            }
            AppConfigTlvType::NumberOfStsSegments => {
                write!(f, "{:#04X} (NUMBER_OF_STS_SEGMENTS)", self.to_u8().unwrap())
            }
            AppConfigTlvType::MaxRrRetry => {
                write!(f, "{:#04X} (MAX_RR_RETRY)", self.to_u8().unwrap())
            }
            AppConfigTlvType::UwbInitiationTime => {
                write!(f, "{:#04X} (UWB_INITIATION_TIME)", self.to_u8().unwrap())
            }
            AppConfigTlvType::HoppingMode => {
                write!(f, "{:#04X} (HOPPING_MODE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::BlockStrideLength => {
                write!(f, "{:#04X} (BLOCK_STRIDE_LENGTH)", self.to_u8().unwrap())
            }
            AppConfigTlvType::ResultReportConfig => {
                write!(f, "{:#04X} (RESULT_REPORT_CONFIG)", self.to_u8().unwrap())
            }
            AppConfigTlvType::InBandTerminationAttemptCount => write!(
                f,
                "{:#04X} (IN_BAND_TERMINATION_ATTEMPT_COUNT)",
                self.to_u8().unwrap()
            ),
            AppConfigTlvType::SubSessionId => {
                write!(f, "{:#04X} (SUB_SESSION_ID)", self.to_u8().unwrap())
            }
            AppConfigTlvType::BprfPhrDataRate => {
                write!(f, "{:#04X} (BPRF_PHR_DATA_RATE)", self.to_u8().unwrap())
            }
            AppConfigTlvType::MaxNumberOfMeasurements => write!(
                f,
                "{:#04X} (MAX_NUMBER_OF_MEASUREMENTS)",
                self.to_u8().unwrap()
            ),
            AppConfigTlvType::StsLength => write!(f, "{:#04X} (STS_LENGTH)", self.to_u8().unwrap()),
            AppConfigTlvType::CccHopModeKey => {
                write!(f, "{:#04X} (CCC_HOP_MODE_KEY)", self.to_u8().unwrap())
            }
            AppConfigTlvType::CccUwbTime0 => {
                write!(f, "{:#04X} (CCC_UWB_TIME0)", self.to_u8().unwrap())
            }
            AppConfigTlvType::CccRangingProtocolVer => write!(
                f,
                "{:#04X} (CCC_RANGING_PROTOCOL_VER)",
                self.to_u8().unwrap()
            ),
            AppConfigTlvType::CccUwbConfigId => {
                write!(f, "{:#04X} (CCC_UWB_CONFIG_ID)", self.to_u8().unwrap())
            }
            AppConfigTlvType::CccPulseshapeCombo => {
                write!(f, "{:#04X} (CCC_PULSESHAPE_COMBO)", self.to_u8().unwrap())
            }
            AppConfigTlvType::CccUrskTtl => {
                write!(f, "{:#04X} (CCC_URSK_TTL)", self.to_u8().unwrap())
            }
            AppConfigTlvType::NbOfRangeMeasurements => write!(
                f,
                "{:#04X} (NB_OF_RANGE_MEASUREMENTS)",
                self.to_u8().unwrap()
            ),
            AppConfigTlvType::NbOfAzimuthMeasurements => write!(
                f,
                "{:#04X} (NB_OF_AZIMUTH_MEASUREMENTS)",
                self.to_u8().unwrap()
            ),
            AppConfigTlvType::NbOfElevationMeasurements => write!(
                f,
                "{:#04X} (NB_OF_ELEVATION_MEASUREMENTS)",
                self.to_u8().unwrap()
            ),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum CapTlvType {
    SupportedFiraPhyVersionRange = 0x0,
    SupportedFiraMacVersionRange = 0x1,
    SupportedDeviceRoles = 0x2,
    SupportedRangingMethod = 0x3,
    SupportedStsConfig = 0x4,
    SupportedMultiNodeModes = 0x5,
    SupportedRangingTimeStruct = 0x6,
    SupportedScheduledMode = 0x7,
    SupportedHoppingMode = 0x8,
    SupportedBlockStriding = 0x9,
    SupportedUwbInitiationTime = 0xa,
    SupportedChannels = 0xb,
    SupportedRframeConfig = 0xc,
    SupportedCcConstraintLength = 0xd,
    SupportedBprfParameterSets = 0xe,
    SupportedHprfParameterSets = 0xf,
    SupportedAoa = 0x10,
    SupportedExtendedMacAddress = 0x11,
    CccSupportedChapsPerSlot = 0xa0,
    CccSupportedSyncCodes = 0xa1,
    CccSupportedHoppingConfigModesAndSequences = 0xa2,
    CccSupportedChannels = 0xa3,
    CccSupportedVersions = 0xa4,
    CccSupportedUwbConfigs = 0xa5,
    CccSupportedPulseShapeCombos = 0xa6,
    CccSupportedRanMultiplier = 0xa7,
    SupportedPowerStats = 0xc0,
    SupportedAoaResultReqAntennaInterleaving = 0xe3,
}
impl fmt::Display for CapTlvType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CapTlvType::SupportedFiraPhyVersionRange => write!(
                f,
                "{:#04X} (SUPPORTED_FIRA_PHY_VERSION_RANGE)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedFiraMacVersionRange => write!(
                f,
                "{:#04X} (SUPPORTED_FIRA_MAC_VERSION_RANGE)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedDeviceRoles => {
                write!(f, "{:#04X} (SUPPORTED_DEVICE_ROLES)", self.to_u8().unwrap())
            }
            CapTlvType::SupportedRangingMethod => write!(
                f,
                "{:#04X} (SUPPORTED_RANGING_METHOD)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedStsConfig => {
                write!(f, "{:#04X} (SUPPORTED_STS_CONFIG)", self.to_u8().unwrap())
            }
            CapTlvType::SupportedMultiNodeModes => write!(
                f,
                "{:#04X} (SUPPORTED_MULTI_NODE_MODES)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedRangingTimeStruct => write!(
                f,
                "{:#04X} (SUPPORTED_RANGING_TIME_STRUCT)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedScheduledMode => write!(
                f,
                "{:#04X} (SUPPORTED_SCHEDULED_MODE)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedHoppingMode => {
                write!(f, "{:#04X} (SUPPORTED_HOPPING_MODE)", self.to_u8().unwrap())
            }
            CapTlvType::SupportedBlockStriding => write!(
                f,
                "{:#04X} (SUPPORTED_BLOCK_STRIDING)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedUwbInitiationTime => write!(
                f,
                "{:#04X} (SUPPORTED_UWB_INITIATION_TIME)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedChannels => {
                write!(f, "{:#04X} (SUPPORTED_CHANNELS)", self.to_u8().unwrap())
            }
            CapTlvType::SupportedRframeConfig => write!(
                f,
                "{:#04X} (SUPPORTED_RFRAME_CONFIG)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedCcConstraintLength => write!(
                f,
                "{:#04X} (SUPPORTED_CC_CONSTRAINT_LENGTH)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedBprfParameterSets => write!(
                f,
                "{:#04X} (SUPPORTED_BPRF_PARAMETER_SETS)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedHprfParameterSets => write!(
                f,
                "{:#04X} (SUPPORTED_HPRF_PARAMETER_SETS)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedAoa => write!(f, "{:#04X} (SUPPORTED_AOA)", self.to_u8().unwrap()),
            CapTlvType::SupportedExtendedMacAddress => write!(
                f,
                "{:#04X} (SUPPORTED_EXTENDED_MAC_ADDRESS)",
                self.to_u8().unwrap()
            ),
            CapTlvType::CccSupportedChapsPerSlot => write!(
                f,
                "{:#04X} (CCC_SUPPORTED_CHAPS_PER_SLOT)",
                self.to_u8().unwrap()
            ),
            CapTlvType::CccSupportedSyncCodes => write!(
                f,
                "{:#04X} (CCC_SUPPORTED_SYNC_CODES)",
                self.to_u8().unwrap()
            ),
            CapTlvType::CccSupportedHoppingConfigModesAndSequences => write!(
                f,
                "{:#04X} (CCC_SUPPORTED_HOPPING_CONFIG_MODES_AND_SEQUENCES)",
                self.to_u8().unwrap()
            ),
            CapTlvType::CccSupportedChannels => {
                write!(f, "{:#04X} (CCC_SUPPORTED_CHANNELS)", self.to_u8().unwrap())
            }
            CapTlvType::CccSupportedVersions => {
                write!(f, "{:#04X} (CCC_SUPPORTED_VERSIONS)", self.to_u8().unwrap())
            }
            CapTlvType::CccSupportedUwbConfigs => write!(
                f,
                "{:#04X} (CCC_SUPPORTED_UWB_CONFIGS)",
                self.to_u8().unwrap()
            ),
            CapTlvType::CccSupportedPulseShapeCombos => write!(
                f,
                "{:#04X} (CCC_SUPPORTED_PULSE_SHAPE_COMBOS)",
                self.to_u8().unwrap()
            ),
            CapTlvType::CccSupportedRanMultiplier => write!(
                f,
                "{:#04X} (CCC_SUPPORTED_RAN_MULTIPLIER)",
                self.to_u8().unwrap()
            ),
            CapTlvType::SupportedPowerStats => {
                write!(f, "{:#04X} (SUPPORTED_POWER_STATS)", self.to_u8().unwrap())
            }
            CapTlvType::SupportedAoaResultReqAntennaInterleaving => write!(
                f,
                "{:#04X} (SUPPORTED_AOA_RESULT_REQ_ANTENNA_INTERLEAVING)",
                self.to_u8().unwrap()
            ),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum AoaResultReqType {
    AoaDisable = 0x0,
    AoaEnable = 0x1,
    AoaEnableAzimuth = 0x2,
    AoaEnableElevation = 0x3,
    AoaEnableInterleaved = 0xf0,
}
impl fmt::Display for AoaResultReqType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AoaResultReqType::AoaDisable => {
                write!(f, "{:#04X} (AOA_DISABLE)", self.to_u8().unwrap())
            }
            AoaResultReqType::AoaEnable => write!(f, "{:#04X} (AOA_ENABLE)", self.to_u8().unwrap()),
            AoaResultReqType::AoaEnableAzimuth => {
                write!(f, "{:#04X} (AOA_ENABLE_AZIMUTH)", self.to_u8().unwrap())
            }
            AoaResultReqType::AoaEnableElevation => {
                write!(f, "{:#04X} (AOA_ENABLE_ELEVATION)", self.to_u8().unwrap())
            }
            AoaResultReqType::AoaEnableInterleaved => {
                write!(f, "{:#04X} (AOA_ENABLE_INTERLEAVED)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum DeviceState {
    DeviceStateReady = 0x1,
    DeviceStateActive = 0x2,
    DeviceStateError = 0xff,
}
impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceState::DeviceStateReady => {
                write!(f, "{:#04X} (DEVICE_STATE_READY)", self.to_u8().unwrap())
            }
            DeviceState::DeviceStateActive => {
                write!(f, "{:#04X} (DEVICE_STATE_ACTIVE)", self.to_u8().unwrap())
            }
            DeviceState::DeviceStateError => {
                write!(f, "{:#04X} (DEVICE_STATE_ERROR)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum SessionState {
    SessionStateInit = 0x0,
    SessionStateDeinit = 0x1,
    SessionStateActive = 0x2,
    SessionStateIdle = 0x3,
}
impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SessionState::SessionStateInit => {
                write!(f, "{:#04X} (SESSION_STATE_INIT)", self.to_u8().unwrap())
            }
            SessionState::SessionStateDeinit => {
                write!(f, "{:#04X} (SESSION_STATE_DEINIT)", self.to_u8().unwrap())
            }
            SessionState::SessionStateActive => {
                write!(f, "{:#04X} (SESSION_STATE_ACTIVE)", self.to_u8().unwrap())
            }
            SessionState::SessionStateIdle => {
                write!(f, "{:#04X} (SESSION_STATE_IDLE)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum ReasonCode {
    StateChangeWithSessionManagementCommands = 0x0,
    MaxRangingRoundRetryCountReached = 0x1,
    MaxNumberOfMeasurementsReached = 0x2,
    ErrorSlotLengthNotSupported = 0x20,
    ErrorInsufficientSlotsPerRr = 0x21,
    ErrorMacAddressModeNotSupported = 0x22,
    ErrorInvalidRangingInterval = 0x23,
    ErrorInvalidStsConfig = 0x24,
    ErrorInvalidRframeConfig = 0x25,
}
impl fmt::Display for ReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReasonCode::StateChangeWithSessionManagementCommands => write!(
                f,
                "{:#04X} (STATE_CHANGE_WITH_SESSION_MANAGEMENT_COMMANDS)",
                self.to_u8().unwrap()
            ),
            ReasonCode::MaxRangingRoundRetryCountReached => write!(
                f,
                "{:#04X} (MAX_RANGING_ROUND_RETRY_COUNT_REACHED)",
                self.to_u8().unwrap()
            ),
            ReasonCode::MaxNumberOfMeasurementsReached => write!(
                f,
                "{:#04X} (MAX_NUMBER_OF_MEASUREMENTS_REACHED)",
                self.to_u8().unwrap()
            ),
            ReasonCode::ErrorSlotLengthNotSupported => write!(
                f,
                "{:#04X} (ERROR_SLOT_LENGTH_NOT_SUPPORTED)",
                self.to_u8().unwrap()
            ),
            ReasonCode::ErrorInsufficientSlotsPerRr => write!(
                f,
                "{:#04X} (ERROR_INSUFFICIENT_SLOTS_PER_RR)",
                self.to_u8().unwrap()
            ),
            ReasonCode::ErrorMacAddressModeNotSupported => write!(
                f,
                "{:#04X} (ERROR_MAC_ADDRESS_MODE_NOT_SUPPORTED)",
                self.to_u8().unwrap()
            ),
            ReasonCode::ErrorInvalidRangingInterval => write!(
                f,
                "{:#04X} (ERROR_INVALID_RANGING_INTERVAL)",
                self.to_u8().unwrap()
            ),
            ReasonCode::ErrorInvalidStsConfig => write!(
                f,
                "{:#04X} (ERROR_INVALID_STS_CONFIG)",
                self.to_u8().unwrap()
            ),
            ReasonCode::ErrorInvalidRframeConfig => write!(
                f,
                "{:#04X} (ERROR_INVALID_RFRAME_CONFIG)",
                self.to_u8().unwrap()
            ),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum MulticastUpdateStatusCode {
    StatusOkMulticastListUpdate = 0x0,
    StatusErrorMulticastListFull = 0x1,
    StatusErrorKeyFetchFail = 0x2,
    StatusErrorSubSessionIdNotFound = 0x3,
}
impl fmt::Display for MulticastUpdateStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MulticastUpdateStatusCode::StatusOkMulticastListUpdate => write!(
                f,
                "{:#04X} (STATUS_OK_MULTICAST_LIST_UPDATE)",
                self.to_u8().unwrap()
            ),
            MulticastUpdateStatusCode::StatusErrorMulticastListFull => write!(
                f,
                "{:#04X} (STATUS_ERROR_MULTICAST_LIST_FULL)",
                self.to_u8().unwrap()
            ),
            MulticastUpdateStatusCode::StatusErrorKeyFetchFail => write!(
                f,
                "{:#04X} (STATUS_ERROR_KEY_FETCH_FAIL)",
                self.to_u8().unwrap()
            ),
            MulticastUpdateStatusCode::StatusErrorSubSessionIdNotFound => write!(
                f,
                "{:#04X} (STATUS_ERROR_SUB_SESSION_ID_NOT_FOUND)",
                self.to_u8().unwrap()
            ),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum MacAddressIndicator {
    ShortAddress = 0x0,
    ExtendedAddress = 0x1,
}
impl fmt::Display for MacAddressIndicator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MacAddressIndicator::ShortAddress => {
                write!(f, "{:#04X} (SHORT_ADDRESS)", self.to_u8().unwrap())
            }
            MacAddressIndicator::ExtendedAddress => {
                write!(f, "{:#04X} (EXTENDED_ADDRESS)", self.to_u8().unwrap())
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum SessionType {
    FiraRangingSession = 0x0,
    FiraDataTransfer = 0x1,
    Ccc = 0xa0,
}
impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SessionType::FiraRangingSession => {
                write!(f, "{:#04X} (FIRA_RANGING_SESSION)", self.to_u8().unwrap())
            }
            SessionType::FiraDataTransfer => {
                write!(f, "{:#04X} (FIRA_DATA_TRANSFER)", self.to_u8().unwrap())
            }
            SessionType::Ccc => write!(f, "{:#04X} (CCC)", self.to_u8().unwrap()),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum MessageType {
    Command = 0x1,
    Response = 0x2,
    Notification = 0x3,
}
impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::Command => write!(f, "{:#04X} (COMMAND)", self.to_u8().unwrap()),
            MessageType::Response => write!(f, "{:#04X} (RESPONSE)", self.to_u8().unwrap()),
            MessageType::Notification => write!(f, "{:#04X} (NOTIFICATION)", self.to_u8().unwrap()),
        }
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum RangingMeasurementType {
    OneWay = 0x0,
    TwoWay = 0x1,
}
impl fmt::Display for RangingMeasurementType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RangingMeasurementType::OneWay => write!(f, "{:#04X} (ONE_WAY)", self.to_u8().unwrap()),
            RangingMeasurementType::TwoWay => write!(f, "{:#04X} (TWO_WAY)", self.to_u8().unwrap()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CapTlv {
    pub t: CapTlvType,
    pub v: Vec<u8>,
}
impl CapTlv {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 2 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "CapTlv".to_string(),
                field: "t".to_string(),
                wanted: 1,
                got: bytes.len(),
            });
        }
        let t = u8::from_le_bytes([bytes[0]]);
        let t = CapTlvType::from_u8(t).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "CapTlv".to_string(),
            field: "t".to_string(),
            value: t as u64,
            type_: "CapTlvType".to_string(),
        })?;
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "CapTlv".to_string(),
                field: "v_count".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let v_count = u8::from_le_bytes([bytes[1]]);
        let want_ = 2 + ((v_count as usize) * 1);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "CapTlv".to_string(),
                field: "v".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let v: Vec<u8> = bytes[2..2 + ((v_count as usize) * 1)]
            .to_vec()
            .chunks_exact(1)
            .into_iter()
            .map(|i| u8::from_le_bytes([i[0]]))
            .collect();
        Ok(Self { t, v })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let t = self.t.to_u8().unwrap();
        buffer[0..1].copy_from_slice(&t.to_le_bytes()[0..1]);
        buffer[1..2].copy_from_slice(&(self.v.len() as u8).to_le_bytes());
        for (i, e) in self.v.iter().enumerate() {
            buffer[2 + i..2 + i + 1].copy_from_slice(&e.to_le_bytes())
        }
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        let ret = ret + (self.v.len() * ((/* Bits: */8 + /* Dynamic: */ 0) / 8));
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceParameter {
    pub id: u8,
    pub value: Vec<u8>,
}
impl DeviceParameter {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 2 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceParameter".to_string(),
                field: "id".to_string(),
                wanted: 1,
                got: bytes.len(),
            });
        }
        let id = u8::from_le_bytes([bytes[0]]);
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceParameter".to_string(),
                field: "value_count".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let value_count = u8::from_le_bytes([bytes[1]]);
        let want_ = 2 + ((value_count as usize) * 1);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "DeviceParameter".to_string(),
                field: "value".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let value: Vec<u8> = bytes[2..2 + ((value_count as usize) * 1)]
            .to_vec()
            .chunks_exact(1)
            .into_iter()
            .map(|i| u8::from_le_bytes([i[0]]))
            .collect();
        Ok(Self { id, value })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let id = self.id;
        buffer[0..1].copy_from_slice(&id.to_le_bytes()[0..1]);
        buffer[1..2].copy_from_slice(&(self.value.len() as u8).to_le_bytes());
        for (i, e) in self.value.iter().enumerate() {
            buffer[2 + i..2 + i + 1].copy_from_slice(&e.to_le_bytes())
        }
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        let ret = ret + (self.value.len() * ((/* Bits: */8 + /* Dynamic: */ 0) / 8));
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceConfigStatus {
    pub parameter_id: u8,
    pub status: StatusCode,
}
impl DeviceConfigStatus {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 2 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceConfigStatus".to_string(),
                field: "parameter_id".to_string(),
                wanted: 1,
                got: bytes.len(),
            });
        }
        let parameter_id = u8::from_le_bytes([bytes[0]]);
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceConfigStatus".to_string(),
                field: "status".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[1]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "DeviceConfigStatus".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self {
            parameter_id,
            status,
        })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let parameter_id = self.parameter_id;
        buffer[0..1].copy_from_slice(&parameter_id.to_le_bytes()[0..1]);
        let status = self.status.to_u8().unwrap();
        buffer[1..2].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppConfigParameter {
    pub id: u8,
    pub value: Vec<u8>,
}
impl AppConfigParameter {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 2 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigParameter".to_string(),
                field: "id".to_string(),
                wanted: 1,
                got: bytes.len(),
            });
        }
        let id = u8::from_le_bytes([bytes[0]]);
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigParameter".to_string(),
                field: "value_count".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let value_count = u8::from_le_bytes([bytes[1]]);
        let want_ = 2 + ((value_count as usize) * 1);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigParameter".to_string(),
                field: "value".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let value: Vec<u8> = bytes[2..2 + ((value_count as usize) * 1)]
            .to_vec()
            .chunks_exact(1)
            .into_iter()
            .map(|i| u8::from_le_bytes([i[0]]))
            .collect();
        Ok(Self { id, value })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let id = self.id;
        buffer[0..1].copy_from_slice(&id.to_le_bytes()[0..1]);
        buffer[1..2].copy_from_slice(&(self.value.len() as u8).to_le_bytes());
        for (i, e) in self.value.iter().enumerate() {
            buffer[2 + i..2 + i + 1].copy_from_slice(&e.to_le_bytes())
        }
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        let ret = ret + (self.value.len() * ((/* Bits: */8 + /* Dynamic: */ 0) / 8));
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppConfigStatus {
    pub config_id: u8,
    pub status: StatusCode,
}
impl AppConfigStatus {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 2 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigStatus".to_string(),
                field: "config_id".to_string(),
                wanted: 1,
                got: bytes.len(),
            });
        }
        let config_id = u8::from_le_bytes([bytes[0]]);
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "AppConfigStatus".to_string(),
                field: "status".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[1]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "AppConfigStatus".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { config_id, status })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let config_id = self.config_id;
        buffer[0..1].copy_from_slice(&config_id.to_le_bytes()[0..1]);
        let status = self.status.to_u8().unwrap();
        buffer[1..2].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Controlee {
    pub short_address: u16,
    pub subsession_id: u32,
}
impl Controlee {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee".to_string(),
                field: "short_address".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let short_address = u16::from_le_bytes([bytes[0], bytes[1]]);
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "Controlee".to_string(),
                field: "subsession_id".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let subsession_id = u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
        Ok(Self {
            short_address,
            subsession_id,
        })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let short_address = self.short_address;
        buffer[0..2].copy_from_slice(&short_address.to_le_bytes()[0..2]);
        let subsession_id = self.subsession_id;
        buffer[2..6].copy_from_slice(&subsession_id.to_le_bytes()[0..4]);
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 6;
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ControleeStatus {
    pub mac_address: u16,
    pub subsession_id: u32,
    pub status: u8,
}
impl ControleeStatus {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 7 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ControleeStatus".to_string(),
                field: "mac_address".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let mac_address = u16::from_le_bytes([bytes[0], bytes[1]]);
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "ControleeStatus".to_string(),
                field: "subsession_id".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let subsession_id = u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
        if bytes.len() < 7 {
            return Err(Error::InvalidLengthError {
                obj: "ControleeStatus".to_string(),
                field: "status".to_string(),
                wanted: 7,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[6]]);
        Ok(Self {
            mac_address,
            subsession_id,
            status,
        })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let mac_address = self.mac_address;
        buffer[0..2].copy_from_slice(&mac_address.to_le_bytes()[0..2]);
        let subsession_id = self.subsession_id;
        buffer[2..6].copy_from_slice(&subsession_id.to_le_bytes()[0..4]);
        let status = self.status;
        buffer[6..7].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 7;
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShortAddressTwoWayRangingMeasurement {
    pub mac_address: u16,
    pub status: StatusCode,
    pub nlos: u8,
    pub distance: u16,
    pub aoa_azimuth: u16,
    pub aoa_azimuth_fom: u8,
    pub aoa_elevation: u16,
    pub aoa_elevation_fom: u8,
    pub aoa_destination_azimuth: u16,
    pub aoa_destination_azimuth_fom: u8,
    pub aoa_destination_elevation: u16,
    pub aoa_destination_elevation_fom: u8,
    pub slot_index: u8,
}
impl ShortAddressTwoWayRangingMeasurement {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 31 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "mac_address".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let mac_address = u16::from_le_bytes([bytes[0], bytes[1]]);
        if bytes.len() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "status".to_string(),
                wanted: 3,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[2]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "nlos".to_string(),
                wanted: 4,
                got: bytes.len(),
            });
        }
        let nlos = u8::from_le_bytes([bytes[3]]);
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "distance".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let distance = u16::from_le_bytes([bytes[4], bytes[5]]);
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_azimuth".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let aoa_azimuth = u16::from_le_bytes([bytes[6], bytes[7]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_azimuth_fom".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let aoa_azimuth_fom = u8::from_le_bytes([bytes[8]]);
        if bytes.len() < 11 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_elevation".to_string(),
                wanted: 11,
                got: bytes.len(),
            });
        }
        let aoa_elevation = u16::from_le_bytes([bytes[9], bytes[10]]);
        if bytes.len() < 12 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_elevation_fom".to_string(),
                wanted: 12,
                got: bytes.len(),
            });
        }
        let aoa_elevation_fom = u8::from_le_bytes([bytes[11]]);
        if bytes.len() < 14 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_destination_azimuth".to_string(),
                wanted: 14,
                got: bytes.len(),
            });
        }
        let aoa_destination_azimuth = u16::from_le_bytes([bytes[12], bytes[13]]);
        if bytes.len() < 15 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_destination_azimuth_fom".to_string(),
                wanted: 15,
                got: bytes.len(),
            });
        }
        let aoa_destination_azimuth_fom = u8::from_le_bytes([bytes[14]]);
        if bytes.len() < 17 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_destination_elevation".to_string(),
                wanted: 17,
                got: bytes.len(),
            });
        }
        let aoa_destination_elevation = u16::from_le_bytes([bytes[15], bytes[16]]);
        if bytes.len() < 18 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_destination_elevation_fom".to_string(),
                wanted: 18,
                got: bytes.len(),
            });
        }
        let aoa_destination_elevation_fom = u8::from_le_bytes([bytes[17]]);
        if bytes.len() < 19 {
            return Err(Error::InvalidLengthError {
                obj: "ShortAddressTwoWayRangingMeasurement".to_string(),
                field: "slot_index".to_string(),
                wanted: 19,
                got: bytes.len(),
            });
        }
        let slot_index = u8::from_le_bytes([bytes[18]]);
        Ok(Self {
            mac_address,
            status,
            nlos,
            distance,
            aoa_azimuth,
            aoa_azimuth_fom,
            aoa_elevation,
            aoa_elevation_fom,
            aoa_destination_azimuth,
            aoa_destination_azimuth_fom,
            aoa_destination_elevation,
            aoa_destination_elevation_fom,
            slot_index,
        })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let mac_address = self.mac_address;
        buffer[0..2].copy_from_slice(&mac_address.to_le_bytes()[0..2]);
        let status = self.status.to_u8().unwrap();
        buffer[2..3].copy_from_slice(&status.to_le_bytes()[0..1]);
        let nlos = self.nlos;
        buffer[3..4].copy_from_slice(&nlos.to_le_bytes()[0..1]);
        let distance = self.distance;
        buffer[4..6].copy_from_slice(&distance.to_le_bytes()[0..2]);
        let aoa_azimuth = self.aoa_azimuth;
        buffer[6..8].copy_from_slice(&aoa_azimuth.to_le_bytes()[0..2]);
        let aoa_azimuth_fom = self.aoa_azimuth_fom;
        buffer[8..9].copy_from_slice(&aoa_azimuth_fom.to_le_bytes()[0..1]);
        let aoa_elevation = self.aoa_elevation;
        buffer[9..11].copy_from_slice(&aoa_elevation.to_le_bytes()[0..2]);
        let aoa_elevation_fom = self.aoa_elevation_fom;
        buffer[11..12].copy_from_slice(&aoa_elevation_fom.to_le_bytes()[0..1]);
        let aoa_destination_azimuth = self.aoa_destination_azimuth;
        buffer[12..14].copy_from_slice(&aoa_destination_azimuth.to_le_bytes()[0..2]);
        let aoa_destination_azimuth_fom = self.aoa_destination_azimuth_fom;
        buffer[14..15].copy_from_slice(&aoa_destination_azimuth_fom.to_le_bytes()[0..1]);
        let aoa_destination_elevation = self.aoa_destination_elevation;
        buffer[15..17].copy_from_slice(&aoa_destination_elevation.to_le_bytes()[0..2]);
        let aoa_destination_elevation_fom = self.aoa_destination_elevation_fom;
        buffer[17..18].copy_from_slice(&aoa_destination_elevation_fom.to_le_bytes()[0..1]);
        let slot_index = self.slot_index;
        buffer[18..19].copy_from_slice(&slot_index.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 31;
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedAddressTwoWayRangingMeasurement {
    pub mac_address: u64,
    pub status: StatusCode,
    pub nlos: u8,
    pub distance: u16,
    pub aoa_azimuth: u16,
    pub aoa_azimuth_fom: u8,
    pub aoa_elevation: u16,
    pub aoa_elevation_fom: u8,
    pub aoa_destination_azimuth: u16,
    pub aoa_destination_azimuth_fom: u8,
    pub aoa_destination_elevation: u16,
    pub aoa_destination_elevation_fom: u8,
    pub slot_index: u8,
}
impl ExtendedAddressTwoWayRangingMeasurement {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 31 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "mac_address".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let mac_address = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "status".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[8]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 10 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "nlos".to_string(),
                wanted: 10,
                got: bytes.len(),
            });
        }
        let nlos = u8::from_le_bytes([bytes[9]]);
        if bytes.len() < 12 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "distance".to_string(),
                wanted: 12,
                got: bytes.len(),
            });
        }
        let distance = u16::from_le_bytes([bytes[10], bytes[11]]);
        if bytes.len() < 14 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_azimuth".to_string(),
                wanted: 14,
                got: bytes.len(),
            });
        }
        let aoa_azimuth = u16::from_le_bytes([bytes[12], bytes[13]]);
        if bytes.len() < 15 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_azimuth_fom".to_string(),
                wanted: 15,
                got: bytes.len(),
            });
        }
        let aoa_azimuth_fom = u8::from_le_bytes([bytes[14]]);
        if bytes.len() < 17 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_elevation".to_string(),
                wanted: 17,
                got: bytes.len(),
            });
        }
        let aoa_elevation = u16::from_le_bytes([bytes[15], bytes[16]]);
        if bytes.len() < 18 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_elevation_fom".to_string(),
                wanted: 18,
                got: bytes.len(),
            });
        }
        let aoa_elevation_fom = u8::from_le_bytes([bytes[17]]);
        if bytes.len() < 20 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_destination_azimuth".to_string(),
                wanted: 20,
                got: bytes.len(),
            });
        }
        let aoa_destination_azimuth = u16::from_le_bytes([bytes[18], bytes[19]]);
        if bytes.len() < 21 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_destination_azimuth_fom".to_string(),
                wanted: 21,
                got: bytes.len(),
            });
        }
        let aoa_destination_azimuth_fom = u8::from_le_bytes([bytes[20]]);
        if bytes.len() < 23 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_destination_elevation".to_string(),
                wanted: 23,
                got: bytes.len(),
            });
        }
        let aoa_destination_elevation = u16::from_le_bytes([bytes[21], bytes[22]]);
        if bytes.len() < 24 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "aoa_destination_elevation_fom".to_string(),
                wanted: 24,
                got: bytes.len(),
            });
        }
        let aoa_destination_elevation_fom = u8::from_le_bytes([bytes[23]]);
        if bytes.len() < 25 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedAddressTwoWayRangingMeasurement".to_string(),
                field: "slot_index".to_string(),
                wanted: 25,
                got: bytes.len(),
            });
        }
        let slot_index = u8::from_le_bytes([bytes[24]]);
        Ok(Self {
            mac_address,
            status,
            nlos,
            distance,
            aoa_azimuth,
            aoa_azimuth_fom,
            aoa_elevation,
            aoa_elevation_fom,
            aoa_destination_azimuth,
            aoa_destination_azimuth_fom,
            aoa_destination_elevation,
            aoa_destination_elevation_fom,
            slot_index,
        })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let mac_address = self.mac_address;
        buffer[0..8].copy_from_slice(&mac_address.to_le_bytes()[0..8]);
        let status = self.status.to_u8().unwrap();
        buffer[8..9].copy_from_slice(&status.to_le_bytes()[0..1]);
        let nlos = self.nlos;
        buffer[9..10].copy_from_slice(&nlos.to_le_bytes()[0..1]);
        let distance = self.distance;
        buffer[10..12].copy_from_slice(&distance.to_le_bytes()[0..2]);
        let aoa_azimuth = self.aoa_azimuth;
        buffer[12..14].copy_from_slice(&aoa_azimuth.to_le_bytes()[0..2]);
        let aoa_azimuth_fom = self.aoa_azimuth_fom;
        buffer[14..15].copy_from_slice(&aoa_azimuth_fom.to_le_bytes()[0..1]);
        let aoa_elevation = self.aoa_elevation;
        buffer[15..17].copy_from_slice(&aoa_elevation.to_le_bytes()[0..2]);
        let aoa_elevation_fom = self.aoa_elevation_fom;
        buffer[17..18].copy_from_slice(&aoa_elevation_fom.to_le_bytes()[0..1]);
        let aoa_destination_azimuth = self.aoa_destination_azimuth;
        buffer[18..20].copy_from_slice(&aoa_destination_azimuth.to_le_bytes()[0..2]);
        let aoa_destination_azimuth_fom = self.aoa_destination_azimuth_fom;
        buffer[20..21].copy_from_slice(&aoa_destination_azimuth_fom.to_le_bytes()[0..1]);
        let aoa_destination_elevation = self.aoa_destination_elevation;
        buffer[21..23].copy_from_slice(&aoa_destination_elevation.to_le_bytes()[0..2]);
        let aoa_destination_elevation_fom = self.aoa_destination_elevation_fom;
        buffer[23..24].copy_from_slice(&aoa_destination_elevation_fom.to_le_bytes()[0..1]);
        let slot_index = self.slot_index;
        buffer[24..25].copy_from_slice(&slot_index.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 31;
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PicaPosition {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub yaw: u16,
    pub pitch: u8,
    pub roll: u16,
}
impl PicaPosition {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 11 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                field: "x".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let x = u16::from_le_bytes([bytes[0], bytes[1]]);
        if bytes.len() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                field: "y".to_string(),
                wanted: 4,
                got: bytes.len(),
            });
        }
        let y = u16::from_le_bytes([bytes[2], bytes[3]]);
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                field: "z".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let z = u16::from_le_bytes([bytes[4], bytes[5]]);
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                field: "yaw".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let yaw = u16::from_le_bytes([bytes[6], bytes[7]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                field: "pitch".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let pitch = u8::from_le_bytes([bytes[8]]);
        if bytes.len() < 11 {
            return Err(Error::InvalidLengthError {
                obj: "PicaPosition".to_string(),
                field: "roll".to_string(),
                wanted: 11,
                got: bytes.len(),
            });
        }
        let roll = u16::from_le_bytes([bytes[9], bytes[10]]);
        Ok(Self {
            x,
            y,
            z,
            yaw,
            pitch,
            roll,
        })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let x = self.x;
        buffer[0..2].copy_from_slice(&x.to_le_bytes()[0..2]);
        let y = self.y;
        buffer[2..4].copy_from_slice(&y.to_le_bytes()[0..2]);
        let z = self.z;
        buffer[4..6].copy_from_slice(&z.to_le_bytes()[0..2]);
        let yaw = self.yaw;
        buffer[6..8].copy_from_slice(&yaw.to_le_bytes()[0..2]);
        let pitch = self.pitch;
        buffer[8..9].copy_from_slice(&pitch.to_le_bytes()[0..1]);
        let roll = self.roll;
        buffer[9..11].copy_from_slice(&roll.to_le_bytes()[0..2]);
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 11;
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PowerStats {
    pub status: StatusCode,
    pub idle_time_ms: u32,
    pub tx_time_ms: u32,
    pub rx_time_ms: u32,
    pub total_wake_count: u32,
}
impl PowerStats {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 17 {
            return false;
        }
        true
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                field: "status".to_string(),
                wanted: 1,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[0]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "PowerStats".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                field: "idle_time_ms".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let idle_time_ms = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                field: "tx_time_ms".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let tx_time_ms = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
        if bytes.len() < 13 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                field: "rx_time_ms".to_string(),
                wanted: 13,
                got: bytes.len(),
            });
        }
        let rx_time_ms = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]);
        if bytes.len() < 17 {
            return Err(Error::InvalidLengthError {
                obj: "PowerStats".to_string(),
                field: "total_wake_count".to_string(),
                wanted: 17,
                got: bytes.len(),
            });
        }
        let total_wake_count = u32::from_le_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
        Ok(Self {
            status,
            idle_time_ms,
            tx_time_ms,
            rx_time_ms,
            total_wake_count,
        })
    }
    fn write_to(&self, buffer: &mut [u8]) {
        let status = self.status.to_u8().unwrap();
        buffer[0..1].copy_from_slice(&status.to_le_bytes()[0..1]);
        let idle_time_ms = self.idle_time_ms;
        buffer[1..5].copy_from_slice(&idle_time_ms.to_le_bytes()[0..4]);
        let tx_time_ms = self.tx_time_ms;
        buffer[5..9].copy_from_slice(&tx_time_ms.to_le_bytes()[0..4]);
        let rx_time_ms = self.rx_time_ms;
        buffer[9..13].copy_from_slice(&rx_time_ms.to_le_bytes()[0..4]);
        let total_wake_count = self.total_wake_count;
        buffer[13..17].copy_from_slice(&total_wake_count.to_le_bytes()[0..4]);
    }
    fn get_total_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 17;
        ret
    }
}

#[derive(Debug)]
enum UciPacketDataChild {
    UciCommand(Arc<UciCommandData>),
    UciResponse(Arc<UciResponseData>),
    UciNotification(Arc<UciNotificationData>),
    Payload(Bytes),
    None,
}
impl UciPacketDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciPacketDataChild::UciCommand(value) => value.get_total_size(),
            UciPacketDataChild::UciResponse(value) => value.get_total_size(),
            UciPacketDataChild::UciNotification(value) => value.get_total_size(),
            UciPacketDataChild::Payload(p) => p.len(),
            UciPacketDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciPacketChild {
    UciCommand(UciCommandPacket),
    UciResponse(UciResponsePacket),
    UciNotification(UciNotificationPacket),
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciPacketData {
    group_id: GroupId,
    packet_boundary_flag: PacketBoundaryFlag,
    message_type: MessageType,
    opcode: u8,
    child: UciPacketDataChild,
}
#[derive(Debug, Clone)]
pub struct UciPacketPacket {
    uci_packet: Arc<UciPacketData>,
}
#[derive(Debug)]
pub struct UciPacketBuilder {
    pub group_id: GroupId,
    pub packet_boundary_flag: PacketBoundaryFlag,
    pub message_type: MessageType,
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciPacketData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let group_id = u8::from_le_bytes([bytes[0]]);
        let group_id = group_id & 0xf;
        let group_id = GroupId::from_u8(group_id).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "UciPacket".to_string(),
            field: "group_id".to_string(),
            value: group_id as u64,
            type_: "GroupId".to_string(),
        })?;
        let packet_boundary_flag = u8::from_le_bytes([bytes[0]]);
        let packet_boundary_flag = packet_boundary_flag >> 4;
        let packet_boundary_flag = packet_boundary_flag & 0x1;
        let packet_boundary_flag =
            PacketBoundaryFlag::from_u8(packet_boundary_flag).ok_or_else(|| {
                Error::InvalidEnumValueError {
                    obj: "UciPacket".to_string(),
                    field: "packet_boundary_flag".to_string(),
                    value: packet_boundary_flag as u64,
                    type_: "PacketBoundaryFlag".to_string(),
                }
            })?;
        let message_type = u8::from_le_bytes([bytes[0]]);
        let message_type = message_type >> 5;
        let message_type = message_type & 0x7;
        let message_type =
            MessageType::from_u8(message_type).ok_or_else(|| Error::InvalidEnumValueError {
                obj: "UciPacket".to_string(),
                field: "message_type".to_string(),
                value: message_type as u64,
                type_: "MessageType".to_string(),
            })?;
        let opcode = u8::from_le_bytes([bytes[1]]);
        let opcode = opcode & 0x3f;
        if bytes.len() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "UciPacket".to_string(),
                field: "payload_size".to_string(),
                wanted: 4,
                got: bytes.len(),
            });
        }
        let payload_size = u8::from_le_bytes([bytes[3]]);
        let want_ = 4 + (payload_size as usize);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "UciPacket".to_string(),
                field: "payload".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let payload: Vec<u8> = bytes[4..(4 + payload_size as usize)].into();
        let child = match (packet_boundary_flag, message_type) {
            (PacketBoundaryFlag::Complete, MessageType::Command)
                if UciCommandData::conforms(&bytes[..]) =>
            {
                UciPacketDataChild::UciCommand(Arc::new(UciCommandData::parse(
                    &bytes[..],
                    group_id,
                    opcode,
                )?))
            }
            (PacketBoundaryFlag::Complete, MessageType::Response)
                if UciResponseData::conforms(&bytes[..]) =>
            {
                UciPacketDataChild::UciResponse(Arc::new(UciResponseData::parse(
                    &bytes[..],
                    group_id,
                    opcode,
                )?))
            }
            (PacketBoundaryFlag::Complete, MessageType::Notification)
                if UciNotificationData::conforms(&bytes[..]) =>
            {
                UciPacketDataChild::UciNotification(Arc::new(UciNotificationData::parse(
                    &bytes[..],
                    group_id,
                    opcode,
                )?))
            }
            (_, _) => return Err(Error::InvalidPacketError),
        };
        Ok(Self {
            group_id,
            packet_boundary_flag,
            message_type,
            opcode,
            child,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let group_id = self.group_id.to_u8().unwrap();
        let group_id = group_id & 0xf;
        buffer[0..1].copy_from_slice(&group_id.to_le_bytes()[0..1]);
        let packet_boundary_flag = self.packet_boundary_flag.to_u8().unwrap();
        let packet_boundary_flag = packet_boundary_flag & 0x1;
        let packet_boundary_flag = (packet_boundary_flag << 4) | ((buffer[0] as u8) & 0xf);
        buffer[0..1].copy_from_slice(&packet_boundary_flag.to_le_bytes()[0..1]);
        let message_type = self.message_type.to_u8().unwrap();
        let message_type = message_type & 0x7;
        let message_type = (message_type << 5) | ((buffer[0] as u8) & 0x1f);
        buffer[0..1].copy_from_slice(&message_type.to_le_bytes()[0..1]);
        let opcode = self.opcode;
        let opcode = opcode & 0x3f;
        buffer[1..2].copy_from_slice(&opcode.to_le_bytes()[0..1]);
        let payload_size =
            u8::try_from(self.child.get_total_size()).expect("payload size did not fit");
        buffer[3..4].copy_from_slice(&payload_size.to_le_bytes()[0..1]);
        match &self.child {
            UciPacketDataChild::UciCommand(value) => value.write_to(buffer),
            UciPacketDataChild::UciResponse(value) => value.write_to(buffer),
            UciPacketDataChild::UciNotification(value) => value.write_to(buffer),
            UciPacketDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciPacketDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 4;
        ret
    }
}
impl Packet for UciPacketPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciPacketPacket> for Bytes {
    fn from(packet: UciPacketPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciPacketPacket> for Vec<u8> {
    fn from(packet: UciPacketPacket) -> Self {
        packet.to_vec()
    }
}
impl UciPacketPacket {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self::new(Arc::new(UciPacketData::parse(bytes)?)).unwrap())
    }
    pub fn specialize(&self) -> UciPacketChild {
        match &self.uci_packet.child {
            UciPacketDataChild::UciCommand(_) => {
                UciPacketChild::UciCommand(UciCommandPacket::new(self.uci_packet.clone()).unwrap())
            }
            UciPacketDataChild::UciResponse(_) => UciPacketChild::UciResponse(
                UciResponsePacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciPacketDataChild::UciNotification(_) => UciPacketChild::UciNotification(
                UciNotificationPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciPacketDataChild::Payload(p) => UciPacketChild::Payload(p.clone()),
            UciPacketDataChild::None => UciPacketChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        Ok(Self { uci_packet })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl UciPacketBuilder {
    pub fn build(self) -> UciPacketPacket {
        let uci_packet = Arc::new(UciPacketData {
            group_id: self.group_id,
            packet_boundary_flag: self.packet_boundary_flag,
            message_type: self.message_type,
            opcode: self.opcode,
            child: match self.payload {
                None => UciPacketDataChild::None,
                Some(bytes) => UciPacketDataChild::Payload(bytes),
            },
        });
        UciPacketPacket::new(uci_packet).unwrap()
    }
}

#[derive(Debug)]
enum UciCommandDataChild {
    CoreCommand(Arc<CoreCommandData>),
    SessionCommand(Arc<SessionCommandData>),
    RangingCommand(Arc<RangingCommandData>),
    AndroidCommand(Arc<AndroidCommandData>),
    UciVendor_A_Command(Arc<UciVendor_A_CommandData>),
    UciVendor_B_Command(Arc<UciVendor_B_CommandData>),
    UciVendor_E_Command(Arc<UciVendor_E_CommandData>),
    UciVendor_F_Command(Arc<UciVendor_F_CommandData>),
    Payload(Bytes),
    None,
}
impl UciCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciCommandDataChild::CoreCommand(value) => value.get_total_size(),
            UciCommandDataChild::SessionCommand(value) => value.get_total_size(),
            UciCommandDataChild::RangingCommand(value) => value.get_total_size(),
            UciCommandDataChild::AndroidCommand(value) => value.get_total_size(),
            UciCommandDataChild::UciVendor_A_Command(value) => value.get_total_size(),
            UciCommandDataChild::UciVendor_B_Command(value) => value.get_total_size(),
            UciCommandDataChild::UciVendor_E_Command(value) => value.get_total_size(),
            UciCommandDataChild::UciVendor_F_Command(value) => value.get_total_size(),
            UciCommandDataChild::Payload(p) => p.len(),
            UciCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciCommandChild {
    CoreCommand(CoreCommandPacket),
    SessionCommand(SessionCommandPacket),
    RangingCommand(RangingCommandPacket),
    AndroidCommand(AndroidCommandPacket),
    UciVendor_A_Command(UciVendor_A_CommandPacket),
    UciVendor_B_Command(UciVendor_B_CommandPacket),
    UciVendor_E_Command(UciVendor_E_CommandPacket),
    UciVendor_F_Command(UciVendor_F_CommandPacket),
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciCommandData {
    child: UciCommandDataChild,
}
#[derive(Debug, Clone)]
pub struct UciCommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
}
#[derive(Debug)]
pub struct UciCommandBuilder {
    pub group_id: GroupId,
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], group_id: GroupId, opcode: u8) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = match (group_id) {
            (GroupId::Core) if CoreCommandData::conforms(&bytes[..]) => {
                UciCommandDataChild::CoreCommand(Arc::new(CoreCommandData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::SessionConfig) if SessionCommandData::conforms(&bytes[..]) => {
                UciCommandDataChild::SessionCommand(Arc::new(SessionCommandData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::RangingSessionControl) if RangingCommandData::conforms(&bytes[..]) => {
                UciCommandDataChild::RangingCommand(Arc::new(RangingCommandData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::VendorAndroid) if AndroidCommandData::conforms(&bytes[..]) => {
                UciCommandDataChild::AndroidCommand(Arc::new(AndroidCommandData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::VendorReservedA) if UciVendor_A_CommandData::conforms(&bytes[..]) => {
                UciCommandDataChild::UciVendor_A_Command(Arc::new(UciVendor_A_CommandData::parse(
                    &bytes[..],
                )?))
            }
            (GroupId::VendorReservedB) if UciVendor_B_CommandData::conforms(&bytes[..]) => {
                UciCommandDataChild::UciVendor_B_Command(Arc::new(UciVendor_B_CommandData::parse(
                    &bytes[..],
                )?))
            }
            (GroupId::VendorReservedE) if UciVendor_E_CommandData::conforms(&bytes[..]) => {
                UciCommandDataChild::UciVendor_E_Command(Arc::new(UciVendor_E_CommandData::parse(
                    &bytes[..],
                )?))
            }
            (GroupId::VendorReservedF) if UciVendor_F_CommandData::conforms(&bytes[..]) => {
                UciCommandDataChild::UciVendor_F_Command(Arc::new(UciVendor_F_CommandData::parse(
                    &bytes[..],
                )?))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciCommandDataChild::CoreCommand(value) => value.write_to(buffer),
            UciCommandDataChild::SessionCommand(value) => value.write_to(buffer),
            UciCommandDataChild::RangingCommand(value) => value.write_to(buffer),
            UciCommandDataChild::AndroidCommand(value) => value.write_to(buffer),
            UciCommandDataChild::UciVendor_A_Command(value) => value.write_to(buffer),
            UciCommandDataChild::UciVendor_B_Command(value) => value.write_to(buffer),
            UciCommandDataChild::UciVendor_E_Command(value) => value.write_to(buffer),
            UciCommandDataChild::UciVendor_F_Command(value) => value.write_to(buffer),
            UciCommandDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciCommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciCommandPacket> for Bytes {
    fn from(packet: UciCommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciCommandPacket> for Vec<u8> {
    fn from(packet: UciCommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciCommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciCommandPacket {
    pub fn specialize(&self) -> UciCommandChild {
        match &self.uci_command.child {
            UciCommandDataChild::CoreCommand(_) => UciCommandChild::CoreCommand(
                CoreCommandPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciCommandDataChild::SessionCommand(_) => UciCommandChild::SessionCommand(
                SessionCommandPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciCommandDataChild::RangingCommand(_) => UciCommandChild::RangingCommand(
                RangingCommandPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciCommandDataChild::AndroidCommand(_) => UciCommandChild::AndroidCommand(
                AndroidCommandPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciCommandDataChild::UciVendor_A_Command(_) => UciCommandChild::UciVendor_A_Command(
                UciVendor_A_CommandPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciCommandDataChild::UciVendor_B_Command(_) => UciCommandChild::UciVendor_B_Command(
                UciVendor_B_CommandPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciCommandDataChild::UciVendor_E_Command(_) => UciCommandChild::UciVendor_E_Command(
                UciVendor_E_CommandPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciCommandDataChild::UciVendor_F_Command(_) => UciCommandChild::UciVendor_F_Command(
                UciVendor_F_CommandPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciCommandDataChild::Payload(p) => UciCommandChild::Payload(p.clone()),
            UciCommandDataChild::None => UciCommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciCommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl UciCommandBuilder {
    pub fn build(self) -> UciCommandPacket {
        let uci_command = Arc::new(UciCommandData {
            child: match self.payload {
                None => UciCommandDataChild::None,
                Some(bytes) => UciCommandDataChild::Payload(bytes),
            },
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: self.group_id,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        UciCommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciCommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciResponseDataChild {
    CoreResponse(Arc<CoreResponseData>),
    SessionResponse(Arc<SessionResponseData>),
    RangingResponse(Arc<RangingResponseData>),
    AndroidResponse(Arc<AndroidResponseData>),
    UciVendor_A_Response(Arc<UciVendor_A_ResponseData>),
    UciVendor_B_Response(Arc<UciVendor_B_ResponseData>),
    UciVendor_E_Response(Arc<UciVendor_E_ResponseData>),
    UciVendor_F_Response(Arc<UciVendor_F_ResponseData>),
    Payload(Bytes),
    None,
}
impl UciResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciResponseDataChild::CoreResponse(value) => value.get_total_size(),
            UciResponseDataChild::SessionResponse(value) => value.get_total_size(),
            UciResponseDataChild::RangingResponse(value) => value.get_total_size(),
            UciResponseDataChild::AndroidResponse(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_A_Response(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_B_Response(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_E_Response(value) => value.get_total_size(),
            UciResponseDataChild::UciVendor_F_Response(value) => value.get_total_size(),
            UciResponseDataChild::Payload(p) => p.len(),
            UciResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciResponseChild {
    CoreResponse(CoreResponsePacket),
    SessionResponse(SessionResponsePacket),
    RangingResponse(RangingResponsePacket),
    AndroidResponse(AndroidResponsePacket),
    UciVendor_A_Response(UciVendor_A_ResponsePacket),
    UciVendor_B_Response(UciVendor_B_ResponsePacket),
    UciVendor_E_Response(UciVendor_E_ResponsePacket),
    UciVendor_F_Response(UciVendor_F_ResponsePacket),
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciResponseData {
    child: UciResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct UciResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
}
#[derive(Debug)]
pub struct UciResponseBuilder {
    pub group_id: GroupId,
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], group_id: GroupId, opcode: u8) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = match (group_id) {
            (GroupId::Core) if CoreResponseData::conforms(&bytes[..]) => {
                UciResponseDataChild::CoreResponse(Arc::new(CoreResponseData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::SessionConfig) if SessionResponseData::conforms(&bytes[..]) => {
                UciResponseDataChild::SessionResponse(Arc::new(SessionResponseData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::RangingSessionControl) if RangingResponseData::conforms(&bytes[..]) => {
                UciResponseDataChild::RangingResponse(Arc::new(RangingResponseData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::VendorAndroid) if AndroidResponseData::conforms(&bytes[..]) => {
                UciResponseDataChild::AndroidResponse(Arc::new(AndroidResponseData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::VendorReservedA) if UciVendor_A_ResponseData::conforms(&bytes[..]) => {
                UciResponseDataChild::UciVendor_A_Response(Arc::new(
                    UciVendor_A_ResponseData::parse(&bytes[..])?,
                ))
            }
            (GroupId::VendorReservedB) if UciVendor_B_ResponseData::conforms(&bytes[..]) => {
                UciResponseDataChild::UciVendor_B_Response(Arc::new(
                    UciVendor_B_ResponseData::parse(&bytes[..])?,
                ))
            }
            (GroupId::VendorReservedE) if UciVendor_E_ResponseData::conforms(&bytes[..]) => {
                UciResponseDataChild::UciVendor_E_Response(Arc::new(
                    UciVendor_E_ResponseData::parse(&bytes[..])?,
                ))
            }
            (GroupId::VendorReservedF) if UciVendor_F_ResponseData::conforms(&bytes[..]) => {
                UciResponseDataChild::UciVendor_F_Response(Arc::new(
                    UciVendor_F_ResponseData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciResponseDataChild::CoreResponse(value) => value.write_to(buffer),
            UciResponseDataChild::SessionResponse(value) => value.write_to(buffer),
            UciResponseDataChild::RangingResponse(value) => value.write_to(buffer),
            UciResponseDataChild::AndroidResponse(value) => value.write_to(buffer),
            UciResponseDataChild::UciVendor_A_Response(value) => value.write_to(buffer),
            UciResponseDataChild::UciVendor_B_Response(value) => value.write_to(buffer),
            UciResponseDataChild::UciVendor_E_Response(value) => value.write_to(buffer),
            UciResponseDataChild::UciVendor_F_Response(value) => value.write_to(buffer),
            UciResponseDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciResponsePacket> for Bytes {
    fn from(packet: UciResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciResponsePacket> for Vec<u8> {
    fn from(packet: UciResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciResponsePacket {
    pub fn specialize(&self) -> UciResponseChild {
        match &self.uci_response.child {
            UciResponseDataChild::CoreResponse(_) => UciResponseChild::CoreResponse(
                CoreResponsePacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciResponseDataChild::SessionResponse(_) => UciResponseChild::SessionResponse(
                SessionResponsePacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciResponseDataChild::RangingResponse(_) => UciResponseChild::RangingResponse(
                RangingResponsePacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciResponseDataChild::AndroidResponse(_) => UciResponseChild::AndroidResponse(
                AndroidResponsePacket::new(self.uci_packet.clone()).unwrap(),
            ),
            UciResponseDataChild::UciVendor_A_Response(_) => {
                UciResponseChild::UciVendor_A_Response(
                    UciVendor_A_ResponsePacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciResponseDataChild::UciVendor_B_Response(_) => {
                UciResponseChild::UciVendor_B_Response(
                    UciVendor_B_ResponsePacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciResponseDataChild::UciVendor_E_Response(_) => {
                UciResponseChild::UciVendor_E_Response(
                    UciVendor_E_ResponsePacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciResponseDataChild::UciVendor_F_Response(_) => {
                UciResponseChild::UciVendor_F_Response(
                    UciVendor_F_ResponsePacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciResponseDataChild::Payload(p) => UciResponseChild::Payload(p.clone()),
            UciResponseDataChild::None => UciResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl UciResponseBuilder {
    pub fn build(self) -> UciResponsePacket {
        let uci_response = Arc::new(UciResponseData {
            child: match self.payload {
                None => UciResponseDataChild::None,
                Some(bytes) => UciResponseDataChild::Payload(bytes),
            },
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: self.group_id,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        UciResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciNotificationDataChild {
    CoreNotification(Arc<CoreNotificationData>),
    SessionNotification(Arc<SessionNotificationData>),
    RangingNotification(Arc<RangingNotificationData>),
    AndroidNotification(Arc<AndroidNotificationData>),
    UciVendor_A_Notification(Arc<UciVendor_A_NotificationData>),
    UciVendor_B_Notification(Arc<UciVendor_B_NotificationData>),
    UciVendor_E_Notification(Arc<UciVendor_E_NotificationData>),
    UciVendor_F_Notification(Arc<UciVendor_F_NotificationData>),
    Payload(Bytes),
    None,
}
impl UciNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciNotificationDataChild::CoreNotification(value) => value.get_total_size(),
            UciNotificationDataChild::SessionNotification(value) => value.get_total_size(),
            UciNotificationDataChild::RangingNotification(value) => value.get_total_size(),
            UciNotificationDataChild::AndroidNotification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_A_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_B_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_E_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::UciVendor_F_Notification(value) => value.get_total_size(),
            UciNotificationDataChild::Payload(p) => p.len(),
            UciNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciNotificationChild {
    CoreNotification(CoreNotificationPacket),
    SessionNotification(SessionNotificationPacket),
    RangingNotification(RangingNotificationPacket),
    AndroidNotification(AndroidNotificationPacket),
    UciVendor_A_Notification(UciVendor_A_NotificationPacket),
    UciVendor_B_Notification(UciVendor_B_NotificationPacket),
    UciVendor_E_Notification(UciVendor_E_NotificationPacket),
    UciVendor_F_Notification(UciVendor_F_NotificationPacket),
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciNotificationData {
    child: UciNotificationDataChild,
}
#[derive(Debug, Clone)]
pub struct UciNotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
}
#[derive(Debug)]
pub struct UciNotificationBuilder {
    pub group_id: GroupId,
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], group_id: GroupId, opcode: u8) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = match (group_id) {
            (GroupId::Core) if CoreNotificationData::conforms(&bytes[..]) => {
                UciNotificationDataChild::CoreNotification(Arc::new(CoreNotificationData::parse(
                    &bytes[..],
                    opcode,
                )?))
            }
            (GroupId::SessionConfig) if SessionNotificationData::conforms(&bytes[..]) => {
                UciNotificationDataChild::SessionNotification(Arc::new(
                    SessionNotificationData::parse(&bytes[..], opcode)?,
                ))
            }
            (GroupId::RangingSessionControl) if RangingNotificationData::conforms(&bytes[..]) => {
                UciNotificationDataChild::RangingNotification(Arc::new(
                    RangingNotificationData::parse(&bytes[..], opcode)?,
                ))
            }
            (GroupId::VendorAndroid) if AndroidNotificationData::conforms(&bytes[..]) => {
                UciNotificationDataChild::AndroidNotification(Arc::new(
                    AndroidNotificationData::parse(&bytes[..])?,
                ))
            }
            (GroupId::VendorReservedA) if UciVendor_A_NotificationData::conforms(&bytes[..]) => {
                UciNotificationDataChild::UciVendor_A_Notification(Arc::new(
                    UciVendor_A_NotificationData::parse(&bytes[..])?,
                ))
            }
            (GroupId::VendorReservedB) if UciVendor_B_NotificationData::conforms(&bytes[..]) => {
                UciNotificationDataChild::UciVendor_B_Notification(Arc::new(
                    UciVendor_B_NotificationData::parse(&bytes[..])?,
                ))
            }
            (GroupId::VendorReservedE) if UciVendor_E_NotificationData::conforms(&bytes[..]) => {
                UciNotificationDataChild::UciVendor_E_Notification(Arc::new(
                    UciVendor_E_NotificationData::parse(&bytes[..])?,
                ))
            }
            (GroupId::VendorReservedF) if UciVendor_F_NotificationData::conforms(&bytes[..]) => {
                UciNotificationDataChild::UciVendor_F_Notification(Arc::new(
                    UciVendor_F_NotificationData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciNotificationDataChild::CoreNotification(value) => value.write_to(buffer),
            UciNotificationDataChild::SessionNotification(value) => value.write_to(buffer),
            UciNotificationDataChild::RangingNotification(value) => value.write_to(buffer),
            UciNotificationDataChild::AndroidNotification(value) => value.write_to(buffer),
            UciNotificationDataChild::UciVendor_A_Notification(value) => value.write_to(buffer),
            UciNotificationDataChild::UciVendor_B_Notification(value) => value.write_to(buffer),
            UciNotificationDataChild::UciVendor_E_Notification(value) => value.write_to(buffer),
            UciNotificationDataChild::UciVendor_F_Notification(value) => value.write_to(buffer),
            UciNotificationDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciNotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciNotificationPacket> for Bytes {
    fn from(packet: UciNotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciNotificationPacket> for Vec<u8> {
    fn from(packet: UciNotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciNotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciNotificationPacket {
    pub fn specialize(&self) -> UciNotificationChild {
        match &self.uci_notification.child {
            UciNotificationDataChild::CoreNotification(_) => {
                UciNotificationChild::CoreNotification(
                    CoreNotificationPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::SessionNotification(_) => {
                UciNotificationChild::SessionNotification(
                    SessionNotificationPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::RangingNotification(_) => {
                UciNotificationChild::RangingNotification(
                    RangingNotificationPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::AndroidNotification(_) => {
                UciNotificationChild::AndroidNotification(
                    AndroidNotificationPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_A_Notification(_) => {
                UciNotificationChild::UciVendor_A_Notification(
                    UciVendor_A_NotificationPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_B_Notification(_) => {
                UciNotificationChild::UciVendor_B_Notification(
                    UciVendor_B_NotificationPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_E_Notification(_) => {
                UciNotificationChild::UciVendor_E_Notification(
                    UciVendor_E_NotificationPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::UciVendor_F_Notification(_) => {
                UciNotificationChild::UciVendor_F_Notification(
                    UciVendor_F_NotificationPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            UciNotificationDataChild::Payload(p) => UciNotificationChild::Payload(p.clone()),
            UciNotificationDataChild::None => UciNotificationChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciNotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl UciNotificationBuilder {
    pub fn build(self) -> UciNotificationPacket {
        let uci_notification = Arc::new(UciNotificationData {
            child: match self.payload {
                None => UciNotificationDataChild::None,
                Some(bytes) => UciNotificationDataChild::Payload(bytes),
            },
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: self.group_id,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        UciNotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciNotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum CoreCommandDataChild {
    DeviceResetCmd(Arc<DeviceResetCmdData>),
    GetDeviceInfoCmd(Arc<GetDeviceInfoCmdData>),
    GetCapsInfoCmd(Arc<GetCapsInfoCmdData>),
    SetConfigCmd(Arc<SetConfigCmdData>),
    GetConfigCmd(Arc<GetConfigCmdData>),
    None,
}
impl CoreCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            CoreCommandDataChild::DeviceResetCmd(value) => value.get_total_size(),
            CoreCommandDataChild::GetDeviceInfoCmd(value) => value.get_total_size(),
            CoreCommandDataChild::GetCapsInfoCmd(value) => value.get_total_size(),
            CoreCommandDataChild::SetConfigCmd(value) => value.get_total_size(),
            CoreCommandDataChild::GetConfigCmd(value) => value.get_total_size(),
            CoreCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum CoreCommandChild {
    DeviceResetCmd(DeviceResetCmdPacket),
    GetDeviceInfoCmd(GetDeviceInfoCmdPacket),
    GetCapsInfoCmd(GetCapsInfoCmdPacket),
    SetConfigCmd(SetConfigCmdPacket),
    GetConfigCmd(GetConfigCmdPacket),
    None,
}
#[derive(Debug)]
struct CoreCommandData {
    child: CoreCommandDataChild,
}
#[derive(Debug, Clone)]
pub struct CoreCommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    core_command: Arc<CoreCommandData>,
}
#[derive(Debug)]
pub struct CoreCommandBuilder {
    pub opcode: u8,
}
impl CoreCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (0) if DeviceResetCmdData::conforms(&bytes[..]) => {
                CoreCommandDataChild::DeviceResetCmd(Arc::new(DeviceResetCmdData::parse(
                    &bytes[..],
                )?))
            }
            (2) if GetDeviceInfoCmdData::conforms(&bytes[..]) => {
                CoreCommandDataChild::GetDeviceInfoCmd(Arc::new(GetDeviceInfoCmdData::parse(
                    &bytes[..],
                )?))
            }
            (3) if GetCapsInfoCmdData::conforms(&bytes[..]) => {
                CoreCommandDataChild::GetCapsInfoCmd(Arc::new(GetCapsInfoCmdData::parse(
                    &bytes[..],
                )?))
            }
            (4) if SetConfigCmdData::conforms(&bytes[..]) => {
                CoreCommandDataChild::SetConfigCmd(Arc::new(SetConfigCmdData::parse(&bytes[..])?))
            }
            (5) if GetConfigCmdData::conforms(&bytes[..]) => {
                CoreCommandDataChild::GetConfigCmd(Arc::new(GetConfigCmdData::parse(&bytes[..])?))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            CoreCommandDataChild::DeviceResetCmd(value) => value.write_to(buffer),
            CoreCommandDataChild::GetDeviceInfoCmd(value) => value.write_to(buffer),
            CoreCommandDataChild::GetCapsInfoCmd(value) => value.write_to(buffer),
            CoreCommandDataChild::SetConfigCmd(value) => value.write_to(buffer),
            CoreCommandDataChild::GetConfigCmd(value) => value.write_to(buffer),
            CoreCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for CoreCommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<CoreCommandPacket> for Bytes {
    fn from(packet: CoreCommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<CoreCommandPacket> for Vec<u8> {
    fn from(packet: CoreCommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for CoreCommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl CoreCommandPacket {
    pub fn specialize(&self) -> CoreCommandChild {
        match &self.core_command.child {
            CoreCommandDataChild::DeviceResetCmd(_) => CoreCommandChild::DeviceResetCmd(
                DeviceResetCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreCommandDataChild::GetDeviceInfoCmd(_) => CoreCommandChild::GetDeviceInfoCmd(
                GetDeviceInfoCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreCommandDataChild::GetCapsInfoCmd(_) => CoreCommandChild::GetCapsInfoCmd(
                GetCapsInfoCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreCommandDataChild::SetConfigCmd(_) => CoreCommandChild::SetConfigCmd(
                SetConfigCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreCommandDataChild::GetConfigCmd(_) => CoreCommandChild::GetConfigCmd(
                GetConfigCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreCommandDataChild::None => CoreCommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let core_command = match &uci_command.child {
            UciCommandDataChild::CoreCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreCommand"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            core_command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for CoreCommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for CoreCommandPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl CoreCommandBuilder {
    pub fn build(self) -> CoreCommandPacket {
        let core_command = Arc::new(CoreCommandData {
            child: CoreCommandDataChild::None,
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::CoreCommand(core_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        CoreCommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for CoreCommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for CoreCommandBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum CoreResponseDataChild {
    DeviceResetRsp(Arc<DeviceResetRspData>),
    GetDeviceInfoRsp(Arc<GetDeviceInfoRspData>),
    GetCapsInfoRsp(Arc<GetCapsInfoRspData>),
    SetConfigRsp(Arc<SetConfigRspData>),
    GetConfigRsp(Arc<GetConfigRspData>),
    None,
}
impl CoreResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            CoreResponseDataChild::DeviceResetRsp(value) => value.get_total_size(),
            CoreResponseDataChild::GetDeviceInfoRsp(value) => value.get_total_size(),
            CoreResponseDataChild::GetCapsInfoRsp(value) => value.get_total_size(),
            CoreResponseDataChild::SetConfigRsp(value) => value.get_total_size(),
            CoreResponseDataChild::GetConfigRsp(value) => value.get_total_size(),
            CoreResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum CoreResponseChild {
    DeviceResetRsp(DeviceResetRspPacket),
    GetDeviceInfoRsp(GetDeviceInfoRspPacket),
    GetCapsInfoRsp(GetCapsInfoRspPacket),
    SetConfigRsp(SetConfigRspPacket),
    GetConfigRsp(GetConfigRspPacket),
    None,
}
#[derive(Debug)]
struct CoreResponseData {
    child: CoreResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct CoreResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    core_response: Arc<CoreResponseData>,
}
#[derive(Debug)]
pub struct CoreResponseBuilder {
    pub opcode: u8,
}
impl CoreResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (0) if DeviceResetRspData::conforms(&bytes[..]) => {
                CoreResponseDataChild::DeviceResetRsp(Arc::new(DeviceResetRspData::parse(
                    &bytes[..],
                )?))
            }
            (2) if GetDeviceInfoRspData::conforms(&bytes[..]) => {
                CoreResponseDataChild::GetDeviceInfoRsp(Arc::new(GetDeviceInfoRspData::parse(
                    &bytes[..],
                )?))
            }
            (3) if GetCapsInfoRspData::conforms(&bytes[..]) => {
                CoreResponseDataChild::GetCapsInfoRsp(Arc::new(GetCapsInfoRspData::parse(
                    &bytes[..],
                )?))
            }
            (4) if SetConfigRspData::conforms(&bytes[..]) => {
                CoreResponseDataChild::SetConfigRsp(Arc::new(SetConfigRspData::parse(&bytes[..])?))
            }
            (5) if GetConfigRspData::conforms(&bytes[..]) => {
                CoreResponseDataChild::GetConfigRsp(Arc::new(GetConfigRspData::parse(&bytes[..])?))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            CoreResponseDataChild::DeviceResetRsp(value) => value.write_to(buffer),
            CoreResponseDataChild::GetDeviceInfoRsp(value) => value.write_to(buffer),
            CoreResponseDataChild::GetCapsInfoRsp(value) => value.write_to(buffer),
            CoreResponseDataChild::SetConfigRsp(value) => value.write_to(buffer),
            CoreResponseDataChild::GetConfigRsp(value) => value.write_to(buffer),
            CoreResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for CoreResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<CoreResponsePacket> for Bytes {
    fn from(packet: CoreResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<CoreResponsePacket> for Vec<u8> {
    fn from(packet: CoreResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for CoreResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl CoreResponsePacket {
    pub fn specialize(&self) -> CoreResponseChild {
        match &self.core_response.child {
            CoreResponseDataChild::DeviceResetRsp(_) => CoreResponseChild::DeviceResetRsp(
                DeviceResetRspPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreResponseDataChild::GetDeviceInfoRsp(_) => CoreResponseChild::GetDeviceInfoRsp(
                GetDeviceInfoRspPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreResponseDataChild::GetCapsInfoRsp(_) => CoreResponseChild::GetCapsInfoRsp(
                GetCapsInfoRspPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreResponseDataChild::SetConfigRsp(_) => CoreResponseChild::SetConfigRsp(
                SetConfigRspPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreResponseDataChild::GetConfigRsp(_) => CoreResponseChild::GetConfigRsp(
                GetConfigRspPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreResponseDataChild::None => CoreResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let core_response = match &uci_response.child {
            UciResponseDataChild::CoreResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreResponse"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            core_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for CoreResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for CoreResponsePacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl CoreResponseBuilder {
    pub fn build(self) -> CoreResponsePacket {
        let core_response = Arc::new(CoreResponseData {
            child: CoreResponseDataChild::None,
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::CoreResponse(core_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        CoreResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for CoreResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for CoreResponseBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum CoreNotificationDataChild {
    DeviceStatusNtf(Arc<DeviceStatusNtfData>),
    GenericError(Arc<GenericErrorData>),
    None,
}
impl CoreNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            CoreNotificationDataChild::DeviceStatusNtf(value) => value.get_total_size(),
            CoreNotificationDataChild::GenericError(value) => value.get_total_size(),
            CoreNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum CoreNotificationChild {
    DeviceStatusNtf(DeviceStatusNtfPacket),
    GenericError(GenericErrorPacket),
    None,
}
#[derive(Debug)]
struct CoreNotificationData {
    child: CoreNotificationDataChild,
}
#[derive(Debug, Clone)]
pub struct CoreNotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    core_notification: Arc<CoreNotificationData>,
}
#[derive(Debug)]
pub struct CoreNotificationBuilder {
    pub opcode: u8,
}
impl CoreNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (1) if DeviceStatusNtfData::conforms(&bytes[..]) => {
                CoreNotificationDataChild::DeviceStatusNtf(Arc::new(DeviceStatusNtfData::parse(
                    &bytes[..],
                )?))
            }
            (7) if GenericErrorData::conforms(&bytes[..]) => {
                CoreNotificationDataChild::GenericError(Arc::new(GenericErrorData::parse(
                    &bytes[..],
                )?))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            CoreNotificationDataChild::DeviceStatusNtf(value) => value.write_to(buffer),
            CoreNotificationDataChild::GenericError(value) => value.write_to(buffer),
            CoreNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for CoreNotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<CoreNotificationPacket> for Bytes {
    fn from(packet: CoreNotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<CoreNotificationPacket> for Vec<u8> {
    fn from(packet: CoreNotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for CoreNotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl CoreNotificationPacket {
    pub fn specialize(&self) -> CoreNotificationChild {
        match &self.core_notification.child {
            CoreNotificationDataChild::DeviceStatusNtf(_) => {
                CoreNotificationChild::DeviceStatusNtf(
                    DeviceStatusNtfPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            CoreNotificationDataChild::GenericError(_) => CoreNotificationChild::GenericError(
                GenericErrorPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            CoreNotificationDataChild::None => CoreNotificationChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let core_notification = match &uci_notification.child {
            UciNotificationDataChild::CoreNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreNotification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            core_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for CoreNotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for CoreNotificationPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl CoreNotificationBuilder {
    pub fn build(self) -> CoreNotificationPacket {
        let core_notification = Arc::new(CoreNotificationData {
            child: CoreNotificationDataChild::None,
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::CoreNotification(core_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        CoreNotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for CoreNotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for CoreNotificationBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum SessionCommandDataChild {
    SessionInitCmd(Arc<SessionInitCmdData>),
    SessionDeinitCmd(Arc<SessionDeinitCmdData>),
    SessionSetAppConfigCmd(Arc<SessionSetAppConfigCmdData>),
    SessionGetAppConfigCmd(Arc<SessionGetAppConfigCmdData>),
    SessionGetCountCmd(Arc<SessionGetCountCmdData>),
    SessionGetStateCmd(Arc<SessionGetStateCmdData>),
    SessionUpdateControllerMulticastListCmd(Arc<SessionUpdateControllerMulticastListCmdData>),
    None,
}
impl SessionCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionCommandDataChild::SessionInitCmd(value) => value.get_total_size(),
            SessionCommandDataChild::SessionDeinitCmd(value) => value.get_total_size(),
            SessionCommandDataChild::SessionSetAppConfigCmd(value) => value.get_total_size(),
            SessionCommandDataChild::SessionGetAppConfigCmd(value) => value.get_total_size(),
            SessionCommandDataChild::SessionGetCountCmd(value) => value.get_total_size(),
            SessionCommandDataChild::SessionGetStateCmd(value) => value.get_total_size(),
            SessionCommandDataChild::SessionUpdateControllerMulticastListCmd(value) => {
                value.get_total_size()
            }
            SessionCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum SessionCommandChild {
    SessionInitCmd(SessionInitCmdPacket),
    SessionDeinitCmd(SessionDeinitCmdPacket),
    SessionSetAppConfigCmd(SessionSetAppConfigCmdPacket),
    SessionGetAppConfigCmd(SessionGetAppConfigCmdPacket),
    SessionGetCountCmd(SessionGetCountCmdPacket),
    SessionGetStateCmd(SessionGetStateCmdPacket),
    SessionUpdateControllerMulticastListCmd(SessionUpdateControllerMulticastListCmdPacket),
    None,
}
#[derive(Debug)]
struct SessionCommandData {
    child: SessionCommandDataChild,
}
#[derive(Debug, Clone)]
pub struct SessionCommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    session_command: Arc<SessionCommandData>,
}
#[derive(Debug)]
pub struct SessionCommandBuilder {
    pub opcode: u8,
}
impl SessionCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (0) if SessionInitCmdData::conforms(&bytes[..]) => {
                SessionCommandDataChild::SessionInitCmd(Arc::new(SessionInitCmdData::parse(
                    &bytes[..],
                )?))
            }
            (1) if SessionDeinitCmdData::conforms(&bytes[..]) => {
                SessionCommandDataChild::SessionDeinitCmd(Arc::new(SessionDeinitCmdData::parse(
                    &bytes[..],
                )?))
            }
            (3) if SessionSetAppConfigCmdData::conforms(&bytes[..]) => {
                SessionCommandDataChild::SessionSetAppConfigCmd(Arc::new(
                    SessionSetAppConfigCmdData::parse(&bytes[..])?,
                ))
            }
            (4) if SessionGetAppConfigCmdData::conforms(&bytes[..]) => {
                SessionCommandDataChild::SessionGetAppConfigCmd(Arc::new(
                    SessionGetAppConfigCmdData::parse(&bytes[..])?,
                ))
            }
            (5) if SessionGetCountCmdData::conforms(&bytes[..]) => {
                SessionCommandDataChild::SessionGetCountCmd(Arc::new(
                    SessionGetCountCmdData::parse(&bytes[..])?,
                ))
            }
            (6) if SessionGetStateCmdData::conforms(&bytes[..]) => {
                SessionCommandDataChild::SessionGetStateCmd(Arc::new(
                    SessionGetStateCmdData::parse(&bytes[..])?,
                ))
            }
            (7) if SessionUpdateControllerMulticastListCmdData::conforms(&bytes[..]) => {
                SessionCommandDataChild::SessionUpdateControllerMulticastListCmd(Arc::new(
                    SessionUpdateControllerMulticastListCmdData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            SessionCommandDataChild::SessionInitCmd(value) => value.write_to(buffer),
            SessionCommandDataChild::SessionDeinitCmd(value) => value.write_to(buffer),
            SessionCommandDataChild::SessionSetAppConfigCmd(value) => value.write_to(buffer),
            SessionCommandDataChild::SessionGetAppConfigCmd(value) => value.write_to(buffer),
            SessionCommandDataChild::SessionGetCountCmd(value) => value.write_to(buffer),
            SessionCommandDataChild::SessionGetStateCmd(value) => value.write_to(buffer),
            SessionCommandDataChild::SessionUpdateControllerMulticastListCmd(value) => {
                value.write_to(buffer)
            }
            SessionCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for SessionCommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionCommandPacket> for Bytes {
    fn from(packet: SessionCommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionCommandPacket> for Vec<u8> {
    fn from(packet: SessionCommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionCommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionCommandPacket {
    pub fn specialize(&self) -> SessionCommandChild {
        match &self.session_command.child {
            SessionCommandDataChild::SessionInitCmd(_) => SessionCommandChild::SessionInitCmd(
                SessionInitCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            SessionCommandDataChild::SessionDeinitCmd(_) => SessionCommandChild::SessionDeinitCmd(
                SessionDeinitCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            SessionCommandDataChild::SessionSetAppConfigCmd(_) => {
                SessionCommandChild::SessionSetAppConfigCmd(
                    SessionSetAppConfigCmdPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionCommandDataChild::SessionGetAppConfigCmd(_) => {
                SessionCommandChild::SessionGetAppConfigCmd(
                    SessionGetAppConfigCmdPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionCommandDataChild::SessionGetCountCmd(_) => {
                SessionCommandChild::SessionGetCountCmd(
                    SessionGetCountCmdPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionCommandDataChild::SessionGetStateCmd(_) => {
                SessionCommandChild::SessionGetStateCmd(
                    SessionGetStateCmdPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionCommandDataChild::SessionUpdateControllerMulticastListCmd(_) => {
                SessionCommandChild::SessionUpdateControllerMulticastListCmd(
                    SessionUpdateControllerMulticastListCmdPacket::new(self.uci_packet.clone())
                        .unwrap(),
                )
            }
            SessionCommandDataChild::None => SessionCommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let session_command = match &uci_command.child {
            UciCommandDataChild::SessionCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionCommand"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            session_command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for SessionCommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SessionCommandPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionCommandBuilder {
    pub fn build(self) -> SessionCommandPacket {
        let session_command = Arc::new(SessionCommandData {
            child: SessionCommandDataChild::None,
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::SessionCommand(session_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SessionCommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionCommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SessionCommandBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum SessionResponseDataChild {
    SessionInitRsp(Arc<SessionInitRspData>),
    SessionDeinitRsp(Arc<SessionDeinitRspData>),
    SessionSetAppConfigRsp(Arc<SessionSetAppConfigRspData>),
    SessionGetAppConfigRsp(Arc<SessionGetAppConfigRspData>),
    SessionGetCountRsp(Arc<SessionGetCountRspData>),
    SessionGetStateRsp(Arc<SessionGetStateRspData>),
    SessionUpdateControllerMulticastListRsp(Arc<SessionUpdateControllerMulticastListRspData>),
    None,
}
impl SessionResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionResponseDataChild::SessionInitRsp(value) => value.get_total_size(),
            SessionResponseDataChild::SessionDeinitRsp(value) => value.get_total_size(),
            SessionResponseDataChild::SessionSetAppConfigRsp(value) => value.get_total_size(),
            SessionResponseDataChild::SessionGetAppConfigRsp(value) => value.get_total_size(),
            SessionResponseDataChild::SessionGetCountRsp(value) => value.get_total_size(),
            SessionResponseDataChild::SessionGetStateRsp(value) => value.get_total_size(),
            SessionResponseDataChild::SessionUpdateControllerMulticastListRsp(value) => {
                value.get_total_size()
            }
            SessionResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum SessionResponseChild {
    SessionInitRsp(SessionInitRspPacket),
    SessionDeinitRsp(SessionDeinitRspPacket),
    SessionSetAppConfigRsp(SessionSetAppConfigRspPacket),
    SessionGetAppConfigRsp(SessionGetAppConfigRspPacket),
    SessionGetCountRsp(SessionGetCountRspPacket),
    SessionGetStateRsp(SessionGetStateRspPacket),
    SessionUpdateControllerMulticastListRsp(SessionUpdateControllerMulticastListRspPacket),
    None,
}
#[derive(Debug)]
struct SessionResponseData {
    child: SessionResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct SessionResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    session_response: Arc<SessionResponseData>,
}
#[derive(Debug)]
pub struct SessionResponseBuilder {
    pub opcode: u8,
}
impl SessionResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (0) if SessionInitRspData::conforms(&bytes[..]) => {
                SessionResponseDataChild::SessionInitRsp(Arc::new(SessionInitRspData::parse(
                    &bytes[..],
                )?))
            }
            (1) if SessionDeinitRspData::conforms(&bytes[..]) => {
                SessionResponseDataChild::SessionDeinitRsp(Arc::new(SessionDeinitRspData::parse(
                    &bytes[..],
                )?))
            }
            (3) if SessionSetAppConfigRspData::conforms(&bytes[..]) => {
                SessionResponseDataChild::SessionSetAppConfigRsp(Arc::new(
                    SessionSetAppConfigRspData::parse(&bytes[..])?,
                ))
            }
            (4) if SessionGetAppConfigRspData::conforms(&bytes[..]) => {
                SessionResponseDataChild::SessionGetAppConfigRsp(Arc::new(
                    SessionGetAppConfigRspData::parse(&bytes[..])?,
                ))
            }
            (5) if SessionGetCountRspData::conforms(&bytes[..]) => {
                SessionResponseDataChild::SessionGetCountRsp(Arc::new(
                    SessionGetCountRspData::parse(&bytes[..])?,
                ))
            }
            (6) if SessionGetStateRspData::conforms(&bytes[..]) => {
                SessionResponseDataChild::SessionGetStateRsp(Arc::new(
                    SessionGetStateRspData::parse(&bytes[..])?,
                ))
            }
            (7) if SessionUpdateControllerMulticastListRspData::conforms(&bytes[..]) => {
                SessionResponseDataChild::SessionUpdateControllerMulticastListRsp(Arc::new(
                    SessionUpdateControllerMulticastListRspData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            SessionResponseDataChild::SessionInitRsp(value) => value.write_to(buffer),
            SessionResponseDataChild::SessionDeinitRsp(value) => value.write_to(buffer),
            SessionResponseDataChild::SessionSetAppConfigRsp(value) => value.write_to(buffer),
            SessionResponseDataChild::SessionGetAppConfigRsp(value) => value.write_to(buffer),
            SessionResponseDataChild::SessionGetCountRsp(value) => value.write_to(buffer),
            SessionResponseDataChild::SessionGetStateRsp(value) => value.write_to(buffer),
            SessionResponseDataChild::SessionUpdateControllerMulticastListRsp(value) => {
                value.write_to(buffer)
            }
            SessionResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for SessionResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionResponsePacket> for Bytes {
    fn from(packet: SessionResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionResponsePacket> for Vec<u8> {
    fn from(packet: SessionResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionResponsePacket {
    pub fn specialize(&self) -> SessionResponseChild {
        match &self.session_response.child {
            SessionResponseDataChild::SessionInitRsp(_) => SessionResponseChild::SessionInitRsp(
                SessionInitRspPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            SessionResponseDataChild::SessionDeinitRsp(_) => {
                SessionResponseChild::SessionDeinitRsp(
                    SessionDeinitRspPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionResponseDataChild::SessionSetAppConfigRsp(_) => {
                SessionResponseChild::SessionSetAppConfigRsp(
                    SessionSetAppConfigRspPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionResponseDataChild::SessionGetAppConfigRsp(_) => {
                SessionResponseChild::SessionGetAppConfigRsp(
                    SessionGetAppConfigRspPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionResponseDataChild::SessionGetCountRsp(_) => {
                SessionResponseChild::SessionGetCountRsp(
                    SessionGetCountRspPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionResponseDataChild::SessionGetStateRsp(_) => {
                SessionResponseChild::SessionGetStateRsp(
                    SessionGetStateRspPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionResponseDataChild::SessionUpdateControllerMulticastListRsp(_) => {
                SessionResponseChild::SessionUpdateControllerMulticastListRsp(
                    SessionUpdateControllerMulticastListRspPacket::new(self.uci_packet.clone())
                        .unwrap(),
                )
            }
            SessionResponseDataChild::None => SessionResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let session_response = match &uci_response.child {
            UciResponseDataChild::SessionResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionResponse"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            session_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for SessionResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SessionResponsePacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SessionResponseBuilder {
    pub fn build(self) -> SessionResponsePacket {
        let session_response = Arc::new(SessionResponseData {
            child: SessionResponseDataChild::None,
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::SessionResponse(session_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SessionResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SessionResponseBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum SessionNotificationDataChild {
    SessionStatusNtf(Arc<SessionStatusNtfData>),
    SessionUpdateControllerMulticastListNtf(Arc<SessionUpdateControllerMulticastListNtfData>),
    None,
}
impl SessionNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            SessionNotificationDataChild::SessionStatusNtf(value) => value.get_total_size(),
            SessionNotificationDataChild::SessionUpdateControllerMulticastListNtf(value) => {
                value.get_total_size()
            }
            SessionNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum SessionNotificationChild {
    SessionStatusNtf(SessionStatusNtfPacket),
    SessionUpdateControllerMulticastListNtf(SessionUpdateControllerMulticastListNtfPacket),
    None,
}
#[derive(Debug)]
struct SessionNotificationData {
    child: SessionNotificationDataChild,
}
#[derive(Debug, Clone)]
pub struct SessionNotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    session_notification: Arc<SessionNotificationData>,
}
#[derive(Debug)]
pub struct SessionNotificationBuilder {
    pub opcode: u8,
}
impl SessionNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (2) if SessionStatusNtfData::conforms(&bytes[..]) => {
                SessionNotificationDataChild::SessionStatusNtf(Arc::new(
                    SessionStatusNtfData::parse(&bytes[..])?,
                ))
            }
            (7) if SessionUpdateControllerMulticastListNtfData::conforms(&bytes[..]) => {
                SessionNotificationDataChild::SessionUpdateControllerMulticastListNtf(Arc::new(
                    SessionUpdateControllerMulticastListNtfData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            SessionNotificationDataChild::SessionStatusNtf(value) => value.write_to(buffer),
            SessionNotificationDataChild::SessionUpdateControllerMulticastListNtf(value) => {
                value.write_to(buffer)
            }
            SessionNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for SessionNotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionNotificationPacket> for Bytes {
    fn from(packet: SessionNotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionNotificationPacket> for Vec<u8> {
    fn from(packet: SessionNotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionNotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionNotificationPacket {
    pub fn specialize(&self) -> SessionNotificationChild {
        match &self.session_notification.child {
            SessionNotificationDataChild::SessionStatusNtf(_) => {
                SessionNotificationChild::SessionStatusNtf(
                    SessionStatusNtfPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            SessionNotificationDataChild::SessionUpdateControllerMulticastListNtf(_) => {
                SessionNotificationChild::SessionUpdateControllerMulticastListNtf(
                    SessionUpdateControllerMulticastListNtfPacket::new(self.uci_packet.clone())
                        .unwrap(),
                )
            }
            SessionNotificationDataChild::None => SessionNotificationChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let session_notification = match &uci_notification.child {
            UciNotificationDataChild::SessionNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionNotification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            session_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for SessionNotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for SessionNotificationPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionNotificationBuilder {
    pub fn build(self) -> SessionNotificationPacket {
        let session_notification = Arc::new(SessionNotificationData {
            child: SessionNotificationDataChild::None,
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::SessionNotification(session_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        SessionNotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionNotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for SessionNotificationBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum RangingCommandDataChild {
    RangeStartCmd(Arc<RangeStartCmdData>),
    RangeStopCmd(Arc<RangeStopCmdData>),
    RangeGetRangingCountCmd(Arc<RangeGetRangingCountCmdData>),
    None,
}
impl RangingCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            RangingCommandDataChild::RangeStartCmd(value) => value.get_total_size(),
            RangingCommandDataChild::RangeStopCmd(value) => value.get_total_size(),
            RangingCommandDataChild::RangeGetRangingCountCmd(value) => value.get_total_size(),
            RangingCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum RangingCommandChild {
    RangeStartCmd(RangeStartCmdPacket),
    RangeStopCmd(RangeStopCmdPacket),
    RangeGetRangingCountCmd(RangeGetRangingCountCmdPacket),
    None,
}
#[derive(Debug)]
struct RangingCommandData {
    session_id: u32,
    child: RangingCommandDataChild,
}
#[derive(Debug, Clone)]
pub struct RangingCommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    ranging_command: Arc<RangingCommandData>,
}
#[derive(Debug)]
pub struct RangingCommandBuilder {
    pub opcode: u8,
    pub session_id: u32,
}
impl RangingCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "RangingCommand".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let child = match (opcode) {
            (0) if RangeStartCmdData::conforms(&bytes[..]) => {
                RangingCommandDataChild::RangeStartCmd(Arc::new(RangeStartCmdData::parse(
                    &bytes[..],
                )?))
            }
            (1) if RangeStopCmdData::conforms(&bytes[..]) => RangingCommandDataChild::RangeStopCmd(
                Arc::new(RangeStopCmdData::parse(&bytes[..])?),
            ),
            (3) if RangeGetRangingCountCmdData::conforms(&bytes[..]) => {
                RangingCommandDataChild::RangeGetRangingCountCmd(Arc::new(
                    RangeGetRangingCountCmdData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { session_id, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
        match &self.child {
            RangingCommandDataChild::RangeStartCmd(value) => value.write_to(buffer),
            RangingCommandDataChild::RangeStopCmd(value) => value.write_to(buffer),
            RangingCommandDataChild::RangeGetRangingCountCmd(value) => value.write_to(buffer),
            RangingCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 4;
        ret
    }
}
impl Packet for RangingCommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangingCommandPacket> for Bytes {
    fn from(packet: RangingCommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangingCommandPacket> for Vec<u8> {
    fn from(packet: RangingCommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangingCommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangingCommandPacket {
    pub fn specialize(&self) -> RangingCommandChild {
        match &self.ranging_command.child {
            RangingCommandDataChild::RangeStartCmd(_) => RangingCommandChild::RangeStartCmd(
                RangeStartCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            RangingCommandDataChild::RangeStopCmd(_) => RangingCommandChild::RangeStopCmd(
                RangeStopCmdPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            RangingCommandDataChild::RangeGetRangingCountCmd(_) => {
                RangingCommandChild::RangeGetRangingCountCmd(
                    RangeGetRangingCountCmdPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            RangingCommandDataChild::None => RangingCommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let ranging_command = match &uci_command.child {
            UciCommandDataChild::RangingCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingCommand"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            ranging_command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.ranging_command.as_ref().session_id
    }
}
impl Into<UciPacketPacket> for RangingCommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for RangingCommandPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl RangingCommandBuilder {
    pub fn build(self) -> RangingCommandPacket {
        let ranging_command = Arc::new(RangingCommandData {
            session_id: self.session_id,
            child: RangingCommandDataChild::None,
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::RangingCommand(ranging_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        RangingCommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangingCommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for RangingCommandBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum RangingResponseDataChild {
    RangeStartRsp(Arc<RangeStartRspData>),
    RangeStopRsp(Arc<RangeStopRspData>),
    RangeGetRangingCountRsp(Arc<RangeGetRangingCountRspData>),
    None,
}
impl RangingResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            RangingResponseDataChild::RangeStartRsp(value) => value.get_total_size(),
            RangingResponseDataChild::RangeStopRsp(value) => value.get_total_size(),
            RangingResponseDataChild::RangeGetRangingCountRsp(value) => value.get_total_size(),
            RangingResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum RangingResponseChild {
    RangeStartRsp(RangeStartRspPacket),
    RangeStopRsp(RangeStopRspPacket),
    RangeGetRangingCountRsp(RangeGetRangingCountRspPacket),
    None,
}
#[derive(Debug)]
struct RangingResponseData {
    child: RangingResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct RangingResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    ranging_response: Arc<RangingResponseData>,
}
#[derive(Debug)]
pub struct RangingResponseBuilder {
    pub opcode: u8,
}
impl RangingResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (0) if RangeStartRspData::conforms(&bytes[..]) => {
                RangingResponseDataChild::RangeStartRsp(Arc::new(RangeStartRspData::parse(
                    &bytes[..],
                )?))
            }
            (1) if RangeStopRspData::conforms(&bytes[..]) => {
                RangingResponseDataChild::RangeStopRsp(Arc::new(RangeStopRspData::parse(
                    &bytes[..],
                )?))
            }
            (3) if RangeGetRangingCountRspData::conforms(&bytes[..]) => {
                RangingResponseDataChild::RangeGetRangingCountRsp(Arc::new(
                    RangeGetRangingCountRspData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            RangingResponseDataChild::RangeStartRsp(value) => value.write_to(buffer),
            RangingResponseDataChild::RangeStopRsp(value) => value.write_to(buffer),
            RangingResponseDataChild::RangeGetRangingCountRsp(value) => value.write_to(buffer),
            RangingResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for RangingResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangingResponsePacket> for Bytes {
    fn from(packet: RangingResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangingResponsePacket> for Vec<u8> {
    fn from(packet: RangingResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangingResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangingResponsePacket {
    pub fn specialize(&self) -> RangingResponseChild {
        match &self.ranging_response.child {
            RangingResponseDataChild::RangeStartRsp(_) => RangingResponseChild::RangeStartRsp(
                RangeStartRspPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            RangingResponseDataChild::RangeStopRsp(_) => RangingResponseChild::RangeStopRsp(
                RangeStopRspPacket::new(self.uci_packet.clone()).unwrap(),
            ),
            RangingResponseDataChild::RangeGetRangingCountRsp(_) => {
                RangingResponseChild::RangeGetRangingCountRsp(
                    RangeGetRangingCountRspPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            RangingResponseDataChild::None => RangingResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let ranging_response = match &uci_response.child {
            UciResponseDataChild::RangingResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingResponse"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            ranging_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for RangingResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for RangingResponsePacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl RangingResponseBuilder {
    pub fn build(self) -> RangingResponsePacket {
        let ranging_response = Arc::new(RangingResponseData {
            child: RangingResponseDataChild::None,
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::RangingResponse(ranging_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        RangingResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangingResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for RangingResponseBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum RangingNotificationDataChild {
    RangeDataNtf(Arc<RangeDataNtfData>),
    None,
}
impl RangingNotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            RangingNotificationDataChild::RangeDataNtf(value) => value.get_total_size(),
            RangingNotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum RangingNotificationChild {
    RangeDataNtf(RangeDataNtfPacket),
    None,
}
#[derive(Debug)]
struct RangingNotificationData {
    child: RangingNotificationDataChild,
}
#[derive(Debug, Clone)]
pub struct RangingNotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    ranging_notification: Arc<RangingNotificationData>,
}
#[derive(Debug)]
pub struct RangingNotificationBuilder {
    pub opcode: u8,
}
impl RangingNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match RangeDataNtfData::parse(&bytes[..]) {
            Ok(c) if RangeDataNtfData::conforms(&bytes[..]) => {
                RangingNotificationDataChild::RangeDataNtf(Arc::new(c))
            }
            Err(Error::InvalidLengthError { .. }) => RangingNotificationDataChild::None,
            _ => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            RangingNotificationDataChild::RangeDataNtf(value) => value.write_to(buffer),
            RangingNotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for RangingNotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangingNotificationPacket> for Bytes {
    fn from(packet: RangingNotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangingNotificationPacket> for Vec<u8> {
    fn from(packet: RangingNotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangingNotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangingNotificationPacket {
    pub fn specialize(&self) -> RangingNotificationChild {
        match &self.ranging_notification.child {
            RangingNotificationDataChild::RangeDataNtf(_) => {
                RangingNotificationChild::RangeDataNtf(
                    RangeDataNtfPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            RangingNotificationDataChild::None => RangingNotificationChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let ranging_notification = match &uci_notification.child {
            UciNotificationDataChild::RangingNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingNotification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            ranging_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for RangingNotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for RangingNotificationPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl RangingNotificationBuilder {
    pub fn build(self) -> RangingNotificationPacket {
        let ranging_notification = Arc::new(RangingNotificationData {
            child: RangingNotificationDataChild::None,
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::RangingNotification(ranging_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        RangingNotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangingNotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for RangingNotificationBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum AndroidCommandDataChild {
    AndroidGetPowerStatsCmd(Arc<AndroidGetPowerStatsCmdData>),
    AndroidSetCountryCodeCmd(Arc<AndroidSetCountryCodeCmdData>),
    None,
}
impl AndroidCommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            AndroidCommandDataChild::AndroidGetPowerStatsCmd(value) => value.get_total_size(),
            AndroidCommandDataChild::AndroidSetCountryCodeCmd(value) => value.get_total_size(),
            AndroidCommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum AndroidCommandChild {
    AndroidGetPowerStatsCmd(AndroidGetPowerStatsCmdPacket),
    AndroidSetCountryCodeCmd(AndroidSetCountryCodeCmdPacket),
    None,
}
#[derive(Debug)]
struct AndroidCommandData {
    child: AndroidCommandDataChild,
}
#[derive(Debug, Clone)]
pub struct AndroidCommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    android_command: Arc<AndroidCommandData>,
}
#[derive(Debug)]
pub struct AndroidCommandBuilder {
    pub opcode: u8,
}
impl AndroidCommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (0) if AndroidGetPowerStatsCmdData::conforms(&bytes[..]) => {
                AndroidCommandDataChild::AndroidGetPowerStatsCmd(Arc::new(
                    AndroidGetPowerStatsCmdData::parse(&bytes[..])?,
                ))
            }
            (1) if AndroidSetCountryCodeCmdData::conforms(&bytes[..]) => {
                AndroidCommandDataChild::AndroidSetCountryCodeCmd(Arc::new(
                    AndroidSetCountryCodeCmdData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            AndroidCommandDataChild::AndroidGetPowerStatsCmd(value) => value.write_to(buffer),
            AndroidCommandDataChild::AndroidSetCountryCodeCmd(value) => value.write_to(buffer),
            AndroidCommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for AndroidCommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidCommandPacket> for Bytes {
    fn from(packet: AndroidCommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidCommandPacket> for Vec<u8> {
    fn from(packet: AndroidCommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for AndroidCommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl AndroidCommandPacket {
    pub fn specialize(&self) -> AndroidCommandChild {
        match &self.android_command.child {
            AndroidCommandDataChild::AndroidGetPowerStatsCmd(_) => {
                AndroidCommandChild::AndroidGetPowerStatsCmd(
                    AndroidGetPowerStatsCmdPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            AndroidCommandDataChild::AndroidSetCountryCodeCmd(_) => {
                AndroidCommandChild::AndroidSetCountryCodeCmd(
                    AndroidSetCountryCodeCmdPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            AndroidCommandDataChild::None => AndroidCommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let android_command = match &uci_command.child {
            UciCommandDataChild::AndroidCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidCommand"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            android_command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for AndroidCommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for AndroidCommandPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl AndroidCommandBuilder {
    pub fn build(self) -> AndroidCommandPacket {
        let android_command = Arc::new(AndroidCommandData {
            child: AndroidCommandDataChild::None,
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::AndroidCommand(android_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorAndroid,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        AndroidCommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for AndroidCommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for AndroidCommandBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum AndroidResponseDataChild {
    AndroidGetPowerStatsRsp(Arc<AndroidGetPowerStatsRspData>),
    AndroidSetCountryCodeRsp(Arc<AndroidSetCountryCodeRspData>),
    None,
}
impl AndroidResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            AndroidResponseDataChild::AndroidGetPowerStatsRsp(value) => value.get_total_size(),
            AndroidResponseDataChild::AndroidSetCountryCodeRsp(value) => value.get_total_size(),
            AndroidResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum AndroidResponseChild {
    AndroidGetPowerStatsRsp(AndroidGetPowerStatsRspPacket),
    AndroidSetCountryCodeRsp(AndroidSetCountryCodeRspPacket),
    None,
}
#[derive(Debug)]
struct AndroidResponseData {
    child: AndroidResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct AndroidResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    android_response: Arc<AndroidResponseData>,
}
#[derive(Debug)]
pub struct AndroidResponseBuilder {
    pub opcode: u8,
}
impl AndroidResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8], opcode: u8) -> Result<Self> {
        let child = match (opcode) {
            (0) if AndroidGetPowerStatsRspData::conforms(&bytes[..]) => {
                AndroidResponseDataChild::AndroidGetPowerStatsRsp(Arc::new(
                    AndroidGetPowerStatsRspData::parse(&bytes[..])?,
                ))
            }
            (1) if AndroidSetCountryCodeRspData::conforms(&bytes[..]) => {
                AndroidResponseDataChild::AndroidSetCountryCodeRsp(Arc::new(
                    AndroidSetCountryCodeRspData::parse(&bytes[..])?,
                ))
            }
            (_) => return Err(Error::InvalidPacketError),
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            AndroidResponseDataChild::AndroidGetPowerStatsRsp(value) => value.write_to(buffer),
            AndroidResponseDataChild::AndroidSetCountryCodeRsp(value) => value.write_to(buffer),
            AndroidResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for AndroidResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidResponsePacket> for Bytes {
    fn from(packet: AndroidResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidResponsePacket> for Vec<u8> {
    fn from(packet: AndroidResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for AndroidResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl AndroidResponsePacket {
    pub fn specialize(&self) -> AndroidResponseChild {
        match &self.android_response.child {
            AndroidResponseDataChild::AndroidGetPowerStatsRsp(_) => {
                AndroidResponseChild::AndroidGetPowerStatsRsp(
                    AndroidGetPowerStatsRspPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            AndroidResponseDataChild::AndroidSetCountryCodeRsp(_) => {
                AndroidResponseChild::AndroidSetCountryCodeRsp(
                    AndroidSetCountryCodeRspPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            AndroidResponseDataChild::None => AndroidResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let android_response = match &uci_response.child {
            UciResponseDataChild::AndroidResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidResponse"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            android_response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for AndroidResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for AndroidResponsePacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl AndroidResponseBuilder {
    pub fn build(self) -> AndroidResponsePacket {
        let android_response = Arc::new(AndroidResponseData {
            child: AndroidResponseDataChild::None,
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::AndroidResponse(android_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorAndroid,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        AndroidResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for AndroidResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for AndroidResponseBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}

#[derive(Debug)]
struct AndroidNotificationData {}
#[derive(Debug, Clone)]
pub struct AndroidNotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    android_notification: Arc<AndroidNotificationData>,
}
#[derive(Debug)]
pub struct AndroidNotificationBuilder {
    pub opcode: u8,
}
impl AndroidNotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for AndroidNotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidNotificationPacket> for Bytes {
    fn from(packet: AndroidNotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidNotificationPacket> for Vec<u8> {
    fn from(packet: AndroidNotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for AndroidNotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl AndroidNotificationPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let android_notification = match &uci_notification.child {
            UciNotificationDataChild::AndroidNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidNotification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            android_notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for AndroidNotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for AndroidNotificationPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl AndroidNotificationBuilder {
    pub fn build(self) -> AndroidNotificationPacket {
        let android_notification = Arc::new(AndroidNotificationData {});
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::AndroidNotification(android_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorAndroid,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        AndroidNotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for AndroidNotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for AndroidNotificationBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}

#[derive(Debug)]
struct DeviceResetCmdData {
    reset_config: ResetConfig,
}
#[derive(Debug, Clone)]
pub struct DeviceResetCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    core_command: Arc<CoreCommandData>,
    device_reset_cmd: Arc<DeviceResetCmdData>,
}
#[derive(Debug)]
pub struct DeviceResetCmdBuilder {
    pub reset_config: ResetConfig,
}
impl DeviceResetCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceResetCmd".to_string(),
                field: "reset_config".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let reset_config = u8::from_le_bytes([bytes[4]]);
        let reset_config =
            ResetConfig::from_u8(reset_config).ok_or_else(|| Error::InvalidEnumValueError {
                obj: "DeviceResetCmd".to_string(),
                field: "reset_config".to_string(),
                value: reset_config as u64,
                type_: "ResetConfig".to_string(),
            })?;
        Ok(Self { reset_config })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let reset_config = self.reset_config.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&reset_config.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for DeviceResetCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<DeviceResetCmdPacket> for Bytes {
    fn from(packet: DeviceResetCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<DeviceResetCmdPacket> for Vec<u8> {
    fn from(packet: DeviceResetCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for DeviceResetCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl DeviceResetCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let core_command = match &uci_command.child {
            UciCommandDataChild::CoreCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreCommand"),
        };
        let device_reset_cmd = match &core_command.child {
            CoreCommandDataChild::DeviceResetCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not DeviceResetCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            core_command,
            device_reset_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_reset_config(&self) -> ResetConfig {
        self.device_reset_cmd.as_ref().reset_config
    }
}
impl Into<UciPacketPacket> for DeviceResetCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for DeviceResetCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreCommandPacket> for DeviceResetCmdPacket {
    fn into(self) -> CoreCommandPacket {
        CoreCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl DeviceResetCmdBuilder {
    pub fn build(self) -> DeviceResetCmdPacket {
        let device_reset_cmd = Arc::new(DeviceResetCmdData {
            reset_config: self.reset_config,
        });
        let core_command = Arc::new(CoreCommandData {
            child: CoreCommandDataChild::DeviceResetCmd(device_reset_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::CoreCommand(core_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 0,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        DeviceResetCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for DeviceResetCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for DeviceResetCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<CoreCommandPacket> for DeviceResetCmdBuilder {
    fn into(self) -> CoreCommandPacket {
        self.build().into()
    }
}
macro_rules! device_reset_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::CoreCommand(core_command_packet) => {match core_command_packet.specialize() {/* (3) */
CoreCommandChild::DeviceResetCmd(packet) => {let rebuilder = DeviceResetCmdBuilder {reset_config : packet.get_reset_config(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse device_reset_cmd
 {:#02x?}", core_command_packet); }}}_ => {panic!("Couldn't parse core_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
device_reset_cmd_builder_tests! { device_reset_cmd_builder_test_00: b"\x20\x00\x00\x01\x00",}

#[derive(Debug)]
struct DeviceResetRspData {
    status: StatusCode,
}
#[derive(Debug, Clone)]
pub struct DeviceResetRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    core_response: Arc<CoreResponseData>,
    device_reset_rsp: Arc<DeviceResetRspData>,
}
#[derive(Debug)]
pub struct DeviceResetRspBuilder {
    pub status: StatusCode,
}
impl DeviceResetRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceResetRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "DeviceResetRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for DeviceResetRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<DeviceResetRspPacket> for Bytes {
    fn from(packet: DeviceResetRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<DeviceResetRspPacket> for Vec<u8> {
    fn from(packet: DeviceResetRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for DeviceResetRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl DeviceResetRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let core_response = match &uci_response.child {
            UciResponseDataChild::CoreResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreResponse"),
        };
        let device_reset_rsp = match &core_response.child {
            CoreResponseDataChild::DeviceResetRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not DeviceResetRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            core_response,
            device_reset_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.device_reset_rsp.as_ref().status
    }
}
impl Into<UciPacketPacket> for DeviceResetRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for DeviceResetRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreResponsePacket> for DeviceResetRspPacket {
    fn into(self) -> CoreResponsePacket {
        CoreResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl DeviceResetRspBuilder {
    pub fn build(self) -> DeviceResetRspPacket {
        let device_reset_rsp = Arc::new(DeviceResetRspData {
            status: self.status,
        });
        let core_response = Arc::new(CoreResponseData {
            child: CoreResponseDataChild::DeviceResetRsp(device_reset_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::CoreResponse(core_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 0,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        DeviceResetRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for DeviceResetRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for DeviceResetRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<CoreResponsePacket> for DeviceResetRspBuilder {
    fn into(self) -> CoreResponsePacket {
        self.build().into()
    }
}
macro_rules! device_reset_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::CoreResponse(core_response_packet) => {match core_response_packet.specialize() {/* (3) */
CoreResponseChild::DeviceResetRsp(packet) => {let rebuilder = DeviceResetRspBuilder {status : packet.get_status(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse device_reset_rsp
 {:#02x?}", core_response_packet); }}}_ => {panic!("Couldn't parse core_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
device_reset_rsp_builder_tests! { device_reset_rsp_builder_test_00: b"\x40\x00\x00\x01\x00",}

#[derive(Debug)]
struct DeviceStatusNtfData {
    device_state: DeviceState,
}
#[derive(Debug, Clone)]
pub struct DeviceStatusNtfPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    core_notification: Arc<CoreNotificationData>,
    device_status_ntf: Arc<DeviceStatusNtfData>,
}
#[derive(Debug)]
pub struct DeviceStatusNtfBuilder {
    pub device_state: DeviceState,
}
impl DeviceStatusNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "DeviceStatusNtf".to_string(),
                field: "device_state".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let device_state = u8::from_le_bytes([bytes[4]]);
        let device_state =
            DeviceState::from_u8(device_state).ok_or_else(|| Error::InvalidEnumValueError {
                obj: "DeviceStatusNtf".to_string(),
                field: "device_state".to_string(),
                value: device_state as u64,
                type_: "DeviceState".to_string(),
            })?;
        Ok(Self { device_state })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let device_state = self.device_state.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&device_state.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for DeviceStatusNtfPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<DeviceStatusNtfPacket> for Bytes {
    fn from(packet: DeviceStatusNtfPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<DeviceStatusNtfPacket> for Vec<u8> {
    fn from(packet: DeviceStatusNtfPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for DeviceStatusNtfPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl DeviceStatusNtfPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let core_notification = match &uci_notification.child {
            UciNotificationDataChild::CoreNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreNotification"),
        };
        let device_status_ntf = match &core_notification.child {
            CoreNotificationDataChild::DeviceStatusNtf(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not DeviceStatusNtf"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            core_notification,
            device_status_ntf,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_device_state(&self) -> DeviceState {
        self.device_status_ntf.as_ref().device_state
    }
}
impl Into<UciPacketPacket> for DeviceStatusNtfPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for DeviceStatusNtfPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreNotificationPacket> for DeviceStatusNtfPacket {
    fn into(self) -> CoreNotificationPacket {
        CoreNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl DeviceStatusNtfBuilder {
    pub fn build(self) -> DeviceStatusNtfPacket {
        let device_status_ntf = Arc::new(DeviceStatusNtfData {
            device_state: self.device_state,
        });
        let core_notification = Arc::new(CoreNotificationData {
            child: CoreNotificationDataChild::DeviceStatusNtf(device_status_ntf),
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::CoreNotification(core_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: 1,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        DeviceStatusNtfPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for DeviceStatusNtfBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for DeviceStatusNtfBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}
impl Into<CoreNotificationPacket> for DeviceStatusNtfBuilder {
    fn into(self) -> CoreNotificationPacket {
        self.build().into()
    }
}
macro_rules! device_status_ntf_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciNotification(uci_notification_packet) => {match uci_notification_packet.specialize() {/* (2) */
UciNotificationChild::CoreNotification(core_notification_packet) => {match core_notification_packet.specialize() {/* (3) */
CoreNotificationChild::DeviceStatusNtf(packet) => {let rebuilder = DeviceStatusNtfBuilder {device_state : packet.get_device_state(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse device_status_ntf
 {:#02x?}", core_notification_packet); }}}_ => {panic!("Couldn't parse core_notification
 {:#02x?}", uci_notification_packet); }}}_ => {panic!("Couldn't parse uci_notification
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
device_status_ntf_builder_tests! { device_status_ntf_builder_test_00: b"\x60\x01\x00\x01\x01",}

#[derive(Debug)]
struct GetDeviceInfoCmdData {}
#[derive(Debug, Clone)]
pub struct GetDeviceInfoCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    core_command: Arc<CoreCommandData>,
    get_device_info_cmd: Arc<GetDeviceInfoCmdData>,
}
#[derive(Debug)]
pub struct GetDeviceInfoCmdBuilder {}
impl GetDeviceInfoCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for GetDeviceInfoCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetDeviceInfoCmdPacket> for Bytes {
    fn from(packet: GetDeviceInfoCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<GetDeviceInfoCmdPacket> for Vec<u8> {
    fn from(packet: GetDeviceInfoCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for GetDeviceInfoCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl GetDeviceInfoCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let core_command = match &uci_command.child {
            UciCommandDataChild::CoreCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreCommand"),
        };
        let get_device_info_cmd = match &core_command.child {
            CoreCommandDataChild::GetDeviceInfoCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not GetDeviceInfoCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            core_command,
            get_device_info_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for GetDeviceInfoCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for GetDeviceInfoCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreCommandPacket> for GetDeviceInfoCmdPacket {
    fn into(self) -> CoreCommandPacket {
        CoreCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl GetDeviceInfoCmdBuilder {
    pub fn build(self) -> GetDeviceInfoCmdPacket {
        let get_device_info_cmd = Arc::new(GetDeviceInfoCmdData {});
        let core_command = Arc::new(CoreCommandData {
            child: CoreCommandDataChild::GetDeviceInfoCmd(get_device_info_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::CoreCommand(core_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 2,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        GetDeviceInfoCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for GetDeviceInfoCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for GetDeviceInfoCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<CoreCommandPacket> for GetDeviceInfoCmdBuilder {
    fn into(self) -> CoreCommandPacket {
        self.build().into()
    }
}
macro_rules! get_device_info_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::CoreCommand(core_command_packet) => {match core_command_packet.specialize() {/* (3) */
CoreCommandChild::GetDeviceInfoCmd(packet) => {let rebuilder = GetDeviceInfoCmdBuilder {};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse get_device_info_cmd
 {:#02x?}", core_command_packet); }}}_ => {panic!("Couldn't parse core_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
get_device_info_cmd_builder_tests! { get_device_info_cmd_builder_test_00: b"\x20\x02\x00\x00",}

#[derive(Debug)]
struct GetDeviceInfoRspData {
    status: StatusCode,
    uci_version: u16,
    mac_version: u16,
    phy_version: u16,
    uci_test_version: u16,
    vendor_spec_info: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct GetDeviceInfoRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    core_response: Arc<CoreResponseData>,
    get_device_info_rsp: Arc<GetDeviceInfoRspData>,
}
#[derive(Debug)]
pub struct GetDeviceInfoRspBuilder {
    pub status: StatusCode,
    pub uci_version: u16,
    pub mac_version: u16,
    pub phy_version: u16,
    pub uci_test_version: u16,
    pub vendor_spec_info: Vec<u8>,
}
impl GetDeviceInfoRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 14 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "GetDeviceInfoRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 7 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                field: "uci_version".to_string(),
                wanted: 7,
                got: bytes.len(),
            });
        }
        let uci_version = u16::from_le_bytes([bytes[5], bytes[6]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                field: "mac_version".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let mac_version = u16::from_le_bytes([bytes[7], bytes[8]]);
        if bytes.len() < 11 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                field: "phy_version".to_string(),
                wanted: 11,
                got: bytes.len(),
            });
        }
        let phy_version = u16::from_le_bytes([bytes[9], bytes[10]]);
        if bytes.len() < 13 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                field: "uci_test_version".to_string(),
                wanted: 13,
                got: bytes.len(),
            });
        }
        let uci_test_version = u16::from_le_bytes([bytes[11], bytes[12]]);
        if bytes.len() < 14 {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                field: "vendor_spec_info_count".to_string(),
                wanted: 14,
                got: bytes.len(),
            });
        }
        let vendor_spec_info_count = u8::from_le_bytes([bytes[13]]);
        let want_ = 14 + ((vendor_spec_info_count as usize) * 1);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "GetDeviceInfoRsp".to_string(),
                field: "vendor_spec_info".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let vendor_spec_info: Vec<u8> = bytes[14..14 + ((vendor_spec_info_count as usize) * 1)]
            .to_vec()
            .chunks_exact(1)
            .into_iter()
            .map(|i| u8::from_le_bytes([i[0]]))
            .collect();
        Ok(Self {
            status,
            uci_version,
            mac_version,
            phy_version,
            uci_test_version,
            vendor_spec_info,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        let uci_version = self.uci_version;
        buffer[5..7].copy_from_slice(&uci_version.to_le_bytes()[0..2]);
        let mac_version = self.mac_version;
        buffer[7..9].copy_from_slice(&mac_version.to_le_bytes()[0..2]);
        let phy_version = self.phy_version;
        buffer[9..11].copy_from_slice(&phy_version.to_le_bytes()[0..2]);
        let uci_test_version = self.uci_test_version;
        buffer[11..13].copy_from_slice(&uci_test_version.to_le_bytes()[0..2]);
        buffer[13..14].copy_from_slice(&(self.vendor_spec_info.len() as u8).to_le_bytes());
        for (i, e) in self.vendor_spec_info.iter().enumerate() {
            buffer[14 + i..14 + i + 1].copy_from_slice(&e.to_le_bytes())
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 10;
        let ret = ret + (self.vendor_spec_info.len() * ((/* Bits: */8 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for GetDeviceInfoRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetDeviceInfoRspPacket> for Bytes {
    fn from(packet: GetDeviceInfoRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<GetDeviceInfoRspPacket> for Vec<u8> {
    fn from(packet: GetDeviceInfoRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for GetDeviceInfoRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl GetDeviceInfoRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let core_response = match &uci_response.child {
            UciResponseDataChild::CoreResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreResponse"),
        };
        let get_device_info_rsp = match &core_response.child {
            CoreResponseDataChild::GetDeviceInfoRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not GetDeviceInfoRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            core_response,
            get_device_info_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.get_device_info_rsp.as_ref().status
    }
    pub fn get_uci_version(&self) -> u16 {
        self.get_device_info_rsp.as_ref().uci_version
    }
    pub fn get_mac_version(&self) -> u16 {
        self.get_device_info_rsp.as_ref().mac_version
    }
    pub fn get_phy_version(&self) -> u16 {
        self.get_device_info_rsp.as_ref().phy_version
    }
    pub fn get_uci_test_version(&self) -> u16 {
        self.get_device_info_rsp.as_ref().uci_test_version
    }
    pub fn get_vendor_spec_info(&self) -> &Vec<u8> {
        &self.get_device_info_rsp.as_ref().vendor_spec_info
    }
}
impl Into<UciPacketPacket> for GetDeviceInfoRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for GetDeviceInfoRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreResponsePacket> for GetDeviceInfoRspPacket {
    fn into(self) -> CoreResponsePacket {
        CoreResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl GetDeviceInfoRspBuilder {
    pub fn build(self) -> GetDeviceInfoRspPacket {
        let get_device_info_rsp = Arc::new(GetDeviceInfoRspData {
            status: self.status,
            uci_version: self.uci_version,
            mac_version: self.mac_version,
            phy_version: self.phy_version,
            uci_test_version: self.uci_test_version,
            vendor_spec_info: self.vendor_spec_info,
        });
        let core_response = Arc::new(CoreResponseData {
            child: CoreResponseDataChild::GetDeviceInfoRsp(get_device_info_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::CoreResponse(core_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 2,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        GetDeviceInfoRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for GetDeviceInfoRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for GetDeviceInfoRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<CoreResponsePacket> for GetDeviceInfoRspBuilder {
    fn into(self) -> CoreResponsePacket {
        self.build().into()
    }
}
macro_rules! get_device_info_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::CoreResponse(core_response_packet) => {match core_response_packet.specialize() {/* (3) */
CoreResponseChild::GetDeviceInfoRsp(packet) => {let rebuilder = GetDeviceInfoRspBuilder {status : packet.get_status(),uci_version : packet.get_uci_version(),mac_version : packet.get_mac_version(),phy_version : packet.get_phy_version(),uci_test_version : packet.get_uci_test_version(),vendor_spec_info : packet.get_vendor_spec_info().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse get_device_info_rsp
 {:#02x?}", core_response_packet); }}}_ => {panic!("Couldn't parse core_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
get_device_info_rsp_builder_tests! { get_device_info_rsp_builder_test_00: b"\x40\x02\x00\x0b\x01\x01\x00\x02\x00\x03\x00\x04\x00\x01\x0a",}

#[derive(Debug)]
struct GetCapsInfoCmdData {}
#[derive(Debug, Clone)]
pub struct GetCapsInfoCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    core_command: Arc<CoreCommandData>,
    get_caps_info_cmd: Arc<GetCapsInfoCmdData>,
}
#[derive(Debug)]
pub struct GetCapsInfoCmdBuilder {}
impl GetCapsInfoCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for GetCapsInfoCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetCapsInfoCmdPacket> for Bytes {
    fn from(packet: GetCapsInfoCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<GetCapsInfoCmdPacket> for Vec<u8> {
    fn from(packet: GetCapsInfoCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for GetCapsInfoCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl GetCapsInfoCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let core_command = match &uci_command.child {
            UciCommandDataChild::CoreCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreCommand"),
        };
        let get_caps_info_cmd = match &core_command.child {
            CoreCommandDataChild::GetCapsInfoCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not GetCapsInfoCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            core_command,
            get_caps_info_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for GetCapsInfoCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for GetCapsInfoCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreCommandPacket> for GetCapsInfoCmdPacket {
    fn into(self) -> CoreCommandPacket {
        CoreCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl GetCapsInfoCmdBuilder {
    pub fn build(self) -> GetCapsInfoCmdPacket {
        let get_caps_info_cmd = Arc::new(GetCapsInfoCmdData {});
        let core_command = Arc::new(CoreCommandData {
            child: CoreCommandDataChild::GetCapsInfoCmd(get_caps_info_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::CoreCommand(core_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 3,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        GetCapsInfoCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for GetCapsInfoCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for GetCapsInfoCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<CoreCommandPacket> for GetCapsInfoCmdBuilder {
    fn into(self) -> CoreCommandPacket {
        self.build().into()
    }
}
macro_rules! get_caps_info_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::CoreCommand(core_command_packet) => {match core_command_packet.specialize() {/* (3) */
CoreCommandChild::GetCapsInfoCmd(packet) => {let rebuilder = GetCapsInfoCmdBuilder {};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse get_caps_info_cmd
 {:#02x?}", core_command_packet); }}}_ => {panic!("Couldn't parse core_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
get_caps_info_cmd_builder_tests! { get_caps_info_cmd_builder_test_00: b"\x20\x03\x00\x00",}

#[derive(Debug)]
struct GetCapsInfoRspData {
    status: StatusCode,
    tlvs: Vec<CapTlv>,
}
#[derive(Debug, Clone)]
pub struct GetCapsInfoRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    core_response: Arc<CoreResponseData>,
    get_caps_info_rsp: Arc<GetCapsInfoRspData>,
}
#[derive(Debug)]
pub struct GetCapsInfoRspBuilder {
    pub status: StatusCode,
    pub tlvs: Vec<CapTlv>,
}
impl GetCapsInfoRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "GetCapsInfoRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "GetCapsInfoRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "GetCapsInfoRsp".to_string(),
                field: "tlvs_count".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let tlvs_count = u8::from_le_bytes([bytes[5]]);
        let mut tlvs: Vec<CapTlv> = Vec::new();
        let mut parsable_ = &bytes[6..];
        let count_ = tlvs_count as usize;
        for _ in 0..count_ {
            match CapTlv::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    tlvs.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self { status, tlvs })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        buffer[5..6].copy_from_slice(&(self.tlvs.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[6..];
        for e_ in &self.tlvs {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        let ret = ret + self.tlvs.iter().fold(0, |acc, x| acc + x.get_total_size());
        ret
    }
}
impl Packet for GetCapsInfoRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetCapsInfoRspPacket> for Bytes {
    fn from(packet: GetCapsInfoRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<GetCapsInfoRspPacket> for Vec<u8> {
    fn from(packet: GetCapsInfoRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for GetCapsInfoRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl GetCapsInfoRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let core_response = match &uci_response.child {
            UciResponseDataChild::CoreResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreResponse"),
        };
        let get_caps_info_rsp = match &core_response.child {
            CoreResponseDataChild::GetCapsInfoRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not GetCapsInfoRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            core_response,
            get_caps_info_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.get_caps_info_rsp.as_ref().status
    }
    pub fn get_tlvs(&self) -> &Vec<CapTlv> {
        &self.get_caps_info_rsp.as_ref().tlvs
    }
}
impl Into<UciPacketPacket> for GetCapsInfoRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for GetCapsInfoRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreResponsePacket> for GetCapsInfoRspPacket {
    fn into(self) -> CoreResponsePacket {
        CoreResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl GetCapsInfoRspBuilder {
    pub fn build(self) -> GetCapsInfoRspPacket {
        let get_caps_info_rsp = Arc::new(GetCapsInfoRspData {
            status: self.status,
            tlvs: self.tlvs,
        });
        let core_response = Arc::new(CoreResponseData {
            child: CoreResponseDataChild::GetCapsInfoRsp(get_caps_info_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::CoreResponse(core_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 3,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        GetCapsInfoRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for GetCapsInfoRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for GetCapsInfoRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<CoreResponsePacket> for GetCapsInfoRspBuilder {
    fn into(self) -> CoreResponsePacket {
        self.build().into()
    }
}
macro_rules! get_caps_info_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::CoreResponse(core_response_packet) => {match core_response_packet.specialize() {/* (3) */
CoreResponseChild::GetCapsInfoRsp(packet) => {let rebuilder = GetCapsInfoRspBuilder {status : packet.get_status(),tlvs : packet.get_tlvs().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse get_caps_info_rsp
 {:#02x?}", core_response_packet); }}}_ => {panic!("Couldn't parse core_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
get_caps_info_rsp_builder_tests! { get_caps_info_rsp_builder_test_00: b"\x40\x03\x00\x05\x00\x01\x00\x01\x01",}

#[derive(Debug)]
struct SetConfigCmdData {
    parameters: Vec<DeviceParameter>,
}
#[derive(Debug, Clone)]
pub struct SetConfigCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    core_command: Arc<CoreCommandData>,
    set_config_cmd: Arc<SetConfigCmdData>,
}
#[derive(Debug)]
pub struct SetConfigCmdBuilder {
    pub parameters: Vec<DeviceParameter>,
}
impl SetConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SetConfigCmd".to_string(),
                field: "parameters_count".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let parameters_count = u8::from_le_bytes([bytes[4]]);
        let mut parameters: Vec<DeviceParameter> = Vec::new();
        let mut parsable_ = &bytes[5..];
        let count_ = parameters_count as usize;
        for _ in 0..count_ {
            match DeviceParameter::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    parameters.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self { parameters })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer[4..5].copy_from_slice(&(self.parameters.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[5..];
        for e_ in &self.parameters {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        let ret = ret
            + self
                .parameters
                .iter()
                .fold(0, |acc, x| acc + x.get_total_size());
        ret
    }
}
impl Packet for SetConfigCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SetConfigCmdPacket> for Bytes {
    fn from(packet: SetConfigCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SetConfigCmdPacket> for Vec<u8> {
    fn from(packet: SetConfigCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SetConfigCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SetConfigCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let core_command = match &uci_command.child {
            UciCommandDataChild::CoreCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreCommand"),
        };
        let set_config_cmd = match &core_command.child {
            CoreCommandDataChild::SetConfigCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SetConfigCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            core_command,
            set_config_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_parameters(&self) -> &Vec<DeviceParameter> {
        &self.set_config_cmd.as_ref().parameters
    }
}
impl Into<UciPacketPacket> for SetConfigCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SetConfigCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreCommandPacket> for SetConfigCmdPacket {
    fn into(self) -> CoreCommandPacket {
        CoreCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SetConfigCmdBuilder {
    pub fn build(self) -> SetConfigCmdPacket {
        let set_config_cmd = Arc::new(SetConfigCmdData {
            parameters: self.parameters,
        });
        let core_command = Arc::new(CoreCommandData {
            child: CoreCommandDataChild::SetConfigCmd(set_config_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::CoreCommand(core_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 4,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SetConfigCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SetConfigCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SetConfigCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<CoreCommandPacket> for SetConfigCmdBuilder {
    fn into(self) -> CoreCommandPacket {
        self.build().into()
    }
}
macro_rules! set_config_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::CoreCommand(core_command_packet) => {match core_command_packet.specialize() {/* (3) */
CoreCommandChild::SetConfigCmd(packet) => {let rebuilder = SetConfigCmdBuilder {parameters : packet.get_parameters().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse set_config_cmd
 {:#02x?}", core_command_packet); }}}_ => {panic!("Couldn't parse core_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
set_config_cmd_builder_tests! { set_config_cmd_builder_test_00: b"\x20\x04\x00\x03\x01\x01\x00",}

#[derive(Debug)]
struct SetConfigRspData {
    status: StatusCode,
    parameters: Vec<DeviceConfigStatus>,
}
#[derive(Debug, Clone)]
pub struct SetConfigRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    core_response: Arc<CoreResponseData>,
    set_config_rsp: Arc<SetConfigRspData>,
}
#[derive(Debug)]
pub struct SetConfigRspBuilder {
    pub status: StatusCode,
    pub parameters: Vec<DeviceConfigStatus>,
}
impl SetConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SetConfigRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "SetConfigRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "SetConfigRsp".to_string(),
                field: "parameters_count".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let parameters_count = u8::from_le_bytes([bytes[5]]);
        let want_ = 6 + ((parameters_count as usize) * 2);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "SetConfigRsp".to_string(),
                field: "parameters".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let mut parameters: Vec<DeviceConfigStatus> = Vec::new();
        let mut parsable_ = &bytes[6..];
        let count_ = parameters_count as usize;
        for _ in 0..count_ {
            match DeviceConfigStatus::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    parameters.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self { status, parameters })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        buffer[5..6].copy_from_slice(&(self.parameters.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[6..];
        for e_ in &self.parameters {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        let ret = ret + (self.parameters.len() * ((/* Bits: */16 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for SetConfigRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SetConfigRspPacket> for Bytes {
    fn from(packet: SetConfigRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SetConfigRspPacket> for Vec<u8> {
    fn from(packet: SetConfigRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SetConfigRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SetConfigRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let core_response = match &uci_response.child {
            UciResponseDataChild::CoreResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreResponse"),
        };
        let set_config_rsp = match &core_response.child {
            CoreResponseDataChild::SetConfigRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SetConfigRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            core_response,
            set_config_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.set_config_rsp.as_ref().status
    }
    pub fn get_parameters(&self) -> &Vec<DeviceConfigStatus> {
        &self.set_config_rsp.as_ref().parameters
    }
}
impl Into<UciPacketPacket> for SetConfigRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SetConfigRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreResponsePacket> for SetConfigRspPacket {
    fn into(self) -> CoreResponsePacket {
        CoreResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SetConfigRspBuilder {
    pub fn build(self) -> SetConfigRspPacket {
        let set_config_rsp = Arc::new(SetConfigRspData {
            status: self.status,
            parameters: self.parameters,
        });
        let core_response = Arc::new(CoreResponseData {
            child: CoreResponseDataChild::SetConfigRsp(set_config_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::CoreResponse(core_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 4,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SetConfigRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SetConfigRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SetConfigRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<CoreResponsePacket> for SetConfigRspBuilder {
    fn into(self) -> CoreResponsePacket {
        self.build().into()
    }
}
macro_rules! set_config_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::CoreResponse(core_response_packet) => {match core_response_packet.specialize() {/* (3) */
CoreResponseChild::SetConfigRsp(packet) => {let rebuilder = SetConfigRspBuilder {status : packet.get_status(),parameters : packet.get_parameters().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse set_config_rsp
 {:#02x?}", core_response_packet); }}}_ => {panic!("Couldn't parse core_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
set_config_rsp_builder_tests! { set_config_rsp_builder_test_00: b"\x40\x04\x00\x04\x01\x01\x01\x01",}

#[derive(Debug)]
struct GetConfigCmdData {
    parameter_ids: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct GetConfigCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    core_command: Arc<CoreCommandData>,
    get_config_cmd: Arc<GetConfigCmdData>,
}
#[derive(Debug)]
pub struct GetConfigCmdBuilder {
    pub parameter_ids: Vec<u8>,
}
impl GetConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "GetConfigCmd".to_string(),
                field: "parameter_ids_count".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let parameter_ids_count = u8::from_le_bytes([bytes[4]]);
        let want_ = 5 + ((parameter_ids_count as usize) * 1);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "GetConfigCmd".to_string(),
                field: "parameter_ids".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let parameter_ids: Vec<u8> = bytes[5..5 + ((parameter_ids_count as usize) * 1)]
            .to_vec()
            .chunks_exact(1)
            .into_iter()
            .map(|i| u8::from_le_bytes([i[0]]))
            .collect();
        Ok(Self { parameter_ids })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer[4..5].copy_from_slice(&(self.parameter_ids.len() as u8).to_le_bytes());
        for (i, e) in self.parameter_ids.iter().enumerate() {
            buffer[5 + i..5 + i + 1].copy_from_slice(&e.to_le_bytes())
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        let ret = ret + (self.parameter_ids.len() * ((/* Bits: */8 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for GetConfigCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetConfigCmdPacket> for Bytes {
    fn from(packet: GetConfigCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<GetConfigCmdPacket> for Vec<u8> {
    fn from(packet: GetConfigCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for GetConfigCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl GetConfigCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let core_command = match &uci_command.child {
            UciCommandDataChild::CoreCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreCommand"),
        };
        let get_config_cmd = match &core_command.child {
            CoreCommandDataChild::GetConfigCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not GetConfigCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            core_command,
            get_config_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_parameter_ids(&self) -> &Vec<u8> {
        &self.get_config_cmd.as_ref().parameter_ids
    }
}
impl Into<UciPacketPacket> for GetConfigCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for GetConfigCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreCommandPacket> for GetConfigCmdPacket {
    fn into(self) -> CoreCommandPacket {
        CoreCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl GetConfigCmdBuilder {
    pub fn build(self) -> GetConfigCmdPacket {
        let get_config_cmd = Arc::new(GetConfigCmdData {
            parameter_ids: self.parameter_ids,
        });
        let core_command = Arc::new(CoreCommandData {
            child: CoreCommandDataChild::GetConfigCmd(get_config_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::CoreCommand(core_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 5,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        GetConfigCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for GetConfigCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for GetConfigCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<CoreCommandPacket> for GetConfigCmdBuilder {
    fn into(self) -> CoreCommandPacket {
        self.build().into()
    }
}
macro_rules! get_config_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::CoreCommand(core_command_packet) => {match core_command_packet.specialize() {/* (3) */
CoreCommandChild::GetConfigCmd(packet) => {let rebuilder = GetConfigCmdBuilder {parameter_ids : packet.get_parameter_ids().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse get_config_cmd
 {:#02x?}", core_command_packet); }}}_ => {panic!("Couldn't parse core_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
get_config_cmd_builder_tests! { get_config_cmd_builder_test_00: b"\x20\x05\x00\x02\x01\x01",}

#[derive(Debug)]
struct GetConfigRspData {
    status: StatusCode,
    parameters: Vec<DeviceParameter>,
}
#[derive(Debug, Clone)]
pub struct GetConfigRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    core_response: Arc<CoreResponseData>,
    get_config_rsp: Arc<GetConfigRspData>,
}
#[derive(Debug)]
pub struct GetConfigRspBuilder {
    pub status: StatusCode,
    pub parameters: Vec<DeviceParameter>,
}
impl GetConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "GetConfigRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "GetConfigRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "GetConfigRsp".to_string(),
                field: "parameters_count".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let parameters_count = u8::from_le_bytes([bytes[5]]);
        let mut parameters: Vec<DeviceParameter> = Vec::new();
        let mut parsable_ = &bytes[6..];
        let count_ = parameters_count as usize;
        for _ in 0..count_ {
            match DeviceParameter::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    parameters.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self { status, parameters })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        buffer[5..6].copy_from_slice(&(self.parameters.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[6..];
        for e_ in &self.parameters {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        let ret = ret
            + self
                .parameters
                .iter()
                .fold(0, |acc, x| acc + x.get_total_size());
        ret
    }
}
impl Packet for GetConfigRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GetConfigRspPacket> for Bytes {
    fn from(packet: GetConfigRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<GetConfigRspPacket> for Vec<u8> {
    fn from(packet: GetConfigRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for GetConfigRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl GetConfigRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let core_response = match &uci_response.child {
            UciResponseDataChild::CoreResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreResponse"),
        };
        let get_config_rsp = match &core_response.child {
            CoreResponseDataChild::GetConfigRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not GetConfigRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            core_response,
            get_config_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.get_config_rsp.as_ref().status
    }
    pub fn get_parameters(&self) -> &Vec<DeviceParameter> {
        &self.get_config_rsp.as_ref().parameters
    }
}
impl Into<UciPacketPacket> for GetConfigRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for GetConfigRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreResponsePacket> for GetConfigRspPacket {
    fn into(self) -> CoreResponsePacket {
        CoreResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl GetConfigRspBuilder {
    pub fn build(self) -> GetConfigRspPacket {
        let get_config_rsp = Arc::new(GetConfigRspData {
            status: self.status,
            parameters: self.parameters,
        });
        let core_response = Arc::new(CoreResponseData {
            child: CoreResponseDataChild::GetConfigRsp(get_config_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::CoreResponse(core_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 5,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        GetConfigRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for GetConfigRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for GetConfigRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<CoreResponsePacket> for GetConfigRspBuilder {
    fn into(self) -> CoreResponsePacket {
        self.build().into()
    }
}
macro_rules! get_config_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::CoreResponse(core_response_packet) => {match core_response_packet.specialize() {/* (3) */
CoreResponseChild::GetConfigRsp(packet) => {let rebuilder = GetConfigRspBuilder {status : packet.get_status(),parameters : packet.get_parameters().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse get_config_rsp
 {:#02x?}", core_response_packet); }}}_ => {panic!("Couldn't parse core_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
get_config_rsp_builder_tests! { get_config_rsp_builder_test_00: b"\x40\x05\x00\x05\x01\x01\x00\x01\x01",}

#[derive(Debug)]
struct GenericErrorData {
    status: StatusCode,
}
#[derive(Debug, Clone)]
pub struct GenericErrorPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    core_notification: Arc<CoreNotificationData>,
    generic_error: Arc<GenericErrorData>,
}
#[derive(Debug)]
pub struct GenericErrorBuilder {
    pub status: StatusCode,
}
impl GenericErrorData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "GenericError".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "GenericError".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for GenericErrorPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GenericErrorPacket> for Bytes {
    fn from(packet: GenericErrorPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<GenericErrorPacket> for Vec<u8> {
    fn from(packet: GenericErrorPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for GenericErrorPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl GenericErrorPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let core_notification = match &uci_notification.child {
            UciNotificationDataChild::CoreNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not CoreNotification"),
        };
        let generic_error = match &core_notification.child {
            CoreNotificationDataChild::GenericError(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not GenericError"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            core_notification,
            generic_error,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.generic_error.as_ref().status
    }
}
impl Into<UciPacketPacket> for GenericErrorPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for GenericErrorPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<CoreNotificationPacket> for GenericErrorPacket {
    fn into(self) -> CoreNotificationPacket {
        CoreNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl GenericErrorBuilder {
    pub fn build(self) -> GenericErrorPacket {
        let generic_error = Arc::new(GenericErrorData {
            status: self.status,
        });
        let core_notification = Arc::new(CoreNotificationData {
            child: CoreNotificationDataChild::GenericError(generic_error),
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::CoreNotification(core_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::Core,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: 7,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        GenericErrorPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for GenericErrorBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for GenericErrorBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}
impl Into<CoreNotificationPacket> for GenericErrorBuilder {
    fn into(self) -> CoreNotificationPacket {
        self.build().into()
    }
}
macro_rules! generic_error_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciNotification(uci_notification_packet) => {match uci_notification_packet.specialize() {/* (2) */
UciNotificationChild::CoreNotification(core_notification_packet) => {match core_notification_packet.specialize() {/* (3) */
CoreNotificationChild::GenericError(packet) => {let rebuilder = GenericErrorBuilder {status : packet.get_status(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse generic_error
 {:#02x?}", core_notification_packet); }}}_ => {panic!("Couldn't parse core_notification
 {:#02x?}", uci_notification_packet); }}}_ => {panic!("Couldn't parse uci_notification
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
generic_error_builder_tests! { generic_error_builder_test_00: b"\x60\x07\x00\x01\x01",}

#[derive(Debug)]
struct SessionInitCmdData {
    session_id: u32,
    session_type: SessionType,
}
#[derive(Debug, Clone)]
pub struct SessionInitCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    session_command: Arc<SessionCommandData>,
    session_init_cmd: Arc<SessionInitCmdData>,
}
#[derive(Debug)]
pub struct SessionInitCmdBuilder {
    pub session_id: u32,
    pub session_type: SessionType,
}
impl SessionInitCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 9 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInitCmd".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInitCmd".to_string(),
                field: "session_type".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let session_type = u8::from_le_bytes([bytes[8]]);
        let session_type =
            SessionType::from_u8(session_type).ok_or_else(|| Error::InvalidEnumValueError {
                obj: "SessionInitCmd".to_string(),
                field: "session_type".to_string(),
                value: session_type as u64,
                type_: "SessionType".to_string(),
            })?;
        Ok(Self {
            session_id,
            session_type,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
        let session_type = self.session_type.to_u8().unwrap();
        buffer[8..9].copy_from_slice(&session_type.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 5;
        ret
    }
}
impl Packet for SessionInitCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionInitCmdPacket> for Bytes {
    fn from(packet: SessionInitCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionInitCmdPacket> for Vec<u8> {
    fn from(packet: SessionInitCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionInitCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionInitCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let session_command = match &uci_command.child {
            UciCommandDataChild::SessionCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionCommand"),
        };
        let session_init_cmd = match &session_command.child {
            SessionCommandDataChild::SessionInitCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionInitCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            session_command,
            session_init_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_init_cmd.as_ref().session_id
    }
    pub fn get_session_type(&self) -> SessionType {
        self.session_init_cmd.as_ref().session_type
    }
}
impl Into<UciPacketPacket> for SessionInitCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SessionInitCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionCommandPacket> for SessionInitCmdPacket {
    fn into(self) -> SessionCommandPacket {
        SessionCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionInitCmdBuilder {
    pub fn build(self) -> SessionInitCmdPacket {
        let session_init_cmd = Arc::new(SessionInitCmdData {
            session_id: self.session_id,
            session_type: self.session_type,
        });
        let session_command = Arc::new(SessionCommandData {
            child: SessionCommandDataChild::SessionInitCmd(session_init_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::SessionCommand(session_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 0,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SessionInitCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionInitCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SessionInitCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<SessionCommandPacket> for SessionInitCmdBuilder {
    fn into(self) -> SessionCommandPacket {
        self.build().into()
    }
}
macro_rules! session_init_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::SessionCommand(session_command_packet) => {match session_command_packet.specialize() {/* (3) */
SessionCommandChild::SessionInitCmd(packet) => {let rebuilder = SessionInitCmdBuilder {session_id : packet.get_session_id(),session_type : packet.get_session_type(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_init_cmd
 {:#02x?}", session_command_packet); }}}_ => {panic!("Couldn't parse session_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_init_cmd_builder_tests! { session_init_cmd_builder_test_00: b"\x22\x00\x00\x05\x01\x02\x03\x04\x01",}

#[derive(Debug)]
struct SessionInitRspData {
    status: StatusCode,
}
#[derive(Debug, Clone)]
pub struct SessionInitRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    session_response: Arc<SessionResponseData>,
    session_init_rsp: Arc<SessionInitRspData>,
}
#[derive(Debug)]
pub struct SessionInitRspBuilder {
    pub status: StatusCode,
}
impl SessionInitRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SessionInitRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "SessionInitRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for SessionInitRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionInitRspPacket> for Bytes {
    fn from(packet: SessionInitRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionInitRspPacket> for Vec<u8> {
    fn from(packet: SessionInitRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionInitRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionInitRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let session_response = match &uci_response.child {
            UciResponseDataChild::SessionResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionResponse"),
        };
        let session_init_rsp = match &session_response.child {
            SessionResponseDataChild::SessionInitRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionInitRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            session_response,
            session_init_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.session_init_rsp.as_ref().status
    }
}
impl Into<UciPacketPacket> for SessionInitRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SessionInitRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionResponsePacket> for SessionInitRspPacket {
    fn into(self) -> SessionResponsePacket {
        SessionResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SessionInitRspBuilder {
    pub fn build(self) -> SessionInitRspPacket {
        let session_init_rsp = Arc::new(SessionInitRspData {
            status: self.status,
        });
        let session_response = Arc::new(SessionResponseData {
            child: SessionResponseDataChild::SessionInitRsp(session_init_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::SessionResponse(session_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 0,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SessionInitRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionInitRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SessionInitRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<SessionResponsePacket> for SessionInitRspBuilder {
    fn into(self) -> SessionResponsePacket {
        self.build().into()
    }
}
macro_rules! session_init_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::SessionResponse(session_response_packet) => {match session_response_packet.specialize() {/* (3) */
SessionResponseChild::SessionInitRsp(packet) => {let rebuilder = SessionInitRspBuilder {status : packet.get_status(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_init_rsp
 {:#02x?}", session_response_packet); }}}_ => {panic!("Couldn't parse session_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_init_rsp_builder_tests! { session_init_rsp_builder_test_00: b"\x41\x00\x00\x01\x11",}

#[derive(Debug)]
struct SessionDeinitCmdData {
    session_id: u32,
}
#[derive(Debug, Clone)]
pub struct SessionDeinitCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    session_command: Arc<SessionCommandData>,
    session_deinit_cmd: Arc<SessionDeinitCmdData>,
}
#[derive(Debug)]
pub struct SessionDeinitCmdBuilder {
    pub session_id: u32,
}
impl SessionDeinitCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionDeinitCmd".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        Ok(Self { session_id })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 4;
        ret
    }
}
impl Packet for SessionDeinitCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionDeinitCmdPacket> for Bytes {
    fn from(packet: SessionDeinitCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionDeinitCmdPacket> for Vec<u8> {
    fn from(packet: SessionDeinitCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionDeinitCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionDeinitCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let session_command = match &uci_command.child {
            UciCommandDataChild::SessionCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionCommand"),
        };
        let session_deinit_cmd = match &session_command.child {
            SessionCommandDataChild::SessionDeinitCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionDeinitCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            session_command,
            session_deinit_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_deinit_cmd.as_ref().session_id
    }
}
impl Into<UciPacketPacket> for SessionDeinitCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SessionDeinitCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionCommandPacket> for SessionDeinitCmdPacket {
    fn into(self) -> SessionCommandPacket {
        SessionCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionDeinitCmdBuilder {
    pub fn build(self) -> SessionDeinitCmdPacket {
        let session_deinit_cmd = Arc::new(SessionDeinitCmdData {
            session_id: self.session_id,
        });
        let session_command = Arc::new(SessionCommandData {
            child: SessionCommandDataChild::SessionDeinitCmd(session_deinit_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::SessionCommand(session_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 1,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SessionDeinitCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionDeinitCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SessionDeinitCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<SessionCommandPacket> for SessionDeinitCmdBuilder {
    fn into(self) -> SessionCommandPacket {
        self.build().into()
    }
}
macro_rules! session_deinit_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::SessionCommand(session_command_packet) => {match session_command_packet.specialize() {/* (3) */
SessionCommandChild::SessionDeinitCmd(packet) => {let rebuilder = SessionDeinitCmdBuilder {session_id : packet.get_session_id(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_deinit_cmd
 {:#02x?}", session_command_packet); }}}_ => {panic!("Couldn't parse session_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_deinit_cmd_builder_tests! { session_deinit_cmd_builder_test_00: b"\x21\x01\x00\x04\x01\x02\x03\x04",}

#[derive(Debug)]
struct SessionDeinitRspData {
    status: StatusCode,
}
#[derive(Debug, Clone)]
pub struct SessionDeinitRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    session_response: Arc<SessionResponseData>,
    session_deinit_rsp: Arc<SessionDeinitRspData>,
}
#[derive(Debug)]
pub struct SessionDeinitRspBuilder {
    pub status: StatusCode,
}
impl SessionDeinitRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SessionDeinitRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "SessionDeinitRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for SessionDeinitRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionDeinitRspPacket> for Bytes {
    fn from(packet: SessionDeinitRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionDeinitRspPacket> for Vec<u8> {
    fn from(packet: SessionDeinitRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionDeinitRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionDeinitRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let session_response = match &uci_response.child {
            UciResponseDataChild::SessionResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionResponse"),
        };
        let session_deinit_rsp = match &session_response.child {
            SessionResponseDataChild::SessionDeinitRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionDeinitRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            session_response,
            session_deinit_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.session_deinit_rsp.as_ref().status
    }
}
impl Into<UciPacketPacket> for SessionDeinitRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SessionDeinitRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionResponsePacket> for SessionDeinitRspPacket {
    fn into(self) -> SessionResponsePacket {
        SessionResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SessionDeinitRspBuilder {
    pub fn build(self) -> SessionDeinitRspPacket {
        let session_deinit_rsp = Arc::new(SessionDeinitRspData {
            status: self.status,
        });
        let session_response = Arc::new(SessionResponseData {
            child: SessionResponseDataChild::SessionDeinitRsp(session_deinit_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::SessionResponse(session_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 1,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SessionDeinitRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionDeinitRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SessionDeinitRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<SessionResponsePacket> for SessionDeinitRspBuilder {
    fn into(self) -> SessionResponsePacket {
        self.build().into()
    }
}
macro_rules! session_deinit_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::SessionResponse(session_response_packet) => {match session_response_packet.specialize() {/* (3) */
SessionResponseChild::SessionDeinitRsp(packet) => {let rebuilder = SessionDeinitRspBuilder {status : packet.get_status(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_deinit_rsp
 {:#02x?}", session_response_packet); }}}_ => {panic!("Couldn't parse session_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_deinit_rsp_builder_tests! { session_deinit_rsp_builder_test_00: b"\x41\x01\x00\x01\x00",}

#[derive(Debug)]
struct SessionStatusNtfData {
    session_id: u32,
    session_state: SessionState,
    reason_code: ReasonCode,
}
#[derive(Debug, Clone)]
pub struct SessionStatusNtfPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    session_notification: Arc<SessionNotificationData>,
    session_status_ntf: Arc<SessionStatusNtfData>,
}
#[derive(Debug)]
pub struct SessionStatusNtfBuilder {
    pub session_id: u32,
    pub session_state: SessionState,
    pub reason_code: ReasonCode,
}
impl SessionStatusNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 10 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionStatusNtf".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "SessionStatusNtf".to_string(),
                field: "session_state".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let session_state = u8::from_le_bytes([bytes[8]]);
        let session_state =
            SessionState::from_u8(session_state).ok_or_else(|| Error::InvalidEnumValueError {
                obj: "SessionStatusNtf".to_string(),
                field: "session_state".to_string(),
                value: session_state as u64,
                type_: "SessionState".to_string(),
            })?;
        if bytes.len() < 10 {
            return Err(Error::InvalidLengthError {
                obj: "SessionStatusNtf".to_string(),
                field: "reason_code".to_string(),
                wanted: 10,
                got: bytes.len(),
            });
        }
        let reason_code = u8::from_le_bytes([bytes[9]]);
        let reason_code =
            ReasonCode::from_u8(reason_code).ok_or_else(|| Error::InvalidEnumValueError {
                obj: "SessionStatusNtf".to_string(),
                field: "reason_code".to_string(),
                value: reason_code as u64,
                type_: "ReasonCode".to_string(),
            })?;
        Ok(Self {
            session_id,
            session_state,
            reason_code,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
        let session_state = self.session_state.to_u8().unwrap();
        buffer[8..9].copy_from_slice(&session_state.to_le_bytes()[0..1]);
        let reason_code = self.reason_code.to_u8().unwrap();
        buffer[9..10].copy_from_slice(&reason_code.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 6;
        ret
    }
}
impl Packet for SessionStatusNtfPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionStatusNtfPacket> for Bytes {
    fn from(packet: SessionStatusNtfPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionStatusNtfPacket> for Vec<u8> {
    fn from(packet: SessionStatusNtfPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionStatusNtfPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionStatusNtfPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let session_notification = match &uci_notification.child {
            UciNotificationDataChild::SessionNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionNotification"),
        };
        let session_status_ntf = match &session_notification.child {
            SessionNotificationDataChild::SessionStatusNtf(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionStatusNtf"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            session_notification,
            session_status_ntf,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_status_ntf.as_ref().session_id
    }
    pub fn get_session_state(&self) -> SessionState {
        self.session_status_ntf.as_ref().session_state
    }
    pub fn get_reason_code(&self) -> ReasonCode {
        self.session_status_ntf.as_ref().reason_code
    }
}
impl Into<UciPacketPacket> for SessionStatusNtfPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for SessionStatusNtfPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionNotificationPacket> for SessionStatusNtfPacket {
    fn into(self) -> SessionNotificationPacket {
        SessionNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionStatusNtfBuilder {
    pub fn build(self) -> SessionStatusNtfPacket {
        let session_status_ntf = Arc::new(SessionStatusNtfData {
            session_id: self.session_id,
            session_state: self.session_state,
            reason_code: self.reason_code,
        });
        let session_notification = Arc::new(SessionNotificationData {
            child: SessionNotificationDataChild::SessionStatusNtf(session_status_ntf),
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::SessionNotification(session_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: 2,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        SessionStatusNtfPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionStatusNtfBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for SessionStatusNtfBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}
impl Into<SessionNotificationPacket> for SessionStatusNtfBuilder {
    fn into(self) -> SessionNotificationPacket {
        self.build().into()
    }
}
macro_rules! session_status_ntf_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciNotification(uci_notification_packet) => {match uci_notification_packet.specialize() {/* (2) */
UciNotificationChild::SessionNotification(session_notification_packet) => {match session_notification_packet.specialize() {/* (3) */
SessionNotificationChild::SessionStatusNtf(packet) => {let rebuilder = SessionStatusNtfBuilder {session_id : packet.get_session_id(),session_state : packet.get_session_state(),reason_code : packet.get_reason_code(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_status_ntf
 {:#02x?}", session_notification_packet); }}}_ => {panic!("Couldn't parse session_notification
 {:#02x?}", uci_notification_packet); }}}_ => {panic!("Couldn't parse uci_notification
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_status_ntf_builder_tests! { session_status_ntf_builder_test_00: b"\x61\x02\x00\x06\x01\x02\x03\x04\x02\x21",}

#[derive(Debug)]
struct SessionSetAppConfigCmdData {
    session_id: u32,
    parameters: Vec<AppConfigParameter>,
}
#[derive(Debug, Clone)]
pub struct SessionSetAppConfigCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    session_command: Arc<SessionCommandData>,
    session_set_app_config_cmd: Arc<SessionSetAppConfigCmdData>,
}
#[derive(Debug)]
pub struct SessionSetAppConfigCmdBuilder {
    pub session_id: u32,
    pub parameters: Vec<AppConfigParameter>,
}
impl SessionSetAppConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 9 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigCmd".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigCmd".to_string(),
                field: "parameters_count".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let parameters_count = u8::from_le_bytes([bytes[8]]);
        let mut parameters: Vec<AppConfigParameter> = Vec::new();
        let mut parsable_ = &bytes[9..];
        let count_ = parameters_count as usize;
        for _ in 0..count_ {
            match AppConfigParameter::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    parameters.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self {
            session_id,
            parameters,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
        buffer[8..9].copy_from_slice(&(self.parameters.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[9..];
        for e_ in &self.parameters {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 5;
        let ret = ret
            + self
                .parameters
                .iter()
                .fold(0, |acc, x| acc + x.get_total_size());
        ret
    }
}
impl Packet for SessionSetAppConfigCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionSetAppConfigCmdPacket> for Bytes {
    fn from(packet: SessionSetAppConfigCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionSetAppConfigCmdPacket> for Vec<u8> {
    fn from(packet: SessionSetAppConfigCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionSetAppConfigCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionSetAppConfigCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let session_command = match &uci_command.child {
            UciCommandDataChild::SessionCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionCommand"),
        };
        let session_set_app_config_cmd = match &session_command.child {
            SessionCommandDataChild::SessionSetAppConfigCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionSetAppConfigCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            session_command,
            session_set_app_config_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_set_app_config_cmd.as_ref().session_id
    }
    pub fn get_parameters(&self) -> &Vec<AppConfigParameter> {
        &self.session_set_app_config_cmd.as_ref().parameters
    }
}
impl Into<UciPacketPacket> for SessionSetAppConfigCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SessionSetAppConfigCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionCommandPacket> for SessionSetAppConfigCmdPacket {
    fn into(self) -> SessionCommandPacket {
        SessionCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionSetAppConfigCmdBuilder {
    pub fn build(self) -> SessionSetAppConfigCmdPacket {
        let session_set_app_config_cmd = Arc::new(SessionSetAppConfigCmdData {
            session_id: self.session_id,
            parameters: self.parameters,
        });
        let session_command = Arc::new(SessionCommandData {
            child: SessionCommandDataChild::SessionSetAppConfigCmd(session_set_app_config_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::SessionCommand(session_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 3,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SessionSetAppConfigCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionSetAppConfigCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SessionSetAppConfigCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<SessionCommandPacket> for SessionSetAppConfigCmdBuilder {
    fn into(self) -> SessionCommandPacket {
        self.build().into()
    }
}
macro_rules! session_set_app_config_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::SessionCommand(session_command_packet) => {match session_command_packet.specialize() {/* (3) */
SessionCommandChild::SessionSetAppConfigCmd(packet) => {let rebuilder = SessionSetAppConfigCmdBuilder {session_id : packet.get_session_id(),parameters : packet.get_parameters().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_set_app_config_cmd
 {:#02x?}", session_command_packet); }}}_ => {panic!("Couldn't parse session_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_set_app_config_cmd_builder_tests! { session_set_app_config_cmd_builder_test_00: b"\x21\x03\x00\x05\x01\x02\x03\x04\x00",}

#[derive(Debug)]
struct SessionSetAppConfigRspData {
    status: StatusCode,
    parameters: Vec<AppConfigStatus>,
}
#[derive(Debug, Clone)]
pub struct SessionSetAppConfigRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    session_response: Arc<SessionResponseData>,
    session_set_app_config_rsp: Arc<SessionSetAppConfigRspData>,
}
#[derive(Debug)]
pub struct SessionSetAppConfigRspBuilder {
    pub status: StatusCode,
    pub parameters: Vec<AppConfigStatus>,
}
impl SessionSetAppConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "SessionSetAppConfigRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigRsp".to_string(),
                field: "parameters_count".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let parameters_count = u8::from_le_bytes([bytes[5]]);
        let want_ = 6 + ((parameters_count as usize) * 2);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "SessionSetAppConfigRsp".to_string(),
                field: "parameters".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let mut parameters: Vec<AppConfigStatus> = Vec::new();
        let mut parsable_ = &bytes[6..];
        let count_ = parameters_count as usize;
        for _ in 0..count_ {
            match AppConfigStatus::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    parameters.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self { status, parameters })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        buffer[5..6].copy_from_slice(&(self.parameters.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[6..];
        for e_ in &self.parameters {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        let ret = ret + (self.parameters.len() * ((/* Bits: */16 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for SessionSetAppConfigRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionSetAppConfigRspPacket> for Bytes {
    fn from(packet: SessionSetAppConfigRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionSetAppConfigRspPacket> for Vec<u8> {
    fn from(packet: SessionSetAppConfigRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionSetAppConfigRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionSetAppConfigRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let session_response = match &uci_response.child {
            UciResponseDataChild::SessionResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionResponse"),
        };
        let session_set_app_config_rsp = match &session_response.child {
            SessionResponseDataChild::SessionSetAppConfigRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionSetAppConfigRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            session_response,
            session_set_app_config_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.session_set_app_config_rsp.as_ref().status
    }
    pub fn get_parameters(&self) -> &Vec<AppConfigStatus> {
        &self.session_set_app_config_rsp.as_ref().parameters
    }
}
impl Into<UciPacketPacket> for SessionSetAppConfigRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SessionSetAppConfigRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionResponsePacket> for SessionSetAppConfigRspPacket {
    fn into(self) -> SessionResponsePacket {
        SessionResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SessionSetAppConfigRspBuilder {
    pub fn build(self) -> SessionSetAppConfigRspPacket {
        let session_set_app_config_rsp = Arc::new(SessionSetAppConfigRspData {
            status: self.status,
            parameters: self.parameters,
        });
        let session_response = Arc::new(SessionResponseData {
            child: SessionResponseDataChild::SessionSetAppConfigRsp(session_set_app_config_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::SessionResponse(session_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 3,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SessionSetAppConfigRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionSetAppConfigRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SessionSetAppConfigRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<SessionResponsePacket> for SessionSetAppConfigRspBuilder {
    fn into(self) -> SessionResponsePacket {
        self.build().into()
    }
}
macro_rules! session_set_app_config_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::SessionResponse(session_response_packet) => {match session_response_packet.specialize() {/* (3) */
SessionResponseChild::SessionSetAppConfigRsp(packet) => {let rebuilder = SessionSetAppConfigRspBuilder {status : packet.get_status(),parameters : packet.get_parameters().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_set_app_config_rsp
 {:#02x?}", session_response_packet); }}}_ => {panic!("Couldn't parse session_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_set_app_config_rsp_builder_tests! { session_set_app_config_rsp_builder_test_00: b"\x41\x03\x00\x04\x01\x01\x01\x00",}

#[derive(Debug)]
struct SessionGetAppConfigCmdData {
    session_id: u32,
    parameters: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct SessionGetAppConfigCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    session_command: Arc<SessionCommandData>,
    session_get_app_config_cmd: Arc<SessionGetAppConfigCmdData>,
}
#[derive(Debug)]
pub struct SessionGetAppConfigCmdBuilder {
    pub session_id: u32,
    pub parameters: Vec<u8>,
}
impl SessionGetAppConfigCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 9 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigCmd".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigCmd".to_string(),
                field: "parameters_count".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let parameters_count = u8::from_le_bytes([bytes[8]]);
        let want_ = 9 + ((parameters_count as usize) * 1);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigCmd".to_string(),
                field: "parameters".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let parameters: Vec<u8> = bytes[9..9 + ((parameters_count as usize) * 1)]
            .to_vec()
            .chunks_exact(1)
            .into_iter()
            .map(|i| u8::from_le_bytes([i[0]]))
            .collect();
        Ok(Self {
            session_id,
            parameters,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
        buffer[8..9].copy_from_slice(&(self.parameters.len() as u8).to_le_bytes());
        for (i, e) in self.parameters.iter().enumerate() {
            buffer[9 + i..9 + i + 1].copy_from_slice(&e.to_le_bytes())
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 5;
        let ret = ret + (self.parameters.len() * ((/* Bits: */8 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for SessionGetAppConfigCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetAppConfigCmdPacket> for Bytes {
    fn from(packet: SessionGetAppConfigCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetAppConfigCmdPacket> for Vec<u8> {
    fn from(packet: SessionGetAppConfigCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionGetAppConfigCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionGetAppConfigCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let session_command = match &uci_command.child {
            UciCommandDataChild::SessionCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionCommand"),
        };
        let session_get_app_config_cmd = match &session_command.child {
            SessionCommandDataChild::SessionGetAppConfigCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionGetAppConfigCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            session_command,
            session_get_app_config_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_get_app_config_cmd.as_ref().session_id
    }
    pub fn get_parameters(&self) -> &Vec<u8> {
        &self.session_get_app_config_cmd.as_ref().parameters
    }
}
impl Into<UciPacketPacket> for SessionGetAppConfigCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SessionGetAppConfigCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionCommandPacket> for SessionGetAppConfigCmdPacket {
    fn into(self) -> SessionCommandPacket {
        SessionCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionGetAppConfigCmdBuilder {
    pub fn build(self) -> SessionGetAppConfigCmdPacket {
        let session_get_app_config_cmd = Arc::new(SessionGetAppConfigCmdData {
            session_id: self.session_id,
            parameters: self.parameters,
        });
        let session_command = Arc::new(SessionCommandData {
            child: SessionCommandDataChild::SessionGetAppConfigCmd(session_get_app_config_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::SessionCommand(session_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 4,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SessionGetAppConfigCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionGetAppConfigCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SessionGetAppConfigCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<SessionCommandPacket> for SessionGetAppConfigCmdBuilder {
    fn into(self) -> SessionCommandPacket {
        self.build().into()
    }
}
macro_rules! session_get_app_config_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::SessionCommand(session_command_packet) => {match session_command_packet.specialize() {/* (3) */
SessionCommandChild::SessionGetAppConfigCmd(packet) => {let rebuilder = SessionGetAppConfigCmdBuilder {session_id : packet.get_session_id(),parameters : packet.get_parameters().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_get_app_config_cmd
 {:#02x?}", session_command_packet); }}}_ => {panic!("Couldn't parse session_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_get_app_config_cmd_builder_tests! { session_get_app_config_cmd_builder_test_00: b"\x21\x04\x00\x05\x01\x02\x03\x04\x00",}

#[derive(Debug)]
struct SessionGetAppConfigRspData {
    status: StatusCode,
    parameters: Vec<AppConfigParameter>,
}
#[derive(Debug, Clone)]
pub struct SessionGetAppConfigRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    session_response: Arc<SessionResponseData>,
    session_get_app_config_rsp: Arc<SessionGetAppConfigRspData>,
}
#[derive(Debug)]
pub struct SessionGetAppConfigRspBuilder {
    pub status: StatusCode,
    pub parameters: Vec<AppConfigParameter>,
}
impl SessionGetAppConfigRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "SessionGetAppConfigRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetAppConfigRsp".to_string(),
                field: "parameters_count".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let parameters_count = u8::from_le_bytes([bytes[5]]);
        let mut parameters: Vec<AppConfigParameter> = Vec::new();
        let mut parsable_ = &bytes[6..];
        let count_ = parameters_count as usize;
        for _ in 0..count_ {
            match AppConfigParameter::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    parameters.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self { status, parameters })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        buffer[5..6].copy_from_slice(&(self.parameters.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[6..];
        for e_ in &self.parameters {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        let ret = ret
            + self
                .parameters
                .iter()
                .fold(0, |acc, x| acc + x.get_total_size());
        ret
    }
}
impl Packet for SessionGetAppConfigRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetAppConfigRspPacket> for Bytes {
    fn from(packet: SessionGetAppConfigRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetAppConfigRspPacket> for Vec<u8> {
    fn from(packet: SessionGetAppConfigRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionGetAppConfigRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionGetAppConfigRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let session_response = match &uci_response.child {
            UciResponseDataChild::SessionResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionResponse"),
        };
        let session_get_app_config_rsp = match &session_response.child {
            SessionResponseDataChild::SessionGetAppConfigRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionGetAppConfigRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            session_response,
            session_get_app_config_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.session_get_app_config_rsp.as_ref().status
    }
    pub fn get_parameters(&self) -> &Vec<AppConfigParameter> {
        &self.session_get_app_config_rsp.as_ref().parameters
    }
}
impl Into<UciPacketPacket> for SessionGetAppConfigRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SessionGetAppConfigRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionResponsePacket> for SessionGetAppConfigRspPacket {
    fn into(self) -> SessionResponsePacket {
        SessionResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SessionGetAppConfigRspBuilder {
    pub fn build(self) -> SessionGetAppConfigRspPacket {
        let session_get_app_config_rsp = Arc::new(SessionGetAppConfigRspData {
            status: self.status,
            parameters: self.parameters,
        });
        let session_response = Arc::new(SessionResponseData {
            child: SessionResponseDataChild::SessionGetAppConfigRsp(session_get_app_config_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::SessionResponse(session_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 4,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SessionGetAppConfigRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionGetAppConfigRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SessionGetAppConfigRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<SessionResponsePacket> for SessionGetAppConfigRspBuilder {
    fn into(self) -> SessionResponsePacket {
        self.build().into()
    }
}
macro_rules! session_get_app_config_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::SessionResponse(session_response_packet) => {match session_response_packet.specialize() {/* (3) */
SessionResponseChild::SessionGetAppConfigRsp(packet) => {let rebuilder = SessionGetAppConfigRspBuilder {status : packet.get_status(),parameters : packet.get_parameters().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_get_app_config_rsp
 {:#02x?}", session_response_packet); }}}_ => {panic!("Couldn't parse session_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_get_app_config_rsp_builder_tests! { session_get_app_config_rsp_builder_test_00: b"\x41\x04\x00\x02\x01\x00",}

#[derive(Debug)]
struct SessionGetCountCmdData {}
#[derive(Debug, Clone)]
pub struct SessionGetCountCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    session_command: Arc<SessionCommandData>,
    session_get_count_cmd: Arc<SessionGetCountCmdData>,
}
#[derive(Debug)]
pub struct SessionGetCountCmdBuilder {}
impl SessionGetCountCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for SessionGetCountCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetCountCmdPacket> for Bytes {
    fn from(packet: SessionGetCountCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetCountCmdPacket> for Vec<u8> {
    fn from(packet: SessionGetCountCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionGetCountCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionGetCountCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let session_command = match &uci_command.child {
            UciCommandDataChild::SessionCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionCommand"),
        };
        let session_get_count_cmd = match &session_command.child {
            SessionCommandDataChild::SessionGetCountCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionGetCountCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            session_command,
            session_get_count_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for SessionGetCountCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SessionGetCountCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionCommandPacket> for SessionGetCountCmdPacket {
    fn into(self) -> SessionCommandPacket {
        SessionCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionGetCountCmdBuilder {
    pub fn build(self) -> SessionGetCountCmdPacket {
        let session_get_count_cmd = Arc::new(SessionGetCountCmdData {});
        let session_command = Arc::new(SessionCommandData {
            child: SessionCommandDataChild::SessionGetCountCmd(session_get_count_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::SessionCommand(session_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 5,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SessionGetCountCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionGetCountCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SessionGetCountCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<SessionCommandPacket> for SessionGetCountCmdBuilder {
    fn into(self) -> SessionCommandPacket {
        self.build().into()
    }
}
macro_rules! session_get_count_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::SessionCommand(session_command_packet) => {match session_command_packet.specialize() {/* (3) */
SessionCommandChild::SessionGetCountCmd(packet) => {let rebuilder = SessionGetCountCmdBuilder {};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_get_count_cmd
 {:#02x?}", session_command_packet); }}}_ => {panic!("Couldn't parse session_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_get_count_cmd_builder_tests! { session_get_count_cmd_builder_test_00: b"\x21\x05\x00\x00",}

#[derive(Debug)]
struct SessionGetCountRspData {
    status: StatusCode,
    session_count: u8,
}
#[derive(Debug, Clone)]
pub struct SessionGetCountRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    session_response: Arc<SessionResponseData>,
    session_get_count_rsp: Arc<SessionGetCountRspData>,
}
#[derive(Debug)]
pub struct SessionGetCountRspBuilder {
    pub status: StatusCode,
    pub session_count: u8,
}
impl SessionGetCountRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetCountRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "SessionGetCountRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetCountRsp".to_string(),
                field: "session_count".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let session_count = u8::from_le_bytes([bytes[5]]);
        Ok(Self {
            status,
            session_count,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        let session_count = self.session_count;
        buffer[5..6].copy_from_slice(&session_count.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        ret
    }
}
impl Packet for SessionGetCountRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetCountRspPacket> for Bytes {
    fn from(packet: SessionGetCountRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetCountRspPacket> for Vec<u8> {
    fn from(packet: SessionGetCountRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionGetCountRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionGetCountRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let session_response = match &uci_response.child {
            UciResponseDataChild::SessionResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionResponse"),
        };
        let session_get_count_rsp = match &session_response.child {
            SessionResponseDataChild::SessionGetCountRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionGetCountRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            session_response,
            session_get_count_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.session_get_count_rsp.as_ref().status
    }
    pub fn get_session_count(&self) -> u8 {
        self.session_get_count_rsp.as_ref().session_count
    }
}
impl Into<UciPacketPacket> for SessionGetCountRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SessionGetCountRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionResponsePacket> for SessionGetCountRspPacket {
    fn into(self) -> SessionResponsePacket {
        SessionResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SessionGetCountRspBuilder {
    pub fn build(self) -> SessionGetCountRspPacket {
        let session_get_count_rsp = Arc::new(SessionGetCountRspData {
            status: self.status,
            session_count: self.session_count,
        });
        let session_response = Arc::new(SessionResponseData {
            child: SessionResponseDataChild::SessionGetCountRsp(session_get_count_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::SessionResponse(session_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 5,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SessionGetCountRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionGetCountRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SessionGetCountRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<SessionResponsePacket> for SessionGetCountRspBuilder {
    fn into(self) -> SessionResponsePacket {
        self.build().into()
    }
}
macro_rules! session_get_count_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::SessionResponse(session_response_packet) => {match session_response_packet.specialize() {/* (3) */
SessionResponseChild::SessionGetCountRsp(packet) => {let rebuilder = SessionGetCountRspBuilder {status : packet.get_status(),session_count : packet.get_session_count(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_get_count_rsp
 {:#02x?}", session_response_packet); }}}_ => {panic!("Couldn't parse session_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_get_count_rsp_builder_tests! { session_get_count_rsp_builder_test_00: b"\x41\x05\x00\x02\x00\x01",}

#[derive(Debug)]
struct SessionGetStateCmdData {
    session_id: u32,
}
#[derive(Debug, Clone)]
pub struct SessionGetStateCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    session_command: Arc<SessionCommandData>,
    session_get_state_cmd: Arc<SessionGetStateCmdData>,
}
#[derive(Debug)]
pub struct SessionGetStateCmdBuilder {
    pub session_id: u32,
}
impl SessionGetStateCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetStateCmd".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        Ok(Self { session_id })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 4;
        ret
    }
}
impl Packet for SessionGetStateCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetStateCmdPacket> for Bytes {
    fn from(packet: SessionGetStateCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetStateCmdPacket> for Vec<u8> {
    fn from(packet: SessionGetStateCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionGetStateCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionGetStateCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let session_command = match &uci_command.child {
            UciCommandDataChild::SessionCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionCommand"),
        };
        let session_get_state_cmd = match &session_command.child {
            SessionCommandDataChild::SessionGetStateCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionGetStateCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            session_command,
            session_get_state_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_get_state_cmd.as_ref().session_id
    }
}
impl Into<UciPacketPacket> for SessionGetStateCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SessionGetStateCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionCommandPacket> for SessionGetStateCmdPacket {
    fn into(self) -> SessionCommandPacket {
        SessionCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionGetStateCmdBuilder {
    pub fn build(self) -> SessionGetStateCmdPacket {
        let session_get_state_cmd = Arc::new(SessionGetStateCmdData {
            session_id: self.session_id,
        });
        let session_command = Arc::new(SessionCommandData {
            child: SessionCommandDataChild::SessionGetStateCmd(session_get_state_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::SessionCommand(session_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 6,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SessionGetStateCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionGetStateCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SessionGetStateCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<SessionCommandPacket> for SessionGetStateCmdBuilder {
    fn into(self) -> SessionCommandPacket {
        self.build().into()
    }
}
macro_rules! session_get_state_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::SessionCommand(session_command_packet) => {match session_command_packet.specialize() {/* (3) */
SessionCommandChild::SessionGetStateCmd(packet) => {let rebuilder = SessionGetStateCmdBuilder {session_id : packet.get_session_id(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_get_state_cmd
 {:#02x?}", session_command_packet); }}}_ => {panic!("Couldn't parse session_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_get_state_cmd_builder_tests! { session_get_state_cmd_builder_test_00: b"\x21\x06\x00\x04\x00\x01\x02\x03",}

#[derive(Debug)]
struct SessionGetStateRspData {
    status: StatusCode,
    session_state: SessionState,
}
#[derive(Debug, Clone)]
pub struct SessionGetStateRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    session_response: Arc<SessionResponseData>,
    session_get_state_rsp: Arc<SessionGetStateRspData>,
}
#[derive(Debug)]
pub struct SessionGetStateRspBuilder {
    pub status: StatusCode,
    pub session_state: SessionState,
}
impl SessionGetStateRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetStateRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "SessionGetStateRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "SessionGetStateRsp".to_string(),
                field: "session_state".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let session_state = u8::from_le_bytes([bytes[5]]);
        let session_state =
            SessionState::from_u8(session_state).ok_or_else(|| Error::InvalidEnumValueError {
                obj: "SessionGetStateRsp".to_string(),
                field: "session_state".to_string(),
                value: session_state as u64,
                type_: "SessionState".to_string(),
            })?;
        Ok(Self {
            status,
            session_state,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        let session_state = self.session_state.to_u8().unwrap();
        buffer[5..6].copy_from_slice(&session_state.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        ret
    }
}
impl Packet for SessionGetStateRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionGetStateRspPacket> for Bytes {
    fn from(packet: SessionGetStateRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionGetStateRspPacket> for Vec<u8> {
    fn from(packet: SessionGetStateRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionGetStateRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionGetStateRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let session_response = match &uci_response.child {
            UciResponseDataChild::SessionResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionResponse"),
        };
        let session_get_state_rsp = match &session_response.child {
            SessionResponseDataChild::SessionGetStateRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionGetStateRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            session_response,
            session_get_state_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.session_get_state_rsp.as_ref().status
    }
    pub fn get_session_state(&self) -> SessionState {
        self.session_get_state_rsp.as_ref().session_state
    }
}
impl Into<UciPacketPacket> for SessionGetStateRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SessionGetStateRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionResponsePacket> for SessionGetStateRspPacket {
    fn into(self) -> SessionResponsePacket {
        SessionResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SessionGetStateRspBuilder {
    pub fn build(self) -> SessionGetStateRspPacket {
        let session_get_state_rsp = Arc::new(SessionGetStateRspData {
            status: self.status,
            session_state: self.session_state,
        });
        let session_response = Arc::new(SessionResponseData {
            child: SessionResponseDataChild::SessionGetStateRsp(session_get_state_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::SessionResponse(session_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 6,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SessionGetStateRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionGetStateRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SessionGetStateRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<SessionResponsePacket> for SessionGetStateRspBuilder {
    fn into(self) -> SessionResponsePacket {
        self.build().into()
    }
}
macro_rules! session_get_state_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::SessionResponse(session_response_packet) => {match session_response_packet.specialize() {/* (3) */
SessionResponseChild::SessionGetStateRsp(packet) => {let rebuilder = SessionGetStateRspBuilder {status : packet.get_status(),session_state : packet.get_session_state(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_get_state_rsp
 {:#02x?}", session_response_packet); }}}_ => {panic!("Couldn't parse session_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_get_state_rsp_builder_tests! { session_get_state_rsp_builder_test_00: b"\x41\x06\x00\x02\x00\x01",}

#[derive(Debug)]
struct SessionUpdateControllerMulticastListCmdData {
    session_id: u32,
    action: u8,
    controlees: Vec<Controlee>,
}
#[derive(Debug, Clone)]
pub struct SessionUpdateControllerMulticastListCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    session_command: Arc<SessionCommandData>,
    session_update_controller_multicast_list_cmd: Arc<SessionUpdateControllerMulticastListCmdData>,
}
#[derive(Debug)]
pub struct SessionUpdateControllerMulticastListCmdBuilder {
    pub session_id: u32,
    pub action: u8,
    pub controlees: Vec<Controlee>,
}
impl SessionUpdateControllerMulticastListCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 10 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd".to_string(),
                field: "action".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let action = u8::from_le_bytes([bytes[8]]);
        if bytes.len() < 10 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd".to_string(),
                field: "controlees_count".to_string(),
                wanted: 10,
                got: bytes.len(),
            });
        }
        let controlees_count = u8::from_le_bytes([bytes[9]]);
        let want_ = 10 + ((controlees_count as usize) * 6);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListCmd".to_string(),
                field: "controlees".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let mut controlees: Vec<Controlee> = Vec::new();
        let mut parsable_ = &bytes[10..];
        let count_ = controlees_count as usize;
        for _ in 0..count_ {
            match Controlee::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    controlees.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self {
            session_id,
            action,
            controlees,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
        let action = self.action;
        buffer[8..9].copy_from_slice(&action.to_le_bytes()[0..1]);
        buffer[9..10].copy_from_slice(&(self.controlees.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[10..];
        for e_ in &self.controlees {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 6;
        let ret = ret + (self.controlees.len() * ((/* Bits: */48 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for SessionUpdateControllerMulticastListCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListCmdPacket> for Bytes {
    fn from(packet: SessionUpdateControllerMulticastListCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionUpdateControllerMulticastListCmdPacket> for Vec<u8> {
    fn from(packet: SessionUpdateControllerMulticastListCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionUpdateControllerMulticastListCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionUpdateControllerMulticastListCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let session_command = match &uci_command.child {
            UciCommandDataChild::SessionCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionCommand"),
        };
        let session_update_controller_multicast_list_cmd =
            match &session_command.child {
                SessionCommandDataChild::SessionUpdateControllerMulticastListCmd(value) => {
                    (*value).clone()
                }
                _ => return Err(
                    "inconsistent state - child was not SessionUpdateControllerMulticastListCmd",
                ),
            };
        Ok(Self {
            uci_packet,
            uci_command,
            session_command,
            session_update_controller_multicast_list_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_update_controller_multicast_list_cmd
            .as_ref()
            .session_id
    }
    pub fn get_action(&self) -> u8 {
        self.session_update_controller_multicast_list_cmd
            .as_ref()
            .action
    }
    pub fn get_controlees(&self) -> &Vec<Controlee> {
        &self
            .session_update_controller_multicast_list_cmd
            .as_ref()
            .controlees
    }
}
impl Into<UciPacketPacket> for SessionUpdateControllerMulticastListCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for SessionUpdateControllerMulticastListCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionCommandPacket> for SessionUpdateControllerMulticastListCmdPacket {
    fn into(self) -> SessionCommandPacket {
        SessionCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionUpdateControllerMulticastListCmdBuilder {
    pub fn build(self) -> SessionUpdateControllerMulticastListCmdPacket {
        let session_update_controller_multicast_list_cmd =
            Arc::new(SessionUpdateControllerMulticastListCmdData {
                session_id: self.session_id,
                action: self.action,
                controlees: self.controlees,
            });
        let session_command = Arc::new(SessionCommandData {
            child: SessionCommandDataChild::SessionUpdateControllerMulticastListCmd(
                session_update_controller_multicast_list_cmd,
            ),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::SessionCommand(session_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 7,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        SessionUpdateControllerMulticastListCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionUpdateControllerMulticastListCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for SessionUpdateControllerMulticastListCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<SessionCommandPacket> for SessionUpdateControllerMulticastListCmdBuilder {
    fn into(self) -> SessionCommandPacket {
        self.build().into()
    }
}
macro_rules! session_update_controller_multicast_list_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::SessionCommand(session_command_packet) => {match session_command_packet.specialize() {/* (3) */
SessionCommandChild::SessionUpdateControllerMulticastListCmd(packet) => {let rebuilder = SessionUpdateControllerMulticastListCmdBuilder {session_id : packet.get_session_id(),action : packet.get_action(),controlees : packet.get_controlees().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_update_controller_multicast_list_cmd
 {:#02x?}", session_command_packet); }}}_ => {panic!("Couldn't parse session_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_update_controller_multicast_list_cmd_builder_tests! { session_update_controller_multicast_list_cmd_builder_test_00: b"\x21\x07\x00\x06\x00\x01\x02\x03\x04\x00",}

#[derive(Debug)]
struct SessionUpdateControllerMulticastListRspData {
    status: StatusCode,
}
#[derive(Debug, Clone)]
pub struct SessionUpdateControllerMulticastListRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    session_response: Arc<SessionResponseData>,
    session_update_controller_multicast_list_rsp: Arc<SessionUpdateControllerMulticastListRspData>,
}
#[derive(Debug)]
pub struct SessionUpdateControllerMulticastListRspBuilder {
    pub status: StatusCode,
}
impl SessionUpdateControllerMulticastListRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "SessionUpdateControllerMulticastListRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for SessionUpdateControllerMulticastListRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListRspPacket> for Bytes {
    fn from(packet: SessionUpdateControllerMulticastListRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionUpdateControllerMulticastListRspPacket> for Vec<u8> {
    fn from(packet: SessionUpdateControllerMulticastListRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionUpdateControllerMulticastListRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionUpdateControllerMulticastListRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let session_response = match &uci_response.child {
            UciResponseDataChild::SessionResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionResponse"),
        };
        let session_update_controller_multicast_list_rsp =
            match &session_response.child {
                SessionResponseDataChild::SessionUpdateControllerMulticastListRsp(value) => {
                    (*value).clone()
                }
                _ => return Err(
                    "inconsistent state - child was not SessionUpdateControllerMulticastListRsp",
                ),
            };
        Ok(Self {
            uci_packet,
            uci_response,
            session_response,
            session_update_controller_multicast_list_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.session_update_controller_multicast_list_rsp
            .as_ref()
            .status
    }
}
impl Into<UciPacketPacket> for SessionUpdateControllerMulticastListRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for SessionUpdateControllerMulticastListRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionResponsePacket> for SessionUpdateControllerMulticastListRspPacket {
    fn into(self) -> SessionResponsePacket {
        SessionResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl SessionUpdateControllerMulticastListRspBuilder {
    pub fn build(self) -> SessionUpdateControllerMulticastListRspPacket {
        let session_update_controller_multicast_list_rsp =
            Arc::new(SessionUpdateControllerMulticastListRspData {
                status: self.status,
            });
        let session_response = Arc::new(SessionResponseData {
            child: SessionResponseDataChild::SessionUpdateControllerMulticastListRsp(
                session_update_controller_multicast_list_rsp,
            ),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::SessionResponse(session_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 7,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        SessionUpdateControllerMulticastListRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionUpdateControllerMulticastListRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for SessionUpdateControllerMulticastListRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<SessionResponsePacket> for SessionUpdateControllerMulticastListRspBuilder {
    fn into(self) -> SessionResponsePacket {
        self.build().into()
    }
}
macro_rules! session_update_controller_multicast_list_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::SessionResponse(session_response_packet) => {match session_response_packet.specialize() {/* (3) */
SessionResponseChild::SessionUpdateControllerMulticastListRsp(packet) => {let rebuilder = SessionUpdateControllerMulticastListRspBuilder {status : packet.get_status(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_update_controller_multicast_list_rsp
 {:#02x?}", session_response_packet); }}}_ => {panic!("Couldn't parse session_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_update_controller_multicast_list_rsp_builder_tests! { session_update_controller_multicast_list_rsp_builder_test_00: b"\x41\x07\x00\x01\x00",}

#[derive(Debug)]
struct SessionUpdateControllerMulticastListNtfData {
    session_id: u32,
    remaining_multicast_list_size: u8,
    controlee_status: Vec<ControleeStatus>,
}
#[derive(Debug, Clone)]
pub struct SessionUpdateControllerMulticastListNtfPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    session_notification: Arc<SessionNotificationData>,
    session_update_controller_multicast_list_ntf: Arc<SessionUpdateControllerMulticastListNtfData>,
}
#[derive(Debug)]
pub struct SessionUpdateControllerMulticastListNtfBuilder {
    pub session_id: u32,
    pub remaining_multicast_list_size: u8,
    pub controlee_status: Vec<ControleeStatus>,
}
impl SessionUpdateControllerMulticastListNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 10 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListNtf".to_string(),
                field: "session_id".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListNtf".to_string(),
                field: "remaining_multicast_list_size".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let remaining_multicast_list_size = u8::from_le_bytes([bytes[8]]);
        if bytes.len() < 10 {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListNtf".to_string(),
                field: "controlee_status_count".to_string(),
                wanted: 10,
                got: bytes.len(),
            });
        }
        let controlee_status_count = u8::from_le_bytes([bytes[9]]);
        let want_ = 10 + ((controlee_status_count as usize) * 7);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "SessionUpdateControllerMulticastListNtf".to_string(),
                field: "controlee_status".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let mut controlee_status: Vec<ControleeStatus> = Vec::new();
        let mut parsable_ = &bytes[10..];
        let count_ = controlee_status_count as usize;
        for _ in 0..count_ {
            match ControleeStatus::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    controlee_status.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self {
            session_id,
            remaining_multicast_list_size,
            controlee_status,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let session_id = self.session_id;
        buffer[4..8].copy_from_slice(&session_id.to_le_bytes()[0..4]);
        let remaining_multicast_list_size = self.remaining_multicast_list_size;
        buffer[8..9].copy_from_slice(&remaining_multicast_list_size.to_le_bytes()[0..1]);
        buffer[9..10].copy_from_slice(&(self.controlee_status.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[10..];
        for e_ in &self.controlee_status {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 6;
        let ret = ret + (self.controlee_status.len() * ((/* Bits: */56 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for SessionUpdateControllerMulticastListNtfPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<SessionUpdateControllerMulticastListNtfPacket> for Bytes {
    fn from(packet: SessionUpdateControllerMulticastListNtfPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<SessionUpdateControllerMulticastListNtfPacket> for Vec<u8> {
    fn from(packet: SessionUpdateControllerMulticastListNtfPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for SessionUpdateControllerMulticastListNtfPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl SessionUpdateControllerMulticastListNtfPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let session_notification = match &uci_notification.child {
            UciNotificationDataChild::SessionNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not SessionNotification"),
        };
        let session_update_controller_multicast_list_ntf =
            match &session_notification.child {
                SessionNotificationDataChild::SessionUpdateControllerMulticastListNtf(value) => {
                    (*value).clone()
                }
                _ => return Err(
                    "inconsistent state - child was not SessionUpdateControllerMulticastListNtf",
                ),
            };
        Ok(Self {
            uci_packet,
            uci_notification,
            session_notification,
            session_update_controller_multicast_list_ntf,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_update_controller_multicast_list_ntf
            .as_ref()
            .session_id
    }
    pub fn get_remaining_multicast_list_size(&self) -> u8 {
        self.session_update_controller_multicast_list_ntf
            .as_ref()
            .remaining_multicast_list_size
    }
    pub fn get_controlee_status(&self) -> &Vec<ControleeStatus> {
        &self
            .session_update_controller_multicast_list_ntf
            .as_ref()
            .controlee_status
    }
}
impl Into<UciPacketPacket> for SessionUpdateControllerMulticastListNtfPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for SessionUpdateControllerMulticastListNtfPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<SessionNotificationPacket> for SessionUpdateControllerMulticastListNtfPacket {
    fn into(self) -> SessionNotificationPacket {
        SessionNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl SessionUpdateControllerMulticastListNtfBuilder {
    pub fn build(self) -> SessionUpdateControllerMulticastListNtfPacket {
        let session_update_controller_multicast_list_ntf =
            Arc::new(SessionUpdateControllerMulticastListNtfData {
                session_id: self.session_id,
                remaining_multicast_list_size: self.remaining_multicast_list_size,
                controlee_status: self.controlee_status,
            });
        let session_notification = Arc::new(SessionNotificationData {
            child: SessionNotificationDataChild::SessionUpdateControllerMulticastListNtf(
                session_update_controller_multicast_list_ntf,
            ),
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::SessionNotification(session_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::SessionConfig,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: 7,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        SessionUpdateControllerMulticastListNtfPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for SessionUpdateControllerMulticastListNtfBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for SessionUpdateControllerMulticastListNtfBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}
impl Into<SessionNotificationPacket> for SessionUpdateControllerMulticastListNtfBuilder {
    fn into(self) -> SessionNotificationPacket {
        self.build().into()
    }
}
macro_rules! session_update_controller_multicast_list_ntf_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciNotification(uci_notification_packet) => {match uci_notification_packet.specialize() {/* (2) */
UciNotificationChild::SessionNotification(session_notification_packet) => {match session_notification_packet.specialize() {/* (3) */
SessionNotificationChild::SessionUpdateControllerMulticastListNtf(packet) => {let rebuilder = SessionUpdateControllerMulticastListNtfBuilder {session_id : packet.get_session_id(),remaining_multicast_list_size : packet.get_remaining_multicast_list_size(),controlee_status : packet.get_controlee_status().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse session_update_controller_multicast_list_ntf
 {:#02x?}", session_notification_packet); }}}_ => {panic!("Couldn't parse session_notification
 {:#02x?}", uci_notification_packet); }}}_ => {panic!("Couldn't parse uci_notification
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
session_update_controller_multicast_list_ntf_builder_tests! { session_update_controller_multicast_list_ntf_builder_test_00: b"\x61\x07\x00\x06\x00\x01\x02\x03\x04\x00",}

#[derive(Debug)]
struct RangeStartCmdData {}
#[derive(Debug, Clone)]
pub struct RangeStartCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    ranging_command: Arc<RangingCommandData>,
    range_start_cmd: Arc<RangeStartCmdData>,
}
#[derive(Debug)]
pub struct RangeStartCmdBuilder {
    pub session_id: u32,
}
impl RangeStartCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for RangeStartCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangeStartCmdPacket> for Bytes {
    fn from(packet: RangeStartCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangeStartCmdPacket> for Vec<u8> {
    fn from(packet: RangeStartCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangeStartCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangeStartCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let ranging_command = match &uci_command.child {
            UciCommandDataChild::RangingCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingCommand"),
        };
        let range_start_cmd = match &ranging_command.child {
            RangingCommandDataChild::RangeStartCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeStartCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            ranging_command,
            range_start_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.ranging_command.as_ref().session_id
    }
}
impl Into<UciPacketPacket> for RangeStartCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for RangeStartCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingCommandPacket> for RangeStartCmdPacket {
    fn into(self) -> RangingCommandPacket {
        RangingCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl RangeStartCmdBuilder {
    pub fn build(self) -> RangeStartCmdPacket {
        let range_start_cmd = Arc::new(RangeStartCmdData {});
        let ranging_command = Arc::new(RangingCommandData {
            session_id: self.session_id,
            child: RangingCommandDataChild::RangeStartCmd(range_start_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::RangingCommand(ranging_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 0,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        RangeStartCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangeStartCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for RangeStartCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<RangingCommandPacket> for RangeStartCmdBuilder {
    fn into(self) -> RangingCommandPacket {
        self.build().into()
    }
}
macro_rules! range_start_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::RangingCommand(ranging_command_packet) => {match ranging_command_packet.specialize() {/* (3) */
RangingCommandChild::RangeStartCmd(packet) => {let rebuilder = RangeStartCmdBuilder {session_id : packet.get_session_id(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse range_start_cmd
 {:#02x?}", ranging_command_packet); }}}_ => {panic!("Couldn't parse ranging_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
range_start_cmd_builder_tests! { range_start_cmd_builder_test_00: b"\x22\x00\x00\x04\x00\x01\x02\x03",}

#[derive(Debug)]
struct RangeStartRspData {
    status: StatusCode,
}
#[derive(Debug, Clone)]
pub struct RangeStartRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    ranging_response: Arc<RangingResponseData>,
    range_start_rsp: Arc<RangeStartRspData>,
}
#[derive(Debug)]
pub struct RangeStartRspBuilder {
    pub status: StatusCode,
}
impl RangeStartRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "RangeStartRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "RangeStartRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for RangeStartRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangeStartRspPacket> for Bytes {
    fn from(packet: RangeStartRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangeStartRspPacket> for Vec<u8> {
    fn from(packet: RangeStartRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangeStartRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangeStartRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let ranging_response = match &uci_response.child {
            UciResponseDataChild::RangingResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingResponse"),
        };
        let range_start_rsp = match &ranging_response.child {
            RangingResponseDataChild::RangeStartRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeStartRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            ranging_response,
            range_start_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.range_start_rsp.as_ref().status
    }
}
impl Into<UciPacketPacket> for RangeStartRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for RangeStartRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingResponsePacket> for RangeStartRspPacket {
    fn into(self) -> RangingResponsePacket {
        RangingResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl RangeStartRspBuilder {
    pub fn build(self) -> RangeStartRspPacket {
        let range_start_rsp = Arc::new(RangeStartRspData {
            status: self.status,
        });
        let ranging_response = Arc::new(RangingResponseData {
            child: RangingResponseDataChild::RangeStartRsp(range_start_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::RangingResponse(ranging_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 0,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        RangeStartRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangeStartRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for RangeStartRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<RangingResponsePacket> for RangeStartRspBuilder {
    fn into(self) -> RangingResponsePacket {
        self.build().into()
    }
}
macro_rules! range_start_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::RangingResponse(ranging_response_packet) => {match ranging_response_packet.specialize() {/* (3) */
RangingResponseChild::RangeStartRsp(packet) => {let rebuilder = RangeStartRspBuilder {status : packet.get_status(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse range_start_rsp
 {:#02x?}", ranging_response_packet); }}}_ => {panic!("Couldn't parse ranging_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
range_start_rsp_builder_tests! { range_start_rsp_builder_test_00: b"\x42\x00\x00\x01\x00",}

#[derive(Debug)]
enum RangeDataNtfDataChild {
    ShortMacTwoWayRangeDataNtf(Arc<ShortMacTwoWayRangeDataNtfData>),
    ExtendedMacTwoWayRangeDataNtf(Arc<ExtendedMacTwoWayRangeDataNtfData>),
    None,
}
impl RangeDataNtfDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            RangeDataNtfDataChild::ShortMacTwoWayRangeDataNtf(value) => value.get_total_size(),
            RangeDataNtfDataChild::ExtendedMacTwoWayRangeDataNtf(value) => value.get_total_size(),
            RangeDataNtfDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum RangeDataNtfChild {
    ShortMacTwoWayRangeDataNtf(ShortMacTwoWayRangeDataNtfPacket),
    ExtendedMacTwoWayRangeDataNtf(ExtendedMacTwoWayRangeDataNtfPacket),
    None,
}
#[derive(Debug)]
struct RangeDataNtfData {
    sequence_number: u32,
    session_id: u32,
    rcr_indicator: u8,
    current_ranging_interval: u32,
    ranging_measurement_type: RangingMeasurementType,
    mac_address_indicator: MacAddressIndicator,
    child: RangeDataNtfDataChild,
}
#[derive(Debug, Clone)]
pub struct RangeDataNtfPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    ranging_notification: Arc<RangingNotificationData>,
    range_data_ntf: Arc<RangeDataNtfData>,
}
#[derive(Debug)]
pub struct RangeDataNtfBuilder {
    pub sequence_number: u32,
    pub session_id: u32,
    pub rcr_indicator: u8,
    pub current_ranging_interval: u32,
    pub ranging_measurement_type: RangingMeasurementType,
    pub mac_address_indicator: MacAddressIndicator,
}
impl RangeDataNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 28 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "RangeDataNtf".to_string(),
                field: "sequence_number".to_string(),
                wanted: 8,
                got: bytes.len(),
            });
        }
        let sequence_number = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if bytes.len() < 12 {
            return Err(Error::InvalidLengthError {
                obj: "RangeDataNtf".to_string(),
                field: "session_id".to_string(),
                wanted: 12,
                got: bytes.len(),
            });
        }
        let session_id = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        if bytes.len() < 13 {
            return Err(Error::InvalidLengthError {
                obj: "RangeDataNtf".to_string(),
                field: "rcr_indicator".to_string(),
                wanted: 13,
                got: bytes.len(),
            });
        }
        let rcr_indicator = u8::from_le_bytes([bytes[12]]);
        if bytes.len() < 17 {
            return Err(Error::InvalidLengthError {
                obj: "RangeDataNtf".to_string(),
                field: "current_ranging_interval".to_string(),
                wanted: 17,
                got: bytes.len(),
            });
        }
        let current_ranging_interval =
            u32::from_le_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
        if bytes.len() < 18 {
            return Err(Error::InvalidLengthError {
                obj: "RangeDataNtf".to_string(),
                field: "ranging_measurement_type".to_string(),
                wanted: 18,
                got: bytes.len(),
            });
        }
        let ranging_measurement_type = u8::from_le_bytes([bytes[17]]);
        let ranging_measurement_type = RangingMeasurementType::from_u8(ranging_measurement_type)
            .ok_or_else(|| Error::InvalidEnumValueError {
                obj: "RangeDataNtf".to_string(),
                field: "ranging_measurement_type".to_string(),
                value: ranging_measurement_type as u64,
                type_: "RangingMeasurementType".to_string(),
            })?;
        if bytes.len() < 20 {
            return Err(Error::InvalidLengthError {
                obj: "RangeDataNtf".to_string(),
                field: "mac_address_indicator".to_string(),
                wanted: 20,
                got: bytes.len(),
            });
        }
        let mac_address_indicator = u8::from_le_bytes([bytes[19]]);
        let mac_address_indicator = MacAddressIndicator::from_u8(mac_address_indicator)
            .ok_or_else(|| Error::InvalidEnumValueError {
                obj: "RangeDataNtf".to_string(),
                field: "mac_address_indicator".to_string(),
                value: mac_address_indicator as u64,
                type_: "MacAddressIndicator".to_string(),
            })?;
        let child = match (ranging_measurement_type, mac_address_indicator) {
            (RangingMeasurementType::TwoWay, MacAddressIndicator::ShortAddress)
                if ShortMacTwoWayRangeDataNtfData::conforms(&bytes[..]) =>
            {
                RangeDataNtfDataChild::ShortMacTwoWayRangeDataNtf(Arc::new(
                    ShortMacTwoWayRangeDataNtfData::parse(&bytes[..])?,
                ))
            }
            (RangingMeasurementType::TwoWay, MacAddressIndicator::ExtendedAddress)
                if ExtendedMacTwoWayRangeDataNtfData::conforms(&bytes[..]) =>
            {
                RangeDataNtfDataChild::ExtendedMacTwoWayRangeDataNtf(Arc::new(
                    ExtendedMacTwoWayRangeDataNtfData::parse(&bytes[..])?,
                ))
            }
            (_, _) => return Err(Error::InvalidPacketError),
        };
        Ok(Self {
            sequence_number,
            session_id,
            rcr_indicator,
            current_ranging_interval,
            ranging_measurement_type,
            mac_address_indicator,
            child,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let sequence_number = self.sequence_number;
        buffer[4..8].copy_from_slice(&sequence_number.to_le_bytes()[0..4]);
        let session_id = self.session_id;
        buffer[8..12].copy_from_slice(&session_id.to_le_bytes()[0..4]);
        let rcr_indicator = self.rcr_indicator;
        buffer[12..13].copy_from_slice(&rcr_indicator.to_le_bytes()[0..1]);
        let current_ranging_interval = self.current_ranging_interval;
        buffer[13..17].copy_from_slice(&current_ranging_interval.to_le_bytes()[0..4]);
        let ranging_measurement_type = self.ranging_measurement_type.to_u8().unwrap();
        buffer[17..18].copy_from_slice(&ranging_measurement_type.to_le_bytes()[0..1]);
        let mac_address_indicator = self.mac_address_indicator.to_u8().unwrap();
        buffer[19..20].copy_from_slice(&mac_address_indicator.to_le_bytes()[0..1]);
        match &self.child {
            RangeDataNtfDataChild::ShortMacTwoWayRangeDataNtf(value) => value.write_to(buffer),
            RangeDataNtfDataChild::ExtendedMacTwoWayRangeDataNtf(value) => value.write_to(buffer),
            RangeDataNtfDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 24;
        ret
    }
}
impl Packet for RangeDataNtfPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangeDataNtfPacket> for Bytes {
    fn from(packet: RangeDataNtfPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangeDataNtfPacket> for Vec<u8> {
    fn from(packet: RangeDataNtfPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangeDataNtfPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangeDataNtfPacket {
    pub fn specialize(&self) -> RangeDataNtfChild {
        match &self.range_data_ntf.child {
            RangeDataNtfDataChild::ShortMacTwoWayRangeDataNtf(_) => {
                RangeDataNtfChild::ShortMacTwoWayRangeDataNtf(
                    ShortMacTwoWayRangeDataNtfPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            RangeDataNtfDataChild::ExtendedMacTwoWayRangeDataNtf(_) => {
                RangeDataNtfChild::ExtendedMacTwoWayRangeDataNtf(
                    ExtendedMacTwoWayRangeDataNtfPacket::new(self.uci_packet.clone()).unwrap(),
                )
            }
            RangeDataNtfDataChild::None => RangeDataNtfChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let ranging_notification = match &uci_notification.child {
            UciNotificationDataChild::RangingNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingNotification"),
        };
        let range_data_ntf = match &ranging_notification.child {
            RangingNotificationDataChild::RangeDataNtf(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeDataNtf"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            ranging_notification,
            range_data_ntf,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.range_data_ntf.as_ref().sequence_number
    }
    pub fn get_session_id(&self) -> u32 {
        self.range_data_ntf.as_ref().session_id
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.range_data_ntf.as_ref().rcr_indicator
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.range_data_ntf.as_ref().current_ranging_interval
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.range_data_ntf.as_ref().ranging_measurement_type
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.range_data_ntf.as_ref().mac_address_indicator
    }
}
impl Into<UciPacketPacket> for RangeDataNtfPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for RangeDataNtfPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingNotificationPacket> for RangeDataNtfPacket {
    fn into(self) -> RangingNotificationPacket {
        RangingNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl RangeDataNtfBuilder {
    pub fn build(self) -> RangeDataNtfPacket {
        let range_data_ntf = Arc::new(RangeDataNtfData {
            sequence_number: self.sequence_number,
            session_id: self.session_id,
            rcr_indicator: self.rcr_indicator,
            current_ranging_interval: self.current_ranging_interval,
            ranging_measurement_type: self.ranging_measurement_type,
            mac_address_indicator: self.mac_address_indicator,
            child: RangeDataNtfDataChild::None,
        });
        let ranging_notification = Arc::new(RangingNotificationData {
            child: RangingNotificationDataChild::RangeDataNtf(range_data_ntf),
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::RangingNotification(ranging_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: 0,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        RangeDataNtfPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangeDataNtfBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for RangeDataNtfBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}
impl Into<RangingNotificationPacket> for RangeDataNtfBuilder {
    fn into(self) -> RangingNotificationPacket {
        self.build().into()
    }
}

#[derive(Debug)]
struct ShortMacTwoWayRangeDataNtfData {
    two_way_ranging_measurements: Vec<ShortAddressTwoWayRangingMeasurement>,
}
#[derive(Debug, Clone)]
pub struct ShortMacTwoWayRangeDataNtfPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    ranging_notification: Arc<RangingNotificationData>,
    range_data_ntf: Arc<RangeDataNtfData>,
    short_mac_two_way_range_data_ntf: Arc<ShortMacTwoWayRangeDataNtfData>,
}
#[derive(Debug)]
pub struct ShortMacTwoWayRangeDataNtfBuilder {
    pub sequence_number: u32,
    pub session_id: u32,
    pub rcr_indicator: u8,
    pub current_ranging_interval: u32,
    pub two_way_ranging_measurements: Vec<ShortAddressTwoWayRangingMeasurement>,
}
impl ShortMacTwoWayRangeDataNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 29 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 29 {
            return Err(Error::InvalidLengthError {
                obj: "ShortMacTwoWayRangeDataNtf".to_string(),
                field: "two_way_ranging_measurements_count".to_string(),
                wanted: 29,
                got: bytes.len(),
            });
        }
        let two_way_ranging_measurements_count = u8::from_le_bytes([bytes[28]]);
        let want_ = 29 + ((two_way_ranging_measurements_count as usize) * 31);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "ShortMacTwoWayRangeDataNtf".to_string(),
                field: "two_way_ranging_measurements".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let mut two_way_ranging_measurements: Vec<ShortAddressTwoWayRangingMeasurement> =
            Vec::new();
        let mut parsable_ = &bytes[29..];
        let count_ = two_way_ranging_measurements_count as usize;
        for _ in 0..count_ {
            match ShortAddressTwoWayRangingMeasurement::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    two_way_ranging_measurements.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self {
            two_way_ranging_measurements,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer[28..29]
            .copy_from_slice(&(self.two_way_ranging_measurements.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[29..];
        for e_ in &self.two_way_ranging_measurements {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        let ret = ret
            + (self.two_way_ranging_measurements.len() * ((/* Bits: */248 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for ShortMacTwoWayRangeDataNtfPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<ShortMacTwoWayRangeDataNtfPacket> for Bytes {
    fn from(packet: ShortMacTwoWayRangeDataNtfPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<ShortMacTwoWayRangeDataNtfPacket> for Vec<u8> {
    fn from(packet: ShortMacTwoWayRangeDataNtfPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for ShortMacTwoWayRangeDataNtfPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl ShortMacTwoWayRangeDataNtfPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let ranging_notification = match &uci_notification.child {
            UciNotificationDataChild::RangingNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingNotification"),
        };
        let range_data_ntf = match &ranging_notification.child {
            RangingNotificationDataChild::RangeDataNtf(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeDataNtf"),
        };
        let short_mac_two_way_range_data_ntf = match &range_data_ntf.child {
            RangeDataNtfDataChild::ShortMacTwoWayRangeDataNtf(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not ShortMacTwoWayRangeDataNtf"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            ranging_notification,
            range_data_ntf,
            short_mac_two_way_range_data_ntf,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.range_data_ntf.as_ref().sequence_number
    }
    pub fn get_session_id(&self) -> u32 {
        self.range_data_ntf.as_ref().session_id
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.range_data_ntf.as_ref().rcr_indicator
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.range_data_ntf.as_ref().current_ranging_interval
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.range_data_ntf.as_ref().ranging_measurement_type
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.range_data_ntf.as_ref().mac_address_indicator
    }
    pub fn get_two_way_ranging_measurements(&self) -> &Vec<ShortAddressTwoWayRangingMeasurement> {
        &self
            .short_mac_two_way_range_data_ntf
            .as_ref()
            .two_way_ranging_measurements
    }
}
impl Into<UciPacketPacket> for ShortMacTwoWayRangeDataNtfPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for ShortMacTwoWayRangeDataNtfPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingNotificationPacket> for ShortMacTwoWayRangeDataNtfPacket {
    fn into(self) -> RangingNotificationPacket {
        RangingNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangeDataNtfPacket> for ShortMacTwoWayRangeDataNtfPacket {
    fn into(self) -> RangeDataNtfPacket {
        RangeDataNtfPacket::new(self.uci_packet).unwrap()
    }
}
impl ShortMacTwoWayRangeDataNtfBuilder {
    pub fn build(self) -> ShortMacTwoWayRangeDataNtfPacket {
        let short_mac_two_way_range_data_ntf = Arc::new(ShortMacTwoWayRangeDataNtfData {
            two_way_ranging_measurements: self.two_way_ranging_measurements,
        });
        let range_data_ntf = Arc::new(RangeDataNtfData {
            sequence_number: self.sequence_number,
            session_id: self.session_id,
            rcr_indicator: self.rcr_indicator,
            current_ranging_interval: self.current_ranging_interval,
            ranging_measurement_type: RangingMeasurementType::TwoWay,
            mac_address_indicator: MacAddressIndicator::ShortAddress,
            child: RangeDataNtfDataChild::ShortMacTwoWayRangeDataNtf(
                short_mac_two_way_range_data_ntf,
            ),
        });
        let ranging_notification = Arc::new(RangingNotificationData {
            child: RangingNotificationDataChild::RangeDataNtf(range_data_ntf),
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::RangingNotification(ranging_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: 0,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        ShortMacTwoWayRangeDataNtfPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for ShortMacTwoWayRangeDataNtfBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for ShortMacTwoWayRangeDataNtfBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}
impl Into<RangingNotificationPacket> for ShortMacTwoWayRangeDataNtfBuilder {
    fn into(self) -> RangingNotificationPacket {
        self.build().into()
    }
}
impl Into<RangeDataNtfPacket> for ShortMacTwoWayRangeDataNtfBuilder {
    fn into(self) -> RangeDataNtfPacket {
        self.build().into()
    }
}
macro_rules! short_mac_two_way_range_data_ntf_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciNotification(uci_notification_packet) => {match uci_notification_packet.specialize() {/* (2) */
UciNotificationChild::RangingNotification(ranging_notification_packet) => {match ranging_notification_packet.specialize() {/* (3) */
RangingNotificationChild::RangeDataNtf(range_data_ntf_packet) => {match range_data_ntf_packet.specialize() {/* (4) */
RangeDataNtfChild::ShortMacTwoWayRangeDataNtf(packet) => {let rebuilder = ShortMacTwoWayRangeDataNtfBuilder {sequence_number : packet.get_sequence_number(),session_id : packet.get_session_id(),rcr_indicator : packet.get_rcr_indicator(),current_ranging_interval : packet.get_current_ranging_interval(),two_way_ranging_measurements : packet.get_two_way_ranging_measurements().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse short_mac_two_way_range_data_ntf
 {:#02x?}", range_data_ntf_packet); }}}_ => {panic!("Couldn't parse range_data_ntf
 {:#02x?}", ranging_notification_packet); }}}_ => {panic!("Couldn't parse ranging_notification
 {:#02x?}", uci_notification_packet); }}}_ => {panic!("Couldn't parse uci_notification
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
short_mac_two_way_range_data_ntf_builder_tests! { short_mac_two_way_range_data_ntf_builder_test_00: b"\x62\x00\x00\x19\x00\x02\x03\x04\x05\x06\x07\x08\x00\x0a\x01\x01\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",}

#[derive(Debug)]
struct ExtendedMacTwoWayRangeDataNtfData {
    two_way_ranging_measurements: Vec<ExtendedAddressTwoWayRangingMeasurement>,
}
#[derive(Debug, Clone)]
pub struct ExtendedMacTwoWayRangeDataNtfPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    ranging_notification: Arc<RangingNotificationData>,
    range_data_ntf: Arc<RangeDataNtfData>,
    extended_mac_two_way_range_data_ntf: Arc<ExtendedMacTwoWayRangeDataNtfData>,
}
#[derive(Debug)]
pub struct ExtendedMacTwoWayRangeDataNtfBuilder {
    pub sequence_number: u32,
    pub session_id: u32,
    pub rcr_indicator: u8,
    pub current_ranging_interval: u32,
    pub two_way_ranging_measurements: Vec<ExtendedAddressTwoWayRangingMeasurement>,
}
impl ExtendedMacTwoWayRangeDataNtfData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 29 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 29 {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedMacTwoWayRangeDataNtf".to_string(),
                field: "two_way_ranging_measurements_count".to_string(),
                wanted: 29,
                got: bytes.len(),
            });
        }
        let two_way_ranging_measurements_count = u8::from_le_bytes([bytes[28]]);
        let want_ = 29 + ((two_way_ranging_measurements_count as usize) * 31);
        if bytes.len() < want_ {
            return Err(Error::InvalidLengthError {
                obj: "ExtendedMacTwoWayRangeDataNtf".to_string(),
                field: "two_way_ranging_measurements".to_string(),
                wanted: want_,
                got: bytes.len(),
            });
        }
        let mut two_way_ranging_measurements: Vec<ExtendedAddressTwoWayRangingMeasurement> =
            Vec::new();
        let mut parsable_ = &bytes[29..];
        let count_ = two_way_ranging_measurements_count as usize;
        for _ in 0..count_ {
            match ExtendedAddressTwoWayRangingMeasurement::parse(&parsable_) {
                Ok(parsed) => {
                    parsable_ = &parsable_[parsed.get_total_size()..];
                    two_way_ranging_measurements.push(parsed);
                }
                Err(Error::ImpossibleStructError) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Self {
            two_way_ranging_measurements,
        })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer[28..29]
            .copy_from_slice(&(self.two_way_ranging_measurements.len() as u8).to_le_bytes());
        let mut vec_buffer_ = &mut buffer[29..];
        for e_ in &self.two_way_ranging_measurements {
            e_.write_to(&mut vec_buffer_[0..e_.get_total_size()]);
            vec_buffer_ = &mut vec_buffer_[e_.get_total_size()..];
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        let ret = ret
            + (self.two_way_ranging_measurements.len() * ((/* Bits: */248 + /* Dynamic: */ 0) / 8));
        ret
    }
}
impl Packet for ExtendedMacTwoWayRangeDataNtfPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<ExtendedMacTwoWayRangeDataNtfPacket> for Bytes {
    fn from(packet: ExtendedMacTwoWayRangeDataNtfPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<ExtendedMacTwoWayRangeDataNtfPacket> for Vec<u8> {
    fn from(packet: ExtendedMacTwoWayRangeDataNtfPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for ExtendedMacTwoWayRangeDataNtfPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl ExtendedMacTwoWayRangeDataNtfPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let ranging_notification = match &uci_notification.child {
            UciNotificationDataChild::RangingNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingNotification"),
        };
        let range_data_ntf = match &ranging_notification.child {
            RangingNotificationDataChild::RangeDataNtf(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeDataNtf"),
        };
        let extended_mac_two_way_range_data_ntf = match &range_data_ntf.child {
            RangeDataNtfDataChild::ExtendedMacTwoWayRangeDataNtf(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not ExtendedMacTwoWayRangeDataNtf"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            ranging_notification,
            range_data_ntf,
            extended_mac_two_way_range_data_ntf,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_sequence_number(&self) -> u32 {
        self.range_data_ntf.as_ref().sequence_number
    }
    pub fn get_session_id(&self) -> u32 {
        self.range_data_ntf.as_ref().session_id
    }
    pub fn get_rcr_indicator(&self) -> u8 {
        self.range_data_ntf.as_ref().rcr_indicator
    }
    pub fn get_current_ranging_interval(&self) -> u32 {
        self.range_data_ntf.as_ref().current_ranging_interval
    }
    pub fn get_ranging_measurement_type(&self) -> RangingMeasurementType {
        self.range_data_ntf.as_ref().ranging_measurement_type
    }
    pub fn get_mac_address_indicator(&self) -> MacAddressIndicator {
        self.range_data_ntf.as_ref().mac_address_indicator
    }
    pub fn get_two_way_ranging_measurements(
        &self,
    ) -> &Vec<ExtendedAddressTwoWayRangingMeasurement> {
        &self
            .extended_mac_two_way_range_data_ntf
            .as_ref()
            .two_way_ranging_measurements
    }
}
impl Into<UciPacketPacket> for ExtendedMacTwoWayRangeDataNtfPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for ExtendedMacTwoWayRangeDataNtfPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingNotificationPacket> for ExtendedMacTwoWayRangeDataNtfPacket {
    fn into(self) -> RangingNotificationPacket {
        RangingNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangeDataNtfPacket> for ExtendedMacTwoWayRangeDataNtfPacket {
    fn into(self) -> RangeDataNtfPacket {
        RangeDataNtfPacket::new(self.uci_packet).unwrap()
    }
}
impl ExtendedMacTwoWayRangeDataNtfBuilder {
    pub fn build(self) -> ExtendedMacTwoWayRangeDataNtfPacket {
        let extended_mac_two_way_range_data_ntf = Arc::new(ExtendedMacTwoWayRangeDataNtfData {
            two_way_ranging_measurements: self.two_way_ranging_measurements,
        });
        let range_data_ntf = Arc::new(RangeDataNtfData {
            sequence_number: self.sequence_number,
            session_id: self.session_id,
            rcr_indicator: self.rcr_indicator,
            current_ranging_interval: self.current_ranging_interval,
            ranging_measurement_type: RangingMeasurementType::TwoWay,
            mac_address_indicator: MacAddressIndicator::ExtendedAddress,
            child: RangeDataNtfDataChild::ExtendedMacTwoWayRangeDataNtf(
                extended_mac_two_way_range_data_ntf,
            ),
        });
        let ranging_notification = Arc::new(RangingNotificationData {
            child: RangingNotificationDataChild::RangeDataNtf(range_data_ntf),
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::RangingNotification(ranging_notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: 0,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        ExtendedMacTwoWayRangeDataNtfPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for ExtendedMacTwoWayRangeDataNtfBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for ExtendedMacTwoWayRangeDataNtfBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}
impl Into<RangingNotificationPacket> for ExtendedMacTwoWayRangeDataNtfBuilder {
    fn into(self) -> RangingNotificationPacket {
        self.build().into()
    }
}
impl Into<RangeDataNtfPacket> for ExtendedMacTwoWayRangeDataNtfBuilder {
    fn into(self) -> RangeDataNtfPacket {
        self.build().into()
    }
}
macro_rules! extended_mac_two_way_range_data_ntf_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciNotification(uci_notification_packet) => {match uci_notification_packet.specialize() {/* (2) */
UciNotificationChild::RangingNotification(ranging_notification_packet) => {match ranging_notification_packet.specialize() {/* (3) */
RangingNotificationChild::RangeDataNtf(range_data_ntf_packet) => {match range_data_ntf_packet.specialize() {/* (4) */
RangeDataNtfChild::ExtendedMacTwoWayRangeDataNtf(packet) => {let rebuilder = ExtendedMacTwoWayRangeDataNtfBuilder {sequence_number : packet.get_sequence_number(),session_id : packet.get_session_id(),rcr_indicator : packet.get_rcr_indicator(),current_ranging_interval : packet.get_current_ranging_interval(),two_way_ranging_measurements : packet.get_two_way_ranging_measurements().to_vec(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse extended_mac_two_way_range_data_ntf
 {:#02x?}", range_data_ntf_packet); }}}_ => {panic!("Couldn't parse range_data_ntf
 {:#02x?}", ranging_notification_packet); }}}_ => {panic!("Couldn't parse ranging_notification
 {:#02x?}", uci_notification_packet); }}}_ => {panic!("Couldn't parse uci_notification
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
extended_mac_two_way_range_data_ntf_builder_tests! { extended_mac_two_way_range_data_ntf_builder_test_00: b"\x62\x00\x00\x19\x00\x02\x03\x04\x05\x06\x07\x08\x00\x0a\x01\x01\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",}

#[derive(Debug)]
struct RangeStopCmdData {}
#[derive(Debug, Clone)]
pub struct RangeStopCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    ranging_command: Arc<RangingCommandData>,
    range_stop_cmd: Arc<RangeStopCmdData>,
}
#[derive(Debug)]
pub struct RangeStopCmdBuilder {
    pub session_id: u32,
}
impl RangeStopCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for RangeStopCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangeStopCmdPacket> for Bytes {
    fn from(packet: RangeStopCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangeStopCmdPacket> for Vec<u8> {
    fn from(packet: RangeStopCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangeStopCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangeStopCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let ranging_command = match &uci_command.child {
            UciCommandDataChild::RangingCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingCommand"),
        };
        let range_stop_cmd = match &ranging_command.child {
            RangingCommandDataChild::RangeStopCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeStopCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            ranging_command,
            range_stop_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.ranging_command.as_ref().session_id
    }
}
impl Into<UciPacketPacket> for RangeStopCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for RangeStopCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingCommandPacket> for RangeStopCmdPacket {
    fn into(self) -> RangingCommandPacket {
        RangingCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl RangeStopCmdBuilder {
    pub fn build(self) -> RangeStopCmdPacket {
        let range_stop_cmd = Arc::new(RangeStopCmdData {});
        let ranging_command = Arc::new(RangingCommandData {
            session_id: self.session_id,
            child: RangingCommandDataChild::RangeStopCmd(range_stop_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::RangingCommand(ranging_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 1,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        RangeStopCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangeStopCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for RangeStopCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<RangingCommandPacket> for RangeStopCmdBuilder {
    fn into(self) -> RangingCommandPacket {
        self.build().into()
    }
}
macro_rules! range_stop_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::RangingCommand(ranging_command_packet) => {match ranging_command_packet.specialize() {/* (3) */
RangingCommandChild::RangeStopCmd(packet) => {let rebuilder = RangeStopCmdBuilder {session_id : packet.get_session_id(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse range_stop_cmd
 {:#02x?}", ranging_command_packet); }}}_ => {panic!("Couldn't parse ranging_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
range_stop_cmd_builder_tests! { range_stop_cmd_builder_test_00: b"\x22\x01\x00\x04\x00\x02\x03\x04",}

#[derive(Debug)]
struct RangeStopRspData {
    status: StatusCode,
}
#[derive(Debug, Clone)]
pub struct RangeStopRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    ranging_response: Arc<RangingResponseData>,
    range_stop_rsp: Arc<RangeStopRspData>,
}
#[derive(Debug)]
pub struct RangeStopRspBuilder {
    pub status: StatusCode,
}
impl RangeStopRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "RangeStopRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "RangeStopRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for RangeStopRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangeStopRspPacket> for Bytes {
    fn from(packet: RangeStopRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangeStopRspPacket> for Vec<u8> {
    fn from(packet: RangeStopRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangeStopRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangeStopRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let ranging_response = match &uci_response.child {
            UciResponseDataChild::RangingResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingResponse"),
        };
        let range_stop_rsp = match &ranging_response.child {
            RangingResponseDataChild::RangeStopRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeStopRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            ranging_response,
            range_stop_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.range_stop_rsp.as_ref().status
    }
}
impl Into<UciPacketPacket> for RangeStopRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for RangeStopRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingResponsePacket> for RangeStopRspPacket {
    fn into(self) -> RangingResponsePacket {
        RangingResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl RangeStopRspBuilder {
    pub fn build(self) -> RangeStopRspPacket {
        let range_stop_rsp = Arc::new(RangeStopRspData {
            status: self.status,
        });
        let ranging_response = Arc::new(RangingResponseData {
            child: RangingResponseDataChild::RangeStopRsp(range_stop_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::RangingResponse(ranging_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 1,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        RangeStopRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangeStopRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for RangeStopRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<RangingResponsePacket> for RangeStopRspBuilder {
    fn into(self) -> RangingResponsePacket {
        self.build().into()
    }
}
macro_rules! range_stop_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::RangingResponse(ranging_response_packet) => {match ranging_response_packet.specialize() {/* (3) */
RangingResponseChild::RangeStopRsp(packet) => {let rebuilder = RangeStopRspBuilder {status : packet.get_status(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse range_stop_rsp
 {:#02x?}", ranging_response_packet); }}}_ => {panic!("Couldn't parse ranging_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
range_stop_rsp_builder_tests! { range_stop_rsp_builder_test_00: b"\x42\x01\x00\x01\x00",}

#[derive(Debug)]
struct RangeGetRangingCountCmdData {}
#[derive(Debug, Clone)]
pub struct RangeGetRangingCountCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    ranging_command: Arc<RangingCommandData>,
    range_get_ranging_count_cmd: Arc<RangeGetRangingCountCmdData>,
}
#[derive(Debug)]
pub struct RangeGetRangingCountCmdBuilder {
    pub session_id: u32,
}
impl RangeGetRangingCountCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for RangeGetRangingCountCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangeGetRangingCountCmdPacket> for Bytes {
    fn from(packet: RangeGetRangingCountCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangeGetRangingCountCmdPacket> for Vec<u8> {
    fn from(packet: RangeGetRangingCountCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangeGetRangingCountCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangeGetRangingCountCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let ranging_command = match &uci_command.child {
            UciCommandDataChild::RangingCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingCommand"),
        };
        let range_get_ranging_count_cmd = match &ranging_command.child {
            RangingCommandDataChild::RangeGetRangingCountCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeGetRangingCountCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            ranging_command,
            range_get_ranging_count_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_session_id(&self) -> u32 {
        self.ranging_command.as_ref().session_id
    }
}
impl Into<UciPacketPacket> for RangeGetRangingCountCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for RangeGetRangingCountCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingCommandPacket> for RangeGetRangingCountCmdPacket {
    fn into(self) -> RangingCommandPacket {
        RangingCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl RangeGetRangingCountCmdBuilder {
    pub fn build(self) -> RangeGetRangingCountCmdPacket {
        let range_get_ranging_count_cmd = Arc::new(RangeGetRangingCountCmdData {});
        let ranging_command = Arc::new(RangingCommandData {
            session_id: self.session_id,
            child: RangingCommandDataChild::RangeGetRangingCountCmd(range_get_ranging_count_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::RangingCommand(ranging_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 3,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        RangeGetRangingCountCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangeGetRangingCountCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for RangeGetRangingCountCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<RangingCommandPacket> for RangeGetRangingCountCmdBuilder {
    fn into(self) -> RangingCommandPacket {
        self.build().into()
    }
}
macro_rules! range_get_ranging_count_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::RangingCommand(ranging_command_packet) => {match ranging_command_packet.specialize() {/* (3) */
RangingCommandChild::RangeGetRangingCountCmd(packet) => {let rebuilder = RangeGetRangingCountCmdBuilder {session_id : packet.get_session_id(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse range_get_ranging_count_cmd
 {:#02x?}", ranging_command_packet); }}}_ => {panic!("Couldn't parse ranging_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
range_get_ranging_count_cmd_builder_tests! { range_get_ranging_count_cmd_builder_test_00: b"\x22\x03\x00\x04\x00\x02\x03\x04",}

#[derive(Debug)]
struct RangeGetRangingCountRspData {
    status: StatusCode,
    count: u32,
}
#[derive(Debug, Clone)]
pub struct RangeGetRangingCountRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    ranging_response: Arc<RangingResponseData>,
    range_get_ranging_count_rsp: Arc<RangeGetRangingCountRspData>,
}
#[derive(Debug)]
pub struct RangeGetRangingCountRspBuilder {
    pub status: StatusCode,
    pub count: u32,
}
impl RangeGetRangingCountRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 9 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "RangeGetRangingCountRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "RangeGetRangingCountRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        if bytes.len() < 9 {
            return Err(Error::InvalidLengthError {
                obj: "RangeGetRangingCountRsp".to_string(),
                field: "count".to_string(),
                wanted: 9,
                got: bytes.len(),
            });
        }
        let count = u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
        Ok(Self { status, count })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
        let count = self.count;
        buffer[5..9].copy_from_slice(&count.to_le_bytes()[0..4]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 5;
        ret
    }
}
impl Packet for RangeGetRangingCountRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<RangeGetRangingCountRspPacket> for Bytes {
    fn from(packet: RangeGetRangingCountRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<RangeGetRangingCountRspPacket> for Vec<u8> {
    fn from(packet: RangeGetRangingCountRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for RangeGetRangingCountRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl RangeGetRangingCountRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let ranging_response = match &uci_response.child {
            UciResponseDataChild::RangingResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangingResponse"),
        };
        let range_get_ranging_count_rsp = match &ranging_response.child {
            RangingResponseDataChild::RangeGetRangingCountRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not RangeGetRangingCountRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            ranging_response,
            range_get_ranging_count_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.range_get_ranging_count_rsp.as_ref().status
    }
    pub fn get_count(&self) -> u32 {
        self.range_get_ranging_count_rsp.as_ref().count
    }
}
impl Into<UciPacketPacket> for RangeGetRangingCountRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for RangeGetRangingCountRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<RangingResponsePacket> for RangeGetRangingCountRspPacket {
    fn into(self) -> RangingResponsePacket {
        RangingResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl RangeGetRangingCountRspBuilder {
    pub fn build(self) -> RangeGetRangingCountRspPacket {
        let range_get_ranging_count_rsp = Arc::new(RangeGetRangingCountRspData {
            status: self.status,
            count: self.count,
        });
        let ranging_response = Arc::new(RangingResponseData {
            child: RangingResponseDataChild::RangeGetRangingCountRsp(range_get_ranging_count_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::RangingResponse(ranging_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::RangingSessionControl,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 3,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        RangeGetRangingCountRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for RangeGetRangingCountRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for RangeGetRangingCountRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<RangingResponsePacket> for RangeGetRangingCountRspBuilder {
    fn into(self) -> RangingResponsePacket {
        self.build().into()
    }
}
macro_rules! range_get_ranging_count_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::RangingResponse(ranging_response_packet) => {match ranging_response_packet.specialize() {/* (3) */
RangingResponseChild::RangeGetRangingCountRsp(packet) => {let rebuilder = RangeGetRangingCountRspBuilder {status : packet.get_status(),count : packet.get_count(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse range_get_ranging_count_rsp
 {:#02x?}", ranging_response_packet); }}}_ => {panic!("Couldn't parse ranging_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
range_get_ranging_count_rsp_builder_tests! { range_get_ranging_count_rsp_builder_test_00: b"\x42\x03\x00\x05\x00\x02\x03\x04\x05",}

#[derive(Debug)]
struct AndroidGetPowerStatsCmdData {}
#[derive(Debug, Clone)]
pub struct AndroidGetPowerStatsCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    android_command: Arc<AndroidCommandData>,
    android_get_power_stats_cmd: Arc<AndroidGetPowerStatsCmdData>,
}
#[derive(Debug)]
pub struct AndroidGetPowerStatsCmdBuilder {}
impl AndroidGetPowerStatsCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for AndroidGetPowerStatsCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidGetPowerStatsCmdPacket> for Bytes {
    fn from(packet: AndroidGetPowerStatsCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidGetPowerStatsCmdPacket> for Vec<u8> {
    fn from(packet: AndroidGetPowerStatsCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for AndroidGetPowerStatsCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl AndroidGetPowerStatsCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let android_command = match &uci_command.child {
            UciCommandDataChild::AndroidCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidCommand"),
        };
        let android_get_power_stats_cmd = match &android_command.child {
            AndroidCommandDataChild::AndroidGetPowerStatsCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidGetPowerStatsCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            android_command,
            android_get_power_stats_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for AndroidGetPowerStatsCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for AndroidGetPowerStatsCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<AndroidCommandPacket> for AndroidGetPowerStatsCmdPacket {
    fn into(self) -> AndroidCommandPacket {
        AndroidCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl AndroidGetPowerStatsCmdBuilder {
    pub fn build(self) -> AndroidGetPowerStatsCmdPacket {
        let android_get_power_stats_cmd = Arc::new(AndroidGetPowerStatsCmdData {});
        let android_command = Arc::new(AndroidCommandData {
            child: AndroidCommandDataChild::AndroidGetPowerStatsCmd(android_get_power_stats_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::AndroidCommand(android_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorAndroid,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 0,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        AndroidGetPowerStatsCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for AndroidGetPowerStatsCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for AndroidGetPowerStatsCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<AndroidCommandPacket> for AndroidGetPowerStatsCmdBuilder {
    fn into(self) -> AndroidCommandPacket {
        self.build().into()
    }
}
macro_rules! android_get_power_stats_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::AndroidCommand(android_command_packet) => {match android_command_packet.specialize() {/* (3) */
AndroidCommandChild::AndroidGetPowerStatsCmd(packet) => {let rebuilder = AndroidGetPowerStatsCmdBuilder {};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse android_get_power_stats_cmd
 {:#02x?}", android_command_packet); }}}_ => {panic!("Couldn't parse android_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
android_get_power_stats_cmd_builder_tests! { android_get_power_stats_cmd_builder_test_00: b"\x2e\x00\x00\x00",}

#[derive(Debug)]
struct AndroidGetPowerStatsRspData {
    stats: PowerStats,
}
#[derive(Debug, Clone)]
pub struct AndroidGetPowerStatsRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    android_response: Arc<AndroidResponseData>,
    android_get_power_stats_rsp: Arc<AndroidGetPowerStatsRspData>,
}
#[derive(Debug)]
pub struct AndroidGetPowerStatsRspBuilder {
    pub stats: PowerStats,
}
impl AndroidGetPowerStatsRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 21 {
            return false;
        }
        if !PowerStats::conforms(&bytes[4..21]) {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let stats = PowerStats::parse(&bytes[4..21])?;
        Ok(Self { stats })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let stats = &mut buffer[4..21];
        self.stats.write_to(stats);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 17;
        ret
    }
}
impl Packet for AndroidGetPowerStatsRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidGetPowerStatsRspPacket> for Bytes {
    fn from(packet: AndroidGetPowerStatsRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidGetPowerStatsRspPacket> for Vec<u8> {
    fn from(packet: AndroidGetPowerStatsRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for AndroidGetPowerStatsRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl AndroidGetPowerStatsRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let android_response = match &uci_response.child {
            UciResponseDataChild::AndroidResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidResponse"),
        };
        let android_get_power_stats_rsp = match &android_response.child {
            AndroidResponseDataChild::AndroidGetPowerStatsRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidGetPowerStatsRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            android_response,
            android_get_power_stats_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_stats(&self) -> &PowerStats {
        &self.android_get_power_stats_rsp.as_ref().stats
    }
}
impl Into<UciPacketPacket> for AndroidGetPowerStatsRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for AndroidGetPowerStatsRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<AndroidResponsePacket> for AndroidGetPowerStatsRspPacket {
    fn into(self) -> AndroidResponsePacket {
        AndroidResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl AndroidGetPowerStatsRspBuilder {
    pub fn build(self) -> AndroidGetPowerStatsRspPacket {
        let android_get_power_stats_rsp =
            Arc::new(AndroidGetPowerStatsRspData { stats: self.stats });
        let android_response = Arc::new(AndroidResponseData {
            child: AndroidResponseDataChild::AndroidGetPowerStatsRsp(android_get_power_stats_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::AndroidResponse(android_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorAndroid,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 0,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        AndroidGetPowerStatsRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for AndroidGetPowerStatsRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for AndroidGetPowerStatsRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<AndroidResponsePacket> for AndroidGetPowerStatsRspBuilder {
    fn into(self) -> AndroidResponsePacket {
        self.build().into()
    }
}
macro_rules! android_get_power_stats_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::AndroidResponse(android_response_packet) => {match android_response_packet.specialize() {/* (3) */
AndroidResponseChild::AndroidGetPowerStatsRsp(packet) => {let rebuilder = AndroidGetPowerStatsRspBuilder {stats : packet.get_stats().clone(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse android_get_power_stats_rsp
 {:#02x?}", android_response_packet); }}}_ => {panic!("Couldn't parse android_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
android_get_power_stats_rsp_builder_tests! { android_get_power_stats_rsp_builder_test_00: b"\x4e\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",}

#[derive(Debug)]
struct AndroidSetCountryCodeCmdData {
    country_code: [u8; 2],
}
#[derive(Debug, Clone)]
pub struct AndroidSetCountryCodeCmdPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    android_command: Arc<AndroidCommandData>,
    android_set_country_code_cmd: Arc<AndroidSetCountryCodeCmdData>,
}
#[derive(Debug)]
pub struct AndroidSetCountryCodeCmdBuilder {
    pub country_code: [u8; 2],
}
impl AndroidSetCountryCodeCmdData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "AndroidSetCountryCodeCmd".to_string(),
                field: "country_code".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let country_code = bytes[4..6]
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        Ok(Self { country_code })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        &buffer[4..6].copy_from_slice(&self.country_code);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 2;
        ret
    }
}
impl Packet for AndroidSetCountryCodeCmdPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidSetCountryCodeCmdPacket> for Bytes {
    fn from(packet: AndroidSetCountryCodeCmdPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidSetCountryCodeCmdPacket> for Vec<u8> {
    fn from(packet: AndroidSetCountryCodeCmdPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for AndroidSetCountryCodeCmdPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl AndroidSetCountryCodeCmdPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let android_command = match &uci_command.child {
            UciCommandDataChild::AndroidCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidCommand"),
        };
        let android_set_country_code_cmd = match &android_command.child {
            AndroidCommandDataChild::AndroidSetCountryCodeCmd(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidSetCountryCodeCmd"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            android_command,
            android_set_country_code_cmd,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_country_code(&self) -> &[u8; 2] {
        &self.android_set_country_code_cmd.as_ref().country_code
    }
}
impl Into<UciPacketPacket> for AndroidSetCountryCodeCmdPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for AndroidSetCountryCodeCmdPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<AndroidCommandPacket> for AndroidSetCountryCodeCmdPacket {
    fn into(self) -> AndroidCommandPacket {
        AndroidCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl AndroidSetCountryCodeCmdBuilder {
    pub fn build(self) -> AndroidSetCountryCodeCmdPacket {
        let android_set_country_code_cmd = Arc::new(AndroidSetCountryCodeCmdData {
            country_code: self.country_code,
        });
        let android_command = Arc::new(AndroidCommandData {
            child: AndroidCommandDataChild::AndroidSetCountryCodeCmd(android_set_country_code_cmd),
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::AndroidCommand(android_command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorAndroid,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: 1,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        AndroidSetCountryCodeCmdPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for AndroidSetCountryCodeCmdBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for AndroidSetCountryCodeCmdBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}
impl Into<AndroidCommandPacket> for AndroidSetCountryCodeCmdBuilder {
    fn into(self) -> AndroidCommandPacket {
        self.build().into()
    }
}
macro_rules! android_set_country_code_cmd_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciCommand(uci_command_packet) => {match uci_command_packet.specialize() {/* (2) */
UciCommandChild::AndroidCommand(android_command_packet) => {match android_command_packet.specialize() {/* (3) */
AndroidCommandChild::AndroidSetCountryCodeCmd(packet) => {let rebuilder = AndroidSetCountryCodeCmdBuilder {country_code : packet.get_country_code().clone(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse android_set_country_code_cmd
 {:#02x?}", android_command_packet); }}}_ => {panic!("Couldn't parse android_command
 {:#02x?}", uci_command_packet); }}}_ => {panic!("Couldn't parse uci_command
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
android_set_country_code_cmd_builder_tests! { android_set_country_code_cmd_builder_test_00: b"\x2e\x01\x00\x02\x55\x53",}

#[derive(Debug)]
struct AndroidSetCountryCodeRspData {
    status: StatusCode,
}
#[derive(Debug, Clone)]
pub struct AndroidSetCountryCodeRspPacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    android_response: Arc<AndroidResponseData>,
    android_set_country_code_rsp: Arc<AndroidSetCountryCodeRspData>,
}
#[derive(Debug)]
pub struct AndroidSetCountryCodeRspBuilder {
    pub status: StatusCode,
}
impl AndroidSetCountryCodeRspData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "AndroidSetCountryCodeRsp".to_string(),
                field: "status".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let status = u8::from_le_bytes([bytes[4]]);
        let status = StatusCode::from_u8(status).ok_or_else(|| Error::InvalidEnumValueError {
            obj: "AndroidSetCountryCodeRsp".to_string(),
            field: "status".to_string(),
            value: status as u64,
            type_: "StatusCode".to_string(),
        })?;
        Ok(Self { status })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let status = self.status.to_u8().unwrap();
        buffer[4..5].copy_from_slice(&status.to_le_bytes()[0..1]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 1;
        ret
    }
}
impl Packet for AndroidSetCountryCodeRspPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AndroidSetCountryCodeRspPacket> for Bytes {
    fn from(packet: AndroidSetCountryCodeRspPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<AndroidSetCountryCodeRspPacket> for Vec<u8> {
    fn from(packet: AndroidSetCountryCodeRspPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for AndroidSetCountryCodeRspPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl AndroidSetCountryCodeRspPacket {
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let android_response = match &uci_response.child {
            UciResponseDataChild::AndroidResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidResponse"),
        };
        let android_set_country_code_rsp = match &android_response.child {
            AndroidResponseDataChild::AndroidSetCountryCodeRsp(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not AndroidSetCountryCodeRsp"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            android_response,
            android_set_country_code_rsp,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
    pub fn get_status(&self) -> StatusCode {
        self.android_set_country_code_rsp.as_ref().status
    }
}
impl Into<UciPacketPacket> for AndroidSetCountryCodeRspPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for AndroidSetCountryCodeRspPacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl Into<AndroidResponsePacket> for AndroidSetCountryCodeRspPacket {
    fn into(self) -> AndroidResponsePacket {
        AndroidResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl AndroidSetCountryCodeRspBuilder {
    pub fn build(self) -> AndroidSetCountryCodeRspPacket {
        let android_set_country_code_rsp = Arc::new(AndroidSetCountryCodeRspData {
            status: self.status,
        });
        let android_response = Arc::new(AndroidResponseData {
            child: AndroidResponseDataChild::AndroidSetCountryCodeRsp(android_set_country_code_rsp),
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::AndroidResponse(android_response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorAndroid,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: 1,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        AndroidSetCountryCodeRspPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for AndroidSetCountryCodeRspBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for AndroidSetCountryCodeRspBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}
impl Into<AndroidResponsePacket> for AndroidSetCountryCodeRspBuilder {
    fn into(self) -> AndroidResponsePacket {
        self.build().into()
    }
}
macro_rules! android_set_country_code_rsp_builder_tests { ($($name:ident: $byte_string:expr,)*) => {$(
#[test]
pub fn $name() { let raw_bytes = $byte_string;/* (0) */
match UciPacketPacket::parse(raw_bytes) {Ok(uci_packet_packet) => {match uci_packet_packet.specialize() {/* (1) */
UciPacketChild::UciResponse(uci_response_packet) => {match uci_response_packet.specialize() {/* (2) */
UciResponseChild::AndroidResponse(android_response_packet) => {match android_response_packet.specialize() {/* (3) */
AndroidResponseChild::AndroidSetCountryCodeRsp(packet) => {let rebuilder = AndroidSetCountryCodeRspBuilder {status : packet.get_status(),};let rebuilder_base : UciPacketPacket = rebuilder.into();let rebuilder_bytes : &[u8] = &rebuilder_base.to_bytes();assert_eq!(rebuilder_bytes, raw_bytes);}_ => {panic!("Couldn't parse android_set_country_code_rsp
 {:#02x?}", android_response_packet); }}}_ => {panic!("Couldn't parse android_response
 {:#02x?}", uci_response_packet); }}}_ => {panic!("Couldn't parse uci_response
 {:#02x?}", uci_packet_packet); }}},Err(e) => panic!("could not parse UciPacket: {:?} {:02x?}", e, raw_bytes),}})*}}
android_set_country_code_rsp_builder_tests! { android_set_country_code_rsp_builder_test_00: b"\x4e\x01\x00\x01\x00",}

#[derive(Debug)]
enum UciVendor_A_CommandDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_A_CommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_A_CommandDataChild::Payload(p) => p.len(),
            UciVendor_A_CommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_A_CommandChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_A_CommandData {
    child: UciVendor_A_CommandDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_A_CommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    uci_vendor__a__command: Arc<UciVendor_A_CommandData>,
}
#[derive(Debug)]
pub struct UciVendor_A_CommandBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_A_CommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_A_CommandDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_A_CommandDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_A_CommandDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_A_CommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_A_CommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_A_CommandPacket> for Bytes {
    fn from(packet: UciVendor_A_CommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_A_CommandPacket> for Vec<u8> {
    fn from(packet: UciVendor_A_CommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_A_CommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_A_CommandPacket {
    pub fn specialize(&self) -> UciVendor_A_CommandChild {
        match &self.uci_vendor__a__command.child {
            UciVendor_A_CommandDataChild::Payload(p) => {
                UciVendor_A_CommandChild::Payload(p.clone())
            }
            UciVendor_A_CommandDataChild::None => UciVendor_A_CommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let uci_vendor__a__command = match &uci_command.child {
            UciCommandDataChild::UciVendor_A_Command(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_A_Command"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            uci_vendor__a__command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_A_CommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for UciVendor_A_CommandPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_A_CommandBuilder {
    pub fn build(self) -> UciVendor_A_CommandPacket {
        let uci_vendor__a__command = Arc::new(UciVendor_A_CommandData {
            child: match self.payload {
                None => UciVendor_A_CommandDataChild::None,
                Some(bytes) => UciVendor_A_CommandDataChild::Payload(bytes),
            },
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::UciVendor_A_Command(uci_vendor__a__command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedA,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        UciVendor_A_CommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_A_CommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for UciVendor_A_CommandBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_B_CommandDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_B_CommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_B_CommandDataChild::Payload(p) => p.len(),
            UciVendor_B_CommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_B_CommandChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_B_CommandData {
    child: UciVendor_B_CommandDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_B_CommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    uci_vendor__b__command: Arc<UciVendor_B_CommandData>,
}
#[derive(Debug)]
pub struct UciVendor_B_CommandBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_B_CommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_B_CommandDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_B_CommandDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_B_CommandDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_B_CommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_B_CommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_B_CommandPacket> for Bytes {
    fn from(packet: UciVendor_B_CommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_B_CommandPacket> for Vec<u8> {
    fn from(packet: UciVendor_B_CommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_B_CommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_B_CommandPacket {
    pub fn specialize(&self) -> UciVendor_B_CommandChild {
        match &self.uci_vendor__b__command.child {
            UciVendor_B_CommandDataChild::Payload(p) => {
                UciVendor_B_CommandChild::Payload(p.clone())
            }
            UciVendor_B_CommandDataChild::None => UciVendor_B_CommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let uci_vendor__b__command = match &uci_command.child {
            UciCommandDataChild::UciVendor_B_Command(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_B_Command"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            uci_vendor__b__command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_B_CommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for UciVendor_B_CommandPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_B_CommandBuilder {
    pub fn build(self) -> UciVendor_B_CommandPacket {
        let uci_vendor__b__command = Arc::new(UciVendor_B_CommandData {
            child: match self.payload {
                None => UciVendor_B_CommandDataChild::None,
                Some(bytes) => UciVendor_B_CommandDataChild::Payload(bytes),
            },
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::UciVendor_B_Command(uci_vendor__b__command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedB,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        UciVendor_B_CommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_B_CommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for UciVendor_B_CommandBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_E_CommandDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_E_CommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_E_CommandDataChild::Payload(p) => p.len(),
            UciVendor_E_CommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_E_CommandChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_E_CommandData {
    child: UciVendor_E_CommandDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_E_CommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    uci_vendor__e__command: Arc<UciVendor_E_CommandData>,
}
#[derive(Debug)]
pub struct UciVendor_E_CommandBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_E_CommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_E_CommandDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_E_CommandDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_E_CommandDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_E_CommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_E_CommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_E_CommandPacket> for Bytes {
    fn from(packet: UciVendor_E_CommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_E_CommandPacket> for Vec<u8> {
    fn from(packet: UciVendor_E_CommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_E_CommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_E_CommandPacket {
    pub fn specialize(&self) -> UciVendor_E_CommandChild {
        match &self.uci_vendor__e__command.child {
            UciVendor_E_CommandDataChild::Payload(p) => {
                UciVendor_E_CommandChild::Payload(p.clone())
            }
            UciVendor_E_CommandDataChild::None => UciVendor_E_CommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let uci_vendor__e__command = match &uci_command.child {
            UciCommandDataChild::UciVendor_E_Command(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_E_Command"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            uci_vendor__e__command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_E_CommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for UciVendor_E_CommandPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_E_CommandBuilder {
    pub fn build(self) -> UciVendor_E_CommandPacket {
        let uci_vendor__e__command = Arc::new(UciVendor_E_CommandData {
            child: match self.payload {
                None => UciVendor_E_CommandDataChild::None,
                Some(bytes) => UciVendor_E_CommandDataChild::Payload(bytes),
            },
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::UciVendor_E_Command(uci_vendor__e__command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedE,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        UciVendor_E_CommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_E_CommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for UciVendor_E_CommandBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_F_CommandDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_F_CommandDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_F_CommandDataChild::Payload(p) => p.len(),
            UciVendor_F_CommandDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_F_CommandChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_F_CommandData {
    child: UciVendor_F_CommandDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_F_CommandPacket {
    uci_packet: Arc<UciPacketData>,
    uci_command: Arc<UciCommandData>,
    uci_vendor__f__command: Arc<UciVendor_F_CommandData>,
}
#[derive(Debug)]
pub struct UciVendor_F_CommandBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_F_CommandData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_F_CommandDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_F_CommandDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_F_CommandDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_F_CommandDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_F_CommandPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_F_CommandPacket> for Bytes {
    fn from(packet: UciVendor_F_CommandPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_F_CommandPacket> for Vec<u8> {
    fn from(packet: UciVendor_F_CommandPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_F_CommandPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_F_CommandPacket {
    pub fn specialize(&self) -> UciVendor_F_CommandChild {
        match &self.uci_vendor__f__command.child {
            UciVendor_F_CommandDataChild::Payload(p) => {
                UciVendor_F_CommandChild::Payload(p.clone())
            }
            UciVendor_F_CommandDataChild::None => UciVendor_F_CommandChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_command = match &uci_packet.child {
            UciPacketDataChild::UciCommand(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciCommand"),
        };
        let uci_vendor__f__command = match &uci_command.child {
            UciCommandDataChild::UciVendor_F_Command(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_F_Command"),
        };
        Ok(Self {
            uci_packet,
            uci_command,
            uci_vendor__f__command,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_F_CommandPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciCommandPacket> for UciVendor_F_CommandPacket {
    fn into(self) -> UciCommandPacket {
        UciCommandPacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_F_CommandBuilder {
    pub fn build(self) -> UciVendor_F_CommandPacket {
        let uci_vendor__f__command = Arc::new(UciVendor_F_CommandData {
            child: match self.payload {
                None => UciVendor_F_CommandDataChild::None,
                Some(bytes) => UciVendor_F_CommandDataChild::Payload(bytes),
            },
        });
        let uci_command = Arc::new(UciCommandData {
            child: UciCommandDataChild::UciVendor_F_Command(uci_vendor__f__command),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedF,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Command,
            opcode: self.opcode,
            child: UciPacketDataChild::UciCommand(uci_command),
        });
        UciVendor_F_CommandPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_F_CommandBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciCommandPacket> for UciVendor_F_CommandBuilder {
    fn into(self) -> UciCommandPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_A_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_A_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_A_ResponseDataChild::Payload(p) => p.len(),
            UciVendor_A_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_A_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_A_ResponseData {
    child: UciVendor_A_ResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_A_ResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    uci_vendor__a__response: Arc<UciVendor_A_ResponseData>,
}
#[derive(Debug)]
pub struct UciVendor_A_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_A_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_A_ResponseDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_A_ResponseDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_A_ResponseDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_A_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_A_ResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_A_ResponsePacket> for Bytes {
    fn from(packet: UciVendor_A_ResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_A_ResponsePacket> for Vec<u8> {
    fn from(packet: UciVendor_A_ResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_A_ResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_A_ResponsePacket {
    pub fn specialize(&self) -> UciVendor_A_ResponseChild {
        match &self.uci_vendor__a__response.child {
            UciVendor_A_ResponseDataChild::Payload(p) => {
                UciVendor_A_ResponseChild::Payload(p.clone())
            }
            UciVendor_A_ResponseDataChild::None => UciVendor_A_ResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let uci_vendor__a__response = match &uci_response.child {
            UciResponseDataChild::UciVendor_A_Response(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_A_Response"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            uci_vendor__a__response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_A_ResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for UciVendor_A_ResponsePacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_A_ResponseBuilder {
    pub fn build(self) -> UciVendor_A_ResponsePacket {
        let uci_vendor__a__response = Arc::new(UciVendor_A_ResponseData {
            child: match self.payload {
                None => UciVendor_A_ResponseDataChild::None,
                Some(bytes) => UciVendor_A_ResponseDataChild::Payload(bytes),
            },
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::UciVendor_A_Response(uci_vendor__a__response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedA,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        UciVendor_A_ResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_A_ResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for UciVendor_A_ResponseBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_B_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_B_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_B_ResponseDataChild::Payload(p) => p.len(),
            UciVendor_B_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_B_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_B_ResponseData {
    child: UciVendor_B_ResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_B_ResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    uci_vendor__b__response: Arc<UciVendor_B_ResponseData>,
}
#[derive(Debug)]
pub struct UciVendor_B_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_B_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_B_ResponseDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_B_ResponseDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_B_ResponseDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_B_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_B_ResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_B_ResponsePacket> for Bytes {
    fn from(packet: UciVendor_B_ResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_B_ResponsePacket> for Vec<u8> {
    fn from(packet: UciVendor_B_ResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_B_ResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_B_ResponsePacket {
    pub fn specialize(&self) -> UciVendor_B_ResponseChild {
        match &self.uci_vendor__b__response.child {
            UciVendor_B_ResponseDataChild::Payload(p) => {
                UciVendor_B_ResponseChild::Payload(p.clone())
            }
            UciVendor_B_ResponseDataChild::None => UciVendor_B_ResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let uci_vendor__b__response = match &uci_response.child {
            UciResponseDataChild::UciVendor_B_Response(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_B_Response"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            uci_vendor__b__response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_B_ResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for UciVendor_B_ResponsePacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_B_ResponseBuilder {
    pub fn build(self) -> UciVendor_B_ResponsePacket {
        let uci_vendor__b__response = Arc::new(UciVendor_B_ResponseData {
            child: match self.payload {
                None => UciVendor_B_ResponseDataChild::None,
                Some(bytes) => UciVendor_B_ResponseDataChild::Payload(bytes),
            },
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::UciVendor_B_Response(uci_vendor__b__response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedB,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        UciVendor_B_ResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_B_ResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for UciVendor_B_ResponseBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_E_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_E_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_E_ResponseDataChild::Payload(p) => p.len(),
            UciVendor_E_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_E_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_E_ResponseData {
    child: UciVendor_E_ResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_E_ResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    uci_vendor__e__response: Arc<UciVendor_E_ResponseData>,
}
#[derive(Debug)]
pub struct UciVendor_E_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_E_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_E_ResponseDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_E_ResponseDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_E_ResponseDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_E_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_E_ResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_E_ResponsePacket> for Bytes {
    fn from(packet: UciVendor_E_ResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_E_ResponsePacket> for Vec<u8> {
    fn from(packet: UciVendor_E_ResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_E_ResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_E_ResponsePacket {
    pub fn specialize(&self) -> UciVendor_E_ResponseChild {
        match &self.uci_vendor__e__response.child {
            UciVendor_E_ResponseDataChild::Payload(p) => {
                UciVendor_E_ResponseChild::Payload(p.clone())
            }
            UciVendor_E_ResponseDataChild::None => UciVendor_E_ResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let uci_vendor__e__response = match &uci_response.child {
            UciResponseDataChild::UciVendor_E_Response(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_E_Response"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            uci_vendor__e__response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_E_ResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for UciVendor_E_ResponsePacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_E_ResponseBuilder {
    pub fn build(self) -> UciVendor_E_ResponsePacket {
        let uci_vendor__e__response = Arc::new(UciVendor_E_ResponseData {
            child: match self.payload {
                None => UciVendor_E_ResponseDataChild::None,
                Some(bytes) => UciVendor_E_ResponseDataChild::Payload(bytes),
            },
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::UciVendor_E_Response(uci_vendor__e__response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedE,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        UciVendor_E_ResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_E_ResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for UciVendor_E_ResponseBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_F_ResponseDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_F_ResponseDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_F_ResponseDataChild::Payload(p) => p.len(),
            UciVendor_F_ResponseDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_F_ResponseChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_F_ResponseData {
    child: UciVendor_F_ResponseDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_F_ResponsePacket {
    uci_packet: Arc<UciPacketData>,
    uci_response: Arc<UciResponseData>,
    uci_vendor__f__response: Arc<UciVendor_F_ResponseData>,
}
#[derive(Debug)]
pub struct UciVendor_F_ResponseBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_F_ResponseData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_F_ResponseDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_F_ResponseDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_F_ResponseDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_F_ResponseDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_F_ResponsePacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_F_ResponsePacket> for Bytes {
    fn from(packet: UciVendor_F_ResponsePacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_F_ResponsePacket> for Vec<u8> {
    fn from(packet: UciVendor_F_ResponsePacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_F_ResponsePacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_F_ResponsePacket {
    pub fn specialize(&self) -> UciVendor_F_ResponseChild {
        match &self.uci_vendor__f__response.child {
            UciVendor_F_ResponseDataChild::Payload(p) => {
                UciVendor_F_ResponseChild::Payload(p.clone())
            }
            UciVendor_F_ResponseDataChild::None => UciVendor_F_ResponseChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_response = match &uci_packet.child {
            UciPacketDataChild::UciResponse(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciResponse"),
        };
        let uci_vendor__f__response = match &uci_response.child {
            UciResponseDataChild::UciVendor_F_Response(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_F_Response"),
        };
        Ok(Self {
            uci_packet,
            uci_response,
            uci_vendor__f__response,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_F_ResponsePacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciResponsePacket> for UciVendor_F_ResponsePacket {
    fn into(self) -> UciResponsePacket {
        UciResponsePacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_F_ResponseBuilder {
    pub fn build(self) -> UciVendor_F_ResponsePacket {
        let uci_vendor__f__response = Arc::new(UciVendor_F_ResponseData {
            child: match self.payload {
                None => UciVendor_F_ResponseDataChild::None,
                Some(bytes) => UciVendor_F_ResponseDataChild::Payload(bytes),
            },
        });
        let uci_response = Arc::new(UciResponseData {
            child: UciResponseDataChild::UciVendor_F_Response(uci_vendor__f__response),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedF,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Response,
            opcode: self.opcode,
            child: UciPacketDataChild::UciResponse(uci_response),
        });
        UciVendor_F_ResponsePacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_F_ResponseBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciResponsePacket> for UciVendor_F_ResponseBuilder {
    fn into(self) -> UciResponsePacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_A_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_A_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_A_NotificationDataChild::Payload(p) => p.len(),
            UciVendor_A_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_A_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_A_NotificationData {
    child: UciVendor_A_NotificationDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_A_NotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    uci_vendor__a__notification: Arc<UciVendor_A_NotificationData>,
}
#[derive(Debug)]
pub struct UciVendor_A_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_A_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_A_NotificationDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_A_NotificationDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_A_NotificationDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_A_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_A_NotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_A_NotificationPacket> for Bytes {
    fn from(packet: UciVendor_A_NotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_A_NotificationPacket> for Vec<u8> {
    fn from(packet: UciVendor_A_NotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_A_NotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_A_NotificationPacket {
    pub fn specialize(&self) -> UciVendor_A_NotificationChild {
        match &self.uci_vendor__a__notification.child {
            UciVendor_A_NotificationDataChild::Payload(p) => {
                UciVendor_A_NotificationChild::Payload(p.clone())
            }
            UciVendor_A_NotificationDataChild::None => UciVendor_A_NotificationChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let uci_vendor__a__notification = match &uci_notification.child {
            UciNotificationDataChild::UciVendor_A_Notification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_A_Notification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            uci_vendor__a__notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_A_NotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for UciVendor_A_NotificationPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_A_NotificationBuilder {
    pub fn build(self) -> UciVendor_A_NotificationPacket {
        let uci_vendor__a__notification = Arc::new(UciVendor_A_NotificationData {
            child: match self.payload {
                None => UciVendor_A_NotificationDataChild::None,
                Some(bytes) => UciVendor_A_NotificationDataChild::Payload(bytes),
            },
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::UciVendor_A_Notification(uci_vendor__a__notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedA,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        UciVendor_A_NotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_A_NotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for UciVendor_A_NotificationBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_B_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_B_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_B_NotificationDataChild::Payload(p) => p.len(),
            UciVendor_B_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_B_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_B_NotificationData {
    child: UciVendor_B_NotificationDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_B_NotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    uci_vendor__b__notification: Arc<UciVendor_B_NotificationData>,
}
#[derive(Debug)]
pub struct UciVendor_B_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_B_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_B_NotificationDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_B_NotificationDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_B_NotificationDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_B_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_B_NotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_B_NotificationPacket> for Bytes {
    fn from(packet: UciVendor_B_NotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_B_NotificationPacket> for Vec<u8> {
    fn from(packet: UciVendor_B_NotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_B_NotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_B_NotificationPacket {
    pub fn specialize(&self) -> UciVendor_B_NotificationChild {
        match &self.uci_vendor__b__notification.child {
            UciVendor_B_NotificationDataChild::Payload(p) => {
                UciVendor_B_NotificationChild::Payload(p.clone())
            }
            UciVendor_B_NotificationDataChild::None => UciVendor_B_NotificationChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let uci_vendor__b__notification = match &uci_notification.child {
            UciNotificationDataChild::UciVendor_B_Notification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_B_Notification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            uci_vendor__b__notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_B_NotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for UciVendor_B_NotificationPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_B_NotificationBuilder {
    pub fn build(self) -> UciVendor_B_NotificationPacket {
        let uci_vendor__b__notification = Arc::new(UciVendor_B_NotificationData {
            child: match self.payload {
                None => UciVendor_B_NotificationDataChild::None,
                Some(bytes) => UciVendor_B_NotificationDataChild::Payload(bytes),
            },
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::UciVendor_B_Notification(uci_vendor__b__notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedB,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        UciVendor_B_NotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_B_NotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for UciVendor_B_NotificationBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_E_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_E_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_E_NotificationDataChild::Payload(p) => p.len(),
            UciVendor_E_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_E_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_E_NotificationData {
    child: UciVendor_E_NotificationDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_E_NotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    uci_vendor__e__notification: Arc<UciVendor_E_NotificationData>,
}
#[derive(Debug)]
pub struct UciVendor_E_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_E_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_E_NotificationDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_E_NotificationDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_E_NotificationDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_E_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_E_NotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_E_NotificationPacket> for Bytes {
    fn from(packet: UciVendor_E_NotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_E_NotificationPacket> for Vec<u8> {
    fn from(packet: UciVendor_E_NotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_E_NotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_E_NotificationPacket {
    pub fn specialize(&self) -> UciVendor_E_NotificationChild {
        match &self.uci_vendor__e__notification.child {
            UciVendor_E_NotificationDataChild::Payload(p) => {
                UciVendor_E_NotificationChild::Payload(p.clone())
            }
            UciVendor_E_NotificationDataChild::None => UciVendor_E_NotificationChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let uci_vendor__e__notification = match &uci_notification.child {
            UciNotificationDataChild::UciVendor_E_Notification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_E_Notification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            uci_vendor__e__notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_E_NotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for UciVendor_E_NotificationPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_E_NotificationBuilder {
    pub fn build(self) -> UciVendor_E_NotificationPacket {
        let uci_vendor__e__notification = Arc::new(UciVendor_E_NotificationData {
            child: match self.payload {
                None => UciVendor_E_NotificationDataChild::None,
                Some(bytes) => UciVendor_E_NotificationDataChild::Payload(bytes),
            },
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::UciVendor_E_Notification(uci_vendor__e__notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedE,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        UciVendor_E_NotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_E_NotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for UciVendor_E_NotificationBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}

#[derive(Debug)]
enum UciVendor_F_NotificationDataChild {
    Payload(Bytes),
    None,
}
impl UciVendor_F_NotificationDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            UciVendor_F_NotificationDataChild::Payload(p) => p.len(),
            UciVendor_F_NotificationDataChild::None => 0,
        }
    }
}
#[derive(Debug)]
pub enum UciVendor_F_NotificationChild {
    Payload(Bytes),
    None,
}
#[derive(Debug)]
struct UciVendor_F_NotificationData {
    child: UciVendor_F_NotificationDataChild,
}
#[derive(Debug, Clone)]
pub struct UciVendor_F_NotificationPacket {
    uci_packet: Arc<UciPacketData>,
    uci_notification: Arc<UciNotificationData>,
    uci_vendor__f__notification: Arc<UciVendor_F_NotificationData>,
}
#[derive(Debug)]
pub struct UciVendor_F_NotificationBuilder {
    pub opcode: u8,
    pub payload: Option<Bytes>,
}
impl UciVendor_F_NotificationData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 4 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let payload: Vec<u8> = bytes[4..].into();
        let child = if payload.len() > 0 {
            UciVendor_F_NotificationDataChild::Payload(Bytes::from(payload))
        } else {
            UciVendor_F_NotificationDataChild::None
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            UciVendor_F_NotificationDataChild::Payload(p) => buffer[4..].copy_from_slice(&p[..]),
            UciVendor_F_NotificationDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size() + self.child.get_total_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        ret
    }
}
impl Packet for UciVendor_F_NotificationPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.uci_packet.get_total_size(), 0);
        self.uci_packet.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<UciVendor_F_NotificationPacket> for Bytes {
    fn from(packet: UciVendor_F_NotificationPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<UciVendor_F_NotificationPacket> for Vec<u8> {
    fn from(packet: UciVendor_F_NotificationPacket) -> Self {
        packet.to_vec()
    }
}
impl TryFrom<UciPacketPacket> for UciVendor_F_NotificationPacket {
    type Error = TryFromError;
    fn try_from(value: UciPacketPacket) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uci_packet).map_err(TryFromError)
    }
}
impl UciVendor_F_NotificationPacket {
    pub fn specialize(&self) -> UciVendor_F_NotificationChild {
        match &self.uci_vendor__f__notification.child {
            UciVendor_F_NotificationDataChild::Payload(p) => {
                UciVendor_F_NotificationChild::Payload(p.clone())
            }
            UciVendor_F_NotificationDataChild::None => UciVendor_F_NotificationChild::None,
        }
    }
    fn new(root: Arc<UciPacketData>) -> std::result::Result<Self, &'static str> {
        let uci_packet = root;
        let uci_notification = match &uci_packet.child {
            UciPacketDataChild::UciNotification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciNotification"),
        };
        let uci_vendor__f__notification = match &uci_notification.child {
            UciNotificationDataChild::UciVendor_F_Notification(value) => (*value).clone(),
            _ => return Err("inconsistent state - child was not UciVendor_F_Notification"),
        };
        Ok(Self {
            uci_packet,
            uci_notification,
            uci_vendor__f__notification,
        })
    }
    pub fn get_group_id(&self) -> GroupId {
        self.uci_packet.as_ref().group_id
    }
    pub fn get_packet_boundary_flag(&self) -> PacketBoundaryFlag {
        self.uci_packet.as_ref().packet_boundary_flag
    }
    pub fn get_message_type(&self) -> MessageType {
        self.uci_packet.as_ref().message_type
    }
    pub fn get_opcode(&self) -> u8 {
        self.uci_packet.as_ref().opcode
    }
}
impl Into<UciPacketPacket> for UciVendor_F_NotificationPacket {
    fn into(self) -> UciPacketPacket {
        UciPacketPacket::new(self.uci_packet).unwrap()
    }
}
impl Into<UciNotificationPacket> for UciVendor_F_NotificationPacket {
    fn into(self) -> UciNotificationPacket {
        UciNotificationPacket::new(self.uci_packet).unwrap()
    }
}
impl UciVendor_F_NotificationBuilder {
    pub fn build(self) -> UciVendor_F_NotificationPacket {
        let uci_vendor__f__notification = Arc::new(UciVendor_F_NotificationData {
            child: match self.payload {
                None => UciVendor_F_NotificationDataChild::None,
                Some(bytes) => UciVendor_F_NotificationDataChild::Payload(bytes),
            },
        });
        let uci_notification = Arc::new(UciNotificationData {
            child: UciNotificationDataChild::UciVendor_F_Notification(uci_vendor__f__notification),
        });
        let uci_packet = Arc::new(UciPacketData {
            group_id: GroupId::VendorReservedF,
            packet_boundary_flag: PacketBoundaryFlag::Complete,
            message_type: MessageType::Notification,
            opcode: self.opcode,
            child: UciPacketDataChild::UciNotification(uci_notification),
        });
        UciVendor_F_NotificationPacket::new(uci_packet).unwrap()
    }
}
impl Into<UciPacketPacket> for UciVendor_F_NotificationBuilder {
    fn into(self) -> UciPacketPacket {
        self.build().into()
    }
}
impl Into<UciNotificationPacket> for UciVendor_F_NotificationBuilder {
    fn into(self) -> UciNotificationPacket {
        self.build().into()
    }
}
