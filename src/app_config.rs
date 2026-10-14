use std::collections::HashMap;

use bytes::Bytes;
use thiserror::Error;

use crate::MacAddress;
use crate::packets::uci;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AppConfigError {
    #[error("invalid {id:?} length: expected {expected}, got {actual}")]
    InvalidLength {
        id: uci::AppConfigTlvType,
        expected: usize,
        actual: usize,
    },
    #[error("invalid SubSessionKey length: expected 16 or 32, got {actual}")]
    InvalidSubSessionKeyLength { actual: usize },
    #[error("invalid CccHopModeKey length: expected 4 or 16, got {actual}")]
    InvalidCccHopModeKeyLength { actual: usize },
    #[error("invalid {id:?} enum value: {value:#x}")]
    InvalidEnumValue {
        id: uci::AppConfigTlvType,
        value: u8,
    },
    #[error("mac_address_mode {0:?} is not supported")]
    UnsupportedMacAddressMode(uci::MacAddressMode),
    #[error("unsupported app config parameter {0:?}")]
    UnsupportedParameter(uci::AppConfigTlvType),
}

/// [UCI] 8.3 Application Configuration Parameters.
/// Sub-session Key provided for Provisioned STS for Responder specific Key mode
/// (STS_CONFIG equal to 0x04).
#[derive(Clone, PartialEq, Eq)]
pub enum SubSessionKey {
    None,
    Short([u8; 16]),
    Extended([u8; 32]),
}

const DEFAULT_STS_CONFIG: uci::StsConfig = uci::StsConfig::Static;
const DEFAULT_CHANNEL_NUMBER: uci::ChannelNumber = uci::ChannelNumber::ChannelNumber9;
const DEFAULT_NUMBER_OF_CONTROLEES: u8 = 1;
const DEFAULT_RANGING_DURATION: u32 = 200;
const DEFAULT_SESSION_INFO_NTF_CONFIG: uci::SessionInfoNtfConfig =
    uci::SessionInfoNtfConfig::Enable;
const DEFAULT_NEAR_PROXIMITY_CONFIG: u16 = 0;
const DEFAULT_FAR_PROXIMITY_CONFIG: u16 = 20000;
const DEFAULT_MAC_ADDRESS_MODE: uci::MacAddressMode = uci::MacAddressMode::Mode0;
const DEFAULT_IN_BAND_TERMINATION_ATTEMPT_COUNT: u8 = 1;
const DEFAULT_MAX_NUMBER_OF_MEASUREMENTS: u16 = 0;
const DEFAULT_SESSION_DATA_TRANSFER_STATUS_NTF_CONFIG: uci::SessionDataTransferStatusNtfConfig =
    uci::SessionDataTransferStatusNtfConfig::Disable;
const DEFAULT_DATA_REPETITION_COUNT: u8 = 0;
const DEFAULT_AOA_BOUND_CONFIG: [u8; 8] = [0x00, 0xa6, 0x00, 0x5a, 0x00, 0xd3, 0x00, 0x2d];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AoaBounds {
    min_azimuth: i16,
    max_azimuth: i16,
    min_elevation: i16,
    max_elevation: i16,
}

impl AoaBounds {
    fn contains(&self, azimuth: i16, elevation: i16) -> bool {
        (self.min_azimuth..=self.max_azimuth).contains(&azimuth)
            && (self.min_elevation..=self.max_elevation).contains(&elevation)
    }
}
const DEFAULT_CCC_TLVS: [(uci::AppConfigTlvType, Bytes); 5] = [
    (
        uci::AppConfigTlvType::RangingDuration,
        Bytes::from_static(const { &96u32.to_le_bytes() }),
    ),
    (
        uci::AppConfigTlvType::SessionInfoNtfConfig,
        Bytes::from_static(&[uci::SessionInfoNtfConfig::Disable as u8]),
    ),
    (uci::AppConfigTlvType::SlotsPerRr, Bytes::from_static(&[6])),
    (
        uci::AppConfigTlvType::CccHopModeKey,
        Bytes::from_static(&[0, 0, 0, 0]),
    ),
    (
        uci::AppConfigTlvType::CccLastIndexUsed,
        Bytes::from_static(&[0, 0, 0, 0]),
    ),
];

/// [UCI] 8.3 Application Configuration Parameters.
/// The configuration is initially filled with default values from the
/// specification.
/// See [UCI] Table 45: APP Configuration Parameters IDs
/// for the format of each parameter and the default value.
/// Mandatory APP configuration parameters are declared as optional,
/// and must be set before moving the session from SESSION_STATE_INIT to
/// SESSION_STATE_IDLE.
#[derive(Clone, PartialEq, Eq)]
pub struct AppConfig {
    session_type: uci::SessionType,
    tlvs: HashMap<uci::AppConfigTlvType, Bytes>,
}

impl AppConfig {
    pub fn new(session_type: uci::SessionType) -> Self {
        let mut tlvs = HashMap::from([
            (
                uci::AppConfigTlvType::ChannelNumber,
                Bytes::from_static(&[DEFAULT_CHANNEL_NUMBER as u8]),
            ),
            (
                uci::AppConfigTlvType::NumberOfControlees,
                Bytes::from_static(&[DEFAULT_NUMBER_OF_CONTROLEES]),
            ),
            (
                uci::AppConfigTlvType::SlotDuration,
                Bytes::from_static(const { &2400u16.to_le_bytes() }),
            ),
            (
                uci::AppConfigTlvType::StsIndex,
                Bytes::from_static(const { &0u32.to_le_bytes() }),
            ),
            (
                uci::AppConfigTlvType::NearProximityConfig,
                Bytes::from_static(const { &DEFAULT_NEAR_PROXIMITY_CONFIG.to_le_bytes() }),
            ),
            (
                uci::AppConfigTlvType::FarProximityConfig,
                Bytes::from_static(const { &DEFAULT_FAR_PROXIMITY_CONFIG.to_le_bytes() }),
            ),
            (
                uci::AppConfigTlvType::PreambleCodeIndex,
                Bytes::from_static(&[10]),
            ),
            (
                uci::AppConfigTlvType::AoaBoundConfig,
                Bytes::from_static(&DEFAULT_AOA_BOUND_CONFIG),
            ),
            (
                uci::AppConfigTlvType::UwbInitiationTime,
                Bytes::from_static(&[0u8; 8]),
            ),
            (
                uci::AppConfigTlvType::HoppingMode,
                Bytes::from_static(&[uci::HoppingMode::Disable as u8]),
            ),
        ]);

        match session_type {
            uci::SessionType::FiraRangingSession
            | uci::SessionType::FiraRangingAndInBandDataSession
            | uci::SessionType::FiraDataTransferSession
            | uci::SessionType::FiraRangingOnlyPhase
            | uci::SessionType::FiraInBandDataPhase
            | uci::SessionType::FiraRangingWithDataPhase => tlvs.extend([
                (
                    uci::AppConfigTlvType::RangingDuration,
                    Bytes::from_static(const { &DEFAULT_RANGING_DURATION.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::SessionInfoNtfConfig,
                    Bytes::from_static(&[DEFAULT_SESSION_INFO_NTF_CONFIG as u8]),
                ),
                (uci::AppConfigTlvType::SlotsPerRr, Bytes::from_static(&[25])),
                (
                    uci::AppConfigTlvType::StsConfig,
                    Bytes::from_static(&[DEFAULT_STS_CONFIG as u8]),
                ),
                (uci::AppConfigTlvType::DstMacAddress, Bytes::new()),
                (
                    uci::AppConfigTlvType::MacFcsType,
                    Bytes::from_static(&[uci::MacFcsType::Crc16 as u8]),
                ),
                // The default is 0x03 when Time Scheduled Ranging is used,
                // 0x06 when Contention-based Ranging is used.
                (
                    uci::AppConfigTlvType::RangingRoundControl,
                    Bytes::from_static(&[0x06]),
                ),
                (
                    uci::AppConfigTlvType::AoaResultReq,
                    Bytes::from_static(&[uci::AoaResultReq::AoaEnabled as u8]),
                ),
                (
                    uci::AppConfigTlvType::RframeConfig,
                    Bytes::from_static(&[uci::RframeConfig::Sp3 as u8]),
                ),
                (
                    uci::AppConfigTlvType::RssiReporting,
                    Bytes::from_static(&[uci::RssiReporting::Disable as u8]),
                ),
                (uci::AppConfigTlvType::SfdId, Bytes::from_static(&[2])),
                (
                    uci::AppConfigTlvType::PsduDataRate,
                    Bytes::from_static(&[uci::PsduDataRate::DataRate6m81 as u8]),
                ),
                (
                    uci::AppConfigTlvType::PreambleDuration,
                    Bytes::from_static(&[uci::PreambleDuration::Duration64Symbols as u8]),
                ),
                (
                    uci::AppConfigTlvType::LinkLayerMode,
                    Bytes::from_static(&[uci::LinkLayerMode::BypassMode as u8]),
                ),
                (
                    uci::AppConfigTlvType::DataRepetitionCount,
                    Bytes::from_static(&[DEFAULT_DATA_REPETITION_COUNT]),
                ),
                (
                    uci::AppConfigTlvType::RangingTimeStruct,
                    Bytes::from_static(&[uci::RangingTimeStruct::BlockBasedScheduling as u8]),
                ),
                (
                    uci::AppConfigTlvType::PrfMode,
                    Bytes::from_static(&[uci::PrfMode::BprfMode as u8]),
                ),
                // Default for Octet[0] is SLOTS_PER_RR - 1
                (
                    uci::AppConfigTlvType::CapSizeRange,
                    Bytes::from_static(&[24, 5]),
                ),
                (
                    uci::AppConfigTlvType::TxJitterWindowSize,
                    Bytes::from_static(&[0]),
                ),
                (
                    uci::AppConfigTlvType::KeyRotation,
                    Bytes::from_static(&[uci::KeyRotation::Disable as u8]),
                ),
                (
                    uci::AppConfigTlvType::KeyRotationRate,
                    Bytes::from_static(&[0]),
                ),
                (
                    uci::AppConfigTlvType::SessionPriority,
                    Bytes::from_static(&[50]),
                ),
                (
                    uci::AppConfigTlvType::MacAddressMode,
                    Bytes::from_static(&[DEFAULT_MAC_ADDRESS_MODE as u8]),
                ),
                (
                    uci::AppConfigTlvType::VendorId,
                    Bytes::from_static(const { &0u16.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::StaticStsIv,
                    Bytes::from_static(&[0u8; 6]),
                ),
                (
                    uci::AppConfigTlvType::NumberOfStsSegments,
                    Bytes::from_static(&[1]),
                ),
                (
                    uci::AppConfigTlvType::MaxRrRetry,
                    Bytes::from_static(const { &0u16.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::BlockStrideLength,
                    Bytes::from_static(&[0]),
                ),
                (
                    uci::AppConfigTlvType::ResultReportConfig,
                    Bytes::from_static(&[0x01]),
                ),
                (
                    uci::AppConfigTlvType::InBandTerminationAttemptCount,
                    Bytes::from_static(&[DEFAULT_IN_BAND_TERMINATION_ATTEMPT_COUNT]),
                ),
                (
                    uci::AppConfigTlvType::SubSessionId,
                    Bytes::from_static(const { &0u32.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::BprfPhrDataRate,
                    Bytes::from_static(&[uci::BprfPhrDataRate::DataRate850k as u8]),
                ),
                (
                    uci::AppConfigTlvType::MaxNumberOfMeasurements,
                    Bytes::from_static(const { &DEFAULT_MAX_NUMBER_OF_MEASUREMENTS.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::StsLength,
                    Bytes::from_static(&[uci::StsLength::Length64Symbols as u8]),
                ),
                (
                    uci::AppConfigTlvType::MinFramesPerRr,
                    Bytes::from_static(&[4]),
                ),
                (
                    uci::AppConfigTlvType::MtuSize,
                    Bytes::from_static(const { &0u16.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::InterFrameInterval,
                    Bytes::from_static(&[1]),
                ),
                (uci::AppConfigTlvType::SessionKey, Bytes::new()),
                (uci::AppConfigTlvType::SubSessionKey, Bytes::new()),
                (
                    uci::AppConfigTlvType::SessionDataTransferStatusNtfConfig,
                    Bytes::from_static(&[DEFAULT_SESSION_DATA_TRANSFER_STATUS_NTF_CONFIG as u8]),
                ),
                (
                    uci::AppConfigTlvType::SessionTimeBase,
                    Bytes::from_static(&[0u8; 9]),
                ),
                (
                    uci::AppConfigTlvType::ApplicationDataEndpoint,
                    Bytes::from_static(&[0]),
                ),
            ]),
            uci::SessionType::Ccc => tlvs.extend(DEFAULT_CCC_TLVS),
            uci::SessionType::Aliro => tlvs.extend(DEFAULT_CCC_TLVS.into_iter().chain([(
                uci::AppConfigTlvType::AliroMacMode,
                Bytes::from_static(&[0]),
            )])),
            uci::SessionType::DeviceTestMode => {
                return Self {
                    session_type,
                    tlvs: HashMap::new(),
                };
            }
        }

        Self { session_type, tlvs }
    }

    /// Retrieve the APP configuration value with the selected identifier
    /// Returns `Some` if the identifier is set, `None` otherwise.
    pub fn get(&self, id: uci::AppConfigTlvType) -> Option<&Bytes> {
        self.tlvs.get(&id)
    }

    pub fn to_tlvs(&self) -> Vec<uci::AppConfigTlv> {
        self.tlvs
            .iter()
            .map(|(&cfg_id, v)| uci::AppConfigTlv {
                cfg_id,
                v: v.to_vec(),
            })
            .collect()
    }

    fn parse_tlv(&self, id: uci::AppConfigTlvType, value: &[u8]) -> Result<Bytes, AppConfigError> {
        fn try_parse_array<const N: usize>(
            id: uci::AppConfigTlvType,
            value: &[u8],
        ) -> Result<Bytes, AppConfigError> {
            <[u8; N]>::try_from(value).map_err(|_| AppConfigError::InvalidLength {
                id,
                expected: N,
                actual: value.len(),
            })?;
            Ok(Bytes::copy_from_slice(value))
        }

        fn try_parse_u8(id: uci::AppConfigTlvType, value: &[u8]) -> Result<u8, AppConfigError> {
            let &[byte] = value else {
                return Err(AppConfigError::InvalidLength {
                    id,
                    expected: 1,
                    actual: value.len(),
                });
            };
            Ok(byte)
        }

        fn try_parse_enum<T: TryFrom<u8, Error = u8>>(
            id: uci::AppConfigTlvType,
            value: &[u8],
        ) -> Result<Bytes, AppConfigError> {
            let byte = try_parse_u8(id, value)?;
            T::try_from(byte).map_err(|value| AppConfigError::InvalidEnumValue { id, value })?;
            Ok(Bytes::copy_from_slice(value))
        }

        fn try_parse_u16(id: uci::AppConfigTlvType, value: &[u8]) -> Result<Bytes, AppConfigError> {
            try_parse_array::<2>(id, value)
        }

        fn try_parse_u32(id: uci::AppConfigTlvType, value: &[u8]) -> Result<Bytes, AppConfigError> {
            try_parse_array::<4>(id, value)
        }

        fn try_parse_u64(id: uci::AppConfigTlvType, value: &[u8]) -> Result<Bytes, AppConfigError> {
            try_parse_array::<8>(id, value)
        }

        match id {
            uci::AppConfigTlvType::DeviceType => try_parse_enum::<uci::DeviceType>(id, value),
            uci::AppConfigTlvType::RangingRoundUsage => {
                try_parse_enum::<uci::RangingRoundUsage>(id, value)
            }
            uci::AppConfigTlvType::StsConfig => try_parse_enum::<uci::StsConfig>(id, value),
            uci::AppConfigTlvType::MultiNodeMode => try_parse_enum::<uci::MultiNodeMode>(id, value),
            uci::AppConfigTlvType::ChannelNumber => try_parse_enum::<uci::ChannelNumber>(id, value),
            uci::AppConfigTlvType::NumberOfControlees
            | uci::AppConfigTlvType::RangingRoundControl
            | uci::AppConfigTlvType::PreambleCodeIndex
            | uci::AppConfigTlvType::SfdId
            | uci::AppConfigTlvType::DataRepetitionCount
            | uci::AppConfigTlvType::SlotsPerRr
            | uci::AppConfigTlvType::TxJitterWindowSize
            | uci::AppConfigTlvType::KeyRotationRate
            | uci::AppConfigTlvType::SessionPriority
            | uci::AppConfigTlvType::NumberOfStsSegments
            | uci::AppConfigTlvType::BlockStrideLength
            | uci::AppConfigTlvType::ResultReportConfig
            | uci::AppConfigTlvType::InBandTerminationAttemptCount
            | uci::AppConfigTlvType::AliroMacMode
            | uci::AppConfigTlvType::MinFramesPerRr
            | uci::AppConfigTlvType::InterFrameInterval
            | uci::AppConfigTlvType::ApplicationDataEndpoint
            | uci::AppConfigTlvType::CccPulseshapeCombo => try_parse_array::<1>(id, value),
            uci::AppConfigTlvType::HoppingMode => match self.session_type {
                uci::SessionType::Ccc | uci::SessionType::Aliro => {
                    try_parse_enum::<uci::CccHoppingMode>(id, value)
                }
                _ => try_parse_enum::<uci::HoppingMode>(id, value),
            },
            uci::AppConfigTlvType::DeviceMacAddress => match self.mac_address_mode() {
                uci::MacAddressMode::Mode0 => try_parse_array::<2>(id, value),
                uci::MacAddressMode::Mode1 => Err(AppConfigError::UnsupportedMacAddressMode(
                    uci::MacAddressMode::Mode1,
                )),
                uci::MacAddressMode::Mode2 => try_parse_array::<8>(id, value),
            },
            uci::AppConfigTlvType::DstMacAddress => {
                let mac_address_size = match self.mac_address_mode() {
                    uci::MacAddressMode::Mode0 => 2,
                    uci::MacAddressMode::Mode1 => {
                        return Err(AppConfigError::UnsupportedMacAddressMode(
                            uci::MacAddressMode::Mode1,
                        ));
                    }
                    uci::MacAddressMode::Mode2 => 8,
                };
                let expected = self.number_of_controlees() as usize * mac_address_size;
                if value.len() != expected {
                    return Err(AppConfigError::InvalidLength {
                        id,
                        expected,
                        actual: value.len(),
                    });
                }
                Ok(Bytes::copy_from_slice(value))
            }
            uci::AppConfigTlvType::SlotDuration
            | uci::AppConfigTlvType::NearProximityConfig
            | uci::AppConfigTlvType::FarProximityConfig
            | uci::AppConfigTlvType::VendorId
            | uci::AppConfigTlvType::MaxRrRetry
            | uci::AppConfigTlvType::MaxNumberOfMeasurements
            | uci::AppConfigTlvType::MtuSize
            | uci::AppConfigTlvType::CccUwbConfigId
            | uci::AppConfigTlvType::CccUrskTtl => try_parse_u16(id, value),
            uci::AppConfigTlvType::RangingDuration
            | uci::AppConfigTlvType::StsIndex
            | uci::AppConfigTlvType::SubSessionId
            | uci::AppConfigTlvType::CccLastIndexUsed => try_parse_u32(id, value),
            uci::AppConfigTlvType::MacFcsType => try_parse_enum::<uci::MacFcsType>(id, value),
            uci::AppConfigTlvType::AoaResultReq => try_parse_enum::<uci::AoaResultReq>(id, value),
            uci::AppConfigTlvType::SessionInfoNtfConfig => {
                try_parse_enum::<uci::SessionInfoNtfConfig>(id, value)
            }
            uci::AppConfigTlvType::DeviceRole => try_parse_enum::<uci::DeviceRole>(id, value),
            uci::AppConfigTlvType::RframeConfig => try_parse_enum::<uci::RframeConfig>(id, value),
            uci::AppConfigTlvType::RssiReporting => try_parse_enum::<uci::RssiReporting>(id, value),
            uci::AppConfigTlvType::PsduDataRate => try_parse_enum::<uci::PsduDataRate>(id, value),
            uci::AppConfigTlvType::PreambleDuration => {
                try_parse_enum::<uci::PreambleDuration>(id, value)
            }
            uci::AppConfigTlvType::LinkLayerMode => try_parse_enum::<uci::LinkLayerMode>(id, value),
            uci::AppConfigTlvType::RangingTimeStruct => {
                try_parse_enum::<uci::RangingTimeStruct>(id, value)
            }
            uci::AppConfigTlvType::AoaBoundConfig => try_parse_array::<8>(id, value),
            uci::AppConfigTlvType::PrfMode => try_parse_enum::<uci::PrfMode>(id, value),
            uci::AppConfigTlvType::CapSizeRange | uci::AppConfigTlvType::CccRangingProtocolVer => {
                try_parse_array::<2>(id, value)
            }
            uci::AppConfigTlvType::ScheduleMode => try_parse_enum::<uci::ScheduleMode>(id, value),
            uci::AppConfigTlvType::KeyRotation => try_parse_enum::<uci::KeyRotation>(id, value),
            uci::AppConfigTlvType::MacAddressMode => {
                let mode = try_parse_enum::<uci::MacAddressMode>(id, value)?;
                if value == [uci::MacAddressMode::Mode1 as u8] {
                    return Err(AppConfigError::UnsupportedMacAddressMode(
                        uci::MacAddressMode::Mode1,
                    ));
                }
                Ok(mode)
            }
            uci::AppConfigTlvType::StaticStsIv => try_parse_array::<6>(id, value),
            uci::AppConfigTlvType::UwbInitiationTime => {
                // Implement backward compatibility for UCI 1.0
                // where the value is 4 bytes instead of 8.
                if let Ok(bytes) = <[u8; 4]>::try_from(value) {
                    let v = u64::from(u32::from_le_bytes(bytes));
                    Ok(Bytes::copy_from_slice(&v.to_le_bytes()))
                } else {
                    try_parse_u64(id, value)
                }
            }
            uci::AppConfigTlvType::BprfPhrDataRate => {
                try_parse_enum::<uci::BprfPhrDataRate>(id, value)
            }
            uci::AppConfigTlvType::StsLength => try_parse_enum::<uci::StsLength>(id, value),
            uci::AppConfigTlvType::SessionKey => Ok(Bytes::copy_from_slice(value)),
            uci::AppConfigTlvType::SubSessionKey => {
                if value.len() != 16 && value.len() != 32 {
                    return Err(AppConfigError::InvalidSubSessionKeyLength {
                        actual: value.len(),
                    });
                }
                Ok(Bytes::copy_from_slice(value))
            }
            uci::AppConfigTlvType::SessionDataTransferStatusNtfConfig => {
                try_parse_enum::<uci::SessionDataTransferStatusNtfConfig>(id, value)
            }
            uci::AppConfigTlvType::SessionTimeBase => try_parse_array::<9>(id, value),
            uci::AppConfigTlvType::CccHopModeKey => {
                if value.len() != 4 && value.len() != 16 {
                    return Err(AppConfigError::InvalidCccHopModeKeyLength {
                        actual: value.len(),
                    });
                }
                Ok(Bytes::copy_from_slice(value))
            }
            uci::AppConfigTlvType::CccUwbTime0 => try_parse_u64(id, value),
            _ => Err(AppConfigError::UnsupportedParameter(id)),
        }
    }

    /// Set the APP configuration value with the selected identifier
    /// and value. Returns `Ok` if the identifier is known and the value
    /// well formatted, `Err` otherwise.
    pub fn set(&mut self, id: uci::AppConfigTlvType, value: &[u8]) -> Result<(), AppConfigError> {
        let bytes = self
            .parse_tlv(id, value)
            .inspect_err(|err| tracing::error!("{err}"))?;
        self.tlvs.insert(id, bytes);
        Ok(())
    }

    fn get_enum<T: TryFrom<u8>>(&self, id: uci::AppConfigTlvType) -> Option<T> {
        self.get(id)
            .and_then(|v| v.first().copied())
            .and_then(|b| T::try_from(b).ok())
    }

    fn get_u8(&self, id: uci::AppConfigTlvType) -> Option<u8> {
        self.get(id).and_then(|v| v.first().copied())
    }

    fn get_u16(&self, id: uci::AppConfigTlvType) -> Option<u16> {
        self.get(id)
            .and_then(|v| v.first_chunk::<2>().copied().map(u16::from_le_bytes))
    }

    fn get_u32(&self, id: uci::AppConfigTlvType) -> Option<u32> {
        self.get(id)
            .and_then(|v| v.first_chunk::<4>().copied().map(u32::from_le_bytes))
    }

    pub fn device_type(&self) -> Option<uci::DeviceType> {
        self.get_enum(uci::AppConfigTlvType::DeviceType)
    }

    pub fn ranging_round_usage(&self) -> Option<uci::RangingRoundUsage> {
        self.get_enum(uci::AppConfigTlvType::RangingRoundUsage)
    }

    pub fn multi_node_mode(&self) -> Option<uci::MultiNodeMode> {
        self.get_enum(uci::AppConfigTlvType::MultiNodeMode)
    }

    pub fn device_mac_address(&self) -> Option<MacAddress> {
        let v = self.tlvs.get(&uci::AppConfigTlvType::DeviceMacAddress)?;
        match v.len() {
            2 => Some(MacAddress::Short(v.first_chunk::<2>().copied()?)),
            8 => Some(MacAddress::Extended(v.first_chunk::<8>().copied()?)),
            _ => None,
        }
    }

    pub fn device_role(&self) -> Option<uci::DeviceRole> {
        self.get_enum(uci::AppConfigTlvType::DeviceRole)
    }

    pub fn schedule_mode(&self) -> Option<uci::ScheduleMode> {
        self.get_enum(uci::AppConfigTlvType::ScheduleMode)
    }

    pub fn sts_config(&self) -> uci::StsConfig {
        self.get_enum(uci::AppConfigTlvType::StsConfig)
            .unwrap_or(DEFAULT_STS_CONFIG)
    }

    pub fn set_number_of_controlees(&mut self, value: u8) {
        self.tlvs.insert(
            uci::AppConfigTlvType::NumberOfControlees,
            Bytes::copy_from_slice(&[value]),
        );
    }

    pub fn dst_mac_address(&self) -> impl Iterator<Item = MacAddress> + '_ {
        let (slice, chunk_size) = match (
            self.tlvs.get(&uci::AppConfigTlvType::DstMacAddress),
            self.mac_address_mode(),
        ) {
            (Some(v), uci::MacAddressMode::Mode0) => (v.as_ref(), 2),
            (Some(v), uci::MacAddressMode::Mode2) => (v.as_ref(), 8),
            _ => (&[] as &[u8], 2),
        };
        slice
            .chunks_exact(chunk_size)
            .filter_map(|c| match c.len() {
                2 => c.first_chunk::<2>().copied().map(MacAddress::Short),
                8 => c.first_chunk::<8>().copied().map(MacAddress::Extended),
                _ => None,
            })
    }

    pub fn set_dst_mac_address(&mut self, addresses: &[MacAddress]) {
        let bytes: Bytes = addresses.iter().flat_map(Vec::<u8>::from).collect();
        self.tlvs
            .insert(uci::AppConfigTlvType::DstMacAddress, bytes);
    }

    pub fn ranging_duration(&self) -> u32 {
        self.get_u32(uci::AppConfigTlvType::RangingDuration)
            .unwrap_or(DEFAULT_RANGING_DURATION)
    }

    pub fn session_info_ntf_config(&self) -> uci::SessionInfoNtfConfig {
        self.get_enum(uci::AppConfigTlvType::SessionInfoNtfConfig)
            .unwrap_or(DEFAULT_SESSION_INFO_NTF_CONFIG)
    }

    pub fn in_band_termination_attempt_count(&self) -> u8 {
        self.get_u8(uci::AppConfigTlvType::InBandTerminationAttemptCount)
            .unwrap_or(DEFAULT_IN_BAND_TERMINATION_ATTEMPT_COUNT)
    }

    pub fn data_repetition_count(&self) -> u8 {
        self.get_u8(uci::AppConfigTlvType::DataRepetitionCount)
            .unwrap_or(DEFAULT_DATA_REPETITION_COUNT)
    }

    pub fn session_data_transfer_status_ntf_config(
        &self,
    ) -> uci::SessionDataTransferStatusNtfConfig {
        self.get_enum(uci::AppConfigTlvType::SessionDataTransferStatusNtfConfig)
            .unwrap_or(DEFAULT_SESSION_DATA_TRANSFER_STATUS_NTF_CONFIG)
    }

    fn number_of_controlees(&self) -> u8 {
        self.get_u8(uci::AppConfigTlvType::NumberOfControlees)
            .unwrap_or(DEFAULT_NUMBER_OF_CONTROLEES)
    }

    fn channel_number(&self) -> uci::ChannelNumber {
        self.get_enum(uci::AppConfigTlvType::ChannelNumber)
            .unwrap_or(DEFAULT_CHANNEL_NUMBER)
    }

    fn near_proximity_config(&self) -> u16 {
        self.get_u16(uci::AppConfigTlvType::NearProximityConfig)
            .unwrap_or(DEFAULT_NEAR_PROXIMITY_CONFIG)
    }

    fn far_proximity_config(&self) -> u16 {
        self.get_u16(uci::AppConfigTlvType::FarProximityConfig)
            .unwrap_or(DEFAULT_FAR_PROXIMITY_CONFIG)
    }

    pub fn max_number_of_measurements(&self) -> u16 {
        self.get_u16(uci::AppConfigTlvType::MaxNumberOfMeasurements)
            .unwrap_or(DEFAULT_MAX_NUMBER_OF_MEASUREMENTS)
    }

    pub fn mac_address_mode(&self) -> uci::MacAddressMode {
        self.get_enum(uci::AppConfigTlvType::MacAddressMode)
            .unwrap_or(DEFAULT_MAC_ADDRESS_MODE)
    }

    fn aoa_bound_config(&self) -> AoaBounds {
        let [
            min_az_lo,
            min_az_hi,
            max_az_lo,
            max_az_hi,
            min_el_lo,
            min_el_hi,
            max_el_lo,
            max_el_hi,
        ] = self
            .get(uci::AppConfigTlvType::AoaBoundConfig)
            .and_then(|v| v.first_chunk::<8>().copied())
            .unwrap_or(DEFAULT_AOA_BOUND_CONFIG);
        AoaBounds {
            min_azimuth: i16::from_le_bytes([min_az_lo, min_az_hi]),
            max_azimuth: i16::from_le_bytes([max_az_lo, max_az_hi]),
            min_elevation: i16::from_le_bytes([min_el_lo, min_el_hi]),
            max_elevation: i16::from_le_bytes([max_el_lo, max_el_hi]),
        }
    }

    pub fn is_within_bounds(&self, distance: u16, azimuth: i16, elevation: i16) -> bool {
        let within_proximity =
            (self.near_proximity_config()..=self.far_proximity_config()).contains(&distance);
        let within_aoa = self.aoa_bound_config().contains(azimuth, elevation);
        match self.session_info_ntf_config() {
            uci::SessionInfoNtfConfig::EnableProximityTrigger
            | uci::SessionInfoNtfConfig::EnableProximityEdgeTrigger => within_proximity,
            uci::SessionInfoNtfConfig::EnableAoaTrigger
            | uci::SessionInfoNtfConfig::EnableAoaEdgeTrigger => within_aoa,
            uci::SessionInfoNtfConfig::EnableProximityAoaTrigger
            | uci::SessionInfoNtfConfig::EnableProximityAoaEdgeTrigger => {
                within_proximity && within_aoa
            }
            uci::SessionInfoNtfConfig::Enable | uci::SessionInfoNtfConfig::Disable => true,
        }
    }

    pub fn is_compatible_for_ranging(&self, peer_config: &Self) -> bool {
        self.device_role() != peer_config.device_role()
            && self.device_type() != peer_config.device_type()
            && self.mac_address_mode() == peer_config.mac_address_mode()
            && self.channel_number() == peer_config.channel_number()
            && self
                .device_mac_address()
                .is_some_and(|mac| peer_config.dst_mac_address().any(|a| a == mac))
            && peer_config
                .device_mac_address()
                .is_some_and(|mac| self.dst_mac_address().any(|a| a == mac))
    }

    /// [UCI] 7.5.1 Configuration of a Session
    /// This section defines the mandatory APP Configuration Parameters to be applied
    /// by the Host for FiRa defined UWB Session types. The Host shall apply these
    /// mandatory configurations to move the Session State from SESSION_STATE_INIT
    /// to SESSION_STATE_IDLE.
    ///
    /// - DEVICE_ROLE
    /// - MULTI_NODE_MODE
    /// - RANGING_ROUND_USAGE
    /// - DEVICE_MAC_ADDRESS
    /// - DEVICE_TYPE (see Note1)
    /// - SCHEDULE_MODE
    ///
    /// For CCC and Aliro sessions (`SessionType::Ccc | SessionType::Aliro`), only
    /// `DEVICE_ROLE` and `MULTI_NODE_MODE` are required.
    pub fn has_mandatory_parameters(&self) -> bool {
        match self.session_type {
            uci::SessionType::FiraRangingSession
            | uci::SessionType::FiraRangingAndInBandDataSession
            | uci::SessionType::FiraDataTransferSession
            | uci::SessionType::FiraRangingOnlyPhase
            | uci::SessionType::FiraInBandDataPhase
            | uci::SessionType::FiraRangingWithDataPhase => {
                self.device_role().is_some()
                    && self.multi_node_mode().is_some()
                    && self.ranging_round_usage().is_some()
                    && self.device_mac_address().is_some()
                    && self.schedule_mode().is_some()
            }
            uci::SessionType::Ccc | uci::SessionType::Aliro => {
                self.device_role().is_some() && self.multi_node_mode().is_some()
            }
            uci::SessionType::DeviceTestMode => true,
        }
    }

    pub fn can_start_data_transfer(&self) -> bool {
        self.device_role() == Some(uci::DeviceRole::Initiator)
    }

    pub fn can_receive_data_transfer(&self) -> bool {
        self.device_role() == Some(uci::DeviceRole::Responder)
    }
}
