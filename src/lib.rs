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

//! Virtual UWB Subsystem (UWBS) implementing the FiRa UCI specification.

#![cfg_attr(
    test,
    allow(
        clippy::indexing_slicing,
        clippy::panic,
        clippy::panic_in_result_fn,
        clippy::unwrap_used,
    )
)]

use pdl_runtime::Packet;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use thiserror::Error;

pub mod packets;
pub mod pcapng;

use packets::uci::{self, *};

mod device;
use device::{Device, DeviceEffect};

mod session;

mod mac_address;
pub use mac_address::MacAddress;

mod app_config;
use app_config::AppConfig;

/// Raw serialized UCI packet bytes.
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
pub trait RangingEstimator: Send + Sync {
    /// Evaluate the ranging measurement for the two input devices
    /// identified by their respective handle. The result is a triplet
    /// containing the range, azimuth, and elevation of the right device
    /// relative to the left device.
    /// Return `None` if the measurement could not be estimated, e.g. because
    /// the devices are out of range.
    fn estimate(&self, left: &Handle, right: &Handle) -> Option<RangingMeasurement>;
}

struct DeviceEntry {
    device: Device,
    rx_buffer: Option<Vec<u8>>,
}

impl DeviceEntry {
    fn new(device: Device) -> Self {
        Self {
            device,
            rx_buffer: None,
        }
    }

    /// Buffers fragmented inbound UCI control packets until [`PacketBoundaryFlag::Complete`] and passes data packets through.
    fn reassemble(&mut self, packet: &[u8]) -> Option<UciPacket> {
        let Some(header_bytes) = packet.get(..COMMON_HEADER_SIZE) else {
            let len = packet.len();
            tracing::warn!(
                "Packet shorter than common header size ({COMMON_HEADER_SIZE} bytes): {len}"
            );
            return None;
        };
        let Ok(header) = CommonPacketHeader::decode_full(header_bytes) else {
            tracing::warn!("Failed to decode common packet header");
            return None;
        };
        let Some(payload) = packet.get(HEADER_SIZE..) else {
            let len = packet.len();
            tracing::warn!("Packet shorter than header size ({HEADER_SIZE} bytes): {len}");
            return None;
        };
        if header.mt == MessageType::Data {
            return Some(packet.to_vec());
        }

        match &mut self.rx_buffer {
            Some(rx_buffer) => rx_buffer.extend_from_slice(payload),
            None => self.rx_buffer = Some(packet.to_vec()),
        }

        if header.pbf == PacketBoundaryFlag::Complete {
            self.rx_buffer.take()
        } else {
            None
        }
    }
}

/// Error returned by [`Pica::create_anchor`] when the MAC address is already in use.
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("MAC address already in use by handle {0}")]
pub struct AlreadyExists(pub Handle);

/// Pica emulation environment.
/// All the devices added to this environment are emulated as if they were
/// from the same physical space.
pub struct Pica {
    next_handle: Handle,
    devices: HashMap<Handle, DeviceEntry>,
    anchors: HashMap<Handle, MacAddress>,
    tx_queue: VecDeque<(Handle, UciPacket)>,
    ranging_estimator: Box<dyn RangingEstimator>,
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

struct DrainPackets<'a>(&'a mut VecDeque<(Handle, UciPacket)>);

impl Iterator for DrainPackets<'_> {
    type Item = (Handle, UciPacket);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for DrainPackets<'_> {}

impl Pica {
    /// Creates a new [`Pica`] emulation environment using the given [`RangingEstimator`].
    pub fn new(ranging_estimator: Box<dyn RangingEstimator>) -> Self {
        Pica {
            devices: HashMap::new(),
            anchors: HashMap::new(),
            next_handle: 0,
            tx_queue: VecDeque::new(),
            ranging_estimator,
        }
    }

    fn get_device_mut(&mut self, device_handle: Handle) -> Option<&mut Device> {
        self.devices
            .get_mut(&device_handle)
            .map(|entry| &mut entry.device)
    }

    fn get_device(&self, device_handle: Handle) -> Option<&Device> {
        self.devices.get(&device_handle).map(|entry| &entry.device)
    }

    /// Returns an iterator over the handles of all connected UCI devices.
    pub fn devices(&self) -> impl Iterator<Item = Handle> + '_ {
        self.devices.keys().copied()
    }

    /// Returns an iterator over the `(Handle, MacAddress)` pairs of all active anchors.
    pub fn anchors(&self) -> impl Iterator<Item = (Handle, MacAddress)> + '_ {
        self.anchors
            .iter()
            .map(|(&handle, &mac_address)| (handle, mac_address))
    }

    fn send_packet(&mut self, device_handle: Handle, complete_packet: UciPacket) {
        if !self.devices.contains_key(&device_handle) {
            return;
        }

        let Some(&[common_header, oid, rfu, _]) = complete_packet.first_chunk::<HEADER_SIZE>()
        else {
            return;
        };
        let mut offset = HEADER_SIZE;
        let mt = parse_message_type(common_header);

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

            let Some(payload_chunk) = complete_packet.get(offset..offset + fragment_length) else {
                return;
            };

            const PBF_MASK: u8 = 0x10;
            let common_header = (common_header & !PBF_MASK) | ((pbf as u8) << 4);
            let header = match mt {
                MessageType::Data => {
                    let [len_lo, len_hi] = (fragment_length as u16).to_le_bytes();
                    [common_header, oid, len_lo, len_hi]
                }
                _ => [common_header, oid, rfu, fragment_length as u8],
            };
            let packet: Vec<u8> = header
                .into_iter()
                .chain(payload_chunk.iter().copied())
                .collect();

            self.tx_queue.push_back((device_handle, packet));

            offset += fragment_length;
        }
    }

    fn apply_device_effects(
        &mut self,
        device_handle: Handle,
        effects: impl IntoIterator<Item = DeviceEffect>,
    ) {
        for effect in effects {
            match effect {
                DeviceEffect::SendPacket(packet) => {
                    self.send_packet(device_handle, packet);
                }
                DeviceEffect::StopControleeRanging {
                    mac_address,
                    session_id,
                } => {
                    for (peer_handle, peer_effect) in
                        self.stop_controlee_ranging(&mac_address, session_id)
                    {
                        if let DeviceEffect::SendPacket(packet) = peer_effect {
                            self.send_packet(peer_handle, packet);
                        }
                    }
                }
            }
        }
    }

    /// Connects a new UCI device and returns its allocated [`Handle`].
    pub fn add_device(&mut self) -> Handle {
        let handle = self.next_handle;
        self.next_handle += 1;

        tracing::debug!("[{handle}] Connecting device");

        let mut device = Device::new(handle);
        let effects = device.init();

        self.devices.insert(handle, DeviceEntry::new(device));
        self.apply_device_effects(handle, effects);

        handle
    }

    /// Disconnects and removes the UCI device with `handle`, returning `true` if it existed.
    pub fn remove_device(&mut self, handle: Handle) -> bool {
        if self.devices.remove(&handle).is_some() {
            tracing::debug!("[{handle}] Disconnecting device");
            self.tx_queue.retain(|(h, _)| *h != handle);
            true
        } else {
            false
        }
    }

    /// Feeds an incoming segmented UCI packet from `handle` into `Pica`.
    pub fn handle_packet(&mut self, handle: Handle, packet: &[u8]) {
        if let Some(complete_packet) = self
            .devices
            .get_mut(&handle)
            .and_then(|entry| entry.reassemble(packet))
        {
            self.uci_packet(handle, complete_packet);
        }
    }

    /// Returns the [`Duration`] until [`Pica::tick`] should next be called,
    /// or `None` if no ranging timers are active and `Pica` has no time-driven
    /// work until new input arrives.
    pub fn next_tick(&self) -> Option<Duration> {
        self.devices
            .values()
            .filter_map(|entry| entry.device.next_ranging_deadline())
            .min()
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }

    /// Advances due ranging sessions.
    pub fn tick(&mut self) {
        let now = Instant::now();
        let due_sessions: Vec<_> = self
            .devices
            .iter_mut()
            .flat_map(|(&handle, entry)| {
                entry
                    .device
                    .poll_ranging(now)
                    .map(move |session_id| (handle, session_id))
            })
            .collect();

        for (handle, session_id) in due_sessions {
            self.ranging(handle, session_id);
        }
    }

    /// Drains pending outbound UCI packets as `(Handle, UciPacket)` pairs.
    ///
    /// Safe to drop early: unconsumed packets remain queued in `Pica`.
    pub fn drain_packets(&mut self) -> impl ExactSizeIterator<Item = (Handle, UciPacket)> + '_ {
        DrainPackets(&mut self.tx_queue)
    }

    fn anchor_handle(&self, mac_address: MacAddress) -> Option<Handle> {
        self.anchors
            .iter()
            .find_map(|(&handle, &addr)| (addr == mac_address).then_some(handle))
    }

    fn ranging(&mut self, device_handle: Handle, session_id: u32) {
        tracing::debug!("[{device_handle}] Ranging event");
        tracing::debug!("  session_id={session_id}");

        let Some(device) = self.get_device(device_handle) else {
            return;
        };
        let Some(session) = device.session(session_id) else {
            return;
        };
        if session.session_state() != SessionState::SessionStateActive {
            return;
        }

        if session.app_config.suspend_ranging_rounds() == uci::SuspendRangingRounds::Enable {
            return;
        }

        let mut measurements = Vec::new();

        for mac_address in session.get_dst_mac_address() {
            let peer_handle = if let Some(anchor_handle) = self.anchor_handle(mac_address) {
                Some(anchor_handle)
            } else if let Some(peer_entry) = self.devices.values().find(|peer_entry| {
                peer_entry.device.handle != device_handle
                    && peer_entry.device.can_start_ranging(session, session_id)
                    && peer_entry
                        .device
                        .session(session_id)
                        .and_then(|s| s.app_config.device_mac_address())
                        == Some(mac_address)
            }) {
                if peer_entry
                    .device
                    .session(session_id)
                    .is_some_and(|peer_session| {
                        peer_session.app_config.suspend_ranging_rounds()
                            == uci::SuspendRangingRounds::Enable
                    })
                {
                    if session.app_config.device_type() == Some(uci::DeviceType::Controlee) {
                        return;
                    }
                    None
                } else {
                    Some(peer_entry.device.handle)
                }
            } else {
                None
            };
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

        let peer_handle = if let Some(data_transfer) = &session.data_transfer
            && session.can_start_data_transfer()
        {
            let dst_mac_address = data_transfer.dst_mac_address;
            self.devices
                .values()
                .find(|peer_entry| {
                    peer_entry.device.handle != device_handle
                        && peer_entry
                            .device
                            .can_receive_data_transfer(session_id, dst_mac_address)
                })
                .map(|peer_entry| peer_entry.device.handle)
        } else {
            None
        };

        let Some(entry) = self.devices.get_mut(&device_handle) else {
            return;
        };
        let device = &mut entry.device;
        let Some(session) = device.session_mut(session_id) else {
            return;
        };

        let mut peer_data_packet: Option<(Handle, UciPacket)> = None;
        let mut local_packets: Vec<UciPacket> = Vec::new();

        if let Some(data_transfer) = session.data_transfer.as_mut() {
            data_transfer.tx_count = data_transfer.tx_count.saturating_add(1);
            let tx_count = data_transfer.tx_count;
            let sequence_number = data_transfer.sequence_number;
            let payload = data_transfer.payload.clone();
            let repetition_count = session.app_config.data_repetition_count();
            let is_status_ntf_enabled = session.is_session_data_transfer_status_ntf_enabled();

            let (status, is_complete) = if let Some((peer_handle, source_address)) =
                peer_handle.zip(session.app_config.device_mac_address())
            {
                let packet = DataMessageRcv {
                    application_data: payload.into(),
                    data_sequence_number: sequence_number,
                    pbf: PacketBoundaryFlag::Complete,
                    session_handle: session_id,
                    source_address: source_address.into(),
                    status: uci::DataRcvStatusCode::UciStatusSuccess,
                }
                .encode_to_vec()
                .expect("failed to encode DataMessageRcv");

                peer_data_packet = Some((peer_handle, packet));

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
                local_packets.push(
                    SessionDataCreditNtf {
                        credit_availability: CreditAvailability::CreditAvailable,
                        session_token: session_id,
                    }
                    .encode_to_vec()
                    .expect("failed to encode SessionDataCreditNtf"),
                );
            }

            if is_status_ntf_enabled || is_complete {
                local_packets.push(
                    SessionDataTransferStatusNtf {
                        session_token: session_id,
                        uci_sequence_number: sequence_number,
                        status,
                        tx_count,
                    }
                    .encode_to_vec()
                    .expect("failed to encode SessionDataTransferStatusNtf"),
                );
            }
        }

        let effects = if session.session_type() != SessionType::FiraDataTransferSession {
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
                        .expect("failed to encode SessionInfoNtf"),
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
                        .expect("failed to encode SessionInfoNtf"),
                    ),
                    mode => {
                        tracing::warn!("Unsupported MAC address mode {mode:?}");
                        None
                    }
                };

                if let Some(packet) = ntf_packet {
                    local_packets.push(packet);
                    session.sequence_number += 1;
                }
            }
            session.ranging_count += 1;
            let max_measurements = u32::from(session.app_config.max_number_of_measurements());
            let max_reached = max_measurements > 0 && session.ranging_count >= max_measurements;
            if max_reached {
                device.stop_active_session(session_id, ReasonCode::MaxNumberOfMeasurementsReached)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        if let Some((peer_handle, packet)) = peer_data_packet {
            self.send_packet(peer_handle, packet);
        }
        for packet in local_packets {
            self.send_packet(device_handle, packet);
        }
        self.apply_device_effects(device_handle, effects);
    }

    fn uci_packet(&mut self, device_handle: Handle, packet: Vec<u8>) {
        let effects = match self.get_device_mut(device_handle) {
            Some(device) => device.receive_packet(packet),
            None => {
                tracing::error!("Device {device_handle} not found");
                return;
            }
        };
        self.apply_device_effects(device_handle, effects);
    }
    #[must_use]
    fn stop_controlee_ranging(
        &mut self,
        mac_address: &MacAddress,
        session_id: u32,
    ) -> Vec<(Handle, DeviceEffect)> {
        self.devices
            .iter_mut()
            .filter(|(_, entry)| {
                entry
                    .device
                    .session(session_id)
                    .and_then(|session| session.app_config.device_mac_address())
                    == Some(*mac_address)
            })
            .flat_map(|(&handle, entry)| {
                entry
                    .device
                    .stop_active_session(session_id, ReasonCode::SessionStoppedDueToInbandSignal)
                    .into_iter()
                    .map(move |effect| (handle, effect))
            })
            .collect()
    }

    /// Creates a virtual anchor with `mac_address` and returns its allocated [`Handle`].
    pub fn create_anchor(&mut self, mac_address: MacAddress) -> Result<Handle, AlreadyExists> {
        tracing::debug!("[_] Create anchor");
        tracing::debug!("  mac_address: {mac_address}");

        if let Some(handle) = self.anchor_handle(mac_address) {
            Err(AlreadyExists(handle))
        } else {
            let handle = self.next_handle;
            self.next_handle += 1;

            self.anchors.insert(handle, mac_address);

            Ok(handle)
        }
    }

    /// Removes the anchor with `handle`, returning `true` if it existed.
    pub fn destroy_anchor(&mut self, handle: Handle) -> bool {
        tracing::debug!("[{handle}] Destroy anchor");

        self.anchors.remove(&handle).is_some()
    }
}
