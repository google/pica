use std::collections::HashMap;
use crate::uci_packets::{DeviceState, SessionState, UciPacketPacket};
use anyhow::{anyhow, Result};
use tokio::sync::mpsc;

use super::session::{Session, MAX_SESSION};

pub const MAX_DEVICE: usize = 4;

pub struct Device {
    mac_address: usize,
    pub state: DeviceState,
    pub sessions: HashMap<u32, Session>,
    pub tx: mpsc::Sender<UciPacketPacket>,
    pub country_code: [u8; 2],
}

impl Device {
    pub fn new(device_handle: usize, tx: mpsc::Sender<UciPacketPacket>) -> Self {
        Device {
            mac_address: device_handle,
            state: DeviceState::DeviceStateReady,
            sessions: Default::default(),
            tx,
            country_code: Default::default(),
        }
    }

    pub fn add_session(&mut self, session: Session) -> Result<()> {
        if self.sessions.len() > MAX_SESSION {
            return Err(anyhow!("Can't add session, maximum number of session reached"));
        }
        self.sessions.insert(
            session.id,
            session
        );
        Ok(())
    }
}
