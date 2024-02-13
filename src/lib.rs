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

use anyhow::Result;
use bytes::Bytes;
use pdl_runtime::Packet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc, oneshot};

mod packets;
mod pcapng;

use packets::uci::StatusCode as UciStatusCode;
use packets::uci::*;

mod device;
use device::{Device, MAX_DEVICE};

mod session;
use session::MAX_SESSION;

mod mac_address;
pub use mac_address::MacAddress;

use crate::session::RangeDataNtfConfig;

pub type UciPacket = Vec<u8>;

/// Handle allocated for created devices or anchors.
/// The handle is unique across the lifetime of the Pica context
/// and callers may assume that one handle is never reused.
pub type Handle = usize;

/// Ranging measurement produced by a ranging estimator.
#[derive(Clone, Copy, Default, Debug)]
pub struct RangingMeasurement {
    pub range: u16,
    pub azimuth: i16,
    pub elevation: i8,
}

/// Trait matching the capabilities of a ranging estimator.
/// The estimator manages the position of the devices, and chooses
/// the algorithm used to generate the ranging measurements.
pub trait RangingEstimator {
    /// Evaluate the ranging measurement for the two input devices
    /// identified by their respective handle. The result is a triplet
    /// containing the range, azimuth, and elevation of the right device
    /// relative to the left device.
    fn estimate(&self, left: &Handle, right: &Handle) -> Result<RangingMeasurement>;
}

/// Pica emulation environment.
/// All the devices added to this environment are emulated as if they were
/// from the same physical space.
pub struct Pica {
    counter: usize,
    devices: HashMap<Handle, Device>,
    anchors: HashMap<MacAddress, Anchor>,
    command_rx: mpsc::Receiver<PicaCommand>,
    command_tx: mpsc::Sender<PicaCommand>,
    event_tx: broadcast::Sender<PicaEvent>,
    #[allow(unused)]
    ranging_estimator: Box<dyn RangingEstimator>,
    pcapng_dir: Option<PathBuf>,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PicaCommandError {
    #[error("Device already exists: {0}")]
    DeviceAlreadyExists(MacAddress),
    #[error("Device not found: {0}")]
    DeviceNotFound(MacAddress),
}

#[derive(Debug)]
pub enum PicaCommand {
    // Connect a new device.
    Connect(TcpStream),
    // Disconnect the selected device.
    Disconnect(usize),
    // Execute ranging command for selected device and session.
    Ranging(usize, u32),
    // Send an in-band request to stop ranging to a peer controlee identified by address and session id.
    StopRanging(MacAddress, u32),
    // Execute data message send for selected device and data.
    UciData(usize, DataPacket),
    // Execute UCI command received for selected device.
    UciCommand(usize, UciCommand),
    // Create Anchor
    CreateAnchor(
        MacAddress,
        oneshot::Sender<Result<Handle, PicaCommandError>>,
    ),
    // Destroy Anchor
    DestroyAnchor(
        MacAddress,
        oneshot::Sender<Result<Handle, PicaCommandError>>,
    ),
}

impl Display for PicaCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cmd = match self {
            PicaCommand::Connect(_) => "Connect",
            PicaCommand::Disconnect(_) => "Disconnect",
            PicaCommand::Ranging(_, _) => "Ranging",
            PicaCommand::StopRanging(_, _) => "StopRanging",
            PicaCommand::UciData(_, _) => "UciData",
            PicaCommand::UciCommand(_, _) => "UciCommand",
            PicaCommand::CreateAnchor(_, _) => "CreateAnchor",
            PicaCommand::DestroyAnchor(_, _) => "DestroyAnchor",
        };
        write!(f, "{}", cmd)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum PicaEvent {
    // A UCI connection was added
    Connected {
        handle: Handle,
        mac_address: MacAddress,
    },
    // A UCI connection was lost
    Disconnected {
        handle: Handle,
        mac_address: MacAddress,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Category {
    Uci,
    Anchor,
}

#[derive(Debug, Clone, Copy)]
struct Anchor {
    handle: Handle,
    #[allow(unused)]
    mac_address: MacAddress,
}

/// Result of UCI packet parsing.
enum UciParseResult {
    UciCommand(UciCommand),
    UciData(DataPacket),
    Err(Bytes),
    Skip,
}

/// Parse incoming UCI packets.
/// Handle parsing errors by crafting a suitable error response packet.
fn parse_uci_packet(bytes: &[u8]) -> UciParseResult {
    let message_type = parse_message_type(bytes[0]);
    match message_type {
        MessageType::Data => match DataPacket::parse(bytes) {
            Ok(packet) => UciParseResult::UciData(packet),
            Err(_) => UciParseResult::Skip,
        },
        _ => {
            match ControlPacket::parse(bytes) {
                // Parsing error. Determine what error response should be
                // returned to the host:
                // - response and notifications are ignored, no response
                // - if the group id is not known, STATUS_UNKNOWN_GID,
                // - otherwise, and to simplify the code, STATUS_UNKNOWN_OID is
                //      always returned. That means that malformed commands
                //      get the same status code, instead of
                //      STATUS_SYNTAX_ERROR.
                Err(_) => {
                    let group_id = bytes[0] & 0xf;
                    let opcode_id = bytes[1] & 0x3f;

                    let status = match (message_type, GroupId::try_from(group_id)) {
                        (MessageType::Command, Ok(_)) => UciStatusCode::UciStatusUnknownOid,
                        (MessageType::Command, Err(_)) => UciStatusCode::UciStatusUnknownGid,
                        _ => return UciParseResult::Skip,
                    };
                    // The PDL generated code cannot be used to generate
                    // responses with invalid group identifiers.
                    let response = vec![
                        (u8::from(MessageType::Response) << 5) | group_id,
                        opcode_id,
                        0,
                        1,
                        status.into(),
                    ];
                    UciParseResult::Err(response.into())
                }

                // Parsing success, ignore non command packets.
                Ok(packet) => {
                    if let Ok(cmd) = packet.try_into() {
                        UciParseResult::UciCommand(cmd)
                    } else {
                        UciParseResult::Skip
                    }
                }
            }
        }
    }
}

fn make_measurement(
    mac_address: &MacAddress,
    local: RangingMeasurement,
    remote: RangingMeasurement,
) -> ShortAddressTwoWayRangingMeasurement {
    if let MacAddress::Short(address) = mac_address {
        ShortAddressTwoWayRangingMeasurement {
            mac_address: u16::from_le_bytes(*address),
            status: UciStatusCode::UciStatusOk,
            nlos: 0, // in Line Of Sight
            distance: local.range,
            aoa_azimuth: local.azimuth as u16,
            aoa_azimuth_fom: 100, // Yup, pretty sure about this
            aoa_elevation: local.elevation as u16,
            aoa_elevation_fom: 100, // Yup, pretty sure about this
            aoa_destination_azimuth: remote.azimuth as u16,
            aoa_destination_azimuth_fom: 100,
            aoa_destination_elevation: remote.elevation as u16,
            aoa_destination_elevation_fom: 100,
            slot_index: 0,
            rssi: u8::MAX,
        }
    } else {
        panic!("Extended address is not supported.")
    }
}

impl Pica {
    pub fn new(ranging_estimator: Box<dyn RangingEstimator>, pcapng_dir: Option<PathBuf>) -> Self {
        let (command_tx, command_rx) = mpsc::channel(MAX_SESSION * MAX_DEVICE);
        let (event_tx, _) = broadcast::channel(16);
        Pica {
            devices: HashMap::new(),
            anchors: HashMap::new(),
            counter: 0,
            command_rx,
            command_tx,
            event_tx,
            ranging_estimator,
            pcapng_dir,
        }
    }

    pub fn events(&self) -> broadcast::Receiver<PicaEvent> {
        self.event_tx.subscribe()
    }

    pub fn commands(&self) -> mpsc::Sender<PicaCommand> {
        self.command_tx.clone()
    }

    fn get_device_mut(&mut self, device_handle: usize) -> Option<&mut Device> {
        self.devices.get_mut(&device_handle)
    }

    fn get_device(&self, device_handle: usize) -> Option<&Device> {
        self.devices.get(&device_handle)
    }

    fn get_category(&self, mac_address: &MacAddress) -> Option<Category> {
        if self.anchors.contains_key(mac_address) {
            Some(Category::Anchor)
        } else if self
            .devices
            .iter()
            .any(|(_, device)| device.mac_address == *mac_address)
        {
            Some(Category::Uci)
        } else {
            None
        }
    }

    fn get_device_mut_by_mac(&mut self, mac_address: &MacAddress) -> Option<&mut Device> {
        self.devices
            .values_mut()
            .find(|d| d.mac_address == *mac_address)
    }

    fn get_device_by_mac(&self, mac_address: &MacAddress) -> Option<&Device> {
        self.devices
            .values()
            .find(|d| d.mac_address == *mac_address)
    }

    fn send_event(&self, event: PicaEvent) {
        // An error here means that we have
        // no receivers, so ignore it
        let _ = self.event_tx.send(event);
    }

    async fn connect(&mut self, stream: TcpStream) {
        let (packet_tx, mut packet_rx) = mpsc::channel(MAX_SESSION);
        let device_handle = self.counter;
        let pica_tx = self.command_tx.clone();
        let pcapng_dir = self.pcapng_dir.clone();

        log::debug!("[{}] Connecting device", device_handle);

        self.counter += 1;
        let mac_address = MacAddress::Short((device_handle as u16).to_be_bytes());
        let mut device = Device::new(
            device_handle,
            mac_address,
            packet_tx,
            self.command_tx.clone(),
        );
        device.init();

        self.send_event(PicaEvent::Connected {
            handle: device_handle,
            mac_address: device.mac_address,
        });

        self.devices.insert(device_handle, device);

        // Spawn and detach the connection handling task.
        // The task notifies pica when exiting to let it clean
        // the state.
        tokio::task::spawn(async move {
            let mut pcapng_file = if let Some(dir) = pcapng_dir {
                let full_path = dir.join(format!("device-{}.pcapng", device_handle));
                log::debug!("Recording pcapng to file {}", full_path.as_path().display());
                Some(pcapng::File::create(full_path).await.unwrap())
            } else {
                None
            };

            let (uci_rx, uci_tx) = stream.into_split();
            let mut uci_reader = packets::uci::Reader::new(uci_rx);
            let mut uci_writer = packets::uci::Writer::new(uci_tx);

            'outer: loop {
                tokio::select! {
                    // Read command packet sent from connected UWB host.
                    // Run associated command.
                    result = uci_reader.read(&mut pcapng_file) =>
                        match result {
                            Ok(packet) =>
                                match parse_uci_packet(&packet) {
                                    UciParseResult::UciCommand(cmd) => {
                                        pica_tx.send(PicaCommand::UciCommand(device_handle, cmd)).await.unwrap()
                                    },
                                    UciParseResult::UciData(data) => {
                                        pica_tx.send(PicaCommand::UciData(device_handle, data)).await.unwrap()
                                    },
                                    UciParseResult::Err(response) =>
                                        uci_writer.write(&response, &mut pcapng_file).await.unwrap(),
                                    UciParseResult::Skip => (),
                                },
                            Err(_) => break 'outer
                        },

                    // Send response packets to the connected UWB host.
                    Some(packet) = packet_rx.recv() =>
                        if uci_writer.write(&packet, &mut pcapng_file).await.is_err() {
                            break 'outer
                        }
                }
            }
            pica_tx
                .send(PicaCommand::Disconnect(device_handle))
                .await
                .unwrap()
        });
    }

    fn disconnect(&mut self, device_handle: usize) {
        log::debug!("[{}] Disconnecting device", device_handle);

        if let Some(device) = self.devices.get(&device_handle) {
            self.send_event(PicaEvent::Disconnected {
                handle: device_handle,
                mac_address: device.mac_address,
            });
            self.devices.remove(&device_handle);
        }
    }

    async fn ranging(&mut self, device_handle: usize, session_id: u32) {
        log::debug!("[{}] Ranging event", device_handle);
        log::debug!("  session_id={}", session_id);

        let device = self.get_device(device_handle).unwrap();
        let session = device.session(session_id).unwrap();

        let mut measurements = Vec::new();
        let mut peer_device_data_transfer = Vec::new();
        session
            .get_dst_mac_addresses()
            .iter()
            .for_each(|mac_address| {
                if let Some(other) = self.anchors.get(mac_address) {
                    let local = self
                        .ranging_estimator
                        .estimate(&device.handle, &other.handle)
                        .unwrap_or(Default::default());
                    let remote = self
                        .ranging_estimator
                        .estimate(&other.handle, &device.handle)
                        .unwrap_or(Default::default());
                    measurements.push(make_measurement(mac_address, local, remote));
                }

                if let Some(other) = self.get_device_by_mac(mac_address) {
                    if other.can_start_ranging(session, session_id) {
                        let local = self
                            .ranging_estimator
                            .estimate(&device.handle, &other.handle)
                            .unwrap_or(Default::default());
                        let remote = self
                            .ranging_estimator
                            .estimate(&other.handle, &device.handle)
                            .unwrap_or(Default::default());
                        measurements.push(make_measurement(mac_address, local, remote));
                    }
                    if device.can_start_data_transfer(session_id)
                        && other.can_receive_data_transfer(session_id)
                    {
                        peer_device_data_transfer.push(other);
                    }
                }
            });

        // TODO: Data transfer should be limited in size for
        // each round of ranging
        for peer_device in peer_device_data_transfer.iter() {
            peer_device
                .tx
                .send(
                    DataMessageRcvBuilder {
                        application_data: session.data().clone().into(),
                        data_sequence_number: 0x01,
                        pbf: PacketBoundaryFlag::Complete,
                        session_handle: session_id,
                        source_address: device.mac_address.into(),
                        status: UciStatusCode::UciStatusOk,
                    }
                    .build()
                    .into(),
                )
                .await
                .unwrap();
        }
        if session.is_ranging_data_ntf_enabled() != RangeDataNtfConfig::Disable {
            device
                .tx
                .send(
                    // TODO: support extended address
                    ShortMacTwoWaySessionInfoNtfBuilder {
                        sequence_number: session.sequence_number,
                        session_token: session_id,
                        rcr_indicator: 0,            //TODO
                        current_ranging_interval: 0, //TODO
                        two_way_ranging_measurements: measurements,
                        vendor_data: vec![],
                    }
                    .build()
                    .into(),
                )
                .await
                .unwrap();

            let device = self.get_device_mut(device_handle).unwrap();
            let session = device.session_mut(session_id).unwrap();

            session.sequence_number += 1;
        }

        // TODO: Clean the data only when all the data is transfered
        let device = self.get_device_mut(device_handle).unwrap();
        let session = device.session_mut(session_id).unwrap();

        session.clear_data();
    }

    async fn uci_data(&mut self, device_handle: usize, data: DataPacket) {
        match self.get_device_mut(device_handle) {
            Some(device) => {
                let response: SessionControlNotification = device.data_message_snd(data);
                device.tx.send(response.into()).await.unwrap_or_else(|err| {
                    log::error!("Failed to send UCI data packet response: {}", err)
                });
            }
            None => log::error!("Device {} not found", device_handle),
        }
    }
    async fn uci_command(&mut self, device_handle: usize, cmd: UciCommand) {
        match self.get_device_mut(device_handle) {
            Some(device) => {
                let response: ControlPacket = device.command(cmd).into();
                device
                    .tx
                    .send(response.to_vec())
                    .await
                    .unwrap_or_else(|err| {
                        log::error!("Failed to send UCI command response: {}", err)
                    });
            }
            None => log::error!("Device {} not found", device_handle),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            use PicaCommand::*;
            match self.command_rx.recv().await {
                Some(Connect(stream)) => {
                    self.connect(stream).await;
                }
                Some(Disconnect(device_handle)) => self.disconnect(device_handle),
                Some(Ranging(device_handle, session_id)) => {
                    self.ranging(device_handle, session_id).await;
                }
                Some(StopRanging(mac_address, session_id)) => {
                    self.stop_controlee_ranging(&mac_address, session_id).await;
                }
                Some(UciData(device_handle, data)) => self.uci_data(device_handle, data).await,
                Some(UciCommand(device_handle, cmd)) => self.uci_command(device_handle, cmd).await,
                Some(CreateAnchor(mac_address, pica_cmd_rsp_tx)) => {
                    self.create_anchor(mac_address, pica_cmd_rsp_tx)
                }
                Some(DestroyAnchor(mac_address, pica_cmd_rsp_tx)) => {
                    self.destroy_anchor(mac_address, pica_cmd_rsp_tx)
                }
                None => (),
            };
        }
    }

    // Handle the in-band StopRanging command sent from controller to the controlee with
    // corresponding mac_address and session_id.
    async fn stop_controlee_ranging(&mut self, mac_address: &MacAddress, session_id: u32) {
        if let Some(device) = self.get_device_mut_by_mac(mac_address) {
            if let Some(session) = device.session_mut(session_id) {
                if session.session_state() == SessionState::SessionStateActive {
                    session.stop_ranging_task();
                    session.set_state(
                        SessionState::SessionStateIdle,
                        ReasonCode::SessionStoppedDueToInbandSignal,
                    );
                    device.n_active_sessions = device.n_active_sessions.saturating_sub(1);
                    if device.n_active_sessions == 0 {
                        device.set_state(DeviceState::DeviceStateReady);
                    }
                } else {
                    log::warn!("stop_controlee_ranging: session is not active !");
                }
            }
        }
    }

    #[allow(clippy::map_entry)]
    fn create_anchor(
        &mut self,
        mac_address: MacAddress,
        rsp_tx: oneshot::Sender<Result<Handle, PicaCommandError>>,
    ) {
        log::debug!("[_] Create anchor");
        log::debug!("  mac_address: {}", mac_address);

        let status = if self.get_category(&mac_address).is_some() {
            Err(PicaCommandError::DeviceAlreadyExists(mac_address))
        } else {
            let handle = self.counter;
            self.counter += 1;

            assert!(self
                .anchors
                .insert(
                    mac_address,
                    Anchor {
                        handle,
                        mac_address,
                    },
                )
                .is_none());

            Ok(handle)
        };

        rsp_tx.send(status).unwrap_or_else(|err| {
            log::error!("Failed to send create-anchor command response: {:?}", err)
        })
    }

    fn destroy_anchor(
        &mut self,
        mac_address: MacAddress,
        rsp_tx: oneshot::Sender<Result<Handle, PicaCommandError>>,
    ) {
        log::debug!("[_] Destroy anchor");
        log::debug!("  mac_address: {}", mac_address);

        let status = match self.anchors.remove(&mac_address) {
            None => Err(PicaCommandError::DeviceNotFound(mac_address)),
            Some(anchor) => Ok(anchor.handle),
        };

        rsp_tx.send(status).unwrap_or_else(|err| {
            log::error!("Failed to send destroy-anchor command response: {:?}", err)
        })
    }
}
