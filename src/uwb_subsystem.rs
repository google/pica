use anyhow::Result;
use bytes::BytesMut;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::uci_packets::*;

const MAX_SESSION: usize = 4;
const MAX_PAYLOAD_SIZE: usize = 4096;

#[derive(Copy, Clone)]
enum DeviceState {
    Ready,
    Active,
    Error,
}

#[derive(Copy, Clone)]
enum SessionState {
    Init,
    Idle,
    Active,
    Deinit,
}

struct Session {
    state: SessionState,
    sequence_number: usize,
    ranging_interval: usize,
    ranging_task: Option<JoinHandle<()>>,
}

pub struct Device {
    connection: Connection,

    mac_address: usize,
    state: DeviceState,
    sessions: [Session; MAX_SESSION],

    notify_tx: mpsc::Sender<UciPacketPacket>,
    notify_rx: mpsc::Receiver<UciPacketPacket>,
}

struct Connection {
    socket: TcpStream,
    buffer: BytesMut,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            state: SessionState::Init,
            sequence_number: 0,
            ranging_interval: 0,
            ranging_task: None,
        }
    }
}

impl Connection {
    async fn read(&mut self) -> Result<Option<UciPacketPacket>> {
        let len = self.socket.read_buf(&mut self.buffer).await?;
        if len == 0 {
            return Ok(None);
        }

        let packet = UciPacketPacket::parse(&self.buffer)?;
        self.buffer.clear();
        Ok(Some(packet))
    }

    async fn write(&mut self, packet: UciPacketPacket) -> Result<()> {
        let _ = self.socket.try_write(&packet.to_bytes())?;
        Ok(())
    }
}

impl Device {
    pub fn new(mac_address: usize, socket: TcpStream) -> Self {
        let (notify_tx, notify_rx) = mpsc::channel(MAX_SESSION);
        Device {
            mac_address,
            connection: Connection {
                socket,
                buffer: BytesMut::with_capacity(MAX_PAYLOAD_SIZE),
            },
            state: DeviceState::Ready,
            sessions: Default::default(),
            notify_tx,
            notify_rx,
        }
    }

    async fn device_reset(&mut self, command: DeviceResetCmdPacket) -> Result<()> {
        let reset_config = command.get_reset_config();
        println!("[{}] Device Reset", self.mac_address);
        println!("  reset_config={}", reset_config);
        self.connection
            .write(
                DeviceResetRspBuilder {
                    status: StatusCode::UciStatusOk,
                }
                .build()
                .into(),
            )
            .await
    }

    async fn get_device_info(&mut self, _command: GetDeviceInfoCmdPacket) -> Result<()> {
        todo!()
    }

    async fn get_caps_info(&mut self, _command: GetCapsInfoCmdPacket) -> Result<()> {
        todo!()
    }

    async fn set_config(&mut self, _command: SetConfigCmdPacket) -> Result<()> {
        todo!()
    }

    async fn get_config(&mut self, _command: GetConfigCmdPacket) -> Result<()> {
        todo!()
    }

    async fn session_init(&mut self, _command: SessionInitCmdPacket) -> Result<()> {
        todo!()
    }

    async fn session_deinit(&mut self, _command: SessionDeinitCmdPacket) -> Result<()> {
        todo!()
    }

    async fn session_set_app_config(
        &mut self,
        _command: SessionSetAppConfigCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn session_get_app_config(
        &mut self,
        _command: SessionGetAppConfigCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn session_get_count(&mut self, _command: SessionGetCountCmdPacket) -> Result<()> {
        todo!()
    }

    async fn session_get_state(&mut self, _command: SessionGetStateCmdPacket) -> Result<()> {
        todo!()
    }

    async fn session_update_controller_multicast_list(
        &mut self,
        _command: SessionUpdateControllerMulticastListCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn range_start(&mut self, _command: RangeStartCmdPacket) -> Result<()> {
        todo!()
    }

    async fn range_stop(&mut self, _command: RangeStopCmdPacket) -> Result<()> {
        todo!()
    }

    async fn range_get_ranging_count(
        &mut self,
        _command: RangeGetRangingCountCmdPacket,
    ) -> Result<()> {
        todo!()
    }

    async fn handle_command(&mut self, packet: UciPacketPacket) -> Result<()> {
        let command: UciCommandPacket = packet.try_into().unwrap();
        match command.specialize() {
            UciCommandChild::CoreCommand(core_command) => match core_command.specialize() {
                CoreCommandChild::DeviceResetCmd(cmd) => self.device_reset(cmd).await,
                CoreCommandChild::GetDeviceInfoCmd(cmd) => self.get_device_info(cmd).await,
                CoreCommandChild::GetCapsInfoCmd(cmd) => self.get_caps_info(cmd).await,
                CoreCommandChild::SetConfigCmd(cmd) => self.set_config(cmd).await,
                CoreCommandChild::GetConfigCmd(cmd) => self.get_config(cmd).await,
                CoreCommandChild::None => anyhow::bail!("Unsupported core command"),
            },
            UciCommandChild::SessionCommand(session_command) => {
                match session_command.specialize() {
                    SessionCommandChild::SessionInitCmd(cmd) => self.session_init(cmd).await,
                    SessionCommandChild::SessionDeinitCmd(cmd) => self.session_deinit(cmd).await,
                    SessionCommandChild::SessionSetAppConfigCmd(cmd) => {
                        self.session_set_app_config(cmd).await
                    }
                    SessionCommandChild::SessionGetAppConfigCmd(cmd) => {
                        self.session_get_app_config(cmd).await
                    }
                    SessionCommandChild::SessionGetCountCmd(cmd) => {
                        self.session_get_count(cmd).await
                    }
                    SessionCommandChild::SessionGetStateCmd(cmd) => {
                        self.session_get_state(cmd).await
                    }
                    SessionCommandChild::SessionUpdateControllerMulticastListCmd(cmd) => {
                        self.session_update_controller_multicast_list(cmd).await
                    }
                    SessionCommandChild::None => anyhow::bail!("Unsupported session command"),
                }
            }
            UciCommandChild::RangingCommand(ranging_command) => {
                match ranging_command.specialize() {
                    RangingCommandChild::RangeStartCmd(cmd) => self.range_start(cmd).await,
                    RangingCommandChild::RangeStopCmd(cmd) => self.range_stop(cmd).await,
                    RangingCommandChild::RangeGetRangingCountCmd(cmd) => {
                        self.range_get_ranging_count(cmd).await
                    }
                    RangingCommandChild::None => anyhow::bail!("Unsupported ranging command"),
                }
            }

            _ => anyhow::bail!("Unsupported command type"),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                // Read command packet sent from connected UWB host.
                // Run associated command.
                Ok(Some(packet)) = self.connection.read() =>
                    self.handle_command(packet).await?,

                // Send notifications generated from ranging sessions
                // to the connected host.
                Some(notification) = self.notify_rx.recv() =>
                    self.connection.write(notification).await?,
            }
        }
    }
}
