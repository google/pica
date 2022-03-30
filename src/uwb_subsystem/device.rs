use crate::uci_packets::{DeviceState, UciPacketPacket};
use crate::uwb_subsystem::*;
use std::collections::HashMap;

use crate::position::Position;
use anyhow::{anyhow, Result};
use tokio::sync::mpsc;

use super::session::{Session, MAX_SESSION};

pub const MAX_DEVICE: usize = 4;

pub struct Device {
    pub mac_address: usize,
    pub position: Position,
    pub state: DeviceState,
    pub sessions: HashMap<u32, Session>,
    pub tx: mpsc::Sender<UciPacketPacket>,
    pub country_code: [u8; 2],
}

impl Device {
    pub fn new(device_handle: usize, tx: mpsc::Sender<UciPacketPacket>) -> Self {
        Device {
            mac_address: device_handle,
            position: Position::default(),
            state: DeviceState::DeviceStateReady,
            sessions: Default::default(),
            tx,
            country_code: Default::default(),
        }
    }

    pub fn add_session(&mut self, session: Session) -> Result<()> {
        if self.sessions.len() > MAX_SESSION {
            return Err(anyhow!(
                "Can't add session, maximum number of session reached"
            ));
        }
        self.sessions.insert(session.id, session);
        Ok(())
    }
}

impl Pica {
    // The fira norm specify to send a response, then reset, then
    // send a notification once the reset is done
    pub async fn device_reset(
        &mut self,
        device_handle: usize,
        cmd: DeviceResetCmdPacket,
    ) -> Result<()> {
        let reset_config = cmd.get_reset_config();
        println!("[{}] Device Reset", device_handle);
        println!("  reset_config={}", reset_config);
        {
            let mut device = self.get_device(device_handle);
            let status = match reset_config {
                ResetConfig::UwbsReset => StatusCode::UciStatusOk,
            };
            device.state = DeviceState::DeviceStateReady;
            device
                .tx
                .send(DeviceResetRspBuilder { status }.build().into())
                .await?;
        }

        self.devices.insert(
            device_handle,
            Device::new(device_handle, self.devices[&device_handle].tx.clone()),
        );
        Ok(self
            .get_device(device_handle)
            .tx
            .send(
                DeviceStatusNtfBuilder {
                    device_state: DeviceState::DeviceStateReady,
                }
                .build()
                .into(),
            )
            .await?)
    }

    pub async fn get_device_info(
        &mut self,
        device_handle: usize,
        _cmd: GetDeviceInfoCmdPacket,
    ) -> Result<()> {
        // TODO: Implement a fancy build time state machine instead of crash at runtime
        let device = self.get_device(device_handle);
        assert_eq!(device.state, DeviceState::DeviceStateReady);
        Ok(device
            .tx
            .send(
                GetDeviceInfoRspBuilder {
                    status: StatusCode::UciStatusOk,
                    uci_version: 1,
                    mac_version: 1,
                    phy_version: 1,
                    uci_test_version: 1,
                    vendor_spec_info: Vec::new(),
                }
                .build()
                .into(),
            )
            .await?)
    }

    pub async fn get_caps_info(
        &mut self,
        _device_handle: usize,
        _cmd: GetCapsInfoCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    pub async fn set_config(
        &mut self,
        device_handle: usize,
        _cmd: SetConfigCmdPacket,
    ) -> Result<()> {
        let device = self.get_device(device_handle);
        assert_eq!(device.state, DeviceState::DeviceStateReady);
        // TODO: Check if the config is supported
        // Currently we are saying we support everything
        Ok(device
            .tx
            .send(
                SetConfigRspBuilder {
                    status: StatusCode::UciStatusOk,
                    cfg_status: Vec::new(),
                }
                .build()
                .into(),
            )
            .await?)
    }

    pub async fn get_config(
        &mut self,
        _device_handle: usize,
        _cmd: GetConfigCmdPacket,
    ) -> Result<()> {
        todo!()
    }
}
