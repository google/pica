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

#![cfg_attr(not(feature = "ffi"), forbid(unsafe_code))]
#![cfg_attr(
    test,
    allow(
        clippy::indexing_slicing,
        clippy::panic,
        clippy::panic_in_result_fn,
        clippy::unwrap_used,
    )
)]

use std::collections::{HashMap, VecDeque};
use std::time::Instant;

use pdl_runtime::Packet;
use thiserror::Error;

#[cfg(feature = "ffi")]
pub mod ffi;
pub mod packets;
pub mod pcapng;

use packets::uci::*;

mod app_config;
use app_config::AppConfig;

mod data_transfer;

mod device;
use device::{Device, DeviceEffect};

mod dl_tdoa;

mod mac_address;
pub use mac_address::MacAddress;

mod ranging;

mod session;

/// Raw serialized UCI packet bytes.
pub type UciPacket = Vec<u8>;

/// Handle allocated for created devices or anchors.
/// The handle is unique across the lifetime of the Pica context
/// and callers may assume that one handle is never reused.
pub type Handle = usize;

/// Ranging measurement produced by a ranging estimator.
#[cfg_attr(feature = "ffi", repr(C))]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
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

    /// Feeds an incoming segmented UCI packet from `handle` into `Pica` at timestamp `now`.
    pub fn handle_packet(&mut self, handle: Handle, packet: &[u8], now: Instant) {
        if let Some(complete_packet) = self
            .devices
            .get_mut(&handle)
            .and_then(|entry| entry.reassemble(packet))
        {
            self.uci_packet(handle, complete_packet, now);
        }
    }

    /// Returns the earliest scheduled ranging deadline across all connected devices,
    /// or `None` if no ranging timers are active.
    pub fn next_deadline(&self) -> Option<Instant> {
        self.devices
            .values()
            .filter_map(|entry| entry.device.next_ranging_deadline())
            .min()
    }

    /// Advances ranging sessions whose deadline is at or before `now`.
    pub fn tick(&mut self, now: Instant) {
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

    fn uci_packet(&mut self, device_handle: Handle, packet: Vec<u8>, now: Instant) {
        let effects = match self.get_device_mut(device_handle) {
            Some(device) => device.receive_packet(packet, now),
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
