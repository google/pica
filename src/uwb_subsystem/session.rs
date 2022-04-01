use crate::uci_packets::{AppConfigTlvType, MacAddressIndicator, SessionState, SessionType};
use anyhow::Result;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::uwb_subsystem::*;

pub const MAX_SESSION: usize = 255;
pub const DEFAULT_CHANNEL_NUMBER: u8 = 9;
pub const DEFAULT_RANGING_INTERVAL: u32 = 200; // milliseconds
pub const DEFAULT_SLOT_DURATION: u16 = 2400; // RTSU unit

#[derive(Copy, Clone, FromPrimitive)]
pub enum DeviceType {
    Controlee,
    Controller,
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum DeviceRole {
    Responder,
    Initiator,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum MacAddressMode {
    AddressMode0,
    AddressMode1,
    AddressMode2,
}

#[derive(Clone)]
pub struct AppConfig {
    raw: HashMap<u8, Vec<u8>>,
    pub mac_address_mode: MacAddressMode,
    pub device_type: Option<DeviceType>,
    pub ranging_interval: u32,
    pub slot_duration: u16,
    pub channel_number: u8,
    pub device_role: Option<DeviceRole>,
    pub device_mac_address: Option<MacAddress>,
    pub dst_mac_addresses: Vec<MacAddress>,
}

pub struct Session {
    pub state: SessionState,
    pub id: u32,

    pub session_type: SessionType,
    pub sequence_number: u32,

    app_config: AppConfig,
    ranging_task: Option<JoinHandle<()>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            raw: HashMap::new(),
            mac_address_mode: MacAddressMode::AddressMode0,
            device_role: None,
            device_type: None,
            ranging_interval: DEFAULT_RANGING_INTERVAL,
            slot_duration: DEFAULT_SLOT_DURATION,
            channel_number: DEFAULT_CHANNEL_NUMBER,
            device_mac_address: None,
            dst_mac_addresses: Vec::new(),
        }
    }
}

impl AppConfig {
    fn set_config(
        &mut self,
        id: AppConfigTlvType,
        value: &Vec<u8>,
    ) -> std::result::Result<(), StatusCode> {
        // TODO: raise Err(StatusCode::UciStatusInvalidParam) when
        // an invalid parameter value is received.
        match id {
            AppConfigTlvType::MacAddressMode => {
                assert!(value.len() == 1);
                assert!(value[0] == MacAddressIndicator::ExtendedAddress as u8);
            }
            AppConfigTlvType::RangingInterval => {
                self.ranging_interval = u32::from_le_bytes(value[..].try_into().unwrap());
            }
            AppConfigTlvType::SlotDuration => {
                self.slot_duration = u16::from_le_bytes(value[..].try_into().unwrap());
            }
            AppConfigTlvType::ChannelNumber => {
                self.channel_number = value[0];
            }
            AppConfigTlvType::DstMacAddress => {
                let mac_address_size = std::mem::size_of::<u64>();
                assert!(value.len() > 0);
                assert!((value.len() % mac_address_size) == 0);

                for i in 0..(value.len() / mac_address_size) {
                    self.dst_mac_addresses
                        .push(u64::from_le_bytes(value[i..i + 8].try_into().unwrap()));
                }
            }
            _ => {
                self.raw.insert(id.to_u8().unwrap(), value.clone());
            }
        };
        Ok(())
    }

    fn get_config(&self, id: AppConfigTlvType) -> Option<Vec<u8>> {
        Some(match id {
            AppConfigTlvType::MacAddressMode => [self.mac_address_mode.to_u8().unwrap()].into(),
            AppConfigTlvType::RangingInterval => self.ranging_interval.to_le_bytes().into(),
            AppConfigTlvType::SlotDuration => self.slot_duration.to_le_bytes().into(),
            AppConfigTlvType::ChannelNumber => [self.channel_number].into(),
            AppConfigTlvType::DstMacAddress => {
                self.dst_mac_addresses
                    .iter()
                    .fold(Vec::new(), |mut value, mac_address| {
                        value.extend(mac_address.to_le_bytes().into_iter());
                        value
                    })
            }
            _ => return self.raw.get(&id.to_u8().unwrap()).map(|v| v.clone()),
        })
    }

    fn extend(&mut self, configs: &Vec<AppConfigParameter>) -> Vec<AppConfigStatus> {
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

impl Default for Session {
    fn default() -> Self {
        Session {
            state: SessionState::SessionStateDeinit,
            id: 0,
            session_type: SessionType::FiraRangingSession,
            sequence_number: 0,
            app_config: AppConfig::default(),
            ranging_task: None,
        }
    }
}

impl Session {
    fn start_ranging(&mut self, device_handle: usize, tx: mpsc::Sender<PicaCommand>) {
        assert!(self.ranging_task.is_none());
        let session_id = self.id;
        let ranging_interval = self.app_config.ranging_interval as u64;
        self.ranging_task = Some(tokio::spawn(async move {
            loop {
                time::sleep(Duration::from_millis(ranging_interval as u64)).await;
                tx.send(PicaCommand::Ranging(device_handle, session_id))
                    .await
                    .unwrap();
            }
        }))
    }

    fn stop_ranging(&mut self) {
        if let Some(handle) = &self.ranging_task {
            handle.abort();
            self.ranging_task = None;
        }
    }

    pub fn get_dst_mac_addresses(&self) -> &Vec<u64> {
        &self.app_config.dst_mac_addresses
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Make sure to abort the ranging task when dropping the session,
        // the default behaviour when dropping a task handle is to detach
        // the task, which is undesirable.
        if let Some(handle) = &self.ranging_task {
            handle.abort();
        }
    }
}

impl Pica {
    pub async fn session_init(
        &mut self,
        device_handle: usize,
        cmd: SessionInitCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        let session_type = cmd.get_session_type();
        println!("[{}] Session init", device_handle);
        println!("  session_id=0x{:x}", session_id);
        println!("  session_type={}", session_type);

        let device = self.get_device_mut(device_handle);
        let mut session = Session::default();
        session.state = SessionState::SessionStateInit;
        session.id = session_id;
        session.session_type = session_type;
        let status = device.add_session(session);

        device
            .tx
            .send(SessionInitRspBuilder { status }.build().into())
            .await?;

        if status == StatusCode::UciStatusOk {
            device
                .send_session_status_notification(
                    session_id,
                    SessionState::SessionStateInit,
                    ReasonCode::StateChangeWithSessionManagementCommands,
                )
                .await?;
        }

        Ok(())
    }

    pub async fn session_deinit(
        &mut self,
        device_handle: usize,
        cmd: SessionDeinitCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Session deinit", device_handle);
        println!("  session_id=0x{:x}", session_id);

        let device = self.get_device_mut(device_handle);
        let status = match device.remove_session(session_id) {
            Ok(_) => StatusCode::UciStatusOk,
            Err(_) => StatusCode::UciStatusSesssionNotExist,
        };

        device
            .tx
            .send(SessionDeinitRspBuilder { status }.build().into())
            .await?;

        if status == StatusCode::UciStatusOk {
            device
                .send_session_status_notification(
                    session_id,
                    SessionState::SessionStateDeinit,
                    ReasonCode::StateChangeWithSessionManagementCommands,
                )
                .await?;
        }
        Ok(())
    }

    pub async fn session_set_app_config(
        &mut self,
        device_handle: usize,
        cmd: SessionSetAppConfigCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Session set app config", device_handle);
        println!("  session_id=0x{}", session_id);
        assert_eq!(
            cmd.get_packet_boundary_flag(),
            PacketBoundaryFlag::Complete,
            "Boundary flag is true, implement fragmentation"
        );

        let device = self.get_device_mut(device_handle);
        let (status, session_state, parameters) = match device.get_session_mut(session_id) {
            Some(session)
                if session.state == SessionState::SessionStateInit
                    && session.session_type == SessionType::FiraRangingSession =>
            {
                session.state = SessionState::SessionStateIdle;
                let mut app_config = session.app_config.clone();
                let invalid_parameters = app_config.extend(cmd.get_parameters());
                let status = if invalid_parameters.is_empty() {
                    session.app_config = app_config;
                    session.state = SessionState::SessionStateIdle;
                    StatusCode::UciStatusOk
                } else {
                    StatusCode::UciStatusInvalidParam
                };

                (status, session.state, invalid_parameters)
            }
            Some(_) => (
                StatusCode::UciStatusSesssionActive,
                SessionState::SessionStateActive,
                Vec::new(),
            ),
            None => (
                StatusCode::UciStatusSesssionNotExist,
                SessionState::SessionStateDeinit,
                Vec::new(),
            ),
        };

        device
            .tx
            .send(
                SessionSetAppConfigRspBuilder { status, parameters }
                    .build()
                    .into(),
            )
            .await?;

        if status == StatusCode::UciStatusOk {
            device
                .send_session_status_notification(
                    session_id,
                    session_state,
                    ReasonCode::StateChangeWithSessionManagementCommands,
                )
                .await?
        }
        Ok(())
    }

    pub async fn session_get_app_config(
        &mut self,
        device_handle: usize,
        cmd: SessionGetAppConfigCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Session get app config", device_handle);
        println!("  session_id=0x{:x}", session_id);

        let device = self.get_device(device_handle);
        let (status, parameters) = match device.get_session(session_id) {
            Some(session) => {
                let (valid_parameters, invalid_parameters) = cmd.get_parameters().iter().fold(
                    (Vec::new(), Vec::new()),
                    |(mut valid_parameters, mut invalid_parameters), config_id| {
                        match AppConfigTlvType::from_u8(*config_id)
                            .and_then(|id| session.app_config.get_config(id))
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
                    (StatusCode::UciStatusFailed, invalid_parameters)
                }
            }
            None => (StatusCode::UciStatusSesssionNotExist, Vec::new()),
        };

        Ok(device
            .tx
            .send(
                SessionGetAppConfigRspBuilder { status, parameters }
                    .build()
                    .into(),
            )
            .await?)
    }

    pub async fn session_get_count(
        &mut self,
        device_handle: usize,
        _cmd: SessionGetCountCmdPacket,
    ) -> Result<()> {
        println!("[{}] Session get count", device_handle);

        let device = self.get_device(device_handle);
        let session_count = device.get_session_cnt() as u8;
        Ok(device
            .tx
            .send(
                SessionGetCountRspBuilder {
                    status: StatusCode::UciStatusOk,
                    session_count,
                }
                .build()
                .into(),
            )
            .await?)
    }

    pub async fn session_get_state(
        &mut self,
        device_handle: usize,
        cmd: SessionGetStateCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Session get state", device_handle);
        println!("  session_id=0x{:x}", session_id);

        let device = self.get_device(device_handle);
        let (status, session_state) = match device.get_session(session_id).map(|s| s.state) {
            Some(state) => (StatusCode::UciStatusOk, state),
            None => (
                StatusCode::UciStatusSesssionNotExist,
                SessionState::SessionStateInit,
            ),
        };
        Ok(device
            .tx
            .send(
                SessionGetStateRspBuilder {
                    status,
                    session_state,
                }
                .build()
                .into(),
            )
            .await?)
    }

    pub async fn session_update_controller_multicast_list(
        &mut self,
        device_handle: usize,
        cmd: SessionUpdateControllerMulticastListCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Session get state", device_handle);
        println!("  session_id=0x{:x}", session_id);

        Ok(())
    }

    pub async fn range_start(
        &mut self,
        device_handle: usize,
        cmd: RangeStartCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Range start", device_handle);
        println!("  session_id=0x{:x}", session_id);

        let pica_tx = self.tx.clone();
        let device = self.get_device_mut(device_handle);
        let status = match device.get_session_mut(session_id) {
            Some(session) if session.state == SessionState::SessionStateIdle => {
                session.start_ranging(device_handle, pica_tx);
                session.state = SessionState::SessionStateActive;
                StatusCode::UciStatusOk
            }
            Some(session) if session.state == SessionState::SessionStateActive => {
                StatusCode::UciStatusSesssionActive
            }
            Some(_) => StatusCode::UciStatusFailed,
            None => StatusCode::UciStatusSesssionNotExist,
        };

        device
            .tx
            .send(RangeStartRspBuilder { status }.build().into())
            .await?;

        if status == StatusCode::UciStatusOk {
            device
                .send_session_status_notification(
                    session_id,
                    SessionState::SessionStateActive,
                    ReasonCode::StateChangeWithSessionManagementCommands,
                )
                .await?;
            // TODO when one session becomes active
            device
                .send_device_status_notification(DeviceState::DeviceStateActive)
                .await?;
        }

        Ok(())
    }

    pub async fn range_stop(
        &mut self,
        device_handle: usize,
        cmd: RangeStopCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Range stop", device_handle);
        println!("  session_id=0x{:x}", session_id);

        let device = self.get_device_mut(device_handle);
        let status = match device.get_session_mut(session_id) {
            Some(session) if session.state == SessionState::SessionStateActive => {
                session.stop_ranging();
                session.state = SessionState::SessionStateIdle;
                StatusCode::UciStatusOk
            }
            Some(_) => StatusCode::UciStatusFailed,
            None => StatusCode::UciStatusSesssionNotExist,
        };

        device
            .tx
            .send(RangeStopRspBuilder { status }.build().into())
            .await?;

        if status == StatusCode::UciStatusOk {
            device
                .send_session_status_notification(
                    session_id,
                    SessionState::SessionStateIdle,
                    ReasonCode::StateChangeWithSessionManagementCommands,
                )
                .await?;
            // TODO when all sessions becomes idle
            device
                .send_device_status_notification(DeviceState::DeviceStateReady)
                .await?;
        }

        Ok(())
    }

    pub async fn range_get_ranging_count(
        &mut self,
        device_handle: usize,
        cmd: RangeGetRangingCountCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Range get count", device_handle);
        println!("  session_id=0x{:x}", session_id);

        let device = self.get_device(device_handle);
        let (status, count) = match device.get_session(session_id) {
            Some(session) if session.state == SessionState::SessionStateActive => {
                (StatusCode::UciStatusOk, session.sequence_number as u32)
            }
            Some(_) => (StatusCode::UciStatusFailed, 0),
            None => (StatusCode::UciStatusSesssionNotExist, 0),
        };

        device
            .tx
            .send(
                RangeGetRangingCountRspBuilder { status, count }
                    .build()
                    .into(),
            )
            .await?;
        Ok(())
    }
}
