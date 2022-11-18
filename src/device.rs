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

use crate::position::Position;
use crate::uci_packets::*;
use crate::MacAddress;
use crate::PicaCommand;

use std::collections::HashMap;
use std::iter::Extend;

use tokio::sync::mpsc;

use num_traits::FromPrimitive;

use super::session::{Session, MAX_SESSION};

pub const MAX_DEVICE: usize = 4;
const UCI_VERSION: u16 = 0x1001; // Version 1.1.0
const MAC_VERSION: u16 = 0x3001; // Version 1.3.0
const PHY_VERSION: u16 = 0x3001; // Version 1.3.0
const TEST_VERSION: u16 = 0x1001; // Version 1.1

// Capabilities are vendor defined, Android parses capabilities
// according to these definitions:
// /android/packages/modules/Uwb/service/java/com/android/server/uwb/config/CapabilityParam.java
pub const DEFAULT_CAPS_INFO: &[(CapTlvType, &[u8])] = &[
    // Fira params
    (CapTlvType::SupportedFiraPhyVersionRange, &[1, 1, 1, 3]), // 1.1 - 1.3
    (CapTlvType::SupportedFiraMacVersionRange, &[1, 1, 1, 3]), // 1.1 - 1.3
    (CapTlvType::SupportedDeviceRoles, &[0x3]),                // INTIATOR | RESPONDER
    (CapTlvType::SupportedRangingMethod, &[0x1f]), // DS_TWR_NON_DEFERRED | SS_TWR_NON_DEFERRED | DS_TWR_DEFERRED | SS_TWR_DEFERRED | OWR
    (CapTlvType::SupportedStsConfig, &[0x7]), // STATIC_STS | DYNAMIC_STS | DYNAMIC_STS_RESPONDER_SPECIFIC_SUBSESSION_KEY
    (CapTlvType::SupportedMultiNodeModes, &[0xff]),
    (CapTlvType::SupportedBlockStriding, &[0x1]),
    (CapTlvType::SupportedUwbInitiationTime, &[0x01]),
    (CapTlvType::SupportedChannels, &[0xff]),
    (CapTlvType::SupportedRframeConfig, &[0xff]),
    (CapTlvType::SupportedBprfParameterSets, &[0xff]),
    (CapTlvType::SupportedHprfParameterSets, &[0xff]),
    (CapTlvType::SupportedCcConstraintLength, &[0xff]),
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
    pub tx: mpsc::Sender<UciPacketPacket>,
    pica_tx: mpsc::Sender<PicaCommand>,
    config: HashMap<u8, Vec<u8>>,
    country_code: [u8; 2],

    n_active_sessions: usize,
}

impl Device {
    pub fn new(
        device_handle: usize,
        tx: mpsc::Sender<UciPacketPacket>,
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

    fn set_state(&mut self, device_state: DeviceState) {
        // No transition: ignore
        if device_state == self.state {
            return;
        }

        // Send status notification
        self.state = device_state;
        let tx = self.tx.clone();
        tokio::spawn(async move {
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

    // The fira norm specify to send a response, then reset, then
    // send a notification once the reset is done
    fn command_device_reset(&mut self, cmd: DeviceResetCmdPacket) -> DeviceResetRspPacket {
        let reset_config = cmd.get_reset_config();
        println!("[{}] DeviceReset", self.handle);
        println!("  reset_config={}", reset_config);

        let status = match reset_config {
            ResetConfig::UwbsReset => StatusCode::UciStatusOk,
        };

        *self = Device::new(self.handle, self.tx.clone(), self.pica_tx.clone());

        DeviceResetRspBuilder { status }.build()
    }

    fn command_get_device_info(&self, _cmd: GetDeviceInfoCmdPacket) -> GetDeviceInfoRspPacket {
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

    pub fn command_get_caps_info(&self, cmd: GetCapsInfoCmdPacket) -> GetCapsInfoRspPacket {
        println!("[{}] GetCapsInfo", self.handle);
        assert_eq!(
            cmd.get_packet_boundary_flag(),
            PacketBoundaryFlag::Complete,
            "Boundary flag is true, implement fragmentation"
        );

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

    pub fn command_set_config(&mut self, cmd: SetConfigCmdPacket) -> SetConfigRspPacket {
        println!("[{}] SetConfig", self.handle);
        assert_eq!(self.state, DeviceState::DeviceStateReady); // UCI 6.3
        assert_eq!(
            cmd.get_packet_boundary_flag(),
            PacketBoundaryFlag::Complete,
            "Boundary flag is true, implement fragmentation"
        );

        let (valid_parameters, invalid_config_status) = cmd.get_parameters().iter().fold(
            (HashMap::new(), Vec::new()),
            |(mut valid_parameters, mut invalid_config_status), param| {
                let id = param.id;
                match DeviceConfigId::from_u8(id) {
                    Some(_) => {
                        // TODO: DeviceState is a read only parameter
                        valid_parameters.insert(param.id, param.value.clone());
                    }
                    None => {
                        // TODO: silently ignore vendor parameter
                        invalid_config_status.push(DeviceConfigStatus {
                            parameter_id: id,
                            status: StatusCode::UciStatusInvalidParam,
                        })
                    }
                };
                (valid_parameters, invalid_config_status)
            },
        );

        let (status, parameters) = if invalid_config_status.is_empty() {
            self.config.extend(valid_parameters.into_iter());
            (StatusCode::UciStatusOk, Vec::new())
        } else {
            (StatusCode::UciStatusInvalidParam, invalid_config_status)
        };

        SetConfigRspBuilder { status, parameters }.build()
    }

    pub fn command_get_config(&self, cmd: GetConfigCmdPacket) -> GetConfigRspPacket {
        println!("[{}] GetConfig", self.handle);
        assert_eq!(
            cmd.get_packet_boundary_flag(),
            PacketBoundaryFlag::Complete,
            "Boundary flag is true, implement fragmentation"
        );
        let ids = cmd.get_parameter_ids();

        // TODO: do this config shall be set on device reset
        let (valid_parameters, invalid_parameters) = ids.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut valid_parameters, mut invalid_parameters), id| {
                // UCI Core Section 6.3.2 Table 8
                // UCI Core Section 6.3.2 - Return the Configuration
                // If the status code is ok, return the params
                // If there is at least one invalid param, return the list of invalid params
                // If the ID is not present in our config, return the Type with length = 0
                match self.config.get(id) {
                    Some(value) => valid_parameters.push(DeviceParameter {
                        id: *id,
                        value: value.clone(),
                    }),
                    None => invalid_parameters.push(DeviceParameter {
                        id: *id,
                        value: Vec::new(),
                    }),
                }

                (valid_parameters, invalid_parameters)
            },
        );

        let (status, parameters) = if invalid_parameters.is_empty() {
            (StatusCode::UciStatusOk, valid_parameters)
        } else {
            (StatusCode::UciStatusInvalidParam, invalid_parameters)
        };

        GetConfigRspBuilder { status, parameters }.build()
    }

    fn command_session_init(&mut self, cmd: SessionInitCmdPacket) -> SessionInitRspPacket {
        let session_id = cmd.get_session_id();
        let session_type = cmd.get_session_type();

        println!("[{}] Session init", self.handle);
        println!("  session_id=0x{:x}", session_id);
        println!("  session_type={}", session_type);

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

    fn command_session_deinit(&mut self, cmd: SessionDeinitCmdPacket) -> SessionDeinitRspPacket {
        let session_id = cmd.get_session_id();
        println!("[{}] Session deinit", self.handle);
        println!("  session_id=0x{:x}", session_id);

        let status = if self.sessions.remove(&session_id).is_some() {
            StatusCode::UciStatusOk
        } else {
            StatusCode::UciStatusSessionNotExist
        };

        SessionDeinitRspBuilder { status }.build()
    }

    fn command_session_get_count(
        &self,
        _cmd: SessionGetCountCmdPacket,
    ) -> SessionGetCountRspPacket {
        println!("[{}] Session get count", self.handle);

        SessionGetCountRspBuilder {
            status: StatusCode::UciStatusOk,
            session_count: self.sessions.len() as u8,
        }
        .build()
    }

    fn command_set_country_code(
        &mut self,
        cmd: AndroidSetCountryCodeCmdPacket,
    ) -> AndroidSetCountryCodeRspPacket {
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
        _cmd: AndroidGetPowerStatsCmdPacket,
    ) -> AndroidGetPowerStatsRspPacket {
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

    pub fn command(&mut self, cmd: UciCommandPacket) -> UciResponsePacket {
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
            UciCommandChild::SessionCommand(session_command) => {
                // Session commands directly handled at Device level
                match session_command.specialize() {
                    SessionCommandChild::SessionInitCmd(cmd) => {
                        return self.command_session_init(cmd).into();
                    }
                    SessionCommandChild::SessionDeinitCmd(cmd) => {
                        return self.command_session_deinit(cmd).into();
                    }
                    SessionCommandChild::SessionGetCountCmd(cmd) => {
                        return self.command_session_get_count(cmd).into();
                    }
                    _ => {}
                }

                // Common code for retrieving the session_id in the command
                let session_id = match session_command.specialize() {
                    SessionCommandChild::SessionSetAppConfigCmd(cmd) => cmd.get_session_id(),
                    SessionCommandChild::SessionGetAppConfigCmd(cmd) => cmd.get_session_id(),
                    SessionCommandChild::SessionGetStateCmd(cmd) => cmd.get_session_id(),
                    SessionCommandChild::SessionUpdateControllerMulticastListCmd(cmd) => {
                        cmd.get_session_id()
                    }
                    _ => panic!("Unsupported session command type"),
                };

                if let Some(session) = self.get_session_mut(session_id) {
                    // There is a session matching the session_id in the command
                    // Pass the command through
                    match session_command.specialize() {
                        SessionCommandChild::SessionSetAppConfigCmd(_)
                        | SessionCommandChild::SessionGetAppConfigCmd(_)
                        | SessionCommandChild::SessionGetStateCmd(_)
                        | SessionCommandChild::SessionUpdateControllerMulticastListCmd(_) => {
                            session.session_command(session_command).into()
                        }
                        _ => panic!("Unsupported session command"),
                    }
                } else {
                    // There is no session matching the session_id in the command
                    let status = StatusCode::UciStatusSessionNotExist;
                    match session_command.specialize() {
                        SessionCommandChild::SessionSetAppConfigCmd(_) => {
                            SessionSetAppConfigRspBuilder {
                                status,
                                parameters: Vec::new(),
                            }
                            .build()
                            .into()
                        }
                        SessionCommandChild::SessionGetAppConfigCmd(_) => {
                            SessionGetAppConfigRspBuilder {
                                status,
                                parameters: Vec::new(),
                            }
                            .build()
                            .into()
                        }
                        SessionCommandChild::SessionGetStateCmd(_) => SessionGetStateRspBuilder {
                            status,
                            session_state: SessionState::SessionStateDeinit,
                        }
                        .build()
                        .into(),
                        SessionCommandChild::SessionUpdateControllerMulticastListCmd(_) => {
                            SessionUpdateControllerMulticastListRspBuilder { status }
                                .build()
                                .into()
                        }
                        _ => panic!("Unsupported session command"),
                    }
                }
            }
            UciCommandChild::RangingCommand(ranging_command) => {
                let session_id = ranging_command.get_session_id();
                if let Some(session) = self.get_session_mut(session_id) {
                    // Forward to the proper session
                    let response = session.ranging_command(ranging_command);
                    match response.specialize() {
                        RangingResponseChild::RangeStartRsp(rsp)
                            if rsp.get_status() == StatusCode::UciStatusOk =>
                        {
                            self.n_active_sessions += 1;
                            self.set_state(DeviceState::DeviceStateActive);
                        }
                        RangingResponseChild::RangeStopRsp(rsp)
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
                        RangingCommandChild::RangeStartCmd(_) => {
                            RangeStartRspBuilder { status }.build().into()
                        }
                        RangingCommandChild::RangeStopCmd(_) => {
                            RangeStopRspBuilder { status }.build().into()
                        }
                        RangingCommandChild::RangeGetRangingCountCmd(_) => {
                            RangeGetRangingCountRspBuilder { status, count: 0 }
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
            // TODO: Handle properly without panic
            _ => UciResponseBuilder {
                group_id: GroupId::Core,
                opcode: 0,
                payload: None,
            }
            .build(),
        }
    }
}
