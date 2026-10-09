use std::collections::HashMap;

use bytes::Bytes;

use crate::MacAddress;
use crate::packets::uci;

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
const DEFAULT_NUMBER_OF_CONTROLEES: u8 = 1;
const DEFAULT_RANGING_DURATION: u32 = 200;
const DEFAULT_SESSION_INFO_NTF_CONFIG: uci::SessionInfoNtfConfig =
    uci::SessionInfoNtfConfig::Enable;
const DEFAULT_MAC_ADDRESS_MODE: uci::MacAddressMode = uci::MacAddressMode::Mode0;
const DEFAULT_IN_BAND_TERMINATION_ATTEMPT_COUNT: u8 = 1;
const DEFAULT_SESSION_DATA_TRANSFER_STATUS_NTF_CONFIG: uci::SessionDataTransferStatusNtfConfig =
    uci::SessionDataTransferStatusNtfConfig::Disable;

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
    tlvs: HashMap<uci::AppConfigTlvType, Bytes>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tlvs: HashMap::from([
                (
                    uci::AppConfigTlvType::StsConfig,
                    Bytes::from_static(&[DEFAULT_STS_CONFIG as u8]),
                ),
                (
                    uci::AppConfigTlvType::ChannelNumber,
                    Bytes::from_static(&[uci::ChannelNumber::ChannelNumber9 as u8]),
                ),
                (
                    uci::AppConfigTlvType::NumberOfControlees,
                    Bytes::from_static(&[DEFAULT_NUMBER_OF_CONTROLEES]),
                ),
                (uci::AppConfigTlvType::DstMacAddress, Bytes::new()),
                (
                    uci::AppConfigTlvType::SlotDuration,
                    Bytes::from_static(const { &2400u16.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::RangingDuration,
                    Bytes::from_static(const { &DEFAULT_RANGING_DURATION.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::StsIndex,
                    Bytes::from_static(const { &0u32.to_le_bytes() }),
                ),
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
                    uci::AppConfigTlvType::SessionInfoNtfConfig,
                    Bytes::from_static(&[DEFAULT_SESSION_INFO_NTF_CONFIG as u8]),
                ),
                (
                    uci::AppConfigTlvType::NearProximityConfig,
                    Bytes::from_static(const { &0u16.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::FarProximityConfig,
                    Bytes::from_static(const { &20000u16.to_le_bytes() }),
                ),
                (
                    uci::AppConfigTlvType::RframeConfig,
                    Bytes::from_static(&[uci::RframeConfig::Sp3 as u8]),
                ),
                (
                    uci::AppConfigTlvType::RssiReporting,
                    Bytes::from_static(&[uci::RssiReporting::Disable as u8]),
                ),
                (
                    uci::AppConfigTlvType::PreambleCodeIndex,
                    Bytes::from_static(&[10]),
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
                    Bytes::from_static(&[0]),
                ),
                (
                    uci::AppConfigTlvType::RangingTimeStruct,
                    Bytes::from_static(&[uci::RangingTimeStruct::BlockBasedScheduling as u8]),
                ),
                (uci::AppConfigTlvType::SlotsPerRr, Bytes::from_static(&[25])),
                (
                    uci::AppConfigTlvType::AoaBoundConfig,
                    Bytes::from_static(&[0u8; 8]),
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
                    uci::AppConfigTlvType::UwbInitiationTime,
                    Bytes::from_static(&[0u8; 8]),
                ),
                (
                    uci::AppConfigTlvType::HoppingMode,
                    Bytes::from_static(&[uci::HoppingMode::Disable as u8]),
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
                    Bytes::from_static(const { &0u16.to_le_bytes() }),
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
        }
    }
}

impl AppConfig {
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

    fn validate(&self, id: uci::AppConfigTlvType, value: &[u8]) -> anyhow::Result<()> {
        fn try_parse<T: TryFrom<u8, Error = u8>>(value: &[u8]) -> anyhow::Result<T> {
            T::try_from(u8::from_le_bytes(value.try_into()?)).map_err(anyhow::Error::msg)
        }

        fn try_parse_u8(value: &[u8]) -> anyhow::Result<u8> {
            Ok(u8::from_le_bytes(value.try_into()?))
        }

        fn try_parse_u16(value: &[u8]) -> anyhow::Result<u16> {
            Ok(u16::from_le_bytes(value.try_into()?))
        }

        fn try_parse_u32(value: &[u8]) -> anyhow::Result<u32> {
            Ok(u32::from_le_bytes(value.try_into()?))
        }

        fn try_parse_u64(value: &[u8]) -> anyhow::Result<u64> {
            Ok(u64::from_le_bytes(value.try_into()?))
        }

        match id {
            uci::AppConfigTlvType::DeviceType => {
                let _ = try_parse::<uci::DeviceType>(value)?;
            }
            uci::AppConfigTlvType::RangingRoundUsage => {
                let _ = try_parse::<uci::RangingRoundUsage>(value)?;
            }
            uci::AppConfigTlvType::StsConfig => {
                let _ = try_parse::<uci::StsConfig>(value)?;
            }
            uci::AppConfigTlvType::MultiNodeMode => {
                let _ = try_parse::<uci::MultiNodeMode>(value)?;
            }
            uci::AppConfigTlvType::ChannelNumber => {
                let _ = try_parse::<uci::ChannelNumber>(value)?;
            }
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
            | uci::AppConfigTlvType::MinFramesPerRr
            | uci::AppConfigTlvType::InterFrameInterval
            | uci::AppConfigTlvType::ApplicationDataEndpoint => {
                let _ = try_parse_u8(value)?;
            }
            uci::AppConfigTlvType::DeviceMacAddress => match self.mac_address_mode() {
                uci::MacAddressMode::Mode0 => {
                    let _: [u8; 2] = value.try_into()?;
                }
                uci::MacAddressMode::Mode1 => {
                    anyhow::bail!("mac_address_mode Mode1 is not supported")
                }
                uci::MacAddressMode::Mode2 => {
                    let _: [u8; 8] = value.try_into()?;
                }
            },
            uci::AppConfigTlvType::DstMacAddress => {
                let mac_address_size = match self.mac_address_mode() {
                    uci::MacAddressMode::Mode0 => 2,
                    uci::MacAddressMode::Mode1 => {
                        anyhow::bail!("mac_address_mode Mode1 is not supported")
                    }
                    uci::MacAddressMode::Mode2 => 8,
                };
                if value.len() != self.number_of_controlees() as usize * mac_address_size {
                    let n = self.number_of_controlees();
                    let len = value.len();
                    log::error!(
                        "invalid dst_mac_address len: expected {n}x{mac_address_size}, got {len}"
                    );
                    anyhow::bail!("invalid dst_mac_address len")
                }
            }
            uci::AppConfigTlvType::SlotDuration
            | uci::AppConfigTlvType::NearProximityConfig
            | uci::AppConfigTlvType::FarProximityConfig
            | uci::AppConfigTlvType::VendorId
            | uci::AppConfigTlvType::MaxRrRetry
            | uci::AppConfigTlvType::MaxNumberOfMeasurements
            | uci::AppConfigTlvType::MtuSize => {
                let _ = try_parse_u16(value)?;
            }
            uci::AppConfigTlvType::RangingDuration
            | uci::AppConfigTlvType::StsIndex
            | uci::AppConfigTlvType::SubSessionId => {
                let _ = try_parse_u32(value)?;
            }
            uci::AppConfigTlvType::MacFcsType => {
                let _ = try_parse::<uci::MacFcsType>(value)?;
            }
            uci::AppConfigTlvType::AoaResultReq => {
                let _ = try_parse::<uci::AoaResultReq>(value)?;
            }
            uci::AppConfigTlvType::SessionInfoNtfConfig => {
                let _ = try_parse::<uci::SessionInfoNtfConfig>(value)?;
            }
            uci::AppConfigTlvType::DeviceRole => {
                let _ = try_parse::<uci::DeviceRole>(value)?;
            }
            uci::AppConfigTlvType::RframeConfig => {
                let _ = try_parse::<uci::RframeConfig>(value)?;
            }
            uci::AppConfigTlvType::RssiReporting => {
                let _ = try_parse::<uci::RssiReporting>(value)?;
            }
            uci::AppConfigTlvType::PsduDataRate => {
                let _ = try_parse::<uci::PsduDataRate>(value)?;
            }
            uci::AppConfigTlvType::PreambleDuration => {
                let _ = try_parse::<uci::PreambleDuration>(value)?;
            }
            uci::AppConfigTlvType::LinkLayerMode => {
                let _ = try_parse::<uci::LinkLayerMode>(value)?;
            }
            uci::AppConfigTlvType::RangingTimeStruct => {
                let _ = try_parse::<uci::RangingTimeStruct>(value)?;
            }
            uci::AppConfigTlvType::AoaBoundConfig => {
                if value.len() != 8 {
                    let len = value.len();
                    log::error!("invalid aoa_bound_config len: expected 8, got {len}");
                    anyhow::bail!("invalid aoa_bound_config len")
                }
            }
            uci::AppConfigTlvType::PrfMode => {
                let _ = try_parse::<uci::PrfMode>(value)?;
            }
            uci::AppConfigTlvType::CapSizeRange => {
                let _: [u8; 2] = value.try_into()?;
            }
            uci::AppConfigTlvType::ScheduleMode => {
                let _ = try_parse::<uci::ScheduleMode>(value)?;
            }
            uci::AppConfigTlvType::KeyRotation => {
                let _ = try_parse::<uci::KeyRotation>(value)?;
            }
            uci::AppConfigTlvType::MacAddressMode => {
                let mode = try_parse::<uci::MacAddressMode>(value)?;
                if mode == uci::MacAddressMode::Mode1 {
                    anyhow::bail!("mac_address_mode Mode1 is not supported");
                }
            }
            uci::AppConfigTlvType::StaticStsIv => {
                let _: [u8; 6] = value.try_into()?;
            }
            uci::AppConfigTlvType::UwbInitiationTime => match value.len() {
                4 => {
                    let _ = try_parse_u32(value)?;
                }
                _ => {
                    let _ = try_parse_u64(value)?;
                }
            },
            uci::AppConfigTlvType::HoppingMode => {
                let _ = try_parse::<uci::HoppingMode>(value)?;
            }
            uci::AppConfigTlvType::BprfPhrDataRate => {
                let _ = try_parse::<uci::BprfPhrDataRate>(value)?;
            }
            uci::AppConfigTlvType::StsLength => {
                let _ = try_parse::<uci::StsLength>(value)?;
            }
            uci::AppConfigTlvType::SessionKey => {}
            uci::AppConfigTlvType::SubSessionKey => {
                if value.len() != 16 && value.len() != 32 {
                    let len = value.len();
                    anyhow::bail!("invalid sub-session key size {len}");
                }
            }
            uci::AppConfigTlvType::SessionDataTransferStatusNtfConfig => {
                let _ = try_parse::<uci::SessionDataTransferStatusNtfConfig>(value)?;
            }
            uci::AppConfigTlvType::SessionTimeBase => {
                let _: [u8; 9] = value.try_into()?;
            }
            uci::AppConfigTlvType::CccHopModeKey
            | uci::AppConfigTlvType::CccUwbTime0
            | uci::AppConfigTlvType::CccRangingProtocolVer
            | uci::AppConfigTlvType::CccUwbConfigId
            | uci::AppConfigTlvType::CccPulseshapeCombo
            | uci::AppConfigTlvType::CccUrskTtl
            | uci::AppConfigTlvType::CccLastIndexUsed
            | uci::AppConfigTlvType::NbOfRangeMeasurements
            | uci::AppConfigTlvType::NbOfAzimuthMeasurements
            | uci::AppConfigTlvType::NbOfElevationMeasurements
            | uci::AppConfigTlvType::EnableDiagnostics
            | uci::AppConfigTlvType::DiagramsFrameReportsFields => {
                log::error!("unsupported vendor config type {id:?}");
                anyhow::bail!("unsupported vendor config type {id:?}")
            }
            _ => {
                log::error!("unsupported app config type {id:?}");
                anyhow::bail!("unsupported app config type {id:?}")
            }
        }

        Ok(())
    }

    /// Set the APP configuration value with the selected identifier
    /// and value. Returns `Ok` if the identifier is known and the value
    /// well formatted, `Err` otherwise.
    pub fn set(&mut self, id: uci::AppConfigTlvType, value: &[u8]) -> anyhow::Result<()> {
        self.validate(id, value)?;

        // Implement backward compatiblity for UCI 1.0
        // where the value is 4 bytes instead of 8.
        let bytes = match (id, value) {
            (uci::AppConfigTlvType::UwbInitiationTime, &[a, b, c, d]) => {
                Bytes::copy_from_slice(&(u32::from_le_bytes([a, b, c, d]) as u64).to_le_bytes())
            }
            _ => Bytes::copy_from_slice(value),
        };

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

    fn get_u32(&self, id: uci::AppConfigTlvType) -> Option<u32> {
        self.get(id)
            .and_then(|v| v.as_ref().try_into().ok().map(u32::from_le_bytes))
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
            2 => Some(MacAddress::Short(v.as_ref().try_into().ok()?)),
            8 => Some(MacAddress::Extended(v.as_ref().try_into().ok()?)),
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
            _ => (&[][..], 2),
        };
        slice
            .chunks_exact(chunk_size)
            .filter_map(|c| match c.len() {
                2 => c.try_into().ok().map(MacAddress::Short),
                8 => c.try_into().ok().map(MacAddress::Extended),
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

    pub fn mac_address_mode(&self) -> uci::MacAddressMode {
        self.get_enum(uci::AppConfigTlvType::MacAddressMode)
            .unwrap_or(DEFAULT_MAC_ADDRESS_MODE)
    }

    pub fn is_compatible_for_ranging(&self, peer_config: &Self) -> bool {
        self.device_role() != peer_config.device_role()
            && self.device_type() != peer_config.device_type()
            && self.mac_address_mode() == peer_config.mac_address_mode()
            && self
                .device_mac_address()
                .is_some_and(|mac| peer_config.dst_mac_address().any(|a| a == mac))
            && peer_config
                .device_mac_address()
                .is_some_and(|mac| self.dst_mac_address().any(|a| a == mac))
    }

    pub fn can_start_data_transfer(&self) -> bool {
        self.device_role() == Some(uci::DeviceRole::Initiator)
    }

    pub fn can_receive_data_transfer(&self) -> bool {
        self.device_role() == Some(uci::DeviceRole::Responder)
    }
}
