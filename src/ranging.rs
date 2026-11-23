// Copyright 2026 Google LLC
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

use crate::app_config::AppConfig;
use crate::device::{Device, DeviceEffect};
use crate::packets::uci::{self, *};
use crate::session::Session;
use crate::{Handle, MacAddress, Pica, RangingMeasurement, UciPacket};

/// IEEE 802.15.4z fundamental UWB chipping rate (`499.2 MHz`).
pub const CHIPS_PER_MS: u64 = 499_200;
/// UWB chips per Ranging Scheduling Time Unit (`1 RSTU = 416 chips ≈ 0.833 µs`).
pub const CHIPS_PER_RSTU: u64 = 416;
pub const RSTU_PER_MS: u64 = CHIPS_PER_MS / CHIPS_PER_RSTU;

const fn degrees_to_q9_7(deg: i16) -> i16 {
    deg.saturating_mul(128)
}

pub fn encode_aoa(measurement: RangingMeasurement, req: uci::AoaResultReq) -> (u16, u8, u16, u8) {
    let azimuth = degrees_to_q9_7(measurement.azimuth) as u16;
    let elevation = degrees_to_q9_7(i16::from(measurement.elevation)) as u16;
    match req {
        uci::AoaResultReq::AoaDisabled => (0, 0, 0, 0),
        uci::AoaResultReq::AoaEnabled => (azimuth, 100, elevation, 100),
        uci::AoaResultReq::AoaEnabledAzimuthOnly => (azimuth, 100, 0, 0),
        uci::AoaResultReq::AoaEnabledElevationOnly => (0, 0, elevation, 100),
    }
}

// -50 dBm in UCI's unsigned Q7.1 (-dBm * 2) RSSI encoding.
const DEFAULT_RSSI: u8 = 100;

pub fn reported_rssi(app_config: &AppConfig) -> u8 {
    match app_config.rssi_reporting() {
        uci::RssiReporting::Enable => DEFAULT_RSSI,
        uci::RssiReporting::Disable => 0,
    }
}

#[derive(Debug)]
enum TwoWayRangingMeasurement {
    Short(ShortAddressTwoWayRangingMeasurement),
    Extended(ExtendedAddressTwoWayRangingMeasurement),
}

impl TwoWayRangingMeasurement {
    fn new(
        app_config: &AppConfig,
        mac_address: &MacAddress,
        local: RangingMeasurement,
        remote: RangingMeasurement,
    ) -> Self {
        let (aoa_azimuth, aoa_azimuth_fom, aoa_elevation, aoa_elevation_fom) =
            encode_aoa(local, app_config.aoa_result_req());
        let (
            aoa_destination_azimuth,
            aoa_destination_azimuth_fom,
            aoa_destination_elevation,
            aoa_destination_elevation_fom,
        ) = encode_aoa(remote, uci::AoaResultReq::AoaEnabled);
        let rssi = reported_rssi(app_config);
        match mac_address {
            MacAddress::Short(address) => {
                Self::Short(ShortAddressTwoWayRangingMeasurement {
                    mac_address: u16::from_le_bytes(*address),
                    status: uci::Status::Ok,
                    nlos: 0, // in Line Of Sight
                    distance: local.range,
                    aoa_azimuth,
                    aoa_azimuth_fom,
                    aoa_elevation,
                    aoa_elevation_fom,
                    aoa_destination_azimuth,
                    aoa_destination_azimuth_fom,
                    aoa_destination_elevation,
                    aoa_destination_elevation_fom,
                    slot_index: 0,
                    rssi,
                })
            }
            MacAddress::Extended(address) => {
                Self::Extended(ExtendedAddressTwoWayRangingMeasurement {
                    mac_address: u64::from_le_bytes(*address),
                    status: uci::Status::Ok,
                    nlos: 0,
                    distance: local.range,
                    aoa_azimuth,
                    aoa_azimuth_fom,
                    aoa_elevation,
                    aoa_elevation_fom,
                    aoa_destination_azimuth,
                    aoa_destination_azimuth_fom,
                    aoa_destination_elevation,
                    aoa_destination_elevation_fom,
                    slot_index: 0,
                    rssi,
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

impl Device {
    pub fn can_start_ranging(&self, peer_session: &Session, session_id: u32) -> bool {
        match self.session(session_id) {
            Some(session) => {
                session.session_state() == SessionState::SessionStateActive
                    && session
                        .app_config
                        .is_compatible_for_ranging(&peer_session.app_config)
            }
            None => false,
        }
    }
}

fn finish_twr_round(
    device: &mut Device,
    session_id: u32,
    measurements: Vec<TwoWayRangingMeasurement>,
    local_packets: &mut Vec<UciPacket>,
) -> Vec<DeviceEffect> {
    let Some(session) = device.session_mut(session_id) else {
        return Vec::new();
    };
    if session.session_type() == SessionType::FiraDataTransferSession {
        return Vec::new();
    }

    let (send_ntf, measurements) = match session.app_config.effective_session_info_ntf_config() {
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
                    rcr_indicator: 0,
                    current_ranging_interval: session.ranging_interval_ms(),
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
                    rcr_indicator: 0,
                    current_ranging_interval: session.ranging_interval_ms(),
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
    if session.has_reached_max_measurements() {
        device.stop_active_session(session_id, ReasonCode::MaxNumberOfMeasurementsReached)
    } else {
        Vec::new()
    }
}

impl Pica {
    fn collect_twr_measurements(
        &self,
        device_handle: Handle,
        session_id: u32,
        session: &Session,
    ) -> Option<Vec<TwoWayRangingMeasurement>> {
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
                        return None;
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
                        .estimate(&device_handle, &peer_handle)?;
                    let remote = self
                        .ranging_estimator
                        .estimate(&peer_handle, &device_handle)?;
                    Some(TwoWayRangingMeasurement::new(
                        &session.app_config,
                        &mac_address,
                        local,
                        remote,
                    ))
                })
                .unwrap_or_else(|| TwoWayRangingMeasurement::timeout(&mac_address));
            measurements.push(measurement);
        }

        Some(measurements)
    }

    pub fn ranging(&mut self, device_handle: Handle, session_id: u32) {
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

        if session.app_config.is_dl_tdoa() {
            self.dl_tdoa_ranging(device_handle, session_id);
            return;
        }

        let Some(measurements) = self.collect_twr_measurements(device_handle, session_id, session)
        else {
            return;
        };
        let data_peer_handle = self.resolve_data_transfer_peer(device_handle, session_id, session);

        let Some(entry) = self.devices.get_mut(&device_handle) else {
            return;
        };
        let device = &mut entry.device;
        let Some(session) = device.session_mut(session_id) else {
            return;
        };

        let (peer_data_packet, mut local_packets) =
            session.step_data_transfer(session_id, data_peer_handle);
        let effects = finish_twr_round(device, session_id, measurements, &mut local_packets);

        if let Some((peer_handle, packet)) = peer_data_packet {
            self.send_packet(peer_handle, packet);
        }
        for packet in local_packets {
            self.send_packet(device_handle, packet);
        }
        self.apply_device_effects(device_handle, effects);
    }
}
