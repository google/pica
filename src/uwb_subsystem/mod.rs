use anyhow::{anyhow, Context, Result};
use bytes::BytesMut;
use serde::Serialize;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};

use crate::position::*;
use crate::uci_packets::*;

mod device;
use device::{Device, MAX_DEVICE};

mod session;
use session::MAX_SESSION;

const MAX_PAYLOAD_SIZE: usize = 4096;

type MacAddress = u64;

struct Connection {
    socket: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    fn new(socket: TcpStream) -> Self {
        Connection {
            socket,
            buffer: BytesMut::with_capacity(MAX_PAYLOAD_SIZE),
        }
    }

    async fn read(&mut self) -> Result<Option<UciCommandPacket>> {
        let len = self.socket.read_buf(&mut self.buffer).await?;
        if len == 0 {
            return Ok(None);
        }

        let packet = UciPacketPacket::parse(&self.buffer)?;
        self.buffer.clear();
        Ok(Some(packet.try_into()?))
    }

    async fn write(&mut self, packet: UciPacketPacket) -> Result<()> {
        let _ = self.socket.try_write(&packet.to_bytes())?;
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
}

impl Pica {
    pub fn new(event_tx: broadcast::Sender<PicaEvent>) -> Self {
        let (tx, rx) = mpsc::channel(MAX_SESSION * MAX_DEVICE);
        Pica {
            devices: HashMap::new(),
            beacons: HashMap::new(),
            counter: 0,
            rx,
            tx,
            event_tx,
        }
    }

    pub fn tx(&self) -> mpsc::Sender<PicaCommand> {
        self.tx.clone()
    }

    fn get_device_mut(&mut self, device_handle: usize) -> &mut Device {
        self.devices.get_mut(&device_handle).unwrap()
    }

    fn get_device(&self, device_handle: usize) -> &Device {
        self.devices.get(&device_handle).unwrap()
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

        println!("[{}] Connecting device", device_handle);

        self.counter += 1;
        let device = Device::new(device_handle, packet_tx);

        self.send_event(PicaEvent::AddDevice {
            mac_address: device.mac_address as u64,
            position: device.position.clone(),
        });

        self.devices.insert(device_handle, device);

        // Spawn and detach the connection handling task.
        // The task notifies pica when exiting to let it clean
        // the state.
        tokio::spawn(async move {
            let mut connection = Connection::new(stream);
            'outer: loop {
                tokio::select! {
                    // Read command packet sent from connected UWB host.
                    // Run associated command.
                    result = connection.read() => {
                        match result {
                            Ok(Some(packet)) =>
                                pica_tx.send(PicaCommand::Command(device_handle, packet)).await.unwrap(),
                            Ok(None) |
                            Err(_) => break 'outer
                        }
                    },

                    // Send response packets to the connected UWB host.
                    Some(packet) = packet_rx.recv() =>
                        if connection.write(packet).await.is_err() {
                            break 'outer
                        }
                }
            }
            pica_tx
                .send(PicaCommand::Disconnect(device_handle))
                .await
                .unwrap()
        });

        // Send device status notification with state Ready as required
        // by the UCI specification (section 6.1 Initialization of UWBS).
        self.devices
            .get(&device_handle)
            .unwrap()
            .tx
            .send(
                DeviceStatusNtfBuilder {
                    device_state: DeviceState::DeviceStateReady,
                }
                .build()
                .into(),
            )
            .await
            .unwrap()
    }

    async fn disconnect(&mut self, device_handle: usize) -> Result<()> {
        println!("[{}] Disconnecting device", device_handle);

        let device = self.devices.get(&device_handle).context("Unknown device")?;

        self.send_event(PicaEvent::RemoveDevice {
            mac_address: device.mac_address as u64,
        });

        self.devices.remove(&device_handle);

        Ok(())
    }

    async fn ranging(&mut self, device_handle: usize, session_id: u32) {
        println!("[{}] Ranging event", device_handle);
        println!("  session_id={}", session_id);

        let device = self.get_device(device_handle);
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

                    measurements.push(ExtendedAddressTwoWayRangingMeasurement {
                        mac_address: *mac_address,
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
                ExtendedMacTwoWayRangeDataNtfBuilder {
                    sequence_number: session.sequence_number, //TODO increment
                    session_id: session_id as u32,
                    rcr_indicator: 0,            //TODO
                    current_ranging_interval: 0, //TODO
                    two_way_ranging_measurements: measurements,
                }
                .build()
                .into(),
            )
            .await;
    }

    async fn command(&mut self, device_handle: usize, cmd: UciCommandPacket) -> Result<()> {
        if !self.devices.contains_key(&device_handle) {
            anyhow::bail!("Received command for disconnected device {}", device_handle);
        }

        match cmd.specialize() {
            UciCommandChild::CoreCommand(core_command) => match core_command.specialize() {
                CoreCommandChild::DeviceResetCmd(cmd) => {
                    self.device_reset(device_handle, cmd).await
                }
                CoreCommandChild::GetDeviceInfoCmd(cmd) => {
                    self.get_device_info(device_handle, cmd).await
                }
                CoreCommandChild::GetCapsInfoCmd(cmd) => {
                    self.get_caps_info(device_handle, cmd).await
                }
                CoreCommandChild::SetConfigCmd(cmd) => self.set_config(device_handle, cmd).await,
                CoreCommandChild::GetConfigCmd(cmd) => self.get_config(device_handle, cmd).await,
                CoreCommandChild::None => anyhow::bail!("Unsupported core command"),
            },
            UciCommandChild::SessionCommand(session_command) => {
                match session_command.specialize() {
                    SessionCommandChild::SessionInitCmd(cmd) => {
                        self.session_init(device_handle, cmd).await
                    }
                    SessionCommandChild::SessionDeinitCmd(cmd) => {
                        self.session_deinit(device_handle, cmd).await
                    }
                    SessionCommandChild::SessionSetAppConfigCmd(cmd) => {
                        self.session_set_app_config(device_handle, cmd).await
                    }
                    SessionCommandChild::SessionGetAppConfigCmd(cmd) => {
                        self.session_get_app_config(device_handle, cmd).await
                    }
                    SessionCommandChild::SessionGetCountCmd(cmd) => {
                        self.session_get_count(device_handle, cmd).await
                    }
                    SessionCommandChild::SessionGetStateCmd(cmd) => {
                        self.session_get_state(device_handle, cmd).await
                    }
                    SessionCommandChild::SessionUpdateControllerMulticastListCmd(cmd) => {
                        self.session_update_controller_multicast_list(device_handle, cmd)
                            .await
                    }
                    SessionCommandChild::None => anyhow::bail!("Unsupported session command"),
                }
            }
            UciCommandChild::RangingCommand(ranging_command) => {
                match ranging_command.specialize() {
                    RangingCommandChild::RangeStartCmd(cmd) => {
                        self.range_start(device_handle, cmd).await
                    }
                    RangingCommandChild::RangeStopCmd(cmd) => {
                        self.range_stop(device_handle, cmd).await
                    }
                    RangingCommandChild::RangeGetRangingCountCmd(cmd) => {
                        self.range_get_ranging_count(device_handle, cmd).await
                    }
                    RangingCommandChild::None => anyhow::bail!("Unsupported ranging command"),
                }
            }
            UciCommandChild::PicaCommand(pica_command) => match pica_command.specialize() {
                PicaCommandChild::PicaInitDeviceCmd(cmd) => {
                    self.init_device(device_handle, cmd).await
                }
                PicaCommandChild::PicaSetDevicePositionCmd(cmd) => {
                    self.set_device_position(device_handle, cmd).await
                }
                PicaCommandChild::PicaCreateBeaconCmd(cmd) => {
                    self.create_beacon(device_handle, cmd).await
                }
                PicaCommandChild::PicaSetBeaconPositionCmd(cmd) => {
                    self.set_beacon_position(device_handle, cmd).await
                }
                PicaCommandChild::PicaDestroyBeaconCmd(cmd) => {
                    self.destroy_beacon(device_handle, cmd).await
                }
                PicaCommandChild::None => anyhow::bail!("Unsupported Pica command"),
            },
            UciCommandChild::AndroidCommand(android_command) => {
                match android_command.specialize() {
                    AndroidCommandChild::AndroidSetCountryCodeCmd(cmd) => {
                        self.set_country_code(device_handle, cmd).await
                    }
                    AndroidCommandChild::AndroidGetPowerStatsCmd(_cmd) => todo!(),
                    AndroidCommandChild::None => anyhow::bail!("Unsupported ranging command"),
                }
            }
            _ => anyhow::bail!("Unsupported command type"),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            use PicaCommand::*;
            match self.rx.recv().await {
                Some(Connect(stream)) => self.connect(stream).await,
                Some(Disconnect(device_handle)) => self.disconnect(device_handle).await?,
                Some(Ranging(device_handle, session_id)) => {
                    self.ranging(device_handle, session_id).await
                }
                Some(Command(device_handle, cmd)) => self.command(device_handle, cmd).await?,
                Some(SetPosition(mac, position)) => self.set_position(mac, position)?,
                None => (),
            }
        }
    }

    async fn init_device(
        &mut self,
        device_handle: usize,
        cmd: PicaInitDeviceCmdPacket,
    ) -> Result<()> {
        let mac_address = cmd.get_mac_address();
        let position = cmd.get_position();
        println!("[_] Init device");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        let device = self.get_device_mut(device_handle);
        device.mac_address = mac_address as usize;
        device.position = Position::from(cmd.get_position());
        Ok(self
            .get_device(device_handle)
            .tx
            .send(
                PicaInitDeviceRspBuilder {
                    status: StatusCode::UciStatusOk,
                }
                .build()
                .into(),
            )
            .await?)
    }

    fn set_position(&mut self, mac: MacAddress, position: Position) -> Result<()> {
        /*if let Some(b) = self.beacons.get_mut(&mac_address) {
            b.position = position;
            Ok(())
        } else {
            Err(anyhow!("Beacon not found"))
        }*/
        Ok(())
    }

    async fn set_device_position(
        &mut self,
        device_handle: usize,
        cmd: PicaSetDevicePositionCmdPacket,
    ) -> Result<()> {
        let mut device = self.get_device_mut(device_handle);
        device.position = cmd.get_position().into();
        Ok(self
            .get_device(device_handle)
            .tx
            .send(
                PicaSetDevicePositionRspBuilder {
                    status: StatusCode::UciStatusOk,
                }
                .build()
                .into(),
            )
            .await?)
    }

    async fn create_beacon(
        &mut self,
        device_handle: usize,
        cmd: PicaCreateBeaconCmdPacket,
    ) -> Result<()> {
        let mac_address = cmd.get_mac_address();
        let position = cmd.get_position();
        println!("[_] Create beacon");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        let status = if self.beacons.contains_key(&mac_address) {
            StatusCode::UciStatusFailed
        } else {
            self.send_event(PicaEvent::AddDevice {
                mac_address: mac_address.clone(),
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
                .is_some());
            StatusCode::UciStatusOk
        };

        Ok(self
            .get_device(device_handle)
            .tx
            .send(PicaCreateBeaconRspBuilder { status }.build().into())
            .await?)
    }

    async fn set_beacon_position(
        &mut self,
        device_handle: usize,
        cmd: PicaSetBeaconPositionCmdPacket,
    ) -> Result<()> {
        let mac_address = cmd.get_mac_address();
        let position = cmd.get_position();
        println!("[_] Set beacon position");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        let status = if let Some(b) = self.beacons.get_mut(&mac_address) {
            b.position = Position::from(position);
            StatusCode::UciStatusFailed
        } else {
            StatusCode::UciStatusFailed
        };

        Ok(self
            .get_device(device_handle)
            .tx
            .send(PicaSetBeaconPositionRspBuilder { status }.build().into())
            .await?)
    }

    async fn destroy_beacon(
        &mut self,
        device_handle: usize,
        cmd: PicaDestroyBeaconCmdPacket,
    ) -> Result<()> {
        let mac_address = cmd.get_mac_address();
        println!("[_] Destroy beacon");
        println!("  mac_address=0x{:x}", mac_address);

        let status = if self.beacons.remove(&mac_address).is_some() {
            self.send_event(PicaEvent::RemoveDevice { mac_address });
            StatusCode::UciStatusOk
        } else {
            StatusCode::UciStatusFailed
        };

        Ok(self
            .get_device(device_handle)
            .tx
            .send(PicaDestroyBeaconRspBuilder { status }.build().into())
            .await?)
    }

    async fn set_country_code(
        &mut self,
        device_handle: usize,
        command: AndroidSetCountryCodeCmdPacket,
    ) -> Result<()> {
        let device = self.get_device_mut(device_handle);
        device.country_code = *command.get_country_code();
        println!("android command: set_country_code");
        Ok(device
            .tx
            .send(
                AndroidSetCountryCodeRspBuilder {
                    status: StatusCode::UciStatusOk,
                }
                .build()
                .into(),
            )
            .await?)
    }
}
