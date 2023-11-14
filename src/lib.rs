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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc, oneshot};

mod pcapng;

mod position;
pub use position::Position;

mod uci_packets;
use uci_packets::StatusCode as UciStatusCode;
use uci_packets::*;

mod device;
use device::{Device, MAX_DEVICE};

mod session;
use session::{AppConfig, MAX_SESSION};

mod mac_address;
pub use mac_address::MacAddress;

use crate::session::RangeDataNtfConfig;

/// Size of UCI packet headers.
const HEADER_SIZE: usize = 4;
/// Maximum size of an UCI packet payload.
const MAX_PAYLOAD_SIZE: usize = 255;

struct Connection {
    socket: TcpStream,
    pcapng_file: Option<pcapng::File>,
}

impl Connection {
    fn new(socket: TcpStream, pcapng_file: Option<pcapng::File>) -> Self {
        Connection {
            socket,
            pcapng_file,
        }
    }

    /// Read a single UCI packet from the socket. The packet is automatically
    /// re-assembled if segmented on the UCI transport.
    async fn read(&mut self) -> Result<Vec<u8>> {
        let mut complete_packet = vec![0; HEADER_SIZE];

        // Note on reassembly:
        // For each segment of a Control Message, the
        // header of the Control Packet SHALL contain the same MT, GID and OID
        // values.
        // It is correct to keep only the last header of the segmented packet.
        loop {
            // Read the common packet header.
            self.socket
                .read_exact(&mut complete_packet[0..HEADER_SIZE])
                .await?;
            let header = PacketHeader::parse(&complete_packet[0..HEADER_SIZE])?;

            // Read the packet payload.
            let payload_length = header.get_payload_length() as usize;
            let mut payload_bytes = vec![0; payload_length];
            self.socket.read_exact(&mut payload_bytes).await?;
            complete_packet.extend(&payload_bytes);

            if let Some(ref mut pcapng_file) = self.pcapng_file {
                let mut packet_bytes = vec![];
                packet_bytes.extend(&complete_packet[0..HEADER_SIZE]);
                packet_bytes.extend(&payload_bytes);
                pcapng_file
                    .write(&packet_bytes, pcapng::Direction::Tx)
                    .await?;
            }

            // Check the Packet Boundary Flag.
            match header.get_pbf() {
                PacketBoundaryFlag::Complete => return Ok(complete_packet),
                PacketBoundaryFlag::NotComplete => (),
            }
        }
    }

    /// Write a single UCI packet to the writer. The packet is automatically
    /// segmented if the payload exceeds the maximum size limit.
    async fn write(&mut self, mut packet: &[u8]) -> Result<()> {
        let mut header_bytes = [packet[0], packet[1], packet[2], 0];
        packet = &packet[HEADER_SIZE..];

        loop {
            // Update header with framing information.
            let chunk_length = std::cmp::min(MAX_PAYLOAD_SIZE, packet.len());
            let pbf = if chunk_length < packet.len() {
                PacketBoundaryFlag::NotComplete
            } else {
                PacketBoundaryFlag::Complete
            };
            const PBF_MASK: u8 = 0x10;
            header_bytes[0] &= !PBF_MASK;
            header_bytes[0] |= (pbf as u8) << 4;
            header_bytes[3] = chunk_length as u8;

            if let Some(ref mut pcapng_file) = self.pcapng_file {
                let mut packet_bytes = vec![];
                packet_bytes.extend(&header_bytes);
                packet_bytes.extend(&packet[..chunk_length]);
                pcapng_file
                    .write(&packet_bytes, pcapng::Direction::Rx)
                    .await?
            }

            // Write the header and payload segment bytes.
            self.socket.try_write(&header_bytes)?;
            self.socket.try_write(&packet[..chunk_length])?;
            packet = &packet[chunk_length..];

            if packet.is_empty() {
                return Ok(());
            }
        }
    }
}

pub type PicaCommandStatus = Result<(), PicaCommandError>;

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
    // Execute UCI command received for selected device.
    Command(usize, UciCommand),
    // Init Uci Device
    InitUciDevice(MacAddress, Position, oneshot::Sender<PicaCommandStatus>),
    // Set Position
    SetPosition(MacAddress, Position, oneshot::Sender<PicaCommandStatus>),
    // Create Anchor
    CreateAnchor(MacAddress, Position, oneshot::Sender<PicaCommandStatus>),
    // Destroy Anchor
    DestroyAnchor(MacAddress, oneshot::Sender<PicaCommandStatus>),
    // Get State
    GetState(oneshot::Sender<Vec<(Category, MacAddress, Position)>>),
}

impl Display for PicaCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cmd = match self {
            PicaCommand::Connect(_) => "Connect",
            PicaCommand::Disconnect(_) => "Disconnect",
            PicaCommand::Ranging(_, _) => "Ranging",
            PicaCommand::Command(_, _) => "Command",
            PicaCommand::InitUciDevice(_, _, _) => "InitUciDevice",
            PicaCommand::SetPosition(_, _, _) => "SetPosition",
            PicaCommand::CreateAnchor(_, _, _) => "CreateAnchor",
            PicaCommand::DestroyAnchor(_, _) => "DestroyAnchor",
            PicaCommand::GetState(_) => "GetState",
        };
        write!(f, "{}", cmd)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum PicaEvent {
    // A Device was added
    DeviceAdded {
        category: Category,
        mac_address: MacAddress,
        #[serde(flatten)]
        position: Position,
    },
    // A Device was removed
    DeviceRemoved {
        category: Category,
        mac_address: MacAddress,
    },
    // A Device position has changed
    DeviceUpdated {
        category: Category,
        mac_address: MacAddress,
        #[serde(flatten)]
        position: Position,
    },
    NeighborUpdated {
        source_category: Category,
        source_mac_address: MacAddress,
        destination_category: Category,
        destination_mac_address: MacAddress,
        distance: u16,
        azimuth: i16,
        elevation: i8,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Category {
    Uci,
    Anchor,
}

#[derive(Debug, Clone, Copy)]
struct Anchor {
    mac_address: MacAddress,
    position: Position,
}

pub struct Pica {
    devices: HashMap<usize, Device>,
    anchors: HashMap<MacAddress, Anchor>,
    counter: usize,
    rx: mpsc::Receiver<PicaCommand>,
    tx: mpsc::Sender<PicaCommand>,
    event_tx: broadcast::Sender<PicaEvent>,
    pcapng_dir: Option<PathBuf>,
}

/// Result of UCI packet parsing.
enum UciParseResult {
    Ok(UciCommand),
    Err(Bytes),
    Skip,
}

/// Parse incoming UCI packets.
/// Handle parsing errors by crafting a suitable error response packet.
fn parse_uci_packet(bytes: &[u8]) -> UciParseResult {
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
            let message_type = (bytes[0] >> 5) & 0x7;
            let group_id = bytes[0] & 0xf;
            let opcode_id = bytes[1] & 0x3f;

            let status = match (
                MessageType::try_from(message_type),
                GroupId::try_from(group_id),
            ) {
                (Ok(MessageType::Command), Ok(_)) => UciStatusCode::UciStatusUnknownOid,
                (Ok(MessageType::Command), Err(_)) => UciStatusCode::UciStatusUnknownGid,
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
                UciParseResult::Ok(cmd)
            } else {
                UciParseResult::Skip
            }
        }
    }
}

fn add_measurement(
    mac_address: &MacAddress,
    measurements: &mut Vec<ShortAddressTwoWayRangingMeasurement>,
    local: (u16, i16, i8),
    remote: (u16, i16, i8),
) {
    // TODO: support extended address
    match mac_address {
        MacAddress::Short(address) => {
            measurements.push(ShortAddressTwoWayRangingMeasurement {
                mac_address: u16::from_le_bytes(*address),
                status: UciStatusCode::UciStatusOk,
                nlos: 0, // in Line Of Sight
                distance: local.0,
                aoa_azimuth: local.1 as u16,
                aoa_azimuth_fom: 100, // Yup, pretty sure about this
                aoa_elevation: local.2 as u16,
                aoa_elevation_fom: 100, // Yup, pretty sure about this
                aoa_destination_azimuth: remote.1 as u16,
                aoa_destination_azimuth_fom: 100,
                aoa_destination_elevation: remote.2 as u16,
                aoa_destination_elevation_fom: 100,
                slot_index: 0,
                rssi: u8::MAX,
            })
        }
        MacAddress::Extend(_) => unimplemented!(),
    }
}

impl Pica {
    pub fn new(event_tx: broadcast::Sender<PicaEvent>, pcapng_dir: Option<PathBuf>) -> Self {
        let (tx, rx) = mpsc::channel(MAX_SESSION * MAX_DEVICE);
        Pica {
            devices: HashMap::new(),
            anchors: HashMap::new(),
            counter: 0,
            rx,
            tx,
            event_tx,
            pcapng_dir,
        }
    }

    pub fn tx(&self) -> mpsc::Sender<PicaCommand> {
        self.tx.clone()
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

    fn get_device_mut_by_mac(&mut self, mac_address: MacAddress) -> Option<&mut Device> {
        self.devices
            .values_mut()
            .find(|d| d.mac_address == mac_address)
    }

    fn get_device_by_mac(
        &self,
        mac_address: MacAddress,
        local_app_config: AppConfig,
        session_id: u32,
    ) -> Option<&Device> {
        self.devices.values().find(|device| {
            if let Some(session) = device.get_session(session_id) {
                session.app_config.get_device_mac_address() == mac_address
                    && local_app_config.can_start_ranging_with_peer(&session.app_config.clone())
                    && session.get_session_state() == SessionState::SessionStateActive
            } else {
                false
            }
        })
    }

    fn send_event(&self, event: PicaEvent) {
        // An error here means that we have
        // no receivers, so ignore it
        let _ = self.event_tx.send(event);
    }

    async fn connect(&mut self, stream: TcpStream) {
        let (packet_tx, mut packet_rx) = mpsc::channel(MAX_SESSION);
        let device_handle = self.counter;
        let pica_tx = self.tx.clone();
        let pcapng_dir = self.pcapng_dir.clone();

        println!("[{}] Connecting device", device_handle);

        self.counter += 1;
        let mut device = Device::new(device_handle, packet_tx, self.tx.clone());
        device.init();

        self.send_event(PicaEvent::DeviceAdded {
            category: Category::Uci,
            mac_address: device.mac_address,
            position: device.position,
        });

        self.devices.insert(device_handle, device);

        // Spawn and detach the connection handling task.
        // The task notifies pica when exiting to let it clean
        // the state.
        tokio::spawn(async move {
            let pcapng_file: Option<pcapng::File> = if let Some(dir) = pcapng_dir {
                let full_path = dir.join(format!("device-{}.pcapng", device_handle));
                println!("Recording pcapng to file {}", full_path.as_path().display());
                Some(pcapng::File::create(full_path).await.unwrap())
            } else {
                None
            };

            let mut connection = Connection::new(stream, pcapng_file);
            'outer: loop {
                tokio::select! {
                    // Read command packet sent from connected UWB host.
                    // Run associated command.
                    result = connection.read() =>
                        match result {
                            Ok(packet) =>
                                match parse_uci_packet(&packet) {
                                    UciParseResult::Ok(cmd) =>
                                        pica_tx.send(PicaCommand::Command(device_handle, cmd)).await.unwrap(),
                                    UciParseResult::Err(response) =>
                                        connection.write(&response).await.unwrap(),
                                    UciParseResult::Skip => (),
                                },
                            Err(_) => break 'outer
                        },

                    // Send response packets to the connected UWB host.
                    Some(packet) = packet_rx.recv() =>
                        if connection.write(&packet.to_bytes()).await.is_err() {
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
        println!("[{}] Disconnecting device", device_handle);

        match self
            .devices
            .get(&device_handle)
            .ok_or_else(|| PicaCommandError::DeviceNotFound(device_handle.into()))
        {
            Ok(device) => {
                self.send_event(PicaEvent::DeviceRemoved {
                    category: Category::Uci,
                    mac_address: device.mac_address,
                });
                self.devices.remove(&device_handle);
            }
            Err(err) => println!("{}", err),
        }
    }

    async fn ranging(&mut self, device_handle: usize, session_id: u32) {
        println!("[{}] Ranging event", device_handle);
        println!("  session_id={}", session_id);

        let device = self.get_device(device_handle).unwrap();
        let session = device.get_session(session_id).unwrap();

        let mut measurements = Vec::new();
        session
            .get_dst_mac_addresses()
            .iter()
            .for_each(|mac_address| {
                if let Some(anchor) = self.anchors.get(mac_address) {
                    let local = device
                        .position
                        .compute_range_azimuth_elevation(&anchor.position);
                    let remote = anchor
                        .position
                        .compute_range_azimuth_elevation(&device.position);

                    assert!(local.0 == remote.0);
                    add_measurement(mac_address, &mut measurements, local, remote);
                }
                if let Some(peer_device) =
                    self.get_device_by_mac(*mac_address, session.app_config.clone(), session_id)
                {
                    let local: (u16, i16, i8) = device
                        .position
                        .compute_range_azimuth_elevation(&peer_device.position);
                    let remote = peer_device
                        .position
                        .compute_range_azimuth_elevation(&device.position);

                    assert!(local.0 == remote.0);
                    add_measurement(mac_address, &mut measurements, local, remote);
                }
            });
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
            let session = device.get_session_mut(session_id).unwrap();

            session.sequence_number += 1;
        }
    }

    async fn command(&mut self, device_handle: usize, cmd: UciCommand) {
        match self
            .get_device_mut(device_handle)
            .ok_or_else(|| PicaCommandError::DeviceNotFound(device_handle.into()))
        {
            Ok(device) => {
                let response: ControlPacket = device.command(cmd).into();
                device
                    .tx
                    .send(response)
                    .await
                    .unwrap_or_else(|err| println!("Failed to send UCI command response: {}", err));
            }
            Err(err) => println!("{}", err),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            use PicaCommand::*;
            match self.rx.recv().await {
                Some(Connect(stream)) => {
                    self.connect(stream).await;
                }
                Some(Disconnect(device_handle)) => self.disconnect(device_handle),
                Some(Ranging(device_handle, session_id)) => {
                    self.ranging(device_handle, session_id).await;
                }
                Some(Command(device_handle, cmd)) => self.command(device_handle, cmd).await,
                Some(SetPosition(mac_address, position, pica_cmd_rsp_tx)) => {
                    self.set_position(mac_address, position, pica_cmd_rsp_tx)
                }
                Some(CreateAnchor(mac_address, position, pica_cmd_rsp_tx)) => {
                    self.create_anchor(mac_address, position, pica_cmd_rsp_tx)
                }
                Some(DestroyAnchor(mac_address, pica_cmd_rsp_tx)) => {
                    self.destroy_anchor(mac_address, pica_cmd_rsp_tx)
                }
                Some(GetState(state_tx)) => self.get_state(state_tx),
                Some(InitUciDevice(mac_address, position, pica_cmd_rsp_tx)) => {
                    self.init_uci_device(mac_address, position, pica_cmd_rsp_tx);
                }
                None => (),
            };
        }
    }

    // TODO: Assign a reserved range of mac addresses for UCI devices
    // to protect against conflicts  with user defined Anchor addresses
    // b/246000641
    fn init_uci_device(
        &mut self,
        mac_address: MacAddress,
        position: Position,
        pica_cmd_rsp_tx: oneshot::Sender<PicaCommandStatus>,
    ) {
        println!("[_] Init device");
        println!("  mac_address: {}", mac_address);
        println!("  position={:?}", position);

        let status = self
            .get_device_mut_by_mac(mac_address)
            .ok_or(PicaCommandError::DeviceNotFound(mac_address))
            .map(|uci_device| {
                uci_device.mac_address = mac_address;
                uci_device.position = position;
            });

        pica_cmd_rsp_tx.send(status).unwrap_or_else(|err| {
            println!("Failed to send init-uci-device command response: {:?}", err)
        });
    }

    fn set_position(
        &mut self,
        mac_address: MacAddress,
        position: Position,
        pica_cmd_rsp_tx: oneshot::Sender<PicaCommandStatus>,
    ) {
        let mut status = if let Some(uci_device) = self.get_device_mut_by_mac(mac_address) {
            uci_device.position = position;
            Ok(())
        } else if let Some(anchor) = self.anchors.get_mut(&mac_address) {
            anchor.position = position;
            Ok(())
        } else {
            Err(PicaCommandError::DeviceNotFound(mac_address))
        };

        if status.is_ok() {
            status = self.update_position(mac_address, position)
        }

        pica_cmd_rsp_tx.send(status).unwrap_or_else(|err| {
            println!("Failed to send set-position command response: {:?}", err)
        });
    }

    fn update_position(
        &self,
        mac_address: MacAddress,
        position: Position,
    ) -> Result<(), PicaCommandError> {
        let category = match self.get_category(&mac_address) {
            Some(category) => category,
            None => {
                return Err(PicaCommandError::DeviceNotFound(mac_address));
            }
        };
        self.send_event(PicaEvent::DeviceUpdated {
            category,
            mac_address,
            position,
        });

        let devices = self.devices.values().map(|d| (d.mac_address, d.position));
        let anchors = self.anchors.values().map(|b| (b.mac_address, b.position));

        let update_neighbors = |device_category, device_mac_address, device_position| {
            if mac_address != device_mac_address {
                let local = position.compute_range_azimuth_elevation(&device_position);
                let remote = device_position.compute_range_azimuth_elevation(&position);

                assert!(local.0 == remote.0);

                self.send_event(PicaEvent::NeighborUpdated {
                    source_category: category,
                    source_mac_address: mac_address,
                    destination_category: device_category,
                    destination_mac_address: device_mac_address,
                    distance: local.0,
                    azimuth: local.1,
                    elevation: local.2,
                });

                self.send_event(PicaEvent::NeighborUpdated {
                    source_category: device_category,
                    source_mac_address: device_mac_address,
                    destination_category: category,
                    destination_mac_address: mac_address,
                    distance: remote.0,
                    azimuth: remote.1,
                    elevation: remote.2,
                });
            }
        };

        devices.for_each(|device| update_neighbors(Category::Uci, device.0, device.1));
        anchors.for_each(|anchor| update_neighbors(Category::Anchor, anchor.0, anchor.1));
        Ok(())
    }

    #[allow(clippy::map_entry)]
    fn create_anchor(
        &mut self,
        mac_address: MacAddress,
        position: Position,
        pica_cmd_rsp_tx: oneshot::Sender<PicaCommandStatus>,
    ) {
        println!("Create anchor: {} {}", mac_address, position);
        let status = if self.get_category(&mac_address).is_some() {
            Err(PicaCommandError::DeviceAlreadyExists(mac_address))
        } else {
            self.send_event(PicaEvent::DeviceAdded {
                category: Category::Anchor,
                mac_address,
                position,
            });
            assert!(self
                .anchors
                .insert(
                    mac_address,
                    Anchor {
                        mac_address,
                        position,
                    },
                )
                .is_none());
            Ok(())
        };

        pica_cmd_rsp_tx.send(status).unwrap_or_else(|err| {
            println!("Failed to send create-anchor command response: {:?}", err)
        })
    }

    fn destroy_anchor(
        &mut self,
        mac_address: MacAddress,
        pica_cmd_rsp_tx: oneshot::Sender<PicaCommandStatus>,
    ) {
        println!("[_] Destroy anchor");
        println!("  mac_address: {}", mac_address);

        let status = if self.anchors.remove(&mac_address).is_none() {
            Err(PicaCommandError::DeviceNotFound(mac_address))
        } else {
            self.send_event(PicaEvent::DeviceRemoved {
                category: Category::Anchor,
                mac_address,
            });
            Ok(())
        };
        pica_cmd_rsp_tx.send(status).unwrap_or_else(|err| {
            println!("Failed to send destroy-anchor command response: {:?}", err)
        })
    }

    fn get_state(&self, state_tx: oneshot::Sender<Vec<(Category, MacAddress, Position)>>) {
        println!("[_] Get State");

        state_tx
            .send(
                self.anchors
                    .values()
                    .map(|anchor| (Category::Anchor, anchor.mac_address, anchor.position))
                    .chain(
                        self.devices
                            .values()
                            .map(|device| (Category::Uci, device.mac_address, device.position)),
                    )
                    .collect(),
            )
            .unwrap();
    }
}
