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

use crate::MacAddress;
use crate::packets::uci::{self, *};

use std::collections::{HashMap, hash_map::Entry};
use std::time::Instant;

use pdl_runtime::Packet;

use super::UciPacket;
use super::app_config::{
    DEFAULT_DL_TDOA_RANGING_METHOD, DEFAULT_DL_TDOA_TX_TIMESTAMP_CONF, DlTdoaAnchorLocation,
    SubSessionKey,
};
use super::session::Session;

pub const MAX_SESSION: usize = 255;

const UCI_VERSION: u16 = 0x0002; // Version 2.0
const MAC_VERSION: u16 = 0x3001; // Version 1.3.0
const PHY_VERSION: u16 = 0x3001; // Version 1.3.0
const TEST_VERSION: u16 = 0x1001; // Version 1.1

/// cf. [UCI] 8.3 Table 29
pub const MAX_NUMBER_OF_CONTROLEES: usize = 8;

// Capabilities are vendor defined
// Android compliant: FIRA-287 UCI_Generic_Specification controlee capabilities_r4
// Android parses capabilities, according to these definitions:
// /android/packages/modules/Uwb/service/java/com/android/server/uwb/config/CapabilityParam.java
pub const DEFAULT_CAPS_INFO: &[(CapTlvType, &[u8])] = &[
    // Fira params
    (CapTlvType::SupportedFiraPhyVersionRange, &[1, 1, 1, 3]), // 1.1 - 1.3
    (CapTlvType::SupportedFiraMacVersionRange, &[1, 1, 1, 3]), // 1.1 - 1.3
    (CapTlvType::SupportedDeviceRoles, &[0x3]),                // INTIATOR | RESPONDER
    (CapTlvType::SupportedRangingMethod, &[0x3f]), // OWR_UL_TDOA | SS_TWR_DEFERRED | DS_TWR_DEFERRED | SS_TWR_NON_DEFERRED | DS_TWR_NON_DEFERRED | OWR_DL_TDOA
    (CapTlvType::SupportedStsConfig, &[0x1f]), // STATIC_STS | DYNAMIC_STS | DYNAMIC_STS_RESPONDER_SPECIFIC_SUBSESSION_KEY | PROVISIONED_STS | PROVISIONED_STS_RESPONDER_SPECIFIC_SUBSESSION_KEY
    (CapTlvType::SupportedMultiNodeModes, &[0xff]),
    (CapTlvType::SupportedRangingTimeStruct, &[0x01]), // Block Based Scheduling (default)
    (CapTlvType::SupportedScheduledMode, &[0x01]),     // Time scheduled ranging (default)
    (CapTlvType::SupportedHoppingMode, &[0x00]),       // Hopping disable
    (CapTlvType::SupportedBlockStriding, &[0x1]),
    (CapTlvType::SupportedUwbInitiationTime, &[0x01]),
    (CapTlvType::SupportedChannels, &[0xff]),
    (CapTlvType::SupportedRframeConfig, &[0xff]),
    (CapTlvType::SupportedCcConstraintLength, &[0xff]),
    (CapTlvType::SupportedBprfParameterSets, &[0xff]),
    (CapTlvType::SupportedHprfParameterSets, &[0xff]),
    (CapTlvType::SupportedAoa, &[0xff]),
    (CapTlvType::SupportedAoaResultReqAntennaInterleaving, &[0x1]),
    (CapTlvType::SupportedExtendedMacAddress, &[0x1]),
    (
        CapTlvType::SupportedMaxMessageSize,
        &(MAX_DATA_MESSAGE_SIZE as u16).to_le_bytes(),
    ),
    (
        CapTlvType::SupportedMaxDataPacketPayloadSize,
        &(MAX_DATA_PACKET_PAYLOAD_SIZE as u16).to_le_bytes(),
    ),
    // CCC params
    (CapTlvType::CccSupportedVersions, &[1, 0]),
    (CapTlvType::CccSupportedUwbConfigs, &[0]),
    (CapTlvType::CccSupportedPulseShapeCombos, &[0]),
    (CapTlvType::CccSupportedRanMultiplier, &[1, 0, 0, 0]),
    (CapTlvType::CccSupportedChapsPerSlot, &[0xff]),
    (CapTlvType::CccSupportedSyncCodes, &[0xff, 0xff, 0xff, 0xff]),
    (CapTlvType::CccSupportedChannels, &[0xff]),
    (
        CapTlvType::CccSupportedHoppingConfigModesAndSequences,
        &[0xff],
    ),
    (CapTlvType::AliroSupportedMacModes, &[0x03]),
    (CapTlvType::AliroSupportedProtocolVersion, &[1, 0]),
];

/// [UCI] 8.2 Device Configuration Parameters
pub struct DeviceConfig {
    // This config is used to enable/disable the low power mode.
    //   0x00 = Disable low power mode
    //   0x01 = Enable low power mode (default)
    low_power_mode: bool,
}

// [UCI] 6.3.1 Setting the Configuration
// All device configuration parameters within the UWBS are set to
// default values at Table 44 [..].
impl Default for DeviceConfig {
    fn default() -> Self {
        DeviceConfig {
            low_power_mode: true,
        }
    }
}

pub struct Device {
    /// Flag set when the device has received the Core Device Reset command.
    /// The first command received by the device is expected to be Core Device
    /// Reset, receiving any other command before this is indicative of a
    /// bad host state.
    is_reset: bool,
    pub handle: usize,
    config: DeviceConfig,
    /// [UCI] 5. UWBS Device State Machine
    state: DeviceState,
    sessions: HashMap<u32, Session>,
    country_code: [u8; 2],
    pub n_active_sessions: usize,
    boot_time: Instant,
}

/// Side effect emitted by a device state transition or command.
pub enum DeviceEffect {
    /// Transmit a UCI packet to the host.
    SendPacket(UciPacket),
    /// Signal in-band ranging termination to a controlee.
    StopControleeRanging {
        mac_address: MacAddress,
        session_id: u32,
    },
}

impl DeviceEffect {
    fn send_control(packet: impl Packet) -> Self {
        Self::SendPacket(
            packet
                .encode_to_vec()
                .expect("failed to encode control packet"),
        )
    }
}

struct CommandOutput(Vec<DeviceEffect>);

impl<T: Packet> From<T> for CommandOutput {
    fn from(response: T) -> Self {
        Self(vec![DeviceEffect::send_control(response)])
    }
}

impl CommandOutput {
    fn with_effects(
        response: impl Packet,
        effects: impl IntoIterator<Item = DeviceEffect>,
    ) -> Self {
        Self(
            std::iter::once(DeviceEffect::send_control(response))
                .chain(effects)
                .collect(),
        )
    }
}

fn command_error_response(gid: GroupId, oid: u8, status: uci::Status) -> ControlPacket {
    ControlPacket {
        mt: uci::MessageType::Response,
        gid,
        payload: vec![oid, 0, 0, status.into()],
    }
}

fn packet_error_status<T, E>(result: Result<T, E>) -> uci::Status {
    if result.is_ok() {
        uci::Status::UnknownOid
    } else {
        uci::Status::SyntaxError
    }
}

/// Per-round anchor state unifying static virtual anchors (from `Pica::create_anchor`)
/// and active `DtAnchor` UCI sessions.
#[derive(Debug)]
pub struct DlTdoaAnchor {
    pub mac_address: MacAddress,
    pub location: Option<DlTdoaAnchorLocation>,
    pub round_index: u8,
    pub acting_role: Option<uci::DtAnchorActingRole>,
    pub ranging_method: uci::DlTdoaRangingMethod,
    pub active_ranging_rounds: Vec<u8>,
    pub tx_timestamp_conf: uci::DlTdoaTxTimestampConf,
    pub includes_responder_tof: bool,
    pub includes_anchor_cfo: bool,
    pub is_time_reference: bool,
}

impl DlTdoaAnchor {
    /// Creates a virtual anchor (from `Pica::create_anchor`), enabling Responder ToF by default so
    /// multi-anchor simulations report inter-anchor ToF without per-anchor `AppConfig` setup.
    pub fn virtual_anchor(mac_address: MacAddress, default_round_index: u8) -> Self {
        Self {
            mac_address,
            location: None,
            round_index: default_round_index,
            acting_role: None,
            ranging_method: DEFAULT_DL_TDOA_RANGING_METHOD,
            active_ranging_rounds: Vec::new(),
            tx_timestamp_conf: DEFAULT_DL_TDOA_TX_TIMESTAMP_CONF,
            includes_responder_tof: true,
            includes_anchor_cfo: true,
            is_time_reference: false,
        }
    }

    fn from_session(
        session: &Session,
        mac_address: MacAddress,
        round: &uci::DtAnchorRangingRound,
    ) -> Self {
        let config = &session.app_config;
        let active_ranging_rounds = if config.dl_tdoa_includes_active_ranging_rounds() {
            session
                .dt_anchor_ranging_rounds()
                .iter()
                .map(|r| r.round_index)
                .filter(|&idx| idx != round.round_index)
                .collect()
        } else {
            Vec::new()
        };
        Self {
            mac_address,
            location: config.dl_tdoa_anchor_location(),
            round_index: round.round_index,
            acting_role: Some(round.role),
            ranging_method: config.dl_tdoa_ranging_method(),
            active_ranging_rounds,
            tx_timestamp_conf: config.dl_tdoa_tx_timestamp_conf(),
            includes_responder_tof: config.dl_tdoa_includes_responder_tof(),
            includes_anchor_cfo: config.dl_tdoa_includes_anchor_cfo(),
            is_time_reference: config.dl_tdoa_is_time_reference_anchor(),
        }
    }
}

/// UWBS capability limit (`DT_ANCHOR_MAX_ACTIVE_RR` / `DT_TAG_MAX_ACTIVE_RR`); at 16 rounds,
/// the 15 non-current anchor rounds fit in `DlTdoaRangingMeasurement`'s 4-bit count field.
const MAX_ACTIVE_RANGING_ROUNDS: usize = 16;

impl Device {
    pub fn new(handle: usize) -> Self {
        Device {
            handle,
            is_reset: false,
            config: Default::default(),
            state: DeviceState::DeviceStateError, // Will be overwitten
            sessions: Default::default(),
            country_code: Default::default(),
            n_active_sessions: 0,
            boot_time: Instant::now(),
        }
    }

    #[must_use]
    pub fn set_state(&mut self, device_state: DeviceState) -> Option<CoreDeviceStatusNtf> {
        if device_state == self.state {
            return None;
        }

        self.state = device_state;
        Some(CoreDeviceStatusNtf { device_state })
    }

    #[must_use]
    pub fn init(&mut self) -> Option<DeviceEffect> {
        self.set_state(DeviceState::DeviceStateReady)
            .map(DeviceEffect::send_control)
    }

    #[must_use]
    fn on_session_deactivated(&mut self) -> Option<CoreDeviceStatusNtf> {
        self.n_active_sessions = self.n_active_sessions.saturating_sub(1);
        if self.n_active_sessions == 0 {
            self.set_state(DeviceState::DeviceStateReady)
        } else {
            None
        }
    }

    #[must_use]
    pub fn stop_active_session(
        &mut self,
        session_id: u32,
        reason_code: ReasonCode,
    ) -> Vec<DeviceEffect> {
        let Some(session) = self.session_mut(session_id) else {
            return Vec::new();
        };

        if session.session_state() != SessionState::SessionStateActive {
            tracing::warn!("stop_active_session: session is not active !");
            return Vec::new();
        }

        session.stop_ranging();
        let session_ntf = session
            .set_state(SessionState::SessionStateIdle, reason_code)
            .map(DeviceEffect::send_control);
        let device_ntf = self
            .on_session_deactivated()
            .map(DeviceEffect::send_control);

        session_ntf.into_iter().chain(device_ntf).collect()
    }

    pub fn session(&self, session_id: u32) -> Option<&Session> {
        self.sessions.get(&session_id)
    }

    pub fn session_mut(&mut self, session_id: u32) -> Option<&mut Session> {
        self.sessions.get_mut(&session_id)
    }

    pub fn next_ranging_deadline(&self) -> Option<Instant> {
        self.sessions
            .values()
            .filter_map(Session::next_ranging_at)
            .min()
    }

    pub fn poll_ranging(&mut self, now: Instant) -> impl Iterator<Item = u32> + '_ {
        self.sessions
            .iter_mut()
            .filter_map(move |(&session_id, session)| {
                session.poll_ranging(now).then_some(session_id)
            })
    }

    pub fn can_start_ranging(&self, peer_session: &Session, session_id: u32) -> bool {
        match self.session(session_id) {
            Some(session) => {
                session.session_state() == SessionState::SessionStateActive
                    && session
                        .app_config
                        .is_compatible_for_ranging(&peer_session.app_config)
            }
            None => false,
        }
    }

    pub fn active_dl_tdoa_anchors(
        &self,
        peer_session: &Session,
        session_id: u32,
    ) -> Vec<DlTdoaAnchor> {
        let Some(session) = self.session(session_id) else {
            return Vec::new();
        };
        let config = &session.app_config;
        if session.session_state() != SessionState::SessionStateActive
            || !config.is_compatible_for_dl_tdoa(&peer_session.app_config)
        {
            return Vec::new();
        }
        let Some(mac_address) = config.device_mac_address() else {
            return Vec::new();
        };
        let tag_rounds = peer_session.dt_tag_active_rounds();
        session
            .dt_anchor_ranging_rounds()
            .iter()
            .filter(|round| tag_rounds.contains(&round.round_index))
            .map(|round| DlTdoaAnchor::from_session(session, mac_address, round))
            .collect()
    }

    pub fn can_receive_data_transfer(&self, session_id: u32, dst_mac_address: MacAddress) -> bool {
        self.session(session_id)
            .is_some_and(|session| session.can_receive_data_transfer(dst_mac_address))
    }

    // The fira norm specify to send a response, then reset, then
    // send a notification once the reset is done
    fn core_device_reset(&mut self, cmd: CoreDeviceResetCmd) -> CommandOutput {
        let reset_config = cmd.reset_config;
        tracing::debug!("[{}] DeviceReset", self.handle);
        tracing::debug!("  reset_config={reset_config:?}");

        *self = Device::new(self.handle);
        self.is_reset = true;
        let effects = self.init();

        CommandOutput::with_effects(
            CoreDeviceResetRsp {
                status: uci::Status::Ok,
            },
            effects,
        )
    }

    fn core_get_device_info(&self, _cmd: CoreGetDeviceInfoCmd) -> CoreGetDeviceInfoRsp {
        tracing::debug!("[{}] GetDeviceInfo", self.handle);
        CoreGetDeviceInfoRsp {
            status: uci::Status::Ok,
            uci_version: UCI_VERSION,
            mac_version: MAC_VERSION,
            phy_version: PHY_VERSION,
            uci_test_version: TEST_VERSION,
            vendor_spec_info: Vec::new(),
        }
    }

    pub fn core_get_caps_info(&self, _cmd: CoreGetCapsInfoCmd) -> CoreGetCapsInfoRsp {
        tracing::debug!("[{}] GetCapsInfo", self.handle);

        let caps = DEFAULT_CAPS_INFO
            .iter()
            .map(|(id, value)| CapTlv {
                t: *id,
                v: (*value).into(),
            })
            .collect();

        CoreGetCapsInfoRsp {
            status: uci::Status::Ok,
            tlvs: caps,
        }
    }

    pub fn core_set_config(&mut self, cmd: CoreSetConfigCmd) -> CoreSetConfigRsp {
        tracing::debug!("[{}] SetConfig", self.handle);
        // UCI 6.3
        if self.state != DeviceState::DeviceStateReady {
            return CoreSetConfigRsp {
                status: uci::Status::Rejected,
                parameters: vec![],
            };
        }

        // [UCI] 6.3.1 Setting the Configuration
        // The UWBS shall respond with CORE_SET_CONFIG_RSP setting the Status
        // field of STATUS_INVALID_PARAM and including one or more invalid
        // Parameter ID(s) If the Host tries to set a parameter that is not
        // available in the UWBS. All other configuration parameters should
        // have been set to the new values within the UWBS.
        let mut invalid_parameters = vec![];
        for parameter in cmd.parameters {
            match parameter.id {
                uci::ConfigParameterId::DeviceState => {
                    invalid_parameters.push(uci::ConfigParameterStatus {
                        id: parameter.id,
                        status: uci::Status::ReadOnly,
                    })
                }
                uci::ConfigParameterId::LowPowerMode => {
                    self.config.low_power_mode = parameter.value.first().copied().unwrap_or(1) != 0;
                }
                uci::ConfigParameterId::Rfu(id) => {
                    tracing::warn!("unknown config parameter id 0x{:02x}", *id);
                    invalid_parameters.push(uci::ConfigParameterStatus {
                        id: parameter.id,
                        status: uci::Status::InvalidParam,
                    })
                }
            }
        }

        CoreSetConfigRsp {
            status: if invalid_parameters.is_empty() {
                uci::Status::Ok
            } else {
                uci::Status::InvalidParam
            },
            parameters: invalid_parameters,
        }
    }

    pub fn core_get_config(&self, cmd: CoreGetConfigCmd) -> CoreGetConfigRsp {
        tracing::debug!("[{}] GetConfig", self.handle);

        // [UCI] 6.3.2 Retrieve the Configuration
        // If the Host tries to retrieve any Parameter(s) that are not available
        // in the UWBS, the UWBS shall respond with a CORE_GET_CONFIG_RSP with
        // a Status field of STATUS_INVALID_PARAM, containing each unavailable
        // Device Configuration Parameter Type with Length field is zero. In
        // this case, the CORE_GET_CONFIG_RSP shall not include any parameter(s)
        // that are available in the UWBS.
        let mut valid_parameters = vec![];
        let mut invalid_parameters = vec![];
        for id in cmd.parameter_ids {
            match id {
                ConfigParameterId::DeviceState => valid_parameters.push(ConfigParameter {
                    id,
                    value: vec![self.state.into()],
                }),
                ConfigParameterId::LowPowerMode => valid_parameters.push(ConfigParameter {
                    id,
                    value: vec![self.config.low_power_mode.into()],
                }),
                ConfigParameterId::Rfu(_) => {
                    invalid_parameters.push(ConfigParameter { id, value: vec![] })
                }
            }
        }

        if invalid_parameters.is_empty() {
            CoreGetConfigRsp {
                status: uci::Status::Ok,
                parameters: valid_parameters,
            }
        } else {
            CoreGetConfigRsp {
                status: uci::Status::InvalidParam,
                parameters: invalid_parameters,
            }
        }
    }

    fn core_query_timestamp(&self) -> CoreQueryTimeStampRsp {
        CoreQueryTimeStampRsp {
            status: uci::Status::Ok,
            timeStamp: self.boot_time.elapsed().as_micros() as u64,
        }
    }

    fn session_init(&mut self, cmd: SessionInitCmd) -> CommandOutput {
        let session_id = cmd.session_id;
        let session_type = cmd.session_type;

        tracing::debug!("[{}] Session init", self.handle);
        tracing::debug!("  session_id=0x{session_id:x}");
        tracing::debug!("  session_type={session_type:?}");

        if self.sessions.len() >= MAX_SESSION {
            return SessionInitRsp {
                status: uci::Status::ErrorMaxSessionsExceeded,
            }
            .into();
        }

        match self.sessions.entry(session_id) {
            Entry::Occupied(_) => SessionInitRsp {
                status: uci::Status::ErrorSessionDuplicate,
            }
            .into(),
            Entry::Vacant(entry) => {
                let mut session = Session::new(session_id, session_type, self.handle);
                let effects = session.init().into_iter().map(DeviceEffect::send_control);
                entry.insert(session);
                CommandOutput::with_effects(
                    SessionInitRsp {
                        status: uci::Status::Ok,
                    },
                    effects,
                )
            }
        }
    }

    fn session_deinit(&mut self, cmd: SessionDeinitCmd) -> CommandOutput {
        let session_id = cmd.session_token;
        tracing::debug!("[{}] Session deinit", self.handle);
        tracing::debug!("  session_id=0x{session_id:x}");

        let Some(mut session) = self.sessions.remove(&session_id) else {
            return SessionDeinitRsp {
                status: uci::Status::ErrorSessionNotExist,
            }
            .into();
        };

        let was_active = session.state == SessionState::SessionStateActive;
        let session_ntf = session
            .set_state(
                SessionState::SessionStateDeinit,
                ReasonCode::StateChangeWithSessionManagementCommands,
            )
            .map(DeviceEffect::send_control);
        let device_ntf = if was_active {
            self.on_session_deactivated()
                .map(DeviceEffect::send_control)
        } else {
            None
        };
        let effects = session_ntf.into_iter().chain(device_ntf);

        CommandOutput::with_effects(
            SessionDeinitRsp {
                status: uci::Status::Ok,
            },
            effects,
        )
    }

    fn session_get_count(&self, _cmd: SessionGetCountCmd) -> SessionGetCountRsp {
        tracing::debug!("[{}] Session get count", self.handle);

        SessionGetCountRsp {
            status: uci::Status::Ok,
            session_count: self.sessions.len() as u8,
        }
    }

    fn session_query_max_data_size_in_ranging(
        &self,
        cmd: SessionQueryMaxDataSizeInRangingCmd,
    ) -> SessionQueryMaxDataSizeInRangingRsp {
        if self.session(cmd.session_token).is_some() {
            SessionQueryMaxDataSizeInRangingRsp {
                session_token: cmd.session_token,
                status: uci::Status::Ok,
                max_data_size: MAX_DATA_PACKET_PAYLOAD_SIZE as u16,
            }
        } else {
            SessionQueryMaxDataSizeInRangingRsp {
                session_token: cmd.session_token,
                status: uci::Status::ErrorSessionNotExist,
                max_data_size: 0,
            }
        }
    }

    fn configured_dl_tdoa_session_mut(
        &mut self,
        session_token: u32,
        expected_role: uci::DeviceRole,
    ) -> Result<&mut Session, uci::Status> {
        let Some(session) = self.session_mut(session_token) else {
            return Err(uci::Status::ErrorSessionNotExist);
        };
        let is_configured = session.state != uci::SessionState::SessionStateInit
            && session.app_config.dl_tdoa_role() == Some(expected_role);
        if !is_configured {
            return Err(uci::Status::Rejected);
        }
        Ok(session)
    }

    fn session_update_dt_tag_ranging_rounds(
        &mut self,
        cmd: SessionUpdateDtTagRangingRoundsCmd,
    ) -> SessionUpdateDtTagRangingRoundsRsp {
        let status =
            match self.configured_dl_tdoa_session_mut(cmd.session_token, uci::DeviceRole::DtTag) {
                Ok(_) if cmd.ranging_round_indexes.is_empty() => uci::Status::InvalidParam,
                Ok(_) if cmd.ranging_round_indexes.len() > MAX_ACTIVE_RANGING_ROUNDS => {
                    uci::Status::ErrorNumberOfActiveRangingRoundsExceeded
                }
                Ok(session) => {
                    session.set_dt_tag_ranging_rounds(cmd.ranging_round_indexes);
                    uci::Status::Ok
                }
                Err(status) => status,
            };
        SessionUpdateDtTagRangingRoundsRsp {
            status,
            ranging_round_indexes: Vec::new(),
        }
    }

    fn session_update_dt_anchor_ranging_rounds(
        &mut self,
        cmd: SessionUpdateDtAnchorRangingRoundsCmd,
    ) -> SessionUpdateDtAnchorRangingRoundsRsp {
        let status = match self
            .configured_dl_tdoa_session_mut(cmd.session_token, uci::DeviceRole::DtAnchor)
        {
            Ok(_) if cmd.ranging_rounds.is_empty() => uci::Status::InvalidParam,
            Ok(_) if cmd.ranging_rounds.len() > MAX_ACTIVE_RANGING_ROUNDS => {
                uci::Status::ErrorNumberOfActiveRangingRoundsExceeded
            }
            Ok(session) => {
                session.set_dt_anchor_ranging_rounds(cmd.ranging_rounds);
                uci::Status::Ok
            }
            Err(status) => status,
        };
        SessionUpdateDtAnchorRangingRoundsRsp {
            status,
            ranging_round_indexes: Vec::new(),
        }
    }

    fn session_set_app_config(&mut self, cmd: SessionSetAppConfigCmd) -> CommandOutput {
        let session_handle = cmd.session_token;

        tracing::debug!(
            "[{}:0x{session_handle:x}] Session Set App Config",
            self.handle
        );

        let Some(session) = self.sessions.get_mut(&session_handle) else {
            return SessionSetAppConfigRsp {
                cfg_status: Vec::new(),
                status: uci::Status::ErrorSessionNotExist,
            }
            .into();
        };

        if !session.is_supported_session_type() {
            return SessionSetAppConfigRsp {
                status: uci::Status::Rejected,
                cfg_status: vec![],
            }
            .into();
        }

        if session.state == SessionState::SessionStateActive {
            const IMMUTABLE_PARAMETERS: &[AppConfigTlvType] = &[AppConfigTlvType::AoaResultReq];
            if cmd
                .tlvs
                .iter()
                .any(|cfg| IMMUTABLE_PARAMETERS.contains(&cfg.cfg_id))
            {
                return SessionSetAppConfigRsp {
                    status: uci::Status::ErrorSessionActive,
                    cfg_status: vec![],
                }
                .into();
            }
        }

        if !matches!(
            session.state,
            SessionState::SessionStateInit
                | SessionState::SessionStateIdle
                | SessionState::SessionStateActive
        ) {
            return SessionSetAppConfigRsp {
                status: uci::Status::Rejected,
                cfg_status: Vec::new(),
            }
            .into();
        }

        let mut app_config = session.app_config.clone();
        let mut invalid_parameters = vec![];
        // Defer DeviceMacAddress and DstMacAddress so MacAddressMode and
        // NumberOfControlees take effect first regardless of TLV order in the command.
        let (deferred_tlvs, immediate_tlvs): (Vec<_>, Vec<_>) =
            cmd.tlvs.into_iter().partition(|cfg| {
                matches!(
                    cfg.cfg_id,
                    uci::AppConfigTlvType::DeviceMacAddress | uci::AppConfigTlvType::DstMacAddress
                )
            });
        for cfg in immediate_tlvs.into_iter().chain(deferred_tlvs) {
            match app_config.set(cfg.cfg_id, &cfg.v) {
                Ok(_) => (),
                Err(_) => invalid_parameters.push(AppConfigStatus {
                    cfg_id: cfg.cfg_id,
                    status: uci::Status::InvalidParam,
                }),
            }
        }

        if !app_config.has_mandatory_parameters() {
            tracing::error!(
                "[{}:0x{session_handle:x}] missing mandatory APP config parameters",
                self.handle
            );
            return SessionSetAppConfigRsp {
                status: uci::Status::Rejected,
                cfg_status: vec![],
            }
            .into();
        }

        if invalid_parameters.is_empty() {
            session.app_config = app_config;
            let effects = if session.state == SessionState::SessionStateInit {
                session
                    .set_state(
                        SessionState::SessionStateIdle,
                        ReasonCode::StateChangeWithSessionManagementCommands,
                    )
                    .map(DeviceEffect::send_control)
            } else {
                None
            };
            CommandOutput::with_effects(
                SessionSetAppConfigRsp {
                    status: uci::Status::Ok,
                    cfg_status: invalid_parameters,
                },
                effects,
            )
        } else {
            SessionSetAppConfigRsp {
                status: uci::Status::InvalidParam,
                cfg_status: invalid_parameters,
            }
            .into()
        }
    }

    fn session_get_app_config(&self, cmd: SessionGetAppConfigCmd) -> SessionGetAppConfigRsp {
        let session_handle = cmd.session_token;

        tracing::debug!(
            "[{}:0x{session_handle:x}] Session Get App Config",
            self.handle
        );

        let Some(session) = self.sessions.get(&session_handle) else {
            return SessionGetAppConfigRsp {
                tlvs: vec![],
                status: uci::Status::ErrorSessionNotExist,
            };
        };

        let (status, tlvs) = if cmd.app_cfg.is_empty() {
            (uci::Status::Ok, session.app_config.to_tlvs())
        } else {
            let mut valid_parameters = vec![];
            let mut invalid_parameters = vec![];
            for id in cmd.app_cfg {
                match session.app_config.get(id) {
                    Some(value) => valid_parameters.push(AppConfigTlv {
                        cfg_id: id,
                        v: value.to_vec(),
                    }),
                    None => invalid_parameters.push(AppConfigTlv {
                        cfg_id: id,
                        v: vec![],
                    }),
                }
            }

            if invalid_parameters.is_empty() {
                (uci::Status::Ok, valid_parameters)
            } else {
                (uci::Status::InvalidParam, invalid_parameters)
            }
        };

        SessionGetAppConfigRsp { status, tlvs }
    }

    fn session_get_state(&self, cmd: SessionGetStateCmd) -> SessionGetStateRsp {
        let session_handle = cmd.session_token;

        tracing::debug!("[{}:0x{session_handle:x}] Session Get State", self.handle);

        let Some(session) = self.sessions.get(&session_handle) else {
            return SessionGetStateRsp {
                session_state: SessionState::SessionStateInit,
                status: uci::Status::ErrorSessionNotExist,
            };
        };

        SessionGetStateRsp {
            status: uci::Status::Ok,
            session_state: session.state,
        }
    }

    fn session_update_controller_multicast_list(
        &mut self,
        cmd: SessionUpdateControllerMulticastListCmd,
    ) -> CommandOutput {
        let session_handle = cmd.session_token;

        tracing::debug!(
            "[{}:0x{session_handle:x}] Session Update Controller Multicast List",
            self.handle
        );

        let Some(session) = self.sessions.get_mut(&session_handle) else {
            return SessionUpdateControllerMulticastListRsp {
                status: uci::Status::ErrorSessionNotExist,
                controlee_status: vec![],
            }
            .into();
        };

        if (session.state != SessionState::SessionStateActive
            && session.state != SessionState::SessionStateIdle)
            || session.app_config.device_type() != Some(DeviceType::Controller)
            || session.app_config.multi_node_mode() != Some(MultiNodeMode::OneToMany)
        {
            return SessionUpdateControllerMulticastListRsp {
                status: uci::Status::Rejected,
                controlee_status: vec![],
            }
            .into();
        }
        let action = cmd.action;
        let mut dst_addresses: Vec<MacAddress> = session.app_config.dst_mac_address().collect();
        let new_controlees: Vec<Controlee> = match action {
            UpdateMulticastListAction::AddControlee
            | UpdateMulticastListAction::RemoveControlee => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmdPayload::decode_full(&cmd.payload)
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRsp {
                        status: uci::Status::SyntaxError,
                        controlee_status: vec![],
                    }
                    .into();
                }
            }
            UpdateMulticastListAction::AddControleeWithShortSubSessionKey => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload::decode_full(
                        &cmd.payload,
                    )
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRsp {
                        status: uci::Status::SyntaxError,
                        controlee_status: vec![],
                    }
                    .into();
                }
            }
            UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload::decode_full(
                        &cmd.payload,
                    )
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRsp {
                        status: uci::Status::SyntaxError,
                        controlee_status: vec![],
                    }
                    .into();
                }
            }
        };
        let mut controlee_status_ntf = Vec::new();
        let mut controlee_status_rsp = Vec::new();
        let mut status = uci::Status::Ok;
        let mut effects = Vec::new();

        match action {
            UpdateMulticastListAction::AddControlee
            | UpdateMulticastListAction::AddControleeWithShortSubSessionKey
            | UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey => {
                for controlee in new_controlees {
                    let mut update_status = MulticastUpdateStatus::OkMulticastListUpdate;
                    if (action == UpdateMulticastListAction::AddControleeWithShortSubSessionKey
                        || action
                            == UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey)
                        && session.app_config.sts_config()
                            != uci::StsConfig::ProvisionedForResponderSubSessionKey
                    {
                        // If Action is 0x02 or 0x03 for STS_CONFIG values other than
                        // 0x04, the UWBS shall return SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF
                        // with Status set to STATUS_ERROR_SUB_SESSION_KEY_NOT_APPLICABLE for each
                        // Controlee in the Controlee List.
                        status = uci::Status::Failed;
                        update_status = MulticastUpdateStatus::ErrorSubSessionKeyNotApplicable;
                        controlee_status_ntf.push(ControleeStatus {
                            mac_address: controlee.short_address,
                            status: update_status,
                        });
                    } else {
                        let address = MacAddress::Short(controlee.short_address);
                        if !dst_addresses.contains(&address) {
                            if dst_addresses.len() == MAX_NUMBER_OF_CONTROLEES {
                                status = uci::Status::ErrorMulticastListFull;
                                update_status = MulticastUpdateStatus::ErrorMulticastListFull;
                            } else {
                                dst_addresses.push(address);
                            };
                        } else {
                            status = uci::Status::Failed;
                            update_status = MulticastUpdateStatus::ErrorAddressAlreadyPresent;
                        }
                        controlee_status_rsp.push(ControleeStatus {
                            mac_address: controlee.short_address,
                            status: update_status,
                        });
                    }
                }
            }
            UpdateMulticastListAction::RemoveControlee => {
                for controlee in new_controlees {
                    let address = MacAddress::Short(controlee.short_address);
                    let attempt_count = session.app_config.in_band_termination_attempt_count();
                    let mut update_status = MulticastUpdateStatus::OkMulticastListUpdate;
                    if !dst_addresses.contains(&address) {
                        status = uci::Status::Failed;
                        update_status = MulticastUpdateStatus::ErrorAddressNotFound;
                    } else {
                        dst_addresses.retain(|value| *value != address);
                        // If IN_BAND_TERMINATION_ATTEMPT_COUNT is not equal to 0x00, then the
                        // UWBS shall transmit the RCM with the “Stop Ranging” bit set to ‘1’
                        // for IN_BAND_TERMINATION_ATTEMPT_COUNT times to the corresponding
                        // Controlee.
                        if attempt_count != 0 {
                            effects.push(DeviceEffect::StopControleeRanging {
                                mac_address: address,
                                session_id: session_handle,
                            });
                        }
                        // Following requirements are applicable when the action is set to Delete (Action field set to 0x01):
                        // When the command is received while the Session State is SESSION_STATE_ACTIVE,
                        // For requested Controlees present in the multicast list,
                        // UWBS shall send the SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF and the
                        // corresponding Controlee status shall be set to STATUS_OK_MULTICAST_LIST_UPDATE
                        // in the Status List of SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF.
                        if session.state == SessionState::SessionStateActive {
                            controlee_status_ntf.push(ControleeStatus {
                                mac_address: controlee.short_address,
                                status: update_status,
                            });
                        }
                    }
                    controlee_status_rsp.push(ControleeStatus {
                        mac_address: controlee.short_address,
                        status: update_status,
                    });
                }
            }
        }

        session
            .app_config
            .set_number_of_controlees(dst_addresses.len() as u8);
        session.app_config.set_dst_mac_address(&dst_addresses);

        // If the multicast list becomes empty, the UWBS shall move the session to
        // SESSION_STATE_IDLE by sending the SESSION_STATUS_NTF with Reason Code
        // set to ERROR_INVALID_NUM_OF_CONTROLEES.
        let deactivated =
            dst_addresses.is_empty() && session.state == SessionState::SessionStateActive;
        if deactivated {
            session.stop_ranging();
            effects.extend(
                session
                    .set_state(
                        SessionState::SessionStateIdle,
                        ReasonCode::ErrorInvalidNumOfControlees,
                    )
                    .map(DeviceEffect::send_control),
            );
        }

        if !controlee_status_ntf.is_empty() {
            effects.push(DeviceEffect::send_control(
                SessionUpdateControllerMulticastListNtf {
                    controlee_status: controlee_status_ntf,
                    session_token: session_handle,
                },
            ));
        }

        if deactivated {
            effects.extend(
                self.on_session_deactivated()
                    .map(DeviceEffect::send_control),
            );
        }

        CommandOutput::with_effects(
            SessionUpdateControllerMulticastListRsp {
                status,
                controlee_status: controlee_status_rsp,
            },
            effects,
        )
    }

    fn session_start(&mut self, cmd: SessionStartCmd) -> CommandOutput {
        let session_id = cmd.session_id;

        tracing::debug!("[{}:0x{session_id:x}] Session Start", self.handle);

        let Some(session) = self.sessions.get_mut(&session_id) else {
            return SessionStartRsp {
                status: uci::Status::ErrorSessionNotExist,
            }
            .into();
        };

        if session.state != SessionState::SessionStateIdle {
            return SessionStartRsp {
                status: uci::Status::ErrorSessionNotConfigured,
            }
            .into();
        }

        if let Some(reason_code) = session.missing_dl_tdoa_rounds_reason() {
            return CommandOutput::with_effects(
                SessionStartRsp {
                    status: uci::Status::Ok,
                },
                [DeviceEffect::send_control(SessionStatusNtf {
                    session_token: session_id,
                    session_state: SessionState::SessionStateIdle,
                    reason_code: reason_code.into(),
                })],
            );
        }

        session.start_ranging();

        let session_ntf = session
            .set_state(
                SessionState::SessionStateActive,
                ReasonCode::StateChangeWithSessionManagementCommands,
            )
            .map(DeviceEffect::send_control);

        self.n_active_sessions += 1;
        let device_ntf = self
            .set_state(DeviceState::DeviceStateActive)
            .map(DeviceEffect::send_control);

        let effects = session_ntf.into_iter().chain(device_ntf);

        CommandOutput::with_effects(
            SessionStartRsp {
                status: uci::Status::Ok,
            },
            effects,
        )
    }

    fn session_stop(&mut self, cmd: SessionStopCmd) -> CommandOutput {
        let session_id = cmd.session_id;

        tracing::debug!("[{}:0x{session_id:x}] Session Stop", self.handle);

        let Some(session) = self.sessions.get_mut(&session_id) else {
            return SessionStopRsp {
                status: uci::Status::ErrorSessionNotExist,
            }
            .into();
        };

        if session.state != SessionState::SessionStateActive {
            return SessionStopRsp {
                status: uci::Status::Rejected,
            }
            .into();
        }

        let effects = self.stop_active_session(
            session_id,
            ReasonCode::StateChangeWithSessionManagementCommands,
        );

        CommandOutput::with_effects(
            SessionStopRsp {
                status: uci::Status::Ok,
            },
            effects,
        )
    }

    fn session_get_ranging_count(
        &self,
        cmd: SessionGetRangingCountCmd,
    ) -> SessionGetRangingCountRsp {
        let session_id = cmd.session_id;

        tracing::debug!(
            "[{}:0x{session_id:x}] Session Get Ranging Count",
            self.handle
        );

        let Some(session) = self.sessions.get(&session_id) else {
            return SessionGetRangingCountRsp {
                status: uci::Status::ErrorSessionNotExist,
                count: 0,
            };
        };

        SessionGetRangingCountRsp {
            status: uci::Status::Ok,
            count: session.ranging_count,
        }
    }

    fn android_set_country_code(
        &mut self,
        cmd: AndroidSetCountryCodeCmd,
    ) -> AndroidSetCountryCodeRsp {
        tracing::debug!("[{}] Set country code", self.handle);
        tracing::debug!(
            "  country_code={},{}",
            char::from(cmd.country_code[0]),
            char::from(cmd.country_code[1])
        );

        self.country_code = cmd.country_code;
        AndroidSetCountryCodeRsp {
            status: uci::Status::Ok,
        }
    }

    fn android_get_power_stats(
        &mut self,
        _cmd: AndroidGetPowerStatsCmd,
    ) -> AndroidGetPowerStatsRsp {
        tracing::debug!("[{}] Get power stats", self.handle);

        // TODO
        AndroidGetPowerStatsRsp {
            stats: PowerStats {
                status: uci::Status::Ok,
                idle_time_ms: 0,
                tx_time_ms: 0,
                rx_time_ms: 0,
                total_wake_count: 0,
            },
        }
    }

    pub fn data_message_snd(
        &mut self,
        data: DataPacket,
    ) -> Result<ControlPacket, pdl_runtime::EncodeError> {
        tracing::debug!("[{}] data_message_send", self.handle);
        let session_token = data
            .payload
            .first_chunk::<4>()
            .copied()
            .map_or(0, u32::from_le_bytes);

        let (status, uci_sequence_number) = match data.dpf {
            DataPacketFormat::DataSnd => {
                if let Some(session) = self.sessions.values_mut().find(|s| s.has_pending_data()) {
                    return session.data_message_snd(data);
                }

                if data.payload.len() < DATA_MESSAGE_SND_HEADER_SIZE {
                    (
                        DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat,
                        0,
                    )
                } else {
                    if let Some(session) = self.session_mut(session_token) {
                        return session.data_message_snd(data);
                    }
                    let uci_sequence_number = data
                        .payload
                        .get(12..)
                        .and_then(|s| s.first_chunk::<2>())
                        .copied()
                        .map_or(0, u16::from_le_bytes);
                    (
                        DataTransferNtfStatusCode::UciDataTransferStatusErrorRejected,
                        uci_sequence_number,
                    )
                }
            }
            DataPacketFormat::DataRcv => (
                DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat,
                0,
            ),
        };

        SessionDataTransferStatusNtf {
            session_token,
            status,
            tx_count: 1,
            uci_sequence_number,
        }
        .try_into()
    }

    fn receive_command(&mut self, cmd: ControlPacket) -> Vec<DeviceEffect> {
        use AndroidPacketChild::*;
        use ControlPacketChild::*;
        use CorePacketChild::*;
        use SessionConfigPacketChild::*;
        use SessionControlPacketChild::*;

        // Check whether the first command received is the Core Device
        // Reset command. The controller responds with Device Status
        // Notification with DEVICE_STATE_ERROR otherwise.
        if !self.is_reset && !cmd.is_core_device_reset_cmd() {
            return CommandOutput::from(uci::CoreDeviceStatusNtf {
                device_state: DeviceState::DeviceStateError,
            })
            .0;
        }

        let Ok(child) = cmd.specialize() else {
            let status = if cmd.payload.len() >= 3 {
                uci::Status::UnknownOid
            } else {
                uci::Status::SyntaxError
            };
            return CommandOutput::from(command_error_response(
                cmd.gid,
                cmd.payload.first().copied().unwrap_or(0),
                status,
            ))
            .0;
        };

        let output: CommandOutput = match child {
            CorePacket(cmd) => match cmd.specialize() {
                Ok(CoreDeviceResetCmd(cmd)) => self.core_device_reset(cmd),
                Ok(CoreGetDeviceInfoCmd(cmd)) => self.core_get_device_info(cmd).into(),
                Ok(CoreGetCapsInfoCmd(cmd)) => self.core_get_caps_info(cmd).into(),
                Ok(CoreSetConfigCmd(cmd)) => self.core_set_config(cmd).into(),
                Ok(CoreGetConfigCmd(cmd)) => self.core_get_config(cmd).into(),
                Ok(CoreQueryTimeStampCmd(_)) => self.core_query_timestamp().into(),
                child => command_error_response(
                    GroupId::Core,
                    cmd.oid.into(),
                    packet_error_status(child),
                )
                .into(),
            },
            SessionConfigPacket(cmd) => match cmd.specialize() {
                Ok(SessionInitCmd(cmd)) => self.session_init(cmd),
                Ok(SessionDeinitCmd(cmd)) => self.session_deinit(cmd),
                Ok(SessionGetCountCmd(cmd)) => self.session_get_count(cmd).into(),
                Ok(SessionSetAppConfigCmd(cmd)) => self.session_set_app_config(cmd),
                Ok(SessionGetAppConfigCmd(cmd)) => self.session_get_app_config(cmd).into(),
                Ok(SessionGetStateCmd(cmd)) => self.session_get_state(cmd).into(),
                Ok(SessionUpdateControllerMulticastListCmd(cmd)) => {
                    self.session_update_controller_multicast_list(cmd)
                }
                Ok(SessionQueryMaxDataSizeInRangingCmd(cmd)) => {
                    self.session_query_max_data_size_in_ranging(cmd).into()
                }
                Ok(SessionUpdateDtTagRangingRoundsCmd(cmd)) => {
                    self.session_update_dt_tag_ranging_rounds(cmd).into()
                }
                Ok(SessionUpdateDtAnchorRangingRoundsCmd(cmd)) => {
                    self.session_update_dt_anchor_ranging_rounds(cmd).into()
                }
                child => command_error_response(
                    GroupId::SessionConfig,
                    cmd.oid.into(),
                    packet_error_status(child),
                )
                .into(),
            },
            SessionControlPacket(cmd) => match cmd.specialize() {
                Ok(SessionStartCmd(cmd)) => self.session_start(cmd),
                Ok(SessionStopCmd(cmd)) => self.session_stop(cmd),
                Ok(SessionGetRangingCountCmd(cmd)) => self.session_get_ranging_count(cmd).into(),
                child => command_error_response(
                    GroupId::SessionControl,
                    cmd.oid.into(),
                    packet_error_status(child),
                )
                .into(),
            },
            AndroidPacket(cmd) => match cmd.specialize() {
                Ok(AndroidSetCountryCodeCmd(cmd)) => self.android_set_country_code(cmd).into(),
                Ok(AndroidGetPowerStatsCmd(cmd)) => self.android_get_power_stats(cmd).into(),
                child => command_error_response(
                    GroupId::VendorAndroid,
                    cmd.oid.into(),
                    packet_error_status(child),
                )
                .into(),
            },
            ControlPacketChild::None => {
                // [UCI] 4.3.2 Exception Handling for Control Messages
                // The UWBS shall respond to an unknown Command (unknown GID
                // or OID) with a Response having the same GID and OID field
                // values as the Command, followed by a Status field with the
                // value of STATUS_UNKNOWN_GID/STATUS_UNKNOWN_OID respectively
                // and no additional fields.
                tracing::error!("Unsupported gid {:?}", cmd.gid);
                command_error_response(
                    cmd.gid,
                    cmd.payload.first().copied().unwrap_or(0),
                    uci::Status::UnknownGid,
                )
                .into()
            }
        };

        output.0
    }

    pub fn receive_packet(&mut self, packet: Vec<u8>) -> Vec<DeviceEffect> {
        let Some(&[mt_gid, oid]) = packet.first_chunk::<2>() else {
            return Vec::new();
        };
        let mt = parse_message_type(mt_gid);
        match mt {
            MessageType::Data => match DataPacket::decode_full(&packet) {
                Ok(packet) => {
                    let notification = self
                        .data_message_snd(packet)
                        .expect("failed to encode data transfer notification");
                    vec![DeviceEffect::send_control(notification)]
                }
                Err(err) => {
                    tracing::error!("failed to parse incoming Data packet: {err}");
                    Vec::new()
                }
            },
            MessageType::Command => {
                match ControlPacket::decode_full(&packet) {
                    // Parsing error. Determine what error response should be
                    // returned to the host:
                    // - response and notifications are ignored, no response
                    // - if the group id is not known, STATUS_UNKNOWN_GID,
                    // - otherwise, and to simplify the code, STATUS_UNKNOWN_OID is
                    //      always returned. That means that malformed commands
                    //      get the same status code, instead of
                    //      STATUS_SYNTAX_ERROR.
                    Err(_) => {
                        let group_id = mt_gid & 0xf;
                        let opcode_id = oid & 0x3f;

                        let status = if GroupId::try_from(group_id).is_ok() {
                            uci::Status::UnknownOid
                        } else {
                            uci::Status::UnknownGid
                        };
                        // The PDL generated code cannot be used to generate
                        // responses with invalid group identifiers.
                        let response = vec![
                            (u8::from(MessageType::Response) << 5) | group_id,
                            opcode_id,
                            0,
                            1,
                            status.into(),
                        ];
                        vec![DeviceEffect::SendPacket(response)]
                    }

                    // Parsing success, ignore non command packets.
                    Ok(cmd) => self.receive_command(cmd),
                }
            }

            // Message types for notifications and responses ignored
            // by the controller.
            _ => {
                tracing::warn!("received unexpected packet of MT {mt:?}");
                Vec::new()
            }
        }
    }
}

struct Controlee {
    short_address: [u8; 2],
    #[allow(dead_code)]
    sub_session_id: u32,
    #[allow(dead_code)]
    session_key: SubSessionKey,
}

impl From<&uci::Controlee> for Controlee {
    fn from(value: &uci::Controlee) -> Self {
        Controlee {
            short_address: value.short_address,
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::None,
        }
    }
}

impl From<&uci::Controlee_V2_0_16_Byte_Version> for Controlee {
    fn from(value: &uci::Controlee_V2_0_16_Byte_Version) -> Self {
        Controlee {
            short_address: value.short_address,
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::Short(value.subsession_key),
        }
    }
}

impl From<&uci::Controlee_V2_0_32_Byte_Version> for Controlee {
    fn from(value: &uci::Controlee_V2_0_32_Byte_Version) -> Self {
        Controlee {
            short_address: value.short_address,
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::Extended(value.subsession_key),
        }
    }
}
