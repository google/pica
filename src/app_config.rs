use crate::packets::uci;
use crate::MacAddress;

/// [UCI] 8.3 Application Configuration Parameters.
/// Sub-session Key provided for Provisioned STS for Responder specific Key mode
/// (STS_CONFIG equal to 0x04).
#[derive(Clone, PartialEq, Eq)]
pub enum SubSessionKey {
    None,
    Short([u8; 16]),
    Extended([u8; 32]),
}

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
    pub device_type: Option<uci::DeviceType>,
    pub ranging_round_usage: Option<uci::RangingRoundUsage>,
    pub sts_config: uci::StsConfig,
    pub multi_node_mode: Option<uci::MultiNodeMode>,
    channel_number: uci::ChannelNumber,
    /// Number of Controlees(N) 1<=N<=8 (Default is 1)
    pub number_of_controlees: u8,
    /// MAC Address of the UWBS itself participating in UWB session.
    /// The short address (2 bytes) or extended MAC address (8 bytes)
    /// shall be indicated via MAC_ADDRESS_MODE config.
    pub device_mac_address: Option<MacAddress>,
    /// MAC Address list(N) for NUMBER_OF_CONTROLEES
    /// devices participating in UWB Session.
    ///
    /// The size of this list shall be:
    /// - equal to 1 when MULTI_NODE_MODE is set 0x00 (O2O).
    /// - ranging from 1 to 8 when MULTI_NODE_MODE is set to 0x01 (O2M).
    pub dst_mac_address: Vec<MacAddress>,
    slot_duration: u16,
    pub ranging_duration: u32,
    sts_index: u32,
    mac_fcs_type: uci::MacFcsType,
    ranging_round_control: u8,
    aoa_result_req: uci::AoaResultReq,
    pub session_info_ntf_config: uci::SessionInfoNtfConfig,
    near_proximity_config: u16,
    far_proximity_config: u16,
    pub device_role: Option<uci::DeviceRole>,
    rframe_config: uci::RframeConfig,
    rssi_reporting: uci::RssiReporting,
    preamble_code_index: u8,
    sfd_id: u8,
    psdu_data_rate: uci::PsduDataRate,
    preamble_duration: uci::PreambleDuration,
    link_layer_mode: uci::LinkLayerMode,
    data_repetition_count: u8,
    ranging_time_struct: uci::RangingTimeStruct,
    slots_per_rr: u8,
    aoa_bound_config: [u16; 4],
    prf_mode: uci::PrfMode,
    cap_size_range: [u8; 2],
    tx_jitter_window_size: u8,
    pub schedule_mode: Option<uci::ScheduleMode>,
    key_rotation: uci::KeyRotation,
    key_rotation_rate: u8,
    session_priority: u8,
    pub mac_address_mode: uci::MacAddressMode,
    vendor_id: u16,
    static_sts_iv: [u8; 6],
    number_of_sts_segments: u8,
    max_rr_retry: u16,
    uwb_initiation_time: u64,
    hopping_mode: uci::HoppingMode,
    block_stride_length: u8,
    result_report_config: u8,
    pub in_band_termination_attempt_count: u8,
    sub_session_id: u32,
    bprf_phr_data_rate: uci::BprfPhrDataRate,
    max_number_of_measurements: u16,
    sts_length: uci::StsLength,
    min_frames_per_rr: u8,
    mtu_size: u16,
    inter_frame_interval: u8,
    session_key: Vec<u8>,
    sub_session_key: SubSessionKey,
    pub session_data_transfer_status_ntf_config: uci::SessionDataTransferStatusNtfConfig,
    session_time_base: [u8; 9],
    application_data_endpoint: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            device_type: None,
            ranging_round_usage: None,
            sts_config: uci::StsConfig::Static,
            multi_node_mode: None,
            channel_number: uci::ChannelNumber::ChannelNumber9,
            number_of_controlees: 1,
            device_mac_address: None,
            dst_mac_address: vec![],
            slot_duration: 2400,
            ranging_duration: 200,
            sts_index: 0,
            mac_fcs_type: uci::MacFcsType::Crc16,
            // The default is 0x03 when Time Scheduled Ranging is used,
            // 0x06 when Contention-based Ranging is used.
            ranging_round_control: 0x06,
            aoa_result_req: uci::AoaResultReq::AoaEnabled,
            session_info_ntf_config: uci::SessionInfoNtfConfig::Enable,
            near_proximity_config: 0,
            far_proximity_config: 20000,
            device_role: None,
            rframe_config: uci::RframeConfig::Sp3,
            rssi_reporting: uci::RssiReporting::Disable,
            preamble_code_index: 10,
            sfd_id: 2,
            psdu_data_rate: uci::PsduDataRate::DataRate6m81,
            preamble_duration: uci::PreambleDuration::Duration64Symbols,
            link_layer_mode: uci::LinkLayerMode::BypassMode,
            data_repetition_count: 0,
            ranging_time_struct: uci::RangingTimeStruct::BlockBasedScheduling,
            slots_per_rr: 25,
            aoa_bound_config: [0; 4],
            prf_mode: uci::PrfMode::BprfMode,
            // Default for Octet[0] is SLOTS_PER_RR - 1
            cap_size_range: [24, 5],
            tx_jitter_window_size: 0,
            schedule_mode: None,
            key_rotation: uci::KeyRotation::Disable,
            key_rotation_rate: 0,
            session_priority: 50,
            mac_address_mode: uci::MacAddressMode::Mode0,
            vendor_id: 0,
            static_sts_iv: [0; 6],
            number_of_sts_segments: 1,
            max_rr_retry: 0,
            uwb_initiation_time: 0,
            hopping_mode: uci::HoppingMode::Disable,
            block_stride_length: 0,
            result_report_config: 0x01,
            in_band_termination_attempt_count: 1,
            sub_session_id: 0, // XX
            bprf_phr_data_rate: uci::BprfPhrDataRate::DataRate850k,
            max_number_of_measurements: 0,
            sts_length: uci::StsLength::Length64Symbols,
            min_frames_per_rr: 4,
            mtu_size: 0, // XX
            inter_frame_interval: 1,
            session_key: vec![],
            sub_session_key: SubSessionKey::None,
            session_data_transfer_status_ntf_config:
                uci::SessionDataTransferStatusNtfConfig::Disable,
            session_time_base: [0; 9],
            application_data_endpoint: 0,
        }
    }
}

impl AppConfig {
    /// Set the APP configuration value with the selected identifier
    /// and value. Returns `Ok` if the identifier is known and the value
    /// well formatted, `Err` otherwise.
    pub fn set(&mut self, id: uci::AppConfigTlvType, value: &[u8]) -> anyhow::Result<()> {
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
            uci::AppConfigTlvType::DeviceType => self.device_type = Some(try_parse(value)?),
            uci::AppConfigTlvType::RangingRoundUsage => {
                self.ranging_round_usage = Some(try_parse(value)?)
            }
            uci::AppConfigTlvType::StsConfig => self.sts_config = try_parse(value)?,
            uci::AppConfigTlvType::MultiNodeMode => self.multi_node_mode = Some(try_parse(value)?),
            uci::AppConfigTlvType::ChannelNumber => self.channel_number = try_parse(value)?,
            uci::AppConfigTlvType::NumberOfControlees => {
                self.number_of_controlees = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::DeviceMacAddress => {
                self.device_mac_address = Some(match self.mac_address_mode {
                    uci::MacAddressMode::Mode0 => MacAddress::Short(value.try_into()?),
                    uci::MacAddressMode::Mode1 => unimplemented!(),
                    uci::MacAddressMode::Mode2 => MacAddress::Extended(value.try_into()?),
                })
            }
            uci::AppConfigTlvType::DstMacAddress => {
                let mac_address_size = match self.mac_address_mode {
                    uci::MacAddressMode::Mode0 => 2,
                    uci::MacAddressMode::Mode1 => unimplemented!(),
                    uci::MacAddressMode::Mode2 => 8,
                };
                if value.len() != self.number_of_controlees as usize * mac_address_size {
                    log::error!(
                        "invalid dst_mac_address len: expected {}x{}, got {}",
                        self.number_of_controlees,
                        mac_address_size,
                        value.len()
                    );
                    anyhow::bail!("invalid dst_mac_address len")
                }
                self.dst_mac_address = value
                    .chunks(mac_address_size)
                    .map(|value| match self.mac_address_mode {
                        uci::MacAddressMode::Mode0 => MacAddress::Short(value.try_into().unwrap()),
                        uci::MacAddressMode::Mode1 => unimplemented!(),
                        uci::MacAddressMode::Mode2 => {
                            MacAddress::Extended(value.try_into().unwrap())
                        }
                    })
                    .collect();
            }
            uci::AppConfigTlvType::SlotDuration => self.slot_duration = try_parse_u16(value)?,
            uci::AppConfigTlvType::RangingDuration => self.ranging_duration = try_parse_u32(value)?,
            uci::AppConfigTlvType::StsIndex => self.sts_index = try_parse_u32(value)?,
            uci::AppConfigTlvType::MacFcsType => self.mac_fcs_type = try_parse(value)?,
            uci::AppConfigTlvType::RangingRoundControl => {
                self.ranging_round_control = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::AoaResultReq => self.aoa_result_req = try_parse(value)?,
            uci::AppConfigTlvType::SessionInfoNtfConfig => {
                self.session_info_ntf_config = try_parse(value)?
            }
            uci::AppConfigTlvType::NearProximityConfig => {
                self.near_proximity_config = try_parse_u16(value)?
            }
            uci::AppConfigTlvType::FarProximityConfig => {
                self.far_proximity_config = try_parse_u16(value)?
            }
            uci::AppConfigTlvType::DeviceRole => self.device_role = Some(try_parse(value)?),
            uci::AppConfigTlvType::RframeConfig => self.rframe_config = try_parse(value)?,
            uci::AppConfigTlvType::RssiReporting => self.rssi_reporting = try_parse(value)?,
            uci::AppConfigTlvType::PreambleCodeIndex => {
                self.preamble_code_index = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::SfdId => self.sfd_id = try_parse_u8(value)?,
            uci::AppConfigTlvType::PsduDataRate => self.psdu_data_rate = try_parse(value)?,
            uci::AppConfigTlvType::PreambleDuration => self.preamble_duration = try_parse(value)?,
            uci::AppConfigTlvType::LinkLayerMode => self.link_layer_mode = try_parse(value)?,
            uci::AppConfigTlvType::DataRepetitionCount => {
                self.data_repetition_count = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::RangingTimeStruct => {
                self.ranging_time_struct = try_parse(value)?
            }
            uci::AppConfigTlvType::SlotsPerRr => self.slots_per_rr = try_parse_u8(value)?,
            uci::AppConfigTlvType::AoaBoundConfig => {
                if value.len() != 8 {
                    log::error!(
                        "invalid aoa_bound_config len: expected 8, got {}",
                        value.len()
                    );
                    anyhow::bail!("invalid aoa_bound_config len")
                }
                self.aoa_bound_config = [
                    u16::from_le_bytes([value[0], value[1]]),
                    u16::from_le_bytes([value[2], value[3]]),
                    u16::from_le_bytes([value[4], value[5]]),
                    u16::from_le_bytes([value[6], value[7]]),
                ]
            }
            uci::AppConfigTlvType::PrfMode => self.prf_mode = try_parse(value)?,
            uci::AppConfigTlvType::CapSizeRange => self.cap_size_range = value.try_into()?,
            uci::AppConfigTlvType::TxJitterWindowSize => {
                self.tx_jitter_window_size = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::ScheduleMode => self.schedule_mode = Some(try_parse(value)?),
            uci::AppConfigTlvType::KeyRotation => self.key_rotation = try_parse(value)?,
            uci::AppConfigTlvType::KeyRotationRate => self.key_rotation_rate = try_parse_u8(value)?,
            uci::AppConfigTlvType::SessionPriority => self.session_priority = try_parse_u8(value)?,
            uci::AppConfigTlvType::MacAddressMode => self.mac_address_mode = try_parse(value)?,
            uci::AppConfigTlvType::VendorId => self.vendor_id = try_parse_u16(value)?,
            uci::AppConfigTlvType::StaticStsIv => self.static_sts_iv = value.try_into()?,
            uci::AppConfigTlvType::NumberOfStsSegments => {
                self.number_of_sts_segments = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::MaxRrRetry => self.max_rr_retry = try_parse_u16(value)?,
            uci::AppConfigTlvType::UwbInitiationTime => {
                // Implement backward compatiblity for UCI 1.0
                // where the value is 4 bytes instead of 8.
                self.uwb_initiation_time = match value.len() {
                    4 => try_parse_u32(value)? as u64,
                    _ => try_parse_u64(value)?,
                }
            }
            uci::AppConfigTlvType::HoppingMode => self.hopping_mode = try_parse(value)?,
            uci::AppConfigTlvType::BlockStrideLength => {
                self.block_stride_length = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::ResultReportConfig => {
                self.result_report_config = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::InBandTerminationAttemptCount => {
                self.in_band_termination_attempt_count = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::SubSessionId => self.sub_session_id = try_parse_u32(value)?,
            uci::AppConfigTlvType::BprfPhrDataRate => self.bprf_phr_data_rate = try_parse(value)?,
            uci::AppConfigTlvType::MaxNumberOfMeasurements => {
                self.max_number_of_measurements = try_parse_u16(value)?
            }
            uci::AppConfigTlvType::StsLength => self.sts_length = try_parse(value)?,
            uci::AppConfigTlvType::MinFramesPerRr => self.min_frames_per_rr = try_parse_u8(value)?,
            uci::AppConfigTlvType::MtuSize => self.mtu_size = try_parse_u16(value)?,
            uci::AppConfigTlvType::InterFrameInterval => {
                self.inter_frame_interval = try_parse_u8(value)?
            }
            uci::AppConfigTlvType::SessionKey => self.session_key = value.to_vec(),
            uci::AppConfigTlvType::SubSessionKey => {
                self.sub_session_key = match value.len() {
                    16 => SubSessionKey::Short(value.try_into().unwrap()),
                    32 => SubSessionKey::Extended(value.try_into().unwrap()),
                    _ => anyhow::bail!("invalid sub-session key size {}", value.len()),
                }
            }
            uci::AppConfigTlvType::SessionDataTransferStatusNtfConfig => {
                self.session_data_transfer_status_ntf_config = try_parse(value)?
            }
            uci::AppConfigTlvType::SessionTimeBase => self.session_time_base = value.try_into()?,
            uci::AppConfigTlvType::ApplicationDataEndpoint => {
                self.application_data_endpoint = try_parse_u8(value)?
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
                log::error!("unsupported vendor config type {:?}", id);
                anyhow::bail!("unsupported vendor config type {:?}", id)
            }
            _ => {
                log::error!("unsupported app config type {:?}", id);
                anyhow::bail!("unsupported app config type {:?}", id)
            }
        }
        Ok(())
    }

    /// Retrieve the APP configuration value with the selected identifier
    /// Returns `Ok` if the identifier is known, `Err` otherwise.
    pub fn get(&self, id: uci::AppConfigTlvType) -> anyhow::Result<Vec<u8>> {
        match id {
            uci::AppConfigTlvType::DeviceType => Ok(vec![self
                .device_type
                .ok_or(anyhow::anyhow!("optional app config not set"))?
                .into()]),
            uci::AppConfigTlvType::RangingRoundUsage => Ok(vec![self
                .ranging_round_usage
                .ok_or(anyhow::anyhow!("optional app config not set"))?
                .into()]),
            uci::AppConfigTlvType::StsConfig => Ok(vec![self.sts_config.into()]),
            uci::AppConfigTlvType::MultiNodeMode => Ok(vec![self
                .multi_node_mode
                .ok_or(anyhow::anyhow!("optional app config not set"))?
                .into()]),
            uci::AppConfigTlvType::ChannelNumber => Ok(vec![self.channel_number.into()]),
            uci::AppConfigTlvType::NumberOfControlees => Ok(vec![self.number_of_controlees]),
            uci::AppConfigTlvType::DeviceMacAddress => Ok(self
                .device_mac_address
                .ok_or(anyhow::anyhow!("optional app config not set"))?
                .into()),
            uci::AppConfigTlvType::DstMacAddress => Ok(self
                .dst_mac_address
                .iter()
                .flat_map(Vec::<u8>::from)
                .collect()),
            uci::AppConfigTlvType::SlotDuration => Ok(self.slot_duration.to_le_bytes().to_vec()),
            uci::AppConfigTlvType::RangingDuration => {
                Ok(self.ranging_duration.to_le_bytes().to_vec())
            }
            uci::AppConfigTlvType::StsIndex => Ok(self.sts_index.to_le_bytes().to_vec()),
            uci::AppConfigTlvType::MacFcsType => Ok(vec![self.mac_fcs_type.into()]),
            uci::AppConfigTlvType::RangingRoundControl => Ok(vec![self.ranging_round_control]),
            uci::AppConfigTlvType::AoaResultReq => Ok(vec![self.aoa_result_req.into()]),
            uci::AppConfigTlvType::SessionInfoNtfConfig => {
                Ok(vec![self.session_info_ntf_config.into()])
            }
            uci::AppConfigTlvType::NearProximityConfig => {
                Ok(self.near_proximity_config.to_le_bytes().to_vec())
            }
            uci::AppConfigTlvType::FarProximityConfig => {
                Ok(self.far_proximity_config.to_le_bytes().to_vec())
            }
            uci::AppConfigTlvType::DeviceRole => Ok(vec![self
                .device_role
                .ok_or(anyhow::anyhow!("optional app config not set"))?
                .into()]),
            uci::AppConfigTlvType::RframeConfig => Ok(vec![self.rframe_config.into()]),
            uci::AppConfigTlvType::RssiReporting => Ok(vec![self.rssi_reporting.into()]),
            uci::AppConfigTlvType::PreambleCodeIndex => Ok(vec![self.preamble_code_index]),
            uci::AppConfigTlvType::SfdId => Ok(vec![self.sfd_id]),
            uci::AppConfigTlvType::PsduDataRate => Ok(vec![self.psdu_data_rate.into()]),
            uci::AppConfigTlvType::PreambleDuration => Ok(vec![self.preamble_duration.into()]),
            uci::AppConfigTlvType::LinkLayerMode => Ok(vec![self.link_layer_mode.into()]),
            uci::AppConfigTlvType::DataRepetitionCount => Ok(vec![self.data_repetition_count]),
            uci::AppConfigTlvType::RangingTimeStruct => Ok(vec![self.ranging_time_struct.into()]),
            uci::AppConfigTlvType::SlotsPerRr => Ok(vec![self.slots_per_rr]),
            uci::AppConfigTlvType::AoaBoundConfig => Ok(self
                .aoa_bound_config
                .iter()
                .copied()
                .flat_map(u16::to_le_bytes)
                .collect()),
            uci::AppConfigTlvType::PrfMode => Ok(vec![self.prf_mode.into()]),
            uci::AppConfigTlvType::CapSizeRange => Ok(self.cap_size_range.to_vec()),
            uci::AppConfigTlvType::TxJitterWindowSize => Ok(vec![self.tx_jitter_window_size]),
            uci::AppConfigTlvType::ScheduleMode => Ok(vec![self
                .schedule_mode
                .ok_or(anyhow::anyhow!("optional app config not set"))?
                .into()]),
            uci::AppConfigTlvType::KeyRotation => Ok(vec![self.key_rotation.into()]),
            uci::AppConfigTlvType::KeyRotationRate => Ok(vec![self.key_rotation_rate]),
            uci::AppConfigTlvType::SessionPriority => Ok(vec![self.session_priority]),
            uci::AppConfigTlvType::MacAddressMode => Ok(vec![self.mac_address_mode.into()]),
            uci::AppConfigTlvType::VendorId => Ok(self.vendor_id.to_le_bytes().to_vec()),
            uci::AppConfigTlvType::StaticStsIv => Ok(self.static_sts_iv.to_vec()),
            uci::AppConfigTlvType::NumberOfStsSegments => Ok(vec![self.number_of_sts_segments]),
            uci::AppConfigTlvType::MaxRrRetry => Ok(self.max_rr_retry.to_le_bytes().to_vec()),
            uci::AppConfigTlvType::UwbInitiationTime => {
                Ok(self.uwb_initiation_time.to_le_bytes().to_vec())
            }
            uci::AppConfigTlvType::HoppingMode => Ok(vec![self.hopping_mode.into()]),
            uci::AppConfigTlvType::BlockStrideLength => Ok(vec![self.block_stride_length]),
            uci::AppConfigTlvType::ResultReportConfig => Ok(vec![self.result_report_config]),
            uci::AppConfigTlvType::InBandTerminationAttemptCount => {
                Ok(vec![self.in_band_termination_attempt_count])
            }
            uci::AppConfigTlvType::SubSessionId => Ok(self.sub_session_id.to_le_bytes().to_vec()),
            uci::AppConfigTlvType::BprfPhrDataRate => Ok(vec![self.bprf_phr_data_rate.into()]),
            uci::AppConfigTlvType::MaxNumberOfMeasurements => {
                Ok(self.max_number_of_measurements.to_le_bytes().to_vec())
            }
            uci::AppConfigTlvType::StsLength => Ok(vec![self.sts_length.into()]),
            uci::AppConfigTlvType::MinFramesPerRr => Ok(vec![self.min_frames_per_rr]),
            uci::AppConfigTlvType::MtuSize => Ok(self.mtu_size.to_le_bytes().to_vec()),
            uci::AppConfigTlvType::InterFrameInterval => Ok(vec![self.inter_frame_interval]),
            uci::AppConfigTlvType::SessionKey => Ok(self.session_key.clone()),
            uci::AppConfigTlvType::SubSessionKey => Ok(match self.sub_session_key {
                SubSessionKey::None => vec![],
                SubSessionKey::Short(key) => key.to_vec(),
                SubSessionKey::Extended(key) => key.to_vec(),
            }),
            uci::AppConfigTlvType::SessionDataTransferStatusNtfConfig => {
                Ok(vec![self.session_data_transfer_status_ntf_config.into()])
            }
            uci::AppConfigTlvType::SessionTimeBase => Ok(self.session_time_base.to_vec()),
            uci::AppConfigTlvType::ApplicationDataEndpoint => {
                Ok(vec![self.application_data_endpoint])
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
                log::error!("unsupported vendor config type {:?}", id);
                anyhow::bail!("unsupported vendor config type {:?}", id)
            }
            _ => {
                log::error!("unsupported app config type {:?}", id);
                anyhow::bail!("unsupported app config type {:?}", id)
            }
        }
    }

    pub fn is_compatible_for_ranging(&self, peer_config: &Self) -> bool {
        self.device_role != peer_config.device_role
            && self.device_type != peer_config.device_type
            && peer_config
                .dst_mac_address
                .contains(&self.device_mac_address.unwrap())
            && self
                .dst_mac_address
                .contains(&peer_config.device_mac_address.unwrap())
    }

    pub fn can_start_data_transfer(&self) -> bool {
        self.device_role == Some(uci::DeviceRole::Initiator)
    }

    pub fn can_receive_data_transfer(&self) -> bool {
        self.device_role == Some(uci::DeviceRole::Responder)
    }
}
