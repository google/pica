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

use anyhow::{Context, Result};
use bytes::{Bytes, BytesMut};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc, oneshot};

use num_traits::{FromPrimitive, ToPrimitive};

mod pcapng;

mod position;
use position::Position;

mod uci_packets;
use uci_packets::*;

mod device;
use device::{Device, MAX_DEVICE};

mod session;
use session::MAX_SESSION;

pub mod web;
use web::{Category, WebDevice};

const MAX_PAYLOAD_SIZE: usize = 4096;

pub type MacAddress = u64;

struct Connection {
    socket: TcpStream,
    buffer: BytesMut,
    pcapng_file: Option<pcapng::File>,
}

impl Connection {
    fn new(socket: TcpStream, pcapng_file: Option<pcapng::File>) -> Self {
        Connection {
            socket,
            buffer: BytesMut::with_capacity(MAX_PAYLOAD_SIZE),
            pcapng_file,
        }
    }

    async fn read(&mut self) -> Result<Option<BytesMut>> {
        let len = self.socket.read_buf(&mut self.buffer).await?;
        if len == 0 {
            return Ok(None);
        }

        if let Some(ref mut pcapng_file) = self.pcapng_file {
            pcapng_file
                .write(&self.buffer, pcapng::Direction::Tx)
                .await?
        }

        let bytes = self.buffer.split_to(self.buffer.len());
        Ok(Some(bytes))
    }

    async fn write(&mut self, packet: Bytes) -> Result<()> {
        if let Some(ref mut pcapng_file) = self.pcapng_file {
            pcapng_file.write(&packet, pcapng::Direction::Rx).await?
        }

        let _ = self.socket.try_write(&packet)?;
        Ok(())
    }
}

#[derive(Error, Debug)]
enum WebError {
    #[error("Anchor already exists: 0")]
    AddAnchorFailed(MacAddress),
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
    Command(usize, UciCommandPacket),
    // Init Uci Device
    InitUciDevice(MacAddress, Position),
    // Set Position
    SetPosition(MacAddress, Position),
    // Create Anchor
    CreateAnchor(MacAddress, Position),
    // Destroy Anchor
    DestroyAnchor(MacAddress),
    // Get State
    GetState(oneshot::Sender<Vec<WebDevice>>),
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
        mac_address: MacAddress,
    },
    // A Device position has changed
    PositionUpdated {
        mac_address: MacAddress,
        #[serde(flatten)]
        position: Position,
    },
    NeighborUpdated {
        mac_address: MacAddress,
        neighbor: MacAddress,
        distance: u16,
        azimuth: i16,
        elevation: i8,
    },
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
    Ok(UciCommandPacket),
    Err(Bytes),
    Skip,
}

/// Parse incoming UCI packets.
/// Handle parsing errors by crafting a suitable error response packet.
fn parse_uci_packet(bytes: &[u8]) -> UciParseResult {
    match UciPacketPacket::parse(bytes) {
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
                MessageType::from_u8(message_type),
                GroupId::from_u8(group_id),
            ) {
                (Some(MessageType::Command), Some(_)) => StatusCode::UciStatusUnknownOid,
                (Some(MessageType::Command), None) => StatusCode::UciStatusUnknownGid,
                _ => return UciParseResult::Skip,
            };
            // The PDL generated code cannot be used to generate
            // responses with invalid group identifiers.
            let response = vec![
                (MessageType::Response.to_u8().unwrap() << 5) | group_id,
                opcode_id,
                0,
                1,
                status.to_u8().unwrap(),
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

    fn get_device_mut_by_mac(&mut self, mac_address: MacAddress) -> Option<&mut Device> {
        self.devices
            .values_mut()
            .find(|d| d.mac_address == mac_address)
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
            mac_address: device.mac_address as u64,
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
                            Ok(Some(packet)) =>
                                match parse_uci_packet(&packet) {
                                    UciParseResult::Ok(cmd) =>
                                        pica_tx.send(PicaCommand::Command(device_handle, cmd)).await.unwrap(),
                                    UciParseResult::Err(response) =>
                                        connection.write(response).await.unwrap(),
                                    UciParseResult::Skip => (),
                                },
                            Ok(None) | Err(_) => break 'outer
                        },

                    // Send response packets to the connected UWB host.
                    Some(packet) = packet_rx.recv() =>
                        if connection.write(packet.to_bytes()).await.is_err() {
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

    fn disconnect(&mut self, device_handle: usize) -> Result<()> {
        println!("[{}] Disconnecting device", device_handle);

        let device = self.devices.get(&device_handle).context("Unknown device")?;

        self.send_event(PicaEvent::DeviceRemoved {
            mac_address: device.mac_address,
        });

        self.devices.remove(&device_handle);

        Ok(())
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

                    // TODO: support extended address
                    measurements.push(ShortAddressTwoWayRangingMeasurement {
                        mac_address: *mac_address as u16,
                        status: StatusCode::UciStatusOk,
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
                    });
                }
            });

        device
            .tx
            .send(
                // TODO: support extended address
                ShortMacTwoWayRangeDataNtfBuilder {
                    sequence_number: session.sequence_number,
                    session_id: session_id as u32,
                    rcr_indicator: 0,            //TODO
                    current_ranging_interval: 0, //TODO
                    two_way_ranging_measurements: measurements,
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

    async fn command(&mut self, device_handle: usize, cmd: UciCommandPacket) -> Result<()> {
        // TODO: implement fragmentation support
        assert_eq!(
            cmd.get_packet_boundary_flag(),
            PacketBoundaryFlag::Complete,
            "Boundary flag is true, implement fragmentation"
        );

        let device = self.get_device_mut(device_handle).unwrap();
        let response = device.command(cmd).into();
        Ok(device.tx.send(response).await?)
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            use PicaCommand::*;
            match self.rx.recv().await {
                Some(Connect(stream)) => self.connect(stream).await,
                Some(Disconnect(device_handle)) => self.disconnect(device_handle)?,
                Some(Ranging(device_handle, session_id)) => {
                    self.ranging(device_handle, session_id).await
                }
                Some(Command(device_handle, cmd)) => self.command(device_handle, cmd).await?,
                Some(SetPosition(mac_address, position)) => {
                    self.set_position(mac_address, position)?
                }
                Some(CreateAnchor(mac_address, position)) => {
                    self.create_anchor(mac_address, position)?
                }
                Some(DestroyAnchor(mac_address)) => self.destroy_anchor(mac_address)?,
                Some(GetState(state_tx)) => self.get_state(state_tx)?,
                Some(InitUciDevice(mac_address, position)) => {
                    self.init_uci_device(mac_address, position)?
                }
                None => (),
            }
        }
    }

    fn init_uci_device(
        &mut self,
        mac_address: MacAddress,
        position: Position,
    ) -> Result<(), WebError> {
        println!("[_] Init device");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        let uci_device = self
            .get_device_mut_by_mac(mac_address)
            .ok_or(WebError::DeviceNotFound(mac_address))?;
        uci_device.mac_address = mac_address;
        uci_device.position = position;
        Ok(())
    }

    fn set_position(
        &mut self,
        mac_address: MacAddress,
        position: Position,
    ) -> Result<(), WebError> {
        if let Some(uci_device) = self.get_device_mut_by_mac(mac_address) {
            uci_device.position = position;
        } else if let Some(anchor) = self.anchors.get_mut(&mac_address) {
            anchor.position = position;
        } else {
            return Err(WebError::DeviceNotFound(mac_address));
        }

        self.update_position(mac_address, position);

        Ok(())
    }

    fn update_position(&self, mac_address: MacAddress, position: Position) {
        self.send_event(PicaEvent::PositionUpdated {
            mac_address,
            position,
        });

        let devices = self.devices.values().map(|d| (d.mac_address, d.position));
        let anchors = self.anchors.values().map(|b| (b.mac_address, b.position));

        for (device_mac_address, device_position) in devices.chain(anchors) {
            if mac_address != device_mac_address {
                let local = position.compute_range_azimuth_elevation(&device_position);
                let remote = device_position.compute_range_azimuth_elevation(&position);

                assert!(local.0 == remote.0);

                self.send_event(PicaEvent::NeighborUpdated {
                    mac_address,
                    neighbor: device_mac_address,
                    distance: local.0,
                    azimuth: local.1,
                    elevation: local.2,
                });

                self.send_event(PicaEvent::NeighborUpdated {
                    mac_address: device_mac_address,
                    neighbor: mac_address,
                    distance: remote.0,
                    azimuth: remote.1,
                    elevation: remote.2,
                });
            }
        }
    }

    #[allow(clippy::map_entry)]
    fn create_anchor(
        &mut self,
        mac_address: MacAddress,
        position: Position,
    ) -> Result<(), WebError> {
        println!("[_] Create anchor");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        if self.anchors.contains_key(&mac_address) {
            return Err(WebError::AddAnchorFailed(mac_address));
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
        };

        Ok(())
    }

    fn destroy_anchor(&mut self, mac_address: MacAddress) -> Result<(), WebError> {
        println!("[_] Destroy anchor");
        println!("  mac_address=0x{:x}", mac_address);

        if self.anchors.remove(&mac_address).is_none() {
            return Err(WebError::DeviceNotFound(mac_address));
        } else {
            self.send_event(PicaEvent::DeviceRemoved { mac_address });
        };

        Ok(())
    }

    fn get_state(&self, state_tx: oneshot::Sender<Vec<WebDevice>>) -> Result<(), WebError> {
        println!("[_] Get State");
        let web_devices: Vec<WebDevice> = self
            .anchors
            .iter()
            .map(|(_, anchor)| WebDevice::from(*anchor))
            .chain(self.devices.iter().map(|(_, uci_device)| {
                WebDevice::new(Category::Uci, uci_device.mac_address, uci_device.position)
            }))
            .collect();

        state_tx.send(web_devices).unwrap();
        Ok(())
    }
}
