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

use crate::packets::uci::{self, *};
use crate::MacAddress;
use crate::PicaCommand;

use std::collections::HashMap;
use std::time::Duration;

use pdl_runtime::Packet;
use tokio::sync::mpsc;
use tokio::time;

use super::app_config::SubSessionKey;
use super::session::Session;
use super::UciPacket;

pub const MAX_DEVICE: usize = 4;
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
    (CapTlvType::SupportedRangingMethod, &[0x1f]), // DS_TWR_NON_DEFERRED | SS_TWR_NON_DEFERRED | DS_TWR_DEFERRED | SS_TWR_DEFERRED | OWR
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
    // CCC params
    (CapTlvType::CccSupportedVersions, &[1, 0]),
    (CapTlvType::CccSupportedUwbConfigs, &[0]),
    (CapTlvType::CccSupportedPulseShapeCombos, &[0]),
    (CapTlvType::CccSupportedRanMultiplier, &[0, 0, 0, 0]),
    (CapTlvType::CccSupportedChapsPerSlot, &[0xff]),
    (CapTlvType::CccSupportedSyncCodes, &[0xff, 0xff, 0xff, 0xff]),
    (CapTlvType::CccSupportedChannels, &[0xff]),
    (
        CapTlvType::CccSupportedHoppingConfigModesAndSequences,
        &[0xff],
    ),
];

/// [UCI] 8.2 Device Configuration Parameters
pub struct DeviceConfig {
    device_state: DeviceState,
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
            device_state: DeviceState::DeviceStateError,
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
    pub mac_address: MacAddress,
    config: DeviceConfig,
    /// [UCI] 5. UWBS Device State Machine
    state: DeviceState,
    sessions: HashMap<u32, Session>,
    pub tx: mpsc::UnboundedSender<UciPacket>,
    pica_tx: mpsc::Sender<PicaCommand>,
    country_code: [u8; 2],
    pub n_active_sessions: usize,
}

impl Device {
    pub fn new(
        handle: usize,
        mac_address: MacAddress,
        tx: mpsc::UnboundedSender<UciPacket>,
        pica_tx: mpsc::Sender<PicaCommand>,
    ) -> Self {
        Device {
            handle,
            mac_address,
            is_reset: false,
            config: Default::default(),
            state: DeviceState::DeviceStateError, // Will be overwitten
            sessions: Default::default(),
            tx,
            pica_tx,
            country_code: Default::default(),
            n_active_sessions: 0,
        }
    }

    pub fn set_state(&mut self, device_state: DeviceState) {
        // No transition: ignore
        if device_state == self.state {
            return;
        }

        // Send status notification
        self.state = device_state;
        let tx = self.tx.clone();
        tokio::spawn(async move {
            time::sleep(Duration::from_millis(5)).await;
            tx.send(
                CoreDeviceStatusNtf { device_state }
                    .encode_to_vec()
                    .unwrap(),
            )
            .unwrap()
        });
    }

    pub fn init(&mut self) {
        self.set_state(DeviceState::DeviceStateReady);
    }

    pub fn session(&self, session_id: u32) -> Option<&Session> {
        self.sessions.get(&session_id)
    }

    pub fn session_mut(&mut self, session_id: u32) -> Option<&mut Session> {
        self.sessions.get_mut(&session_id)
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

    pub fn can_start_data_transfer(&self, session_id: u32) -> bool {
        match self.session(session_id) {
            Some(session) => {
                session.session_state() == SessionState::SessionStateActive
                    && session.session_type() == SessionType::FiraRangingAndInBandDataSession
                    && session.app_config.can_start_data_transfer()
            }
            None => false,
        }
    }

    pub fn can_receive_data_transfer(&self, session_id: u32) -> bool {
        match self.session(session_id) {
            Some(session) => {
                session.session_state() == SessionState::SessionStateActive
                    && session.session_type() == SessionType::FiraRangingAndInBandDataSession
                    && session.app_config.can_receive_data_transfer()
            }
            None => false,
        }
    }

    // Send a response or notification to the Host.
    fn send_raw_control(&mut self, packet: Vec<u8>) {
        let _ = self.tx.send(packet);
    }

    // Send a response or notification to the Host.
    fn send_control(&mut self, packet: impl Packet) {
        self.send_raw_control(packet.encode_to_vec().unwrap());
    }

    // The fira norm specify to send a response, then reset, then
    // send a notification once the reset is done
    fn core_device_reset(&mut self, cmd: CoreDeviceResetCmd) -> CoreDeviceResetRsp {
        let reset_config = cmd.reset_config;
        log::debug!("[{}] DeviceReset", self.handle);
        log::debug!("  reset_config={:?}", reset_config);

        *self = Device::new(
            self.handle,
            self.mac_address,
            self.tx.clone(),
            self.pica_tx.clone(),
        );
        self.is_reset = true;
        self.init();

        CoreDeviceResetRsp {
            status: uci::Status::Ok,
        }
    }

    fn core_get_device_info(&self, _cmd: CoreGetDeviceInfoCmd) -> CoreGetDeviceInfoRsp {
        // TODO: Implement a fancy build time state machine instead of crash at runtime
        log::debug!("[{}] GetDeviceInfo", self.handle);
        assert_eq!(self.state, DeviceState::DeviceStateReady);
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
        log::debug!("[{}] GetCapsInfo", self.handle);

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
        log::debug!("[{}] SetConfig", self.handle);
        assert_eq!(self.state, DeviceState::DeviceStateReady); // UCI 6.3

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
                    log::warn!("unknown config parameter id 0x{:02x}", *id);
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
        log::debug!("[{}] GetConfig", self.handle);

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
                    value: vec![self.config.device_state.into()],
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

    fn session_init(&mut self, cmd: SessionInitCmd) -> SessionInitRsp {
        let session_id = cmd.session_id;
        let session_type = cmd.session_type;

        log::debug!("[{}] Session init", self.handle);
        log::debug!("  session_id=0x{:x}", session_id);
        log::debug!("  session_type={:?}", session_type);

        let status = if self.sessions.len() >= MAX_SESSION {
            uci::Status::ErrorMaxSessionsExceeded
        } else {
            match self.sessions.insert(
                session_id,
                Session::new(session_id, session_type, self.handle, self.tx.clone()),
            ) {
                Some(_) => uci::Status::ErrorSessionDuplicate,
                None => {
                    // Should not fail
                    self.session_mut(session_id).unwrap().init();
                    uci::Status::Ok
                }
            }
        };

        SessionInitRsp { status }
    }

    fn session_deinit(&mut self, cmd: SessionDeinitCmd) -> SessionDeinitRsp {
        let session_id = cmd.session_token;
        log::debug!("[{}] Session deinit", self.handle);
        log::debug!("  session_id=0x{:x}", session_id);

        let status = match self.sessions.get_mut(&session_id) {
            Some(session) => {
                if session.state == SessionState::SessionStateActive {
                    self.n_active_sessions -= 1;
                    if self.n_active_sessions == 0 {
                        self.set_state(DeviceState::DeviceStateReady);
                    }
                }
                self.sessions.remove(&session_id);
                uci::Status::Ok
            }
            None => uci::Status::ErrorSessionNotExist,
        };
        SessionDeinitRsp { status }
    }

    fn session_get_count(&self, _cmd: SessionGetCountCmd) -> SessionGetCountRsp {
        log::debug!("[{}] Session get count", self.handle);

        SessionGetCountRsp {
            status: uci::Status::Ok,
            session_count: self.sessions.len() as u8,
        }
    }

    fn session_set_app_config(&mut self, cmd: SessionSetAppConfigCmd) -> SessionSetAppConfigRsp {
        let session_handle = cmd.session_token;

        log::debug!(
            "[{}:0x{:x}] Session Set App Config",
            self.handle,
            session_handle
        );

        let Some(session) = self.sessions.get_mut(&session_handle) else {
            return SessionSetAppConfigRsp {
                cfg_status: Vec::new(),
                status: uci::Status::ErrorSessionNotExist,
            };
        };

        assert!(
            session.session_type == SessionType::FiraRangingSession
                || session.session_type == SessionType::FiraRangingAndInBandDataSession
        );

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
                };
            }
        }

        let (status, invalid_parameters) = if session.state != SessionState::SessionStateInit
            && session.state != SessionState::SessionStateActive
        {
            (uci::Status::Rejected, Vec::new())
        } else {
            let mut app_config = session.app_config.clone();
            let mut invalid_parameters = vec![];
            for cfg in cmd.tlvs {
                match app_config.set(cfg.cfg_id, &cfg.v) {
                    Ok(_) => (),
                    Err(_) => invalid_parameters.push(AppConfigStatus {
                        cfg_id: cfg.cfg_id,
                        status: uci::Status::InvalidParam,
                    }),
                }
            }

            // [UCI] 7.5.1 Configuration of a Session
            // This section defines the mandatory APP Configuration Parameters to be applied
            // by the Host for FiRa defined UWB Session types. The Host shall apply these
            // mandatory configurations to move the Session State from SESSION_STATE_INIT
            // to SESSION_STATE_IDLE.
            //
            // - DEVICE_ROLE
            // - MULTI_NODE_MODE
            // - RANGING_ROUND_USAGE
            // - DEVICE_MAC_ADDRESS
            // - DEVICE_TYPE (see Note1)
            // - SCHEDULE_MODE
            if app_config.device_role.is_none()
                || app_config.multi_node_mode.is_none()
                || app_config.ranging_round_usage.is_none()
                || app_config.device_mac_address.is_none()
                || app_config.schedule_mode.is_none()
            {
                log::error!(
                    "[{}:0x{:x}] missing mandatory APP config parameters",
                    self.handle,
                    session_handle
                );
                return SessionSetAppConfigRsp {
                    status: uci::Status::Rejected,
                    cfg_status: vec![],
                };
            }

            if invalid_parameters.is_empty() {
                session.app_config = app_config;
                if session.state == SessionState::SessionStateInit {
                    session.set_state(
                        SessionState::SessionStateIdle,
                        ReasonCode::StateChangeWithSessionManagementCommands,
                    );
                }
                (uci::Status::Ok, invalid_parameters)
            } else {
                (uci::Status::InvalidParam, invalid_parameters)
            }
        };

        SessionSetAppConfigRsp {
            status,
            cfg_status: invalid_parameters,
        }
    }

    fn session_get_app_config(&self, cmd: SessionGetAppConfigCmd) -> SessionGetAppConfigRsp {
        let session_handle = cmd.session_token;

        log::debug!(
            "[{}:0x{:x}] Session Get App Config",
            self.handle,
            session_handle
        );

        let Some(session) = self.sessions.get(&session_handle) else {
            return SessionGetAppConfigRsp {
                tlvs: vec![],
                status: uci::Status::ErrorSessionNotExist,
            };
        };

        let (status, valid_parameters) = {
            let mut valid_parameters = vec![];
            let mut invalid_parameters = vec![];
            for id in cmd.app_cfg {
                match session.app_config.get(id) {
                    Ok(value) => valid_parameters.push(AppConfigTlv {
                        cfg_id: id,
                        v: value,
                    }),
                    Err(_) => invalid_parameters.push(AppConfigTlv {
                        cfg_id: id,
                        v: vec![],
                    }),
                }
            }

            if invalid_parameters.is_empty() {
                (uci::Status::Ok, valid_parameters)
            } else {
                (uci::Status::Failed, Vec::new())
            }
        };

        SessionGetAppConfigRsp {
            status,
            tlvs: valid_parameters,
        }
    }

    fn session_get_state(&self, cmd: SessionGetStateCmd) -> SessionGetStateRsp {
        let session_handle = cmd.session_token;

        log::debug!("[{}:0x{:x}] Session Get State", self.handle, session_handle);

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
    ) -> SessionUpdateControllerMulticastListRsp {
        let session_handle = cmd.session_token;

        log::debug!(
            "[{}:0x{:x}] Session Update Controller Multicast List",
            self.handle,
            session_handle
        );

        let Some(session) = self.sessions.get_mut(&session_handle) else {
            return SessionUpdateControllerMulticastListRsp {
                status: uci::Status::ErrorSessionNotExist,
                controlee_status: vec![],
            };
        };

        if (session.state != SessionState::SessionStateActive
            && session.state != SessionState::SessionStateIdle)
            || session.app_config.device_type != Some(DeviceType::Controller)
            || session.app_config.multi_node_mode != Some(MultiNodeMode::OneToMany)
        {
            return SessionUpdateControllerMulticastListRsp {
                status: uci::Status::Rejected,
                controlee_status: vec![],
            };
        }
        let action = cmd.action;
        let mut dst_addresses = session.app_config.dst_mac_address.clone();
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
                    };
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
                    };
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
                    };
                }
            }
        };
        let mut controlee_status_ntf = Vec::new();
        let mut controlee_status_rsp = Vec::new();
        let mut status = uci::Status::Ok;

        match action {
            UpdateMulticastListAction::AddControlee
            | UpdateMulticastListAction::AddControleeWithShortSubSessionKey
            | UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey => {
                new_controlees.iter().for_each(|controlee| {
                    let mut update_status = MulticastUpdateStatus::OkMulticastListUpdate;
                    if (action == UpdateMulticastListAction::AddControleeWithShortSubSessionKey
                        || action
                            == UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey)
                        && session.app_config.sts_config
                            != uci::StsConfig::ProvisionedForResponderSubSessionKey
                    {
                        // If Action is 0x02 or 0x03 for STS_CONFIG values other than
                        // 0x04, the UWBS shall return SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF
                        // with Status set to STATUS_ERROR_SUB_SESSION_KEY_NOT_APPLICABLE for each
                        // Controlee in the Controlee List.
                        status = uci::Status::Failed;
                        update_status = MulticastUpdateStatus::ErrorSubSessionKeyNotApplicable;
                        controlee_status_ntf.push(ControleeStatus {
                            mac_address: match controlee.short_address {
                                MacAddress::Short(address) => address,
                                MacAddress::Extended(_) => {
                                    panic!("Extended address is not supported!")
                                }
                            },
                            status: update_status,
                        });
                    } else {
                        if !dst_addresses.contains(&controlee.short_address) {
                            if dst_addresses.len() == MAX_NUMBER_OF_CONTROLEES {
                                status = uci::Status::ErrorMulticastListFull;
                                update_status = MulticastUpdateStatus::ErrorMulticastListFull;
                            } else {
                                dst_addresses.push(controlee.short_address);
                            };
                        } else {
                            status = uci::Status::Failed;
                            update_status = MulticastUpdateStatus::ErrorAddressAlreadyPresent;
                        }
                        controlee_status_rsp.push(ControleeStatus {
                            mac_address: match controlee.short_address {
                                MacAddress::Short(address) => address,
                                MacAddress::Extended(_) => {
                                    panic!("Extended address is not supported!")
                                }
                            },
                            status: update_status,
                        });
                    }
                });
            }
            UpdateMulticastListAction::RemoveControlee => {
                new_controlees.iter().for_each(|controlee: &Controlee| {
                    let pica_tx = self.pica_tx.clone();
                    let address = controlee.short_address;
                    let attempt_count = session.app_config.in_band_termination_attempt_count;
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
                            tokio::spawn(async move {
                                for _ in 0..attempt_count {
                                    pica_tx
                                        .send(PicaCommand::StopRanging(address, session_handle))
                                        .await
                                        .unwrap()
                                }
                            });
                        }
                        controlee_status_ntf.push(ControleeStatus {
                            mac_address: match address {
                                MacAddress::Short(addr) => addr,
                                MacAddress::Extended(_) => {
                                    panic!("Extended address is not supported!")
                                }
                            },
                            status: update_status,
                        });
                    }
                    controlee_status_rsp.push(ControleeStatus {
                        mac_address: match address {
                            MacAddress::Short(addr) => addr,
                            MacAddress::Extended(_) => panic!("Extended address is not supported!"),
                        },
                        status: update_status,
                    });
                });
                // Following requirements are applicable when the action is set to Delete (Action field set to 0x01):
                // When the command is received while the Session State is SESSION_STATE_ACTIVE,
                // For requested Controlees present in the multicast list,
                // UWBS shall send the SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF and the
                // corresponding Controlee status shall be set to STATUS_OK_MULTICAST_LIST_UPDATE
                // in the Status List of SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF.
                if session.state == SessionState::SessionStateActive {
                    let tx = self.tx.clone();
                    tokio::spawn(async move {
                        // Sleep for 5ms to make sure the notification is not being
                        // sent before the response.
                        // TODO(#84) remove the sleep.
                        time::sleep(Duration::from_millis(5)).await;
                        tx.send(
                            SessionUpdateControllerMulticastListNtf {
                                controlee_status: controlee_status_ntf,
                                session_token: session_handle,
                            }
                            .encode_to_vec()
                            .unwrap(),
                        )
                        .unwrap()
                    });
                }
            }
        }
        session.app_config.number_of_controlees = dst_addresses.len() as u8;
        session.app_config.dst_mac_address = dst_addresses.clone();
        // If the multicast list becomes empty, the UWBS shall move the session to
        // SESSION_STATE_IDLE by sending the SESSION_STATUS_NTF with Reason Code
        // set to ERROR_INVALID_NUM_OF_CONTROLEES.
        if session.app_config.dst_mac_address.is_empty() {
            session.set_state(
                SessionState::SessionStateIdle,
                ReasonCode::ErrorInvalidNumOfControlees,
            )
        }
        SessionUpdateControllerMulticastListRsp {
            status,
            controlee_status: controlee_status_rsp,
        }
    }

    fn session_start(&mut self, cmd: SessionStartCmd) -> SessionStartRsp {
        let session_id = cmd.session_id;

        log::debug!("[{}:0x{:x}] Session Start", self.handle, session_id);

        let Some(session) = self.sessions.get_mut(&session_id) else {
            return SessionStartRsp {
                status: uci::Status::ErrorSessionNotExist,
            };
        };

        if session.state != SessionState::SessionStateIdle {
            return SessionStartRsp {
                status: uci::Status::ErrorSessionNotConfigured,
            };
        }

        assert!(session.ranging_task.is_none());

        let ranging_interval =
            time::Duration::from_millis(session.app_config.ranging_duration as u64);

        let tx = self.pica_tx.clone();
        let handle = self.handle;
        session.ranging_task = Some(tokio::spawn(async move {
            loop {
                time::sleep(ranging_interval).await;
                tx.send(PicaCommand::Ranging(handle, session_id))
                    .await
                    .unwrap();
            }
        }));

        session.set_state(
            SessionState::SessionStateActive,
            ReasonCode::StateChangeWithSessionManagementCommands,
        );

        self.n_active_sessions += 1;
        self.set_state(DeviceState::DeviceStateActive);

        SessionStartRsp {
            status: uci::Status::Ok,
        }
    }

    fn session_stop(&mut self, cmd: SessionStopCmd) -> SessionStopRsp {
        let session_id = cmd.session_id;

        log::debug!("[{}:0x{:x}] Session Stop", self.handle, session_id);

        let Some(session) = self.sessions.get_mut(&session_id) else {
            return SessionStopRsp {
                status: uci::Status::ErrorSessionNotExist,
            };
        };

        if session.state != SessionState::SessionStateActive {
            return SessionStopRsp {
                status: uci::Status::ErrorSessionActive,
            };
        }

        session.stop_ranging_task();
        session.set_state(
            SessionState::SessionStateIdle,
            ReasonCode::StateChangeWithSessionManagementCommands,
        );

        self.n_active_sessions -= 1;
        if self.n_active_sessions == 0 {
            self.set_state(DeviceState::DeviceStateReady);
        }

        SessionStopRsp {
            status: uci::Status::Ok,
        }
    }

    fn session_get_ranging_count(
        &self,
        cmd: SessionGetRangingCountCmd,
    ) -> SessionGetRangingCountRsp {
        let session_id = cmd.session_id;

        log::debug!(
            "[{}:0x{:x}] Session Get Ranging Count",
            self.handle,
            session_id
        );

        let Some(session) = self.sessions.get(&session_id) else {
            return SessionGetRangingCountRsp {
                status: uci::Status::ErrorSessionNotExist,
                count: 0,
            };
        };

        SessionGetRangingCountRsp {
            status: uci::Status::Ok,
            count: session.sequence_number,
        }
    }

    fn android_set_country_code(
        &mut self,
        cmd: AndroidSetCountryCodeCmd,
    ) -> AndroidSetCountryCodeRsp {
        let country_code = cmd.country_code;

        log::debug!("[{}] Set country code", self.handle);
        log::debug!("  country_code={},{}", country_code[0], country_code[1]);

        self.country_code = country_code;
        AndroidSetCountryCodeRsp {
            status: uci::Status::Ok,
        }
    }

    fn android_get_power_stats(
        &mut self,
        _cmd: AndroidGetPowerStatsCmd,
    ) -> AndroidGetPowerStatsRsp {
        log::debug!("[{}] Get power stats", self.handle);

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

    pub fn data_message_snd(&mut self, data: DataPacket) -> ControlPacket {
        log::debug!("[{}] data_message_send", self.handle);
        match data
            .specialize()
            .expect("failed to parse Data packet child")
        {
            DataPacketChild::DataMessageSnd(data_msg_snd) => {
                let session_token = data_msg_snd.session_handle;
                if let Some(session) = self.session_mut(session_token) {
                    session.data_message_snd(data_msg_snd)
                } else {
                    SessionDataTransferStatusNtf {
                        session_token,
                        status: DataTransferNtfStatusCode::UciDataTransferStatusErrorRejected,
                        tx_count: 1, // TODO: support for retries?
                        uci_sequence_number: 0,
                    }
                    .try_into()
                    .unwrap()
                }
            }
            DataPacketChild::DataMessageRcv(data_msg_rcv) => {
                // This function should not be passed anything besides DataMessageSnd
                let session_token = data_msg_rcv.session_handle;
                SessionDataTransferStatusNtf {
                    session_token,
                    status: DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat,
                    tx_count: 1, // TODO: support for retries?
                    uci_sequence_number: 0,
                }
                .try_into()
                .unwrap()
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn receive_command(&mut self, cmd: ControlPacket) -> ControlPacket {
        use AndroidPacketChild::*;
        use ControlPacketChild::*;
        use CorePacketChild::*;
        use SessionConfigPacketChild::*;
        use SessionControlPacketChild::*;

        // Check whether the first command received is the Core Device
        // Reset command. The controller responds with Device Status
        // Notification with DEVICE_STATE_ERROR otherwise.
        if !self.is_reset && !cmd.is_core_device_reset_cmd() {
            return uci::CoreDeviceStatusNtf {
                device_state: DeviceState::DeviceStateError,
            }
            .try_into()
            .unwrap();
        }

        match cmd
            .specialize()
            .expect("Failed to parse Control packet child")
        {
            CorePacket(cmd) => match cmd.specialize() {
                Ok(CoreDeviceResetCmd(cmd)) => self.core_device_reset(cmd).try_into().unwrap(),
                Ok(CoreGetDeviceInfoCmd(cmd)) => self.core_get_device_info(cmd).try_into().unwrap(),
                Ok(CoreGetCapsInfoCmd(cmd)) => self.core_get_caps_info(cmd).try_into().unwrap(),
                Ok(CoreSetConfigCmd(cmd)) => self.core_set_config(cmd).try_into().unwrap(),
                Ok(CoreGetConfigCmd(cmd)) => self.core_get_config(cmd).try_into().unwrap(),
                _ => uci::CorePacket {
                    mt: uci::MessageType::Response,
                    oid: cmd.oid,
                    payload: vec![0x1, uci::Status::SyntaxError.into()],
                }
                .try_into()
                .unwrap(),
            },
            SessionConfigPacket(cmd) => match cmd.specialize() {
                Ok(SessionInitCmd(cmd)) => self.session_init(cmd).try_into().unwrap(),
                Ok(SessionDeinitCmd(cmd)) => self.session_deinit(cmd).try_into().unwrap(),
                Ok(SessionGetCountCmd(cmd)) => self.session_get_count(cmd).try_into().unwrap(),
                Ok(SessionSetAppConfigCmd(cmd)) => {
                    self.session_set_app_config(cmd).try_into().unwrap()
                }
                Ok(SessionGetAppConfigCmd(cmd)) => {
                    self.session_get_app_config(cmd).try_into().unwrap()
                }
                Ok(SessionGetStateCmd(cmd)) => self.session_get_state(cmd).try_into().unwrap(),
                Ok(SessionUpdateControllerMulticastListCmd(cmd)) => self
                    .session_update_controller_multicast_list(cmd)
                    .try_into()
                    .unwrap(),
                _ => uci::SessionConfigPacket {
                    mt: uci::MessageType::Response,
                    oid: cmd.oid,
                    payload: vec![0x1, uci::Status::SyntaxError.into()],
                }
                .try_into()
                .unwrap(),
            },
            SessionControlPacket(cmd) => match cmd.specialize() {
                Ok(SessionStartCmd(cmd)) => self.session_start(cmd).try_into().unwrap(),
                Ok(SessionStopCmd(cmd)) => self.session_stop(cmd).try_into().unwrap(),
                Ok(SessionGetRangingCountCmd(cmd)) => {
                    self.session_get_ranging_count(cmd).try_into().unwrap()
                }
                _ => uci::SessionControlPacket {
                    mt: uci::MessageType::Response,
                    oid: cmd.oid,
                    payload: vec![0x1, uci::Status::SyntaxError.into()],
                }
                .try_into()
                .unwrap(),
            },
            AndroidPacket(cmd) => match cmd.specialize() {
                Ok(AndroidSetCountryCodeCmd(cmd)) => {
                    self.android_set_country_code(cmd).try_into().unwrap()
                }
                Ok(AndroidGetPowerStatsCmd(cmd)) => {
                    self.android_get_power_stats(cmd).try_into().unwrap()
                }
                _ => uci::AndroidPacket {
                    mt: uci::MessageType::Response,
                    oid: cmd.oid,
                    payload: vec![0x1, uci::Status::SyntaxError.into()],
                }
                .try_into()
                .unwrap(),
            },
            ControlPacketChild::None
                if matches!(
                    cmd.mt,
                    uci::MessageType::Response | uci::MessageType::Notification
                ) =>
            {
                unreachable!("Unhandled control messsage with type {:?}", cmd.mt);
            }
            ControlPacketChild::None => {
                // [UCI] 4.3.2 Exception Handling for Control Messages
                // The UWBS shall respond to an unknown Command (unknown GID
                // or OID) with a Response having the same GID and OID field
                // values as the Command, followed by a Status field with the
                // value of STATUS_UNKNOWN_GID/STATUS_UNKNOWN_OID respectively
                // and no additional fields.
                log::error!("Unsupported gid {:?}", cmd.gid);
                ControlPacket {
                    mt: uci::MessageType::Response,
                    gid: cmd.gid,
                    payload: vec![
                        cmd.payload[0],
                        cmd.payload[1],
                        0x1,
                        uci::Status::UnknownGid.into(),
                    ],
                }
            }
        }
    }

    pub fn receive_packet(&mut self, packet: Vec<u8>) {
        let mt = parse_message_type(packet[0]);
        match mt {
            MessageType::Data => match DataPacket::decode_full(&packet) {
                Ok(packet) => {
                    let notification = self.data_message_snd(packet);
                    self.send_control(notification)
                }
                Err(err) => log::error!("failed to parse incoming Data packet: {}", err),
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
                        let group_id = packet[0] & 0xf;
                        let opcode_id = packet[1] & 0x3f;

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
                        self.send_raw_control(response)
                    }

                    // Parsing success, ignore non command packets.
                    Ok(cmd) => {
                        let response = self.receive_command(cmd);
                        self.send_control(response)
                    }
                }
            }

            // Message types for notifications and responses ignored
            // by the controller.
            _ => log::warn!("received unexpected packet of MT {:?}", mt),
        }
    }
}

struct Controlee {
    short_address: MacAddress,
    #[allow(dead_code)]
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
