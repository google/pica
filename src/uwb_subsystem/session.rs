use crate::uci_packets::{AppConfigTlvType, SessionState, SessionType};
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

    pub app_config: AppConfig,
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
                self.mac_address_mode = MacAddressMode::from_u8(value[0]).unwrap()
            }
            AppConfigTlvType::RangingInterval => {
                self.ranging_interval = u32::from_le_bytes(value[..].try_into().unwrap())
            }
            AppConfigTlvType::SlotDuration => {
                self.slot_duration = u16::from_le_bytes(value[..].try_into().unwrap())
            }
            AppConfigTlvType::ChannelNumber => self.channel_number = value[0],
            AppConfigTlvType::DstMacAddress => {
                let mac_address_size = match self.mac_address_mode {
                    MacAddressMode::AddressMode0 => 2,
                    MacAddressMode::AddressMode1 | MacAddressMode::AddressMode2 => 8,
                };
                if value.len() % mac_address_size != 0 {
                    return Err(StatusCode::UciStatusInvalidParam);
                }
                self.dst_mac_addresses = value
                    .chunks(mac_address_size)
                    .map(|c| match self.mac_address_mode {
                        MacAddressMode::AddressMode0 => {
                            u16::from_le_bytes(c.try_into().unwrap()) as u64
                        }
                        MacAddressMode::AddressMode1 | MacAddressMode::AddressMode2 => {
                            u64::from_le_bytes(c.try_into().unwrap())
                        }
                    })
                    .collect();
            }
            _ => {
                self.raw.insert(id.to_u8().unwrap(), value.clone());
            }
        };

        Ok(())
    }

    pub fn get_config(&self, id: AppConfigTlvType) -> Option<Vec<u8>> {
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

    pub fn extend(&mut self, configs: &Vec<AppConfigParameter>) -> Vec<AppConfigStatus> {
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
    pub fn start_ranging(&mut self, device_handle: usize, tx: mpsc::Sender<PicaCommand>) {
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

    pub fn stop_ranging(&mut self) {
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
