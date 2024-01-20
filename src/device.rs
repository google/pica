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

use crate::packets::uci::*;
use crate::position::Position;
use crate::MacAddress;
use crate::PicaCommand;

use std::collections::HashMap;
use std::iter::Extend;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time;

use super::session::{Session, MAX_SESSION};

pub const MAX_DEVICE: usize = 4;
const UCI_VERSION: u16 = 0x0002; // Version 2.0
const MAC_VERSION: u16 = 0x3001; // Version 1.3.0
const PHY_VERSION: u16 = 0x3001; // Version 1.3.0
const TEST_VERSION: u16 = 0x1001; // Version 1.1

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

pub struct Device {
    handle: usize,
    pub mac_address: MacAddress,
    pub position: Position,
    /// [UCI] 5. UWBS Device State Machine
    state: DeviceState,
    sessions: HashMap<u32, Session>,
    pub tx: mpsc::Sender<ControlPacket>,
    pica_tx: mpsc::Sender<PicaCommand>,
    config: HashMap<DeviceConfigId, Vec<u8>>,
    country_code: [u8; 2],

    pub n_active_sessions: usize,
}

impl Device {
    pub fn new(
        device_handle: usize,
        tx: mpsc::Sender<ControlPacket>,
        pica_tx: mpsc::Sender<PicaCommand>,
    ) -> Self {
        let mac_address = {
            let handle = device_handle as u16;
            MacAddress::Short(handle.to_be_bytes())
        };
        Device {
            handle: device_handle,
            mac_address,
            position: Position::default(),
            state: DeviceState::DeviceStateError, // Will be overwitten
            sessions: Default::default(),
            tx,
            pica_tx,
            config: HashMap::new(),
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
            tx.send(DeviceStatusNtfBuilder { device_state }.build().into())
                .await
                .unwrap()
        });
    }

    pub fn init(&mut self) {
        self.set_state(DeviceState::DeviceStateReady);
    }

    pub fn get_session(&self, session_id: u32) -> Option<&Session> {
        self.sessions.get(&session_id)
    }

    pub fn get_session_mut(&mut self, session_id: u32) -> Option<&mut Session> {
        self.sessions.get_mut(&session_id)
    }

    pub fn can_start_ranging(&self, peer_session: &Session, session_id: u32) -> bool {
        match self.get_session(session_id) {
            Some(session) => {
                session.session_state() == SessionState::SessionStateActive
                    && session
                        .app_config
                        .is_compatible_for_ranging(&peer_session.app_config)
            }
            None => false,
        }
    }

    // The fira norm specify to send a response, then reset, then
    // send a notification once the reset is done
    fn command_device_reset(&mut self, cmd: DeviceResetCmd) -> DeviceResetRsp {
        let reset_config = cmd.get_reset_config();
        println!("[{}] DeviceReset", self.handle);
        println!("  reset_config={:?}", reset_config);

        let status = match reset_config {
            ResetConfig::UwbsReset => StatusCode::UciStatusOk,
        };
        *self = Device::new(self.handle, self.tx.clone(), self.pica_tx.clone());
        self.init();

        DeviceResetRspBuilder { status }.build()
    }

    fn command_get_device_info(&self, _cmd: GetDeviceInfoCmd) -> GetDeviceInfoRsp {
        // TODO: Implement a fancy build time state machine instead of crash at runtime
        println!("[{}] GetDeviceInfo", self.handle);
        assert_eq!(self.state, DeviceState::DeviceStateReady);
        GetDeviceInfoRspBuilder {
            status: StatusCode::UciStatusOk,
            uci_version: UCI_VERSION,
            mac_version: MAC_VERSION,
            phy_version: PHY_VERSION,
            uci_test_version: TEST_VERSION,
            vendor_spec_info: Vec::new(),
        }
        .build()
    }

    pub fn command_get_caps_info(&self, _cmd: GetCapsInfoCmd) -> GetCapsInfoRsp {
        println!("[{}] GetCapsInfo", self.handle);

        let caps = DEFAULT_CAPS_INFO
            .iter()
            .map(|(id, value)| CapTlv {
                t: *id,
                v: (*value).into(),
            })
            .collect();

        GetCapsInfoRspBuilder {
            status: StatusCode::UciStatusOk,
            tlvs: caps,
        }
        .build()
    }

    pub fn command_set_config(&mut self, cmd: SetConfigCmd) -> SetConfigRsp {
        println!("[{}] SetConfig", self.handle);
        assert_eq!(self.state, DeviceState::DeviceStateReady); // UCI 6.3

        let (valid_parameters, invalid_config_status) = cmd.get_tlvs().iter().fold(
            (HashMap::new(), Vec::new()),
            |(mut valid_parameters, invalid_config_status), param| {
                // TODO: DeviceState is a read only parameter
                valid_parameters.insert(param.cfg_id, param.v.clone());

                (valid_parameters, invalid_config_status)
            },
        );

        let (status, parameters) = if invalid_config_status.is_empty() {
            self.config.extend(valid_parameters);
            (StatusCode::UciStatusOk, Vec::new())
        } else {
            (StatusCode::UciStatusInvalidParam, invalid_config_status)
        };

        SetConfigRspBuilder {
            cfg_status: parameters,
            status,
        }
        .build()
    }

    pub fn command_get_config(&self, cmd: GetConfigCmd) -> GetConfigRsp {
        println!("[{}] GetConfig", self.handle);

        // TODO: do this config shall be set on device reset
        let ids = cmd.get_cfg_id();
        let (valid_parameters, invalid_parameters) = ids.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut valid_parameters, mut invalid_parameters), id| {
                // UCI Core Section 6.3.2 Table 8
                // UCI Core Section 6.3.2 - Return the Configuration
                // If the status code is ok, return the params
                // If there is at least one invalid param, return the list of invalid params
                // If the ID is not present in our config, return the Type with length = 0
                match DeviceConfigId::try_from(*id) {
                    Ok(cfg_id) => match self.config.get(&cfg_id) {
                        Some(value) => valid_parameters.push(DeviceConfigTlv {
                            cfg_id,
                            v: value.clone(),
                        }),
                        None => invalid_parameters.push(DeviceConfigTlv {
                            cfg_id,
                            v: Vec::new(),
                        }),
                    },
                    Err(_) => println!("Failed to parse config id: {:?}", id),
                }

                (valid_parameters, invalid_parameters)
            },
        );

        let (status, parameters) = if invalid_parameters.is_empty() {
            (StatusCode::UciStatusOk, valid_parameters)
        } else {
            (StatusCode::UciStatusInvalidParam, invalid_parameters)
        };

        GetConfigRspBuilder {
            status,
            tlvs: parameters,
        }
        .build()
    }

    fn command_session_init(&mut self, cmd: SessionInitCmd) -> SessionInitRsp {
        let session_id = cmd.get_session_id();
        let session_type = cmd.get_session_type();

        println!("[{}] Session init", self.handle);
        println!("  session_id=0x{:x}", session_id);
        println!("  session_type={:?}", session_type);

        let status = if self.sessions.len() >= MAX_SESSION {
            StatusCode::UciStatusMaxSessionsExceeded
        } else {
            match self.sessions.insert(
                session_id,
                Session::new(
                    session_id,
                    session_type,
                    self.handle,
                    self.tx.clone(),
                    self.pica_tx.clone(),
                ),
            ) {
                Some(_) => StatusCode::UciStatusSessionDuplicate,
                None => {
                    // Should not fail
                    self.get_session_mut(session_id).unwrap().init();
                    StatusCode::UciStatusOk
                }
            }
        };

        SessionInitRspBuilder { status }.build()
    }

    fn command_session_deinit(&mut self, cmd: SessionDeinitCmd) -> SessionDeinitRsp {
        let session_id = cmd.get_session_token();
        println!("[{}] Session deinit", self.handle);
        println!("  session_id=0x{:x}", session_id);

        let status = match self.sessions.get_mut(&session_id) {
            Some(session) => {
                if session.state == SessionState::SessionStateActive {
                    self.n_active_sessions -= 1;
                    if self.n_active_sessions == 0 {
                        self.set_state(DeviceState::DeviceStateReady);
                    }
                }
                self.sessions.remove(&session_id);
                StatusCode::UciStatusOk
            }
            None => StatusCode::UciStatusSessionNotExist,
        };
        SessionDeinitRspBuilder { status }.build()
    }

    fn command_session_get_count(&self, _cmd: SessionGetCountCmd) -> SessionGetCountRsp {
        println!("[{}] Session get count", self.handle);

        SessionGetCountRspBuilder {
            status: StatusCode::UciStatusOk,
            session_count: self.sessions.len() as u8,
        }
        .build()
    }

    fn command_set_country_code(
        &mut self,
        cmd: AndroidSetCountryCodeCmd,
    ) -> AndroidSetCountryCodeRsp {
        let country_code = *cmd.get_country_code();
        println!("[{}] Set country code", self.handle);
        println!("  country_code={},{}", country_code[0], country_code[1]);

        self.country_code = country_code;
        AndroidSetCountryCodeRspBuilder {
            status: StatusCode::UciStatusOk,
        }
        .build()
    }

    fn command_get_power_stats(
        &mut self,
        _cmd: AndroidGetPowerStatsCmd,
    ) -> AndroidGetPowerStatsRsp {
        println!("[{}] Get power stats", self.handle);

        // TODO
        AndroidGetPowerStatsRspBuilder {
            stats: PowerStats {
                status: StatusCode::UciStatusOk,
                idle_time_ms: 0,
                tx_time_ms: 0,
                rx_time_ms: 0,
                total_wake_count: 0,
            },
        }
        .build()
    }

    pub fn data_message_snd(&mut self, data: DataPacket) -> SessionControlNotification {
        match data.specialize() {
            DataPacketChild::DataMessageSnd(data_msg_snd) => {
                let session_token = data_msg_snd.get_session_handle();
                if let Some(session) = self.get_session_mut(session_token) {
                    session.data_message_snd(data_msg_snd)
                } else {
                    DataTransferStatusNtfBuilder {
                        session_token,
                        status: DataTransferNtfStatusCode::UciDataTransferStatusErrorRejected,
                        tx_count: 1, // TODO: support for retries?
                        uci_sequence_number: 0,
                    }
                    .build()
                    .into()
                }
            }
            DataPacketChild::DataMessageRcv(data_msg_rcv) => {
                // This function should not be passed anything besides DataMessageSnd
                let session_token = data_msg_rcv.get_session_handle();
                DataTransferStatusNtfBuilder {
                    session_token,
                    status: DataTransferNtfStatusCode::UciDataTransferStatusInvalidFormat,
                    tx_count: 1, // TODO: support for retries?
                    uci_sequence_number: 0,
                }
                .build()
                .into()
            }
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn command(&mut self, cmd: UciCommand) -> UciResponse {
        match cmd.specialize() {
            // Handle commands for this device
            UciCommandChild::CoreCommand(core_command) => match core_command.specialize() {
                CoreCommandChild::DeviceResetCmd(cmd) => self.command_device_reset(cmd).into(),
                CoreCommandChild::GetDeviceInfoCmd(cmd) => self.command_get_device_info(cmd).into(),
                CoreCommandChild::GetCapsInfoCmd(cmd) => self.command_get_caps_info(cmd).into(),
                CoreCommandChild::SetConfigCmd(cmd) => self.command_set_config(cmd).into(),
                CoreCommandChild::GetConfigCmd(cmd) => self.command_get_config(cmd).into(),
                _ => panic!("Unsupported core command"),
            },
            // Handle commands for session management
            UciCommandChild::SessionConfigCommand(session_command) => {
                // Session commands directly handled at Device level
                match session_command.specialize() {
                    SessionConfigCommandChild::SessionInitCmd(cmd) => {
                        return self.command_session_init(cmd).into();
                    }
                    SessionConfigCommandChild::SessionDeinitCmd(cmd) => {
                        return self.command_session_deinit(cmd).into();
                    }
                    SessionConfigCommandChild::SessionGetCountCmd(cmd) => {
                        return self.command_session_get_count(cmd).into();
                    }
                    _ => {}
                }

                // Common code for retrieving the session_id in the command
                let session_id = match session_command.specialize() {
                    SessionConfigCommandChild::SessionSetAppConfigCmd(cmd) => {
                        cmd.get_session_token()
                    }
                    SessionConfigCommandChild::SessionGetAppConfigCmd(cmd) => {
                        cmd.get_session_token()
                    }
                    SessionConfigCommandChild::SessionGetStateCmd(cmd) => cmd.get_session_token(),
                    SessionConfigCommandChild::SessionUpdateControllerMulticastListCmd(cmd) => {
                        cmd.get_session_token()
                    }
                    _ => panic!("Unsupported session command type"),
                };

                if let Some(session) = self.get_session_mut(session_id) {
                    // There is a session matching the session_id in the command
                    // Pass the command through
                    match session_command.specialize() {
                        SessionConfigCommandChild::SessionSetAppConfigCmd(_)
                        | SessionConfigCommandChild::SessionGetAppConfigCmd(_)
                        | SessionConfigCommandChild::SessionGetStateCmd(_)
                        | SessionConfigCommandChild::SessionUpdateControllerMulticastListCmd(_) => {
                            session.session_command(session_command).into()
                        }
                        _ => panic!("Unsupported session command"),
                    }
                } else {
                    // There is no session matching the session_id in the command
                    let status = StatusCode::UciStatusSessionNotExist;
                    match session_command.specialize() {
                        SessionConfigCommandChild::SessionSetAppConfigCmd(_) => {
                            SessionSetAppConfigRspBuilder {
                                cfg_status: Vec::new(),
                                status,
                            }
                            .build()
                            .into()
                        }
                        SessionConfigCommandChild::SessionGetAppConfigCmd(_) => {
                            SessionGetAppConfigRspBuilder {
                                status,
                                tlvs: Vec::new(),
                            }
                            .build()
                            .into()
                        }
                        SessionConfigCommandChild::SessionGetStateCmd(_) => {
                            SessionGetStateRspBuilder {
                                status,
                                session_state: SessionState::SessionStateDeinit,
                            }
                            .build()
                            .into()
                        }
                        SessionConfigCommandChild::SessionUpdateControllerMulticastListCmd(_) => {
                            SessionUpdateControllerMulticastListRspBuilder { status }
                                .build()
                                .into()
                        }
                        _ => panic!("Unsupported session command"),
                    }
                }
            }
            UciCommandChild::SessionControlCommand(ranging_command) => {
                let session_id = ranging_command.get_session_id();
                if let Some(session) = self.get_session_mut(session_id) {
                    // Forward to the proper session
                    let response = session.ranging_command(ranging_command);
                    match response.specialize() {
                        SessionControlResponseChild::SessionStartRsp(rsp)
                            if rsp.get_status() == StatusCode::UciStatusOk =>
                        {
                            self.n_active_sessions += 1;
                            self.set_state(DeviceState::DeviceStateActive);
                        }
                        SessionControlResponseChild::SessionStopRsp(rsp)
                            if rsp.get_status() == StatusCode::UciStatusOk =>
                        {
                            assert!(self.n_active_sessions > 0);
                            self.n_active_sessions -= 1;
                            if self.n_active_sessions == 0 {
                                self.set_state(DeviceState::DeviceStateReady);
                            }
                        }
                        _ => {}
                    }
                    response.into()
                } else {
                    let status = StatusCode::UciStatusSessionNotExist;
                    match ranging_command.specialize() {
                        SessionControlCommandChild::SessionStartCmd(_) => {
                            SessionStartRspBuilder { status }.build().into()
                        }
                        SessionControlCommandChild::SessionStopCmd(_) => {
                            SessionStopRspBuilder { status }.build().into()
                        }
                        SessionControlCommandChild::SessionGetRangingCountCmd(_) => {
                            SessionGetRangingCountRspBuilder { status, count: 0 }
                                .build()
                                .into()
                        }
                        _ => panic!("Unsupported ranging command"),
                    }
                }
            }

            UciCommandChild::AndroidCommand(android_command) => {
                match android_command.specialize() {
                    AndroidCommandChild::AndroidSetCountryCodeCmd(cmd) => {
                        self.command_set_country_code(cmd).into()
                    }
                    AndroidCommandChild::AndroidGetPowerStatsCmd(cmd) => {
                        self.command_get_power_stats(cmd).into()
                    }
                    _ => panic!("Unsupported Android command"),
                }
            }
            UciCommandChild::UciVendor_9_Command(vendor_command) => UciVendor_9_ResponseBuilder {
                opcode: vendor_command.get_opcode(),
                payload: Some(vec![u8::from(StatusCode::UciStatusRejected)].into()),
            }
            .build()
            .into(),
            UciCommandChild::UciVendor_A_Command(vendor_command) => UciVendor_A_ResponseBuilder {
                opcode: vendor_command.get_opcode(),
                payload: Some(vec![u8::from(StatusCode::UciStatusRejected)].into()),
            }
            .build()
            .into(),
            UciCommandChild::UciVendor_B_Command(vendor_command) => UciVendor_B_ResponseBuilder {
                opcode: vendor_command.get_opcode(),
                payload: Some(vec![u8::from(StatusCode::UciStatusRejected)].into()),
            }
            .build()
            .into(),
            UciCommandChild::UciVendor_E_Command(vendor_command) => UciVendor_E_ResponseBuilder {
                opcode: vendor_command.get_opcode(),
                payload: Some(vec![u8::from(StatusCode::UciStatusRejected)].into()),
            }
            .build()
            .into(),
            UciCommandChild::UciVendor_F_Command(vendor_command) => UciVendor_F_ResponseBuilder {
                opcode: vendor_command.get_opcode(),
                payload: Some(vec![u8::from(StatusCode::UciStatusRejected)].into()),
            }
            .build()
            .into(),
            // TODO: Handle properly without panic
            _ => UciResponseBuilder {
                gid: GroupId::Core,
                opcode: 0,
                payload: None,
            }
            .build(),
        }
    }
}
