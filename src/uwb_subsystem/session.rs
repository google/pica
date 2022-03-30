use crate::uci_packets::{SessionState, SessionType};
use tokio::task::JoinHandle;

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
