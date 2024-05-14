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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::pin::Pin;
use thiserror::Error;
use tokio::sync::{broadcast, mpsc, oneshot};

pub mod packets;
mod pcapng;

use packets::uci::{self, *};

mod device;
use device::{Device, MAX_DEVICE, MAX_SESSION};

mod session;

mod mac_address;
pub use mac_address::MacAddress;

mod app_config;
pub use app_config::AppConfig;

pub type UciPacket = Vec<u8>;
pub type UciStream = Pin<Box<dyn futures::stream::Stream<Item = Vec<u8>> + Send>>;
pub type UciSink = Pin<Box<dyn futures::sink::Sink<Vec<u8>, Error = anyhow::Error> + Send>>;

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
pub trait RangingEstimator: Send + Sync {
    /// Evaluate the ranging measurement for the two input devices
    /// identified by their respective handle. The result is a triplet
    /// containing the range, azimuth, and elevation of the right device
    /// relative to the left device.
    /// Return `None` if the measurement could not be estimated, e.g. because
    /// the devices are out of range.
    fn estimate(&self, left: &Handle, right: &Handle) -> Option<RangingMeasurement>;
}

/// Pica emulation environment.
/// All the devices added to this environment are emulated as if they were
/// from the same physical space.
pub struct Pica {
    counter: usize,
    devices: HashMap<Handle, Device>,
    anchors: HashMap<MacAddress, Anchor>,
    command_rx: Option<mpsc::Receiver<PicaCommand>>,
    command_tx: mpsc::Sender<PicaCommand>,
    event_tx: broadcast::Sender<PicaEvent>,
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

pub enum PicaCommand {
    // Connect a new device.
    Connect(UciStream, UciSink),
    // Disconnect the selected device.
    Disconnect(usize),
    // Execute ranging command for selected device and session.
    Ranging(usize, u32),
    // Send an in-band request to stop ranging to a peer controlee identified by address and session id.
    StopRanging(MacAddress, u32),
    // UCI packet received for the selected device.
    UciPacket(usize, Vec<u8>),
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
            PicaCommand::Connect(_, _) => "Connect",
            PicaCommand::Disconnect(_) => "Disconnect",
            PicaCommand::Ranging(_, _) => "Ranging",
            PicaCommand::StopRanging(_, _) => "StopRanging",
            PicaCommand::UciPacket(_, _) => "UciPacket",
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

fn make_measurement(
    mac_address: &MacAddress,
    local: RangingMeasurement,
    remote: RangingMeasurement,
) -> ShortAddressTwoWayRangingMeasurement {
    if let MacAddress::Short(address) = mac_address {
        ShortAddressTwoWayRangingMeasurement {
            mac_address: u16::from_le_bytes(*address),
            status: uci::Status::Ok,
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
            command_rx: Some(command_rx),
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

    fn send_event(&self, event: PicaEvent) {
        // An error here means that we have
        // no receivers, so ignore it
        let _ = self.event_tx.send(event);
    }

    /// Handle an incoming stream of UCI packets.
    /// Reassemble control packets when fragmented, data packets are unmodified.
    async fn read_routine(
        mut uci_stream: impl futures::stream::Stream<Item = Vec<u8>> + Unpin,
        cmd_tx: mpsc::Sender<PicaCommand>,
        handle: Handle,
        pcapng_file: Option<&pcapng::File>,
    ) -> anyhow::Result<()> {
        use futures::stream::StreamExt;

        loop {
            let mut complete_packet: Option<Vec<u8>> = None;
            loop {
                let packet = uci_stream
                    .next()
                    .await
                    .ok_or(anyhow::anyhow!("input packet stream closed"))?;
                let header =
                    packets::uci::CommonPacketHeader::parse(&packet[0..COMMON_HEADER_SIZE])?;

                if let Some(file) = pcapng_file {
                    file.write(&packet, pcapng::Direction::Tx)?;
                }

                match &mut complete_packet {
                    Some(complete_packet) => {
                        complete_packet.extend_from_slice(&packet[HEADER_SIZE..])
                    }
                    None => complete_packet = Some(packet),
                }

                if header.get_pbf() == packets::uci::PacketBoundaryFlag::Complete
                    || header.get_mt() == packets::uci::MessageType::Data
                {
                    break;
                }
            }

            cmd_tx
                .send(PicaCommand::UciPacket(handle, complete_packet.unwrap()))
                .await
                .unwrap()
        }
    }

    /// Segment a stream of UCI packets.
    async fn write_routine(
        mut uci_sink: impl futures::sink::Sink<Vec<u8>> + Unpin,
        mut packet_rx: mpsc::UnboundedReceiver<UciPacket>,
        _handle: Handle,
        pcapng_file: Option<&pcapng::File>,
    ) -> anyhow::Result<()> {
        use futures::sink::SinkExt;

        loop {
            let complete_packet = packet_rx
                .recv()
                .await
                .ok_or(anyhow::anyhow!("output packet stream closed"))?;
            let mut offset = HEADER_SIZE;
            let mt = parse_message_type(complete_packet[0]);

            while offset < complete_packet.len() {
                let remaining_length = complete_packet.len() - offset;
                let fragment_length = std::cmp::min(
                    remaining_length,
                    if mt == MessageType::Data {
                        MAX_DATA_PACKET_PAYLOAD_SIZE
                    } else {
                        MAX_CTRL_PACKET_PAYLOAD_SIZE
                    },
                );
                let pbf = if fragment_length == remaining_length {
                    PacketBoundaryFlag::Complete
                } else {
                    PacketBoundaryFlag::NotComplete
                };

                let mut packet = Vec::with_capacity(HEADER_SIZE + fragment_length);

                packet.extend_from_slice(&complete_packet[0..HEADER_SIZE]);
                const PBF_MASK: u8 = 0x10;
                packet[0] &= !PBF_MASK;
                packet[0] |= (pbf as u8) << 4;

                match mt {
                    MessageType::Data => {
                        packet[2..4].copy_from_slice(&(fragment_length as u16).to_le_bytes())
                    }
                    _ => packet[3] = fragment_length as u8,
                }

                packet.extend_from_slice(&complete_packet[offset..offset + fragment_length]);

                if let Some(file) = pcapng_file {
                    file.write(&packet, pcapng::Direction::Rx)?;
                }

                uci_sink
                    .send(packet)
                    .await
                    .map_err(|_| anyhow::anyhow!("output packet sink closed"))?;

                offset += fragment_length;
            }
        }
    }

    pub fn add_device(&mut self, stream: UciStream, sink: UciSink) -> Result<Handle> {
        let (packet_tx, packet_rx) = mpsc::unbounded_channel();
        let pica_tx = self.command_tx.clone();
        let disconnect_tx = self.command_tx.clone();
        let pcapng_dir = self.pcapng_dir.clone();

        let handle = self.counter;
        self.counter += 1;

        log::debug!("[{}] Connecting device", handle);

        let mac_address = MacAddress::Short((handle as u16).to_be_bytes());
        let mut device = Device::new(handle, mac_address, packet_tx, self.command_tx.clone());
        device.init();

        self.send_event(PicaEvent::Connected {
            handle,
            mac_address: device.mac_address,
        });

        self.devices.insert(handle, device);

        // Spawn and detach the connection handling task.
        // The task notifies pica when exiting to let it clean
        // the state.
        tokio::task::spawn(async move {
            let pcapng_file = if let Some(dir) = pcapng_dir {
                let full_path = dir.join(format!("device-{}.pcapng", handle));
                log::debug!("Recording pcapng to file {}", full_path.as_path().display());
                Some(pcapng::File::create(full_path).unwrap())
            } else {
                None
            };

            let _ = tokio::try_join!(
                async { Self::read_routine(stream, pica_tx, handle, pcapng_file.as_ref()).await },
                async { Self::write_routine(sink, packet_rx, handle, pcapng_file.as_ref()).await }
            );

            disconnect_tx
                .send(PicaCommand::Disconnect(handle))
                .await
                .unwrap()
        });

        Ok(handle)
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

    fn ranging(&mut self, device_handle: usize, session_id: u32) {
        log::debug!("[{}] Ranging event", device_handle);
        log::debug!("  session_id={}", session_id);

        let device = self.get_device(device_handle).unwrap();
        let session = device.session(session_id).unwrap();

        let mut data_transfer = Vec::new();
        let mut measurements = Vec::new();

        // Look for compatible anchors.
        for mac_address in session.get_dst_mac_address() {
            if let Some(other) = self.anchors.get(mac_address) {
                let Some(local) = self
                    .ranging_estimator
                    .estimate(&device.handle, &other.handle)
                else {
                    continue;
                };
                let Some(remote) = self
                    .ranging_estimator
                    .estimate(&other.handle, &device.handle)
                else {
                    continue;
                };
                measurements.push(make_measurement(mac_address, local, remote));
            }
        }

        // Look for compatible ranging sessions in other devices.
        for peer_device in self.devices.values() {
            if peer_device.handle == device_handle {
                continue;
            }

            if peer_device.can_start_ranging(session, session_id) {
                let peer_mac_address = peer_device
                    .session(session_id)
                    .unwrap()
                    .app_config
                    .device_mac_address
                    .unwrap();
                let local = self
                    .ranging_estimator
                    .estimate(&device.handle, &peer_device.handle)
                    .unwrap_or(Default::default());
                let remote = self
                    .ranging_estimator
                    .estimate(&peer_device.handle, &device.handle)
                    .unwrap_or(Default::default());
                measurements.push(make_measurement(&peer_mac_address, local, remote));
            }

            if device.can_start_data_transfer(session_id)
                && peer_device.can_receive_data_transfer(session_id)
            {
                data_transfer.push(peer_device);
            }
        }

        // TODO: Data transfer should be limited in size for
        // each round of ranging
        for peer_device in data_transfer.iter() {
            peer_device
                .tx
                .send(
                    DataMessageRcvBuilder {
                        application_data: session.data().clone().into(),
                        data_sequence_number: 0x01,
                        pbf: PacketBoundaryFlag::Complete,
                        session_handle: session_id,
                        source_address: session.app_config.device_mac_address.unwrap().into(),
                        status: uci::Status::Ok,
                    }
                    .build()
                    .into(),
                )
                .unwrap();
        }
        if session.is_session_info_ntf_enabled() {
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

    fn uci_packet(&mut self, device_handle: usize, packet: Vec<u8>) {
        match self.get_device_mut(device_handle) {
            Some(device) => device.receive_packet(packet),
            None => log::error!("Device {} not found", device_handle),
        }
    }

    fn pica_command(&mut self, command: PicaCommand) {
        use PicaCommand::*;
        match command {
            Connect(stream, sink) => {
                let _ = self.add_device(stream, sink);
            }
            Disconnect(device_handle) => self.disconnect(device_handle),
            Ranging(device_handle, session_id) => self.ranging(device_handle, session_id),
            StopRanging(mac_address, session_id) => {
                self.stop_controlee_ranging(&mac_address, session_id)
            }
            UciPacket(device_handle, packet) => self.uci_packet(device_handle, packet),
            CreateAnchor(mac_address, pica_cmd_rsp_tx) => {
                self.create_anchor(mac_address, pica_cmd_rsp_tx)
            }
            DestroyAnchor(mac_address, pica_cmd_rsp_tx) => {
                self.destroy_anchor(mac_address, pica_cmd_rsp_tx)
            }
        }
    }

    /// Run the internal pica event loop.
    pub async fn run(mut self) -> Result<()> {
        let Some(mut command_rx) = self.command_rx.take() else {
            anyhow::bail!("missing pica command receiver")
        };
        loop {
            if let Some(command) = command_rx.recv().await {
                self.pica_command(command)
            }
        }
    }

    // Handle the in-band StopRanging command sent from controller to the controlee with
    // corresponding mac_address and session_id.
    fn stop_controlee_ranging(&mut self, mac_address: &MacAddress, session_id: u32) {
        for device in self.devices.values_mut() {
            let Some(session) = device.session_mut(session_id) else {
                continue;
            };

            if session.app_config.device_mac_address != Some(*mac_address) {
                continue;
            }

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

/// Run the internal pica event loop.
/// As opposed to Pica::run, the context is passed under a mutex, which
/// allows synchronous access to the context for device creation.
pub async fn run(this: &std::sync::Mutex<Pica>) -> Result<()> {
    // Extract the mpsc receiver from the Pica context.
    // The receiver cannot be cloned.
    let Some(mut command_rx) = this.lock().unwrap().command_rx.take() else {
        anyhow::bail!("missing pica command receiver");
    };

    loop {
        if let Some(command) = command_rx.recv().await {
            this.lock().unwrap().pica_command(command)
        }
    }
}
