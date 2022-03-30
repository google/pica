use anyhow::{anyhow, Result};
use bytes::BytesMut;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::position::*;
use crate::uci_packets::*;

const MAX_DEVICE: usize = 4;
const MAX_SESSION: usize = 4;
const MAX_PAYLOAD_SIZE: usize = 4096;

type MacAddress = u64;

#[derive(Debug, PartialEq, Copy, Clone)]
enum DeviceState {
    Ready,
    Active,
    Error,
}

pub struct Session {
    state: SessionState,
    id: u32,
    session_type: SessionType,
    sequence_number: usize,
    ranging_interval: usize,
    ranging_task: Option<JoinHandle<()>>,
}

pub struct Device {
    mac_address: usize,
    state: DeviceState,
    sessions: [Session; MAX_SESSION],
    tx: mpsc::Sender<UciPacketPacket>,
    country_code: [u8; 2],
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

impl Device {
    pub fn new(device_handle: usize, tx: mpsc::Sender<UciPacketPacket>) -> Self {
        Device {
            mac_address: device_handle,
            state: DeviceState::Ready,
            sessions: Default::default(),
            tx,
            country_code: [0; 2],
        }
    }

    pub fn add_session(&mut self, session: Session) -> Result<()> {
        match self
            .sessions
            .iter_mut()
            .filter(|session| session.state == SessionState::SessionStateDeinit)
            .next()
        {
            Some(s) => {
                *s = session;
                Ok(())
            }
            None => Err(anyhow!("Maximum number of sessions reached")),
        }
    }

    pub fn get_session(&self, session_id: u32) -> Result<&Session> {
        self.sessions
            .iter()
            .find(|session| session.id == session_id)
            .ok_or(anyhow!("Session not found"))
    }

    pub fn get_mut_session(&mut self, session_id: u32) -> Result<&mut Session> {
        self.sessions
            .iter_mut()
            .find(|session| session.id == session_id)
            .ok_or(anyhow!("Session not found"))
    }
}

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
    Ranging(usize, usize),
    // Execute UCI command received for selected device.
    Command(usize, UciCommandPacket),
}

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
    }

    fn disconnect(&mut self, device_handle: usize) {
        println!("[{}] Disconnecting device", device_handle);
        self.devices.remove(&device_handle);
    }

    async fn ranging(&mut self, _device_handle: usize, _session_id: usize) {
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
                    AndroidCommandChild::AndroidGetPowerStatsCmd(cmd) => todo!(),
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

    async fn device_reset(
        &mut self,
        device_handle: usize,
        cmd: DeviceResetCmdPacket,
    ) -> Result<()> {
        let reset_config = cmd.get_reset_config();
        println!("[{}] Device Reset", device_handle);
        println!("  reset_config={}", reset_config);
        let device = self.get_device(device_handle);
        device.state = DeviceState::Ready;
        Ok(device
            .tx
            .send(
                DeviceResetRspBuilder {
                    status: StatusCode::UciStatusOk,
                }
                .build()
                .into(),
            )
            .await?)
    }

    async fn get_device_info(
        &mut self,
        device_handle: usize,
        _cmd: GetDeviceInfoCmdPacket,
    ) -> Result<()> {
        // TODO: Implement a fancy build time state machine instead of crash at runtime
        let device = self.get_device(device_handle);
        assert_eq!(device.state, DeviceState::Ready);
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

    async fn get_caps_info(
        &mut self,
        _device_handle: usize,
        _cmd: GetCapsInfoCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn set_config(&mut self, device_handle: usize, _cmd: SetConfigCmdPacket) -> Result<()> {
        let device = self.get_device(device_handle);
        assert_eq!(device.state, DeviceState::Ready);
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

    async fn get_config(&mut self, _device_handle: usize, _cmd: GetConfigCmdPacket) -> Result<()> {
        todo!()
    }

    async fn session_init(
        &mut self,
        device_handle: usize,
        cmd: SessionInitCmdPacket,
    ) -> Result<()> {
        println!("session_init");
        let device = self.get_device(device_handle);
        let mut session = Session::default();
        session.state = SessionState::SessionStateInit;
        session.id = cmd.get_session_id();
        session.session_type = cmd.get_session_type();
        let status = match device.add_session(session) {
            Ok(_) => StatusCode::UciStatusOk,
            Err(_) => StatusCode::UciStatusFailed,
        };
        device
            .tx
            .send(SessionInitRspBuilder { status: status }.build().into())
            .await?;
        Ok(device
            .tx
            .send(
                SessionStatusNtfBuilder {
                    session_id: cmd.get_session_id(),
                    session_state: SessionState::SessionStateInit,
                    reason_code: ReasonCode::StateChangeWithSessionManagementCommands,
                }
                .build()
                .into(),
            )
            .await?)
    }

    async fn session_deinit(
        &mut self,
        _device_handle: usize,
        _cmd: SessionDeinitCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn session_set_app_config(
        &mut self,
        device_handle: usize,
        cmd: SessionSetAppConfigCmdPacket,
    ) -> Result<()> {
        // TODO: Set session app configuration regardings the incoming cmd
        let device = self.get_device(device_handle);
        let session = device.get_mut_session(cmd.get_session_id())?;
        assert_eq!(session.state, SessionState::SessionStateInit);
        println!("session_set_app_config");
        session.state = SessionState::SessionStateIdle;
        device
            .tx
            .send(
                SessionSetAppConfigRspBuilder {
                    status: StatusCode::UciStatusOk,
                    cfg_status: Vec::new(),
                }
                .build()
                .into(),
            )
            .await?;

        Ok(device
            .tx
            .send(
                SessionStatusNtfBuilder {
                    session_id: cmd.get_session_id(),
                    session_state: SessionState::SessionStateIdle,
                    reason_code: ReasonCode::StateChangeWithSessionManagementCommands,
                }
                .build()
                .into(),
            )
            .await?)
    }

    async fn session_get_app_config(
        &mut self,
        _device_handle: usize,
        _cmd: SessionGetAppConfigCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn session_get_count(
        &mut self,
        _device_handle: usize,
        _cmd: SessionGetCountCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn session_get_state(
        &mut self,
        _device_handle: usize,
        _cmd: SessionGetStateCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn session_update_controller_multicast_list(
        &mut self,
        _device_handle: usize,
        _cmd: SessionUpdateControllerMulticastListCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn range_start(
        &mut self,
        _device_handle: usize,
        _cmd: RangeStartCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn range_stop(&mut self, _device_handle: usize, _cmd: RangeStopCmdPacket) -> Result<()> {
        todo!()
    }

    async fn range_get_ranging_count(
        &mut self,
        _device_handle: usize,
        _cmd: RangeGetRangingCountCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn init_device(
        &mut self,
        _device_handle: usize,
        _cmd: PicaInitDeviceCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn set_device_position(
        &mut self,
        _device_handle: usize,
        _cmd: PicaSetDevicePositionCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn create_beacon(&mut self, _cmd: PicaCreateBeaconCmdPacket) -> Result<()> {
        todo!()
    }

    async fn set_beacon_position(&mut self, _cmd: PicaSetBeaconPositionCmdPacket) -> Result<()> {
        todo!()
    }

    async fn destroy_beacon(&mut self, _cmd: PicaDestroyBeaconCmdPacket) -> Result<()> {
        todo!()
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
