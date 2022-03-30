use tokio::task::JoinHandle;

use crate::uci_packets::{SessionState, SessionType};
use crate::uwb_subsystem::*;

pub const MAX_SESSION: usize = 255;

pub struct Session {
    pub state: SessionState,
    pub id: u32,
    pub session_type: SessionType,
    sequence_number: usize,
    ranging_interval: usize,
    ranging_task: Option<JoinHandle<()>>,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            state: SessionState::SessionStateDeinit,
            id: 0,
            session_type: SessionType::FiraRangingSession,
            sequence_number: 0,
            ranging_interval: 0,
            ranging_task: None,
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

        let device = self.get_device(device_handle);
        let mut session = Session::default();
        session.state = SessionState::SessionStateInit;
        session.id = session_id;
        session.session_type = session_type;
        let status = match device.add_session(session) {
            Ok(_) => StatusCode::UciStatusOk,
            Err(_) => StatusCode::UciStatusFailed,
        };
        device
            .tx
            .send(SessionInitRspBuilder { status: status }.build().into())
            .await?;
        Ok(device
            .tx
            .send(
                SessionStatusNtfBuilder {
                    session_id: cmd.get_session_id(),
                    session_state: SessionState::SessionStateInit,
                    reason_code: ReasonCode::StateChangeWithSessionManagementCommands,
                }
                .build()
                .into(),
            )
            .await?)
    }

    pub async fn session_deinit(
        &mut self,
        device_handle: usize,
        cmd: SessionDeinitCmdPacket,
    ) -> Result<()> {
        let session_id = cmd.get_session_id();
        println!("[{}] Session deinit", device_handle);
        println!("  session_id=0x{:x}", session_id);

        let device = self.get_device(device_handle);
        let (status, notif) = match device.sessions.remove(&session_id) {
            Some(_) => (
                StatusCode::UciStatusOk,
                Some(SessionStatusNtfBuilder {
                    session_id,
                    session_state: SessionState::SessionStateDeinit,
                    reason_code: ReasonCode::StateChangeWithSessionManagementCommands,
                }),
            ),
            None => (StatusCode::UciStatusSesssionNotExist, None),
        };
        let _ = device
            .tx
            .send(SessionDeinitRspBuilder { status }.build().into())
            .await?;
        // Send session state change notification if required.
        if let Some(notif) = notif {
            device.tx.send(notif.build().into()).await?
        }
        Ok(())
    }

    pub async fn session_set_app_config(
        &mut self,
        device_handle: usize,
        cmd: SessionSetAppConfigCmdPacket,
    ) -> Result<()> {
        println!("[{}] Session set app config", device_handle);

        // TODO: Set session app configuration regardings the incoming cmd
        let device = self.get_device(device_handle);
        let session = device.sessions.get_mut(&cmd.get_session_id()).unwrap();
        assert_eq!(session.state, SessionState::SessionStateInit);
        session.state = SessionState::SessionStateIdle;
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

        Ok(device
            .tx
            .send(
                SessionStatusNtfBuilder {
                    session_id: cmd.get_session_id(),
                    session_state: session.state,
                    reason_code: ReasonCode::StateChangeWithSessionManagementCommands,
                }
                .build()
                .into(),
            )
            .await?)
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
        let session_count = device.sessions.len() as u8;
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
        let (status, session_state) = match device.sessions.get(&session_id).map(|s| s.state) {
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
        _device_handle: usize,
        _cmd: SessionUpdateControllerMulticastListCmdPacket,
    ) -> Result<()> {
        todo!()
    }
}
