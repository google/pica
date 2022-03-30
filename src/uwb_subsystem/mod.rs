use anyhow::{anyhow, Result};
use bytes::BytesMut;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

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
}

impl Pica {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(MAX_SESSION * MAX_DEVICE);
        Pica {
            devices: HashMap::new(),
            beacons: HashMap::new(),
            counter: 0,
            rx,
            tx,
        }
    }

    pub fn tx(&self) -> mpsc::Sender<PicaCommand> {
        self.tx.clone()
    }

    fn get_device(&mut self, device_handle: usize) -> &mut Device {
        self.devices.get_mut(&device_handle).unwrap()
    }

    async fn connect(&mut self, stream: TcpStream) {
        let (packet_tx, mut packet_rx) = mpsc::channel(MAX_SESSION);
        let device_handle = self.counter;
        let pica_tx = self.tx.clone();

        println!("[{}] Connecting device", device_handle);

        self.counter += 1;
        self.devices
            .insert(device_handle, Device::new(device_handle, packet_tx));

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

    fn disconnect(&mut self, device_handle: usize) {
        println!("[{}] Disconnecting device", device_handle);
        self.devices.remove(&device_handle);
    }

    async fn ranging(&mut self, _device_handle: usize, _session_id: u32) {
        todo!()
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
                PicaCommandChild::PicaCreateBeaconCmd(cmd) => self.create_beacon(cmd).await,
                PicaCommandChild::PicaSetBeaconPositionCmd(cmd) => {
                    self.set_beacon_position(cmd).await
                }
                PicaCommandChild::PicaDestroyBeaconCmd(cmd) => self.destroy_beacon(cmd).await,
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
                Some(Disconnect(device_handle)) => self.disconnect(device_handle),
                Some(Ranging(device_handle, session_id)) => {
                    self.ranging(device_handle, session_id).await
                }
                Some(Command(device_handle, cmd)) => self.command(device_handle, cmd).await?,
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

        let device = self.get_device(device_handle);
        device.mac_address = mac_address as usize;
        device.position = Position::from(position);
        Ok(())
    }

    async fn set_device_position(
        &mut self,
        device_handle: usize,
        cmd: PicaSetDevicePositionCmdPacket,
    ) -> Result<()> {
        let mut device = self.get_device(device_handle);
        device.position = Position::from(cmd.get_position());
        Ok(())
    }

    async fn create_beacon(&mut self, cmd: PicaCreateBeaconCmdPacket) -> Result<()> {
        let mac_address = cmd.get_mac_address();
        let position = cmd.get_position();
        println!("[_] Create beacon");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        if self.beacons.contains_key(&mac_address) {
            Err(anyhow!("Beacon already exists"))
        } else {
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
            Ok(())
        }
    }

    async fn set_beacon_position(&mut self, cmd: PicaSetBeaconPositionCmdPacket) -> Result<()> {
        let mac_address = cmd.get_mac_address();
        let position = cmd.get_position();
        println!("[_] Set beacon position");
        println!("  mac_address=0x{:x}", mac_address);
        println!("  position={:?}", position);

        if let Some(b) = self.beacons.get_mut(&mac_address) {
            b.position = Position::from(position);
            Ok(())
        } else {
            Err(anyhow!("Beacon not found"))
        }
    }

    async fn destroy_beacon(&mut self, cmd: PicaDestroyBeaconCmdPacket) -> Result<()> {
        let mac_address = cmd.get_mac_address();
        println!("[_] Destroy beacon");
        println!("  mac_address=0x{:x}", mac_address);

        if self.beacons.remove(&mac_address).is_some() {
            Ok(())
        } else {
            Err(anyhow!("Beacon not found"))
        }
    }

    async fn set_country_code(
        &mut self,
        device_handle: usize,
        command: AndroidSetCountryCodeCmdPacket,
    ) -> Result<()> {
        let device = self.get_device(device_handle);
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
