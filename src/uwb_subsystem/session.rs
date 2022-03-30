
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
        println!("session_init");
        let device = self.get_device(device_handle);
        let mut session = Session::default();
        session.state = SessionState::SessionStateInit;
        session.id = cmd.get_session_id();
        session.session_type = cmd.get_session_type();
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
        _device_handle: usize,
        _cmd: SessionDeinitCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    pub async fn session_set_app_config(
        &mut self,
        device_handle: usize,
        cmd: SessionSetAppConfigCmdPacket,
    ) -> Result<()> {
        // TODO: Set session app configuration regardings the incoming cmd
        let device = self.get_device(device_handle);
        let session = device.sessions.get_mut(&cmd.get_session_id()).unwrap();
        assert_eq!(session.state, SessionState::SessionStateInit);
        println!("session_set_app_config");
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
                    session_state: SessionState::SessionStateIdle,
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
        _device_handle: usize,
        _cmd: SessionGetCountCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    pub async fn session_get_state(
        &mut self,
        _device_handle: usize,
        _cmd: SessionGetStateCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    pub async fn session_update_controller_multicast_list(
        &mut self,
        _device_handle: usize,
        _cmd: SessionUpdateControllerMulticastListCmdPacket,
    ) -> Result<()> {
        todo!()
    }
}
