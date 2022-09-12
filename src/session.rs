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

use crate::uci_packets::AppConfigTlvType;
use crate::uci_packets::*;
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

#[derive(Copy, Clone, FromPrimitive, PartialEq, Eq)]
pub enum DeviceType {
    /// [MAC] 5.1.1 Device controlling the ranging features through Control Messages
    Controller,
    /// [MAC] 5.1.2 Device utilizing the ranging features set through Control Messages
    Controlee,
}

#[derive(Copy, Clone, FromPrimitive)]
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
}

/// cf. [UCI] 8.3 Table 29
#[derive(Clone)]
pub struct AppConfig {
    /// Copy of the valid App Configuration parameters provided by host
    raw: HashMap<AppConfigTlvType, Vec<u8>>,

    device_type: DeviceType,
    _device_role: DeviceRole,
    mac_address_mode: MacAddressMode,
    device_mac_address: MacAddress,
    number_of_controlees: usize,
    dst_mac_addresses: Vec<MacAddress>,
    ranging_interval: time::Duration,
    slot_duration: u16,
    channel_number: ChannelNumber,
    multi_node_mode: MultiNodeMode,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            raw: HashMap::new(),
            mac_address_mode: MacAddressMode::AddressMode0,
            _device_role: DeviceRole::Responder,
            device_type: DeviceType::Controlee,
            ranging_interval: DEFAULT_RANGING_INTERVAL,
            slot_duration: DEFAULT_SLOT_DURATION,
            channel_number: DEFAULT_CHANNEL_NUMBER,
            device_mac_address: MacAddress::Short([0x00, 0x00]),
            number_of_controlees: 0,
            dst_mac_addresses: Vec::new(),
            multi_node_mode: MultiNodeMode::Unicast,
        }
    }
}

fn app_config_has_mandatory_parameters(configs: &[AppConfigParameter]) -> bool {
    const MANDATORY_PARAMETERS: [AppConfigTlvType; 6] = [
        AppConfigTlvType::DeviceRole,
        AppConfigTlvType::MultiNodeMode,
        AppConfigTlvType::NoOfControlee,
        AppConfigTlvType::DeviceMacAddress,
        AppConfigTlvType::DstMacAddress,
        AppConfigTlvType::DeviceType,
    ];

    MANDATORY_PARAMETERS.iter().all(|&mparam| {
        configs
            .iter()
            .any(|param| mparam == AppConfigTlvType::from_u8(param.id).unwrap())
    })
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
            AppConfigTlvType::RangingInterval => {
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
            id => {
                println!("Ignored AppConfig parameter {}", id);
                return Err(StatusCode::UciStatusInvalidParam);
            }
        };

        self.raw.insert(id, value.to_vec());

        Ok(())
    }

    fn get_config(&self, id: AppConfigTlvType) -> Option<Vec<u8>> {
        self.raw.get(&id).cloned()
    }

    fn extend(&mut self, configs: &[AppConfigParameter]) -> Vec<AppConfigStatus> {
        if !app_config_has_mandatory_parameters(configs) {
            // TODO: What shall we do in this situation?
        }

        configs
            .iter()
            .fold(Vec::new(), |mut invalid_parameters, config| {
                match AppConfigTlvType::from_u8(config.id) {
                    Some(id) => match self.set_config(id, &config.value) {
                        Ok(_) => (),
                        Err(status) => invalid_parameters.push(AppConfigStatus {
                            config_id: config.id,
                            status,
                        }),
                    },
                    None => invalid_parameters.push(AppConfigStatus {
                        config_id: config.id,
                        status: StatusCode::UciStatusInvalidParam,
                    }),
                };
                invalid_parameters
            })
    }
}

pub struct Session {
    /// cf. [UCI] 7.1
    state: SessionState,
    /// cf. [UCI] 7.2 Table 13: 4 octets unique random number generated by application
    id: u32,
    device_handle: usize,

    session_type: SessionType,
    pub sequence_number: u32,
    app_config: AppConfig,
    ranging_task: Option<JoinHandle<()>>,
    tx: mpsc::Sender<UciPacketPacket>,
    pica_tx: mpsc::Sender<PicaCommand>,
}

impl Session {
    pub fn new(
        id: u32,
        session_type: SessionType,
        device_handle: usize,
        tx: mpsc::Sender<UciPacketPacket>,
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

    fn set_state(&mut self, session_state: SessionState) {
        // No transition: ignore
        if session_state == self.state {
            return;
        }

        // Send status notification
        self.state = session_state;
        let tx = self.tx.clone();
        let session_id = self.id;
        tokio::spawn(async move {
            tx.send(
                SessionStatusNtfBuilder {
                    session_id,
                    session_state,
                    reason_code: ReasonCode::StateChangeWithSessionManagementCommands,
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

    pub fn init(&mut self) {
        self.set_state(SessionState::SessionStateInit);
    }

    fn command_set_app_config(
        &mut self,
        cmd: SessionSetAppConfigCmdPacket,
    ) -> SessionSetAppConfigRspPacket {
        // TODO properly handle these asserts
        println!(
            "[{}:0x{:x}] Session Set App Config",
            self.device_handle, self.id
        );
        assert_eq!(self.id, cmd.get_session_id());
        assert_eq!(self.session_type, SessionType::FiraRangingSession);

        let (status, invalid_parameters) = if self.state != SessionState::SessionStateInit {
            (StatusCode::UciStatusRejected, Vec::new())
        } else {
            let mut app_config = self.app_config.clone();
            let invalid_parameters = app_config.extend(cmd.get_parameters());
            if invalid_parameters.is_empty() {
                self.app_config = app_config;
                self.set_state(SessionState::SessionStateIdle);
                (StatusCode::UciStatusOk, invalid_parameters)
            } else {
                (StatusCode::UciStatusInvalidParam, invalid_parameters)
            }
        };

        SessionSetAppConfigRspBuilder {
            status,
            parameters: invalid_parameters,
        }
        .build()
    }

    fn command_get_app_config(
        &self,
        cmd: SessionGetAppConfigCmdPacket,
    ) -> SessionGetAppConfigRspPacket {
        println!(
            "[{}:0x{:x}] Session Get App Config",
            self.device_handle, self.id
        );
        assert_eq!(self.id, cmd.get_session_id());

        let (status, valid_parameters) = {
            let (valid_parameters, invalid_parameters) = cmd.get_parameters().iter().fold(
                (Vec::new(), Vec::new()),
                |(mut valid_parameters, mut invalid_parameters), config_id| {
                    match AppConfigTlvType::from_u8(*config_id)
                        .and_then(|id| self.app_config.get_config(id))
                    {
                        Some(value) => valid_parameters.push(AppConfigParameter {
                            id: *config_id,
                            value,
                        }),
                        None => invalid_parameters.push(AppConfigParameter {
                            id: *config_id,
                            value: Vec::new(),
                        }),
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
            parameters: valid_parameters,
        }
        .build()
    }

    fn command_get_state(&self, cmd: SessionGetStateCmdPacket) -> SessionGetStateRspPacket {
        println!("[{}:0x{:x}] Session Get State", self.device_handle, self.id);
        assert_eq!(self.id, cmd.get_session_id());
        SessionGetStateRspBuilder {
            status: StatusCode::UciStatusOk,
            session_state: self.state,
        }
        .build()
    }

    fn command_update_controller_multicast_list(
        &mut self,
        cmd: SessionUpdateControllerMulticastListCmdPacket,
    ) -> SessionUpdateControllerMulticastListRspPacket {
        println!(
            "[{}:0x{:x}] Session Update Controller Multicast List",
            self.device_handle, self.id
        );
        assert_eq!(self.id, cmd.get_session_id());
        let status = {
            if (self.state != SessionState::SessionStateActive
                && self.state != SessionState::SessionStateIdle)
                || self.app_config.device_type != DeviceType::Controller
                || (self.app_config.multi_node_mode != MultiNodeMode::OneToMany
                    && self.app_config.multi_node_mode != MultiNodeMode::ManyToMany)
            {
                StatusCode::UciStatusRejected
            } else {
                let action = UpdateMulticastListAction::from_u8(cmd.get_action()).unwrap();
                let controlees = cmd.get_controlees();

                if action == UpdateMulticastListAction::Add
                    && (controlees.len() + self.app_config.number_of_controlees)
                        > MAX_NUMBER_OF_CONTROLEES
                {
                    StatusCode::UciStatusMulticastListFull
                } else {
                    // TODO properly Add/Remove the controlees and send notif (?)
                    StatusCode::UciStatusOk
                }
            }
        };
        SessionUpdateControllerMulticastListRspBuilder { status }.build()
    }

    fn command_range_start(&mut self, cmd: RangeStartCmdPacket) -> RangeStartRspPacket {
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
            self.set_state(SessionState::SessionStateActive);
            StatusCode::UciStatusOk
        };
        RangeStartRspBuilder { status }.build()
    }

    fn stop_ranging_task(&mut self) {
        if let Some(handle) = &self.ranging_task {
            handle.abort();
            self.ranging_task = None;
        }
    }
    fn command_range_stop(&mut self, cmd: RangeStopCmdPacket) -> RangeStopRspPacket {
        println!("[{}:0x{:x}] Range Stop", self.device_handle, self.id);
        assert_eq!(self.id, cmd.get_session_id());

        let status = if self.state != SessionState::SessionStateActive {
            StatusCode::UciStatusSessionActive
        } else {
            self.stop_ranging_task();
            self.set_state(SessionState::SessionStateIdle);
            StatusCode::UciStatusOk
        };
        RangeStopRspBuilder { status }.build()
    }

    fn command_get_ranging_count(
        &self,
        cmd: RangeGetRangingCountCmdPacket,
    ) -> RangeGetRangingCountRspPacket {
        println!(
            "[{}:0x{:x}] Range Get Ranging Count",
            self.device_handle, self.id
        );
        assert_eq!(self.id, cmd.get_session_id());

        RangeGetRangingCountRspBuilder {
            status: StatusCode::UciStatusOk,
            count: self.sequence_number,
        }
        .build()
    }

    pub fn session_command(&mut self, cmd: SessionCommandPacket) -> SessionResponsePacket {
        match cmd.specialize() {
            SessionCommandChild::SessionSetAppConfigCmd(cmd) => {
                self.command_set_app_config(cmd).into()
            }
            SessionCommandChild::SessionGetAppConfigCmd(cmd) => {
                self.command_get_app_config(cmd).into()
            }
            SessionCommandChild::SessionGetStateCmd(cmd) => self.command_get_state(cmd).into(),
            SessionCommandChild::SessionUpdateControllerMulticastListCmd(cmd) => {
                self.command_update_controller_multicast_list(cmd).into()
            }
            _ => panic!("Unsupported session command"),
        }
    }

    pub fn ranging_command(&mut self, cmd: RangingCommandPacket) -> RangingResponsePacket {
        match cmd.specialize() {
            RangingCommandChild::RangeStartCmd(cmd) => self.command_range_start(cmd).into(),
            RangingCommandChild::RangeStopCmd(cmd) => self.command_range_stop(cmd).into(),
            RangingCommandChild::RangeGetRangingCountCmd(cmd) => {
                self.command_get_ranging_count(cmd).into()
            }
            _ => panic!("Unsupported ranging command"),
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Make sure to abort the ranging task when dropping the session,
        // the default behaviour when dropping a task handle is to detach
        // the task, which is undesirable.
        self.stop_ranging_task();
        self.set_state(SessionState::SessionStateDeinit);
    }
}
