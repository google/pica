use anyhow::{anyhow, Context, Result};
use bytes::{Bytes, BytesMut};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};

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
    // Set Position
    SetPosition(MacAddress, Position),
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum PicaEvent {
    // A Device was added
    AddDevice {
        mac_address: MacAddress,
        #[serde(flatten)]
        position: Position,
    },
    // A Device was removed
    RemoveDevice {
        mac_address: MacAddress,
    },
    // A Device position has changed
    UpdatePosition {
        mac_address: MacAddress,
        #[serde(flatten)]
        position: Position,
    },
    UpdateNeighbor {
        mac_address: MacAddress,
        neighbor: MacAddress,
        distance: u16,
        azimuth: i16,
        elevation: i8,
    },
}

#[derive(Debug)]
struct Beacon {
    position: Position,
    mac_address: MacAddress,
}

pub struct Pica {
    devices: HashMap<usize, Device>,
    beacons: HashMap<MacAddress, Beacon>,
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
fn parse_uci_packet(bytes: BytesMut) -> UciParseResult {
    match UciPacketPacket::parse(&bytes) {
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
            beacons: HashMap::new(),
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

        self.send_event(PicaEvent::AddDevice {
            mac_address: device.mac_address as u64,
            position: device.position.clone(),
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
                                match parse_uci_packet(packet) {
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

        self.send_event(PicaEvent::RemoveDevice {
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
                if let Some(beacon) = self.beacons.get(mac_address) {
                    let local = device
                        .position
                        .compute_range_azimuth_elevation(&beacon.position);
                    let remote = beacon
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

        let response = match cmd.specialize() {
            // Handle UCI commands for Pica here:
            UciCommandChild::PicaCommand(pica_command) => match pica_command.specialize() {
                PicaCommandChild::PicaInitDeviceCmd(cmd) => {
                    self.init_device(device_handle, cmd).into()
                }
                PicaCommandChild::PicaSetDevicePositionCmd(cmd) => {
                    self.set_device_position(device_handle, cmd).into()
                }
                PicaCommandChild::PicaCreateBeaconCmd(cmd) => self.create_beacon(cmd).into(),
                PicaCommandChild::PicaSetBeaconPositionCmd(cmd) => {
                    self.set_beacon_position(cmd).into()
                }
                PicaCommandChild::PicaDestroyBeaconCmd(cmd) => self.destroy_beacon(cmd).into(),
                PicaCommandChild::None => anyhow::bail!("Unsupported Pica command"),
            },
            // Forward other UCI commands to the proper device:
            _ => {
                let device = self.get_device_mut(device_handle).unwrap();
                device.command(cmd).into()
            }
        };

        let device = self.get_device_mut(device_handle).unwrap();
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
                Some(SetPosition(mac, position)) => self.set_position(mac, position)?,
                None => (),
            }
        }
    }

    fn init_device(
        &mut self,
        device_handle: usize,
        cmd: PicaInitDeviceCmdPacket,
    ) -> PicaInitDeviceRspPacket {
        let mac_address = cmd.get_mac_address();
        let position: Position = cmd.get_position().into();

        println!("[_] Init device");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        let device = self.get_device_mut(device_handle).unwrap();
        device.mac_address = mac_address;
        device.position = Position::from(cmd.get_position());
        // FIXME: send event for the mac_address change
        PicaInitDeviceRspBuilder {
            status: StatusCode::UciStatusOk,
        }
        .build()
        .into()
    }

    fn update_position(&self, mac_address: MacAddress, position: Position) {
        self.send_event(PicaEvent::UpdatePosition {
            mac_address,
            position: position.clone(),
        });

        let devices = self
            .devices
            .values()
            .map(|d| (d.mac_address, d.position.clone()));
        let beacons = self
            .beacons
            .values()
            .map(|b| (b.mac_address, b.position.clone()));

        for (device_mac_address, device_position) in devices.chain(beacons) {
            if mac_address != device_mac_address {
                let local = position.compute_range_azimuth_elevation(&device_position);
                let remote = device_position.compute_range_azimuth_elevation(&position);

                assert!(local.0 == remote.0);

                self.send_event(PicaEvent::UpdateNeighbor {
                    mac_address,
                    neighbor: device_mac_address,
                    distance: local.0,
                    azimuth: local.1,
                    elevation: local.2,
                });

                self.send_event(PicaEvent::UpdateNeighbor {
                    mac_address: device_mac_address,
                    neighbor: mac_address,
                    distance: remote.0,
                    azimuth: remote.1,
                    elevation: remote.2,
                });
            }
        }
    }

    fn set_position(&mut self, mac_address: MacAddress, position: Position) -> Result<()> {
        if let Some(d) = self.get_device_mut_by_mac(mac_address) {
            d.position = position.clone();
        } else if let Some(b) = self.beacons.get_mut(&mac_address) {
            b.position = position.clone();
        } else {
            return Err(anyhow!("Device or Beacon not found"));
        }

        self.update_position(mac_address, position);

        Ok(())
    }

    fn set_device_position(
        &mut self,
        device_handle: usize,
        cmd: PicaSetDevicePositionCmdPacket,
    ) -> PicaSetDevicePositionRspPacket {
        let mut device = self.get_device_mut(device_handle).unwrap();
        device.position = cmd.get_position().into();

        let position = device.position.clone();
        let mac_address = device.mac_address;

        self.update_position(mac_address, position);

        PicaSetDevicePositionRspBuilder {
            status: StatusCode::UciStatusOk,
        }
        .build()
        .into()
    }

    fn create_beacon(&mut self, cmd: PicaCreateBeaconCmdPacket) -> PicaCreateBeaconRspPacket {
        let mac_address = cmd.get_mac_address();
        let position = cmd.get_position();
        println!("[_] Create beacon");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        let status = if self.beacons.contains_key(&mac_address) {
            StatusCode::UciStatusFailed
        } else {
            self.send_event(PicaEvent::AddDevice {
                mac_address,
                position: Position::from(position),
            });
            assert!(self
                .beacons
                .insert(
                    mac_address,
                    Beacon {
                        position: Position::from(position),
                        mac_address,
                    },
                )
                .is_none());
            StatusCode::UciStatusOk
        };

        PicaCreateBeaconRspBuilder { status }.build().into()
    }

    fn set_beacon_position(
        &mut self,
        cmd: PicaSetBeaconPositionCmdPacket,
    ) -> PicaSetBeaconPositionRspPacket {
        let mac_address = cmd.get_mac_address();
        let position = cmd.get_position();
        println!("[_] Set beacon position");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        let status = if let Some(b) = self.beacons.get_mut(&mac_address) {
            b.position = Position::from(position);
            StatusCode::UciStatusOk
        } else {
            StatusCode::UciStatusFailed
        };

        if status == StatusCode::UciStatusOk {
            self.update_position(mac_address, Position::from(position));
        }
        PicaSetBeaconPositionRspBuilder { status }.build().into()
    }

    fn destroy_beacon(&mut self, cmd: PicaDestroyBeaconCmdPacket) -> PicaDestroyBeaconRspPacket {
        let mac_address = cmd.get_mac_address();
        println!("[_] Destroy beacon");
        println!("  mac_address=0x{:x}", mac_address);

        let status = if self.beacons.remove(&mac_address).is_some() {
            self.send_event(PicaEvent::RemoveDevice { mac_address });
            StatusCode::UciStatusOk
        } else {
            StatusCode::UciStatusFailed
        };

        PicaDestroyBeaconRspBuilder { status }.build().into()
    }
}
