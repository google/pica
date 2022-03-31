use crate::uci_packets::{
    AppConfigTlv, AppConfigTlvType, MacAddressIndicator, SessionState, SessionType,
};
use anyhow::Result;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time;

use crate::uwb_subsystem::*;

pub const MAX_SESSION: usize = 255;

pub struct Session {
    pub state: SessionState,
    pub id: u32,

    pub session_type: SessionType,

    pub sequence_number: u32,
    ranging_interval: usize,

    dst_mac_addresses: Vec<u64>,
    ranging_task: Option<JoinHandle<()>>,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            state: SessionState::SessionStateDeinit,
            id: 0,
            session_type: SessionType::FiraRangingSession,
            sequence_number: 0,
            ranging_interval: 1000,
            dst_mac_addresses: Vec::new(),
            ranging_task: None,
        }
    }
}

impl Session {
    fn start_ranging(&mut self, device_handle: usize, tx: mpsc::Sender<PicaCommand>) {
        assert!(self.ranging_task.is_none());
        let session_id = self.id;
        let ranging_interval = self.ranging_interval as u64;
        self.ranging_task = Some(tokio::spawn(async move {
            loop {
                time::sleep(Duration::from_millis(ranging_interval as u64)).await;
                tx.send(PicaCommand::Ranging(device_handle, session_id))
                    .await;
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
        &self.dst_mac_addresses
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

        let device = self.get_device_mut(device_handle);
        let (status, session_state) = match device.get_session_mut(session_id as u32) {
            Some(session) if session.state == SessionState::SessionStateInit => {
                session.state = SessionState::SessionStateIdle;

                match session.session_type {
                    SessionType::FiraRangingSession => {
                        cmd.get_tlvs()
                            .iter()
                            .for_each(|config| match config.cfg_id {
                                // AppConfigTlvType::DeviceRole => {}
                                // AppConfigTlvType::MultiNodeMode => {}
                                // AppConfigTlvType::NoOfControlee => {}
                                // AppConfigTlvType::DeviceMacAddress => {}
                                // AppConfigTlvType::DeviceType => {}
                                AppConfigTlvType::MacAddressMode => {
                                    assert!(config.v.len() == 1);
                                    assert!(
                                        config.v[0] == MacAddressIndicator::ExtendedAddress as u8
                                    );
                                }
                                AppConfigTlvType::DstMacAddress => {
                                    let mac_address_size = std::mem::size_of::<u64>();
                                    assert!(config.v.len() > 0);
                                    assert!((config.v.len() % mac_address_size) == 0);

                                    for i in 0..(config.v.len() / mac_address_size) {
                                        let mut mac_address: u64 = 0;
                                        for j in 0..mac_address_size {
                                            mac_address = (mac_address << 8)
                                                + config.v[i * mac_address_size + j] as u64;
                                        }
                                        session.dst_mac_addresses.push(mac_address);
                                    }
                                }
                                _ => {}
                            });
                    }
                    _ => anyhow::bail!("Unsupported session type"),
                }

                device
                    .tx
                    .send(
                        SessionSetAppConfigRspBuilder {
                            status: StatusCode::UciStatusOk,
                            cfg_status: Vec::new(),
                        }
                        .build()
                        .into(),
                    )
                    .await?;

                // TODO: Set session app configuration regardings the incoming cmd
                (StatusCode::UciStatusOk, SessionState::SessionStateIdle)
            }
            Some(_) => (
                StatusCode::UciStatusSesssionActive,
                SessionState::SessionStateActive,
            ),
            None => (
                StatusCode::UciStatusSesssionNotExist,
                SessionState::SessionStateDeinit,
            ),
        };

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
        _device_handle: usize,
        _cmd: SessionGetAppConfigCmdPacket,
    ) -> Result<()> {
        todo!()
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
        let status = match device.get_session_mut(session_id as u32) {
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
        let status = match device.get_session_mut(session_id as u32) {
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
