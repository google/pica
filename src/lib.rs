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

use pdl_runtime::Packet;
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
pub use app_config::{AppConfig, AppConfigError};

pub type UciPacket = Vec<u8>;
pub type UciStream = Pin<Box<dyn futures::stream::Stream<Item = Vec<u8>> + Send>>;
pub type UciSink = Pin<Box<dyn futures::sink::Sink<Vec<u8>, Error = ()> + Send>>;

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
pub enum PicaError {
    #[error("no available short MAC address for new device")]
    NoAvailableMacAddress,
    #[error("pica is already running")]
    AlreadyRunning,
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
        write!(f, "{cmd}")
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

const fn degrees_to_q9_7(deg: i16) -> i16 {
    deg.saturating_mul(128)
}

#[derive(Debug)]
enum TwoWayRangingMeasurement {
    Short(ShortAddressTwoWayRangingMeasurement),
    Extended(ExtendedAddressTwoWayRangingMeasurement),
}

impl TwoWayRangingMeasurement {
    fn new(
        mac_address: &MacAddress,
        local: RangingMeasurement,
        remote: RangingMeasurement,
    ) -> Self {
        match mac_address {
            MacAddress::Short(address) => {
                Self::Short(ShortAddressTwoWayRangingMeasurement {
                    mac_address: u16::from_le_bytes(*address),
                    status: uci::Status::Ok,
                    nlos: 0, // in Line Of Sight
                    distance: local.range,
                    aoa_azimuth: degrees_to_q9_7(local.azimuth) as u16,
                    aoa_azimuth_fom: 100, // Yup, pretty sure about this
                    aoa_elevation: degrees_to_q9_7(i16::from(local.elevation)) as u16,
                    aoa_elevation_fom: 100, // Yup, pretty sure about this
                    aoa_destination_azimuth: degrees_to_q9_7(remote.azimuth) as u16,
                    aoa_destination_azimuth_fom: 100,
                    aoa_destination_elevation: degrees_to_q9_7(i16::from(remote.elevation)) as u16,
                    aoa_destination_elevation_fom: 100,
                    slot_index: 0,
                    rssi: u8::MAX,
                })
            }
            MacAddress::Extended(address) => {
                Self::Extended(ExtendedAddressTwoWayRangingMeasurement {
                    mac_address: u64::from_le_bytes(*address),
                    status: uci::Status::Ok,
                    nlos: 0,
                    distance: local.range,
                    aoa_azimuth: degrees_to_q9_7(local.azimuth) as u16,
                    aoa_azimuth_fom: 100,
                    aoa_elevation: degrees_to_q9_7(i16::from(local.elevation)) as u16,
                    aoa_elevation_fom: 100,
                    aoa_destination_azimuth: degrees_to_q9_7(remote.azimuth) as u16,
                    aoa_destination_azimuth_fom: 100,
                    aoa_destination_elevation: degrees_to_q9_7(i16::from(remote.elevation)) as u16,
                    aoa_destination_elevation_fom: 100,
                    slot_index: 0,
                    rssi: u8::MAX,
                })
            }
        }
    }

    fn timeout(mac_address: &MacAddress) -> Self {
        match mac_address {
            MacAddress::Short(address) => Self::Short(ShortAddressTwoWayRangingMeasurement {
                mac_address: u16::from_le_bytes(*address),
                status: uci::Status::RangingRxTimeout,
                nlos: 0,
                distance: 0,
                aoa_azimuth: 0,
                aoa_azimuth_fom: 0,
                aoa_elevation: 0,
                aoa_elevation_fom: 0,
                aoa_destination_azimuth: 0,
                aoa_destination_azimuth_fom: 0,
                aoa_destination_elevation: 0,
                aoa_destination_elevation_fom: 0,
                slot_index: 0,
                rssi: 0,
            }),
            MacAddress::Extended(address) => {
                Self::Extended(ExtendedAddressTwoWayRangingMeasurement {
                    mac_address: u64::from_le_bytes(*address),
                    status: uci::Status::RangingRxTimeout,
                    nlos: 0,
                    distance: 0,
                    aoa_azimuth: 0,
                    aoa_azimuth_fom: 0,
                    aoa_elevation: 0,
                    aoa_elevation_fom: 0,
                    aoa_destination_azimuth: 0,
                    aoa_destination_azimuth_fom: 0,
                    aoa_destination_elevation: 0,
                    aoa_destination_elevation_fom: 0,
                    slot_index: 0,
                    rssi: 0,
                })
            }
        }
    }

    fn status(&self) -> uci::Status {
        match self {
            Self::Short(m) => m.status,
            Self::Extended(m) => m.status,
        }
    }

    fn mac_address(&self) -> MacAddress {
        match self {
            Self::Short(m) => MacAddress::Short(m.mac_address.to_le_bytes()),
            Self::Extended(m) => MacAddress::Extended(m.mac_address.to_le_bytes()),
        }
    }

    fn distance(&self) -> u16 {
        match self {
            Self::Short(m) => m.distance,
            Self::Extended(m) => m.distance,
        }
    }

    fn aoa_azimuth(&self) -> i16 {
        match self {
            Self::Short(m) => m.aoa_azimuth as i16,
            Self::Extended(m) => m.aoa_azimuth as i16,
        }
    }

    fn aoa_elevation(&self) -> i16 {
        match self {
            Self::Short(m) => m.aoa_elevation as i16,
            Self::Extended(m) => m.aoa_elevation as i16,
        }
    }

    fn is_within_bounds(&self, app_config: &AppConfig) -> bool {
        self.status() == uci::Status::Ok
            && app_config.is_within_bounds(
                self.distance(),
                self.aoa_azimuth(),
                self.aoa_elevation(),
            )
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
    ) {
        use futures::stream::StreamExt;

        loop {
            let mut complete_packet: Option<Vec<u8>> = None;
            loop {
                let Some(packet) = uci_stream.next().await else {
                    return;
                };
                let Ok(header) =
                    packets::uci::CommonPacketHeader::decode_full(&packet[0..COMMON_HEADER_SIZE])
                else {
                    return;
                };

                if let Some(file) = pcapng_file
                    && file.write(&packet, pcapng::Direction::Tx).is_err()
                {
                    return;
                }

                match &mut complete_packet {
                    Some(complete_packet) => {
                        complete_packet.extend_from_slice(&packet[HEADER_SIZE..])
                    }
                    None => complete_packet = Some(packet),
                }

                if header.pbf == packets::uci::PacketBoundaryFlag::Complete
                    || header.mt == packets::uci::MessageType::Data
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
    ) {
        use futures::sink::SinkExt;

        loop {
            let Some(complete_packet) = packet_rx.recv().await else {
                return;
            };
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

                if let Some(file) = pcapng_file
                    && file.write(&packet, pcapng::Direction::Rx).is_err()
                {
                    return;
                }

                if uci_sink.send(packet).await.is_err() {
                    return;
                }

                offset += fragment_length;
            }
        }
    }

    pub fn add_device(&mut self, stream: UciStream, sink: UciSink) -> Result<Handle, PicaError> {
        let (packet_tx, packet_rx) = mpsc::unbounded_channel();
        let pica_tx = self.command_tx.clone();
        let disconnect_tx = self.command_tx.clone();
        let pcapng_dir = self.pcapng_dir.clone();

        let initial_counter = self.counter;
        let (handle, mac_address) = loop {
            let handle = self.counter;
            self.counter += 1;
            let mac_address = MacAddress::Short((handle as u16).to_be_bytes());
            if self.get_category(&mac_address).is_none() {
                break (handle, mac_address);
            }
            if self.counter - initial_counter == 1 << 16 {
                self.counter = initial_counter;
                return Err(PicaError::NoAvailableMacAddress);
            }
        };

        tracing::debug!("[{handle}] Connecting device");

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
                let full_path = dir.join(format!("device-{handle}.pcapng"));
                tracing::debug!("Recording pcapng to file {}", full_path.as_path().display());
                Some(pcapng::File::create(full_path).unwrap())
            } else {
                None
            };

            tokio::select! {
                () = Self::read_routine(stream, pica_tx, handle, pcapng_file.as_ref()) => (),
                () = Self::write_routine(sink, packet_rx, handle, pcapng_file.as_ref()) => (),
            }

            disconnect_tx
                .send(PicaCommand::Disconnect(handle))
                .await
                .unwrap()
        });

        Ok(handle)
    }

    fn disconnect(&mut self, device_handle: usize) {
        tracing::debug!("[{device_handle}] Disconnecting device");

        if let Some(device) = self.devices.get(&device_handle) {
            self.send_event(PicaEvent::Disconnected {
                handle: device_handle,
                mac_address: device.mac_address,
            });
            self.devices.remove(&device_handle);
        }
    }

    fn ranging(&mut self, device_handle: usize, session_id: u32) {
        tracing::debug!("[{device_handle}] Ranging event");
        tracing::debug!("  session_id={session_id}");

        let Some(device) = self.get_device(device_handle) else {
            return;
        };
        let Some(session) = device.session(session_id) else {
            return;
        };
        // A Ranging command queued before the ranging task was aborted can
        // still be delivered after the session left the active state.
        if session.session_state() != SessionState::SessionStateActive {
            return;
        }

        let mut measurements = Vec::new();

        for mac_address in session.get_dst_mac_address() {
            let peer_handle = self
                .anchors
                .get(&mac_address)
                .map(|anchor| anchor.handle)
                .or_else(|| {
                    self.devices
                        .values()
                        .find(|peer_device| {
                            peer_device.handle != device_handle
                                && peer_device.can_start_ranging(session, session_id)
                                && peer_device
                                    .session(session_id)
                                    .and_then(|s| s.app_config.device_mac_address())
                                    == Some(mac_address)
                        })
                        .map(|peer_device| peer_device.handle)
                });
            let measurement = peer_handle
                .and_then(|peer_handle| {
                    let local = self
                        .ranging_estimator
                        .estimate(&device.handle, &peer_handle)?;
                    let remote = self
                        .ranging_estimator
                        .estimate(&peer_handle, &device.handle)?;
                    Some(TwoWayRangingMeasurement::new(&mac_address, local, remote))
                })
                .unwrap_or_else(|| TwoWayRangingMeasurement::timeout(&mac_address));
            measurements.push(measurement);
        }

        if let Some(data_transfer) = &session.data_transfer {
            let dst_mac_address = data_transfer.dst_mac_address;
            let peer_tx = if session.can_start_data_transfer() {
                self.devices
                    .values()
                    .find(|peer_device| {
                        peer_device.handle != device_handle
                            && peer_device.can_receive_data_transfer(session_id, dst_mac_address)
                    })
                    .map(|peer_device| peer_device.tx.clone())
            } else {
                None
            };
            let device = self.get_device_mut(device_handle).unwrap();
            let session = device.session_mut(session_id).unwrap();
            let data_transfer = session.data_transfer.as_mut().unwrap();
            data_transfer.tx_count = data_transfer.tx_count.saturating_add(1);
            let tx_count = data_transfer.tx_count;
            let sequence_number = data_transfer.sequence_number;
            let payload = data_transfer.payload.clone();
            let repetition_count = session.app_config.data_repetition_count();
            let is_status_ntf_enabled = session.is_session_data_transfer_status_ntf_enabled();

            let (status, is_complete) = if let Some(peer_tx) = peer_tx {
                peer_tx
                    .send(
                        DataMessageRcv {
                            application_data: payload.into(),
                            data_sequence_number: sequence_number,
                            pbf: PacketBoundaryFlag::Complete,
                            session_handle: session_id,
                            source_address: session.app_config.device_mac_address().unwrap().into(),
                            status: uci::DataRcvStatusCode::UciStatusSuccess,
                        }
                        .encode_to_vec()
                        .unwrap(),
                    )
                    .unwrap();

                if tx_count <= repetition_count {
                    (
                        DataTransferNtfStatusCode::UciDataTransferStatusRepetitionOk,
                        false,
                    )
                } else {
                    (DataTransferNtfStatusCode::UciDataTransferStatusOk, true)
                }
            } else {
                (
                    DataTransferNtfStatusCode::UciDataTransferStatusErrorDataTransfer,
                    true,
                )
            };

            if is_complete {
                session.clear_data();
                device
                    .tx
                    .send(
                        SessionDataCreditNtf {
                            credit_availability: CreditAvailability::CreditAvailable,
                            session_token: session_id,
                        }
                        .encode_to_vec()
                        .unwrap(),
                    )
                    .unwrap();
            }

            if is_status_ntf_enabled || is_complete {
                device
                    .tx
                    .send(
                        SessionDataTransferStatusNtf {
                            session_token: session_id,
                            uci_sequence_number: sequence_number,
                            status,
                            tx_count,
                        }
                        .encode_to_vec()
                        .unwrap(),
                    )
                    .unwrap();
            }
        }
        let Some(device) = self.get_device_mut(device_handle) else {
            return;
        };
        let tx = device.tx.clone();
        let Some(session) = device.session_mut(session_id) else {
            return;
        };

        if session.session_type() == SessionType::FiraDataTransferSession {
            return;
        }

        let (send_ntf, measurements) = match session.app_config.session_info_ntf_config() {
            uci::SessionInfoNtfConfig::Disable => (false, Vec::new()),
            uci::SessionInfoNtfConfig::Enable => (true, measurements),
            uci::SessionInfoNtfConfig::EnableProximityTrigger
            | uci::SessionInfoNtfConfig::EnableAoaTrigger
            | uci::SessionInfoNtfConfig::EnableProximityAoaTrigger => {
                let filtered: Vec<_> = measurements
                    .into_iter()
                    .filter(|m| m.is_within_bounds(&session.app_config))
                    .collect();
                (!filtered.is_empty(), filtered)
            }
            uci::SessionInfoNtfConfig::EnableProximityEdgeTrigger
            | uci::SessionInfoNtfConfig::EnableAoaEdgeTrigger
            | uci::SessionInfoNtfConfig::EnableProximityAoaEdgeTrigger => {
                let emitted: Vec<_> = measurements
                    .into_iter()
                    .filter(|m| {
                        if m.is_within_bounds(&session.app_config) {
                            session.in_range_peers.insert(m.mac_address())
                        } else {
                            session.in_range_peers.remove(&m.mac_address())
                        }
                    })
                    .collect();
                (!emitted.is_empty(), emitted)
            }
        };

        if send_ntf {
            let ntf_packet = match session.app_config.mac_address_mode() {
                uci::MacAddressMode::Mode0 => Some(
                    ShortMacTwoWaySessionInfoNtf {
                        sequence_number: session.sequence_number,
                        session_token: session_id,
                        rcr_indicator: 0,            //TODO
                        current_ranging_interval: 0, //TODO
                        two_way_ranging_measurements: measurements
                            .into_iter()
                            .filter_map(|m| match m {
                                TwoWayRangingMeasurement::Short(m) => Some(m),
                                TwoWayRangingMeasurement::Extended(_) => None,
                            })
                            .collect(),
                        vendor_data: vec![],
                    }
                    .encode_to_vec()
                    .unwrap(),
                ),
                uci::MacAddressMode::Mode2 => Some(
                    ExtendedMacTwoWaySessionInfoNtf {
                        sequence_number: session.sequence_number,
                        session_token: session_id,
                        rcr_indicator: 0,            //TODO
                        current_ranging_interval: 0, //TODO
                        two_way_ranging_measurements: measurements
                            .into_iter()
                            .filter_map(|m| match m {
                                TwoWayRangingMeasurement::Extended(m) => Some(m),
                                TwoWayRangingMeasurement::Short(_) => None,
                            })
                            .collect(),
                        vendor_data: vec![],
                    }
                    .encode_to_vec()
                    .unwrap(),
                ),
                mode => {
                    tracing::warn!("Unsupported MAC address mode {mode:?}");
                    None
                }
            };

            if let Some(packet) = ntf_packet {
                tx.send(packet).unwrap();
                session.sequence_number += 1;
            }
        }
        session.ranging_count += 1;
        let max_measurements = u32::from(session.app_config.max_number_of_measurements());
        let max_reached = max_measurements > 0 && session.ranging_count >= max_measurements;
        if max_reached {
            Self::stop_active_session(
                device,
                session_id,
                ReasonCode::MaxNumberOfMeasurementsReached,
            );
        }
    }

    fn uci_packet(&mut self, device_handle: usize, packet: Vec<u8>) {
        match self.get_device_mut(device_handle) {
            Some(device) => device.receive_packet(packet),
            None => tracing::error!("Device {device_handle} not found"),
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
    pub async fn run(mut self) -> Result<(), PicaError> {
        let Some(mut command_rx) = self.command_rx.take() else {
            return Err(PicaError::AlreadyRunning);
        };
        loop {
            if let Some(command) = command_rx.recv().await {
                self.pica_command(command)
            }
        }
    }

    fn stop_active_session(device: &mut Device, session_id: u32, reason_code: ReasonCode) {
        let Some(session) = device.session_mut(session_id) else {
            return;
        };

        if session.session_state() == SessionState::SessionStateActive {
            session.stop_ranging_task();
            session.set_state(SessionState::SessionStateIdle, reason_code);
            device.on_session_deactivated();
        } else {
            tracing::warn!("stop_active_session: session is not active !");
        }
    }

    // Handle the in-band StopRanging command sent from controller to the controlee with
    // corresponding mac_address and session_id.
    fn stop_controlee_ranging(&mut self, mac_address: &MacAddress, session_id: u32) {
        for device in self.devices.values_mut() {
            let Some(session) = device.session(session_id) else {
                continue;
            };

            if session.app_config.device_mac_address() != Some(*mac_address) {
                continue;
            }

            Self::stop_active_session(
                device,
                session_id,
                ReasonCode::SessionStoppedDueToInbandSignal,
            );
        }
    }

    #[allow(clippy::map_entry)]
    fn create_anchor(
        &mut self,
        mac_address: MacAddress,
        rsp_tx: oneshot::Sender<Result<Handle, PicaCommandError>>,
    ) {
        tracing::debug!("[_] Create anchor");
        tracing::debug!("  mac_address: {mac_address}");

        let status = if self.get_category(&mac_address).is_some() {
            Err(PicaCommandError::DeviceAlreadyExists(mac_address))
        } else {
            let handle = self.counter;
            self.counter += 1;

            assert!(
                self.anchors
                    .insert(
                        mac_address,
                        Anchor {
                            handle,
                            mac_address,
                        },
                    )
                    .is_none()
            );

            Ok(handle)
        };

        rsp_tx.send(status).unwrap_or_else(|err| {
            tracing::error!("Failed to send create-anchor command response: {err:?}")
        })
    }

    fn destroy_anchor(
        &mut self,
        mac_address: MacAddress,
        rsp_tx: oneshot::Sender<Result<Handle, PicaCommandError>>,
    ) {
        tracing::debug!("[_] Destroy anchor");
        tracing::debug!("  mac_address: {mac_address}");

        let status = match self.anchors.remove(&mac_address) {
            None => Err(PicaCommandError::DeviceNotFound(mac_address)),
            Some(anchor) => Ok(anchor.handle),
        };

        rsp_tx.send(status).unwrap_or_else(|err| {
            tracing::error!("Failed to send destroy-anchor command response: {err:?}")
        })
    }
}

/// Run the internal pica event loop.
/// As opposed to Pica::run, the context is passed under a mutex, which
/// allows synchronous access to the context for device creation.
pub async fn run(this: &std::sync::Mutex<Pica>) -> Result<(), PicaError> {
    // Extract the mpsc receiver from the Pica context.
    // The receiver cannot be cloned.
    let Some(mut command_rx) = this.lock().unwrap().command_rx.take() else {
        return Err(PicaError::AlreadyRunning);
    };

    loop {
        if let Some(command) = command_rx.recv().await {
            this.lock().unwrap().pica_command(command)
        }
    }
}
