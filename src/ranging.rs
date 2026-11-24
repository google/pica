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

use std::collections::BTreeMap;

use pdl_runtime::Packet;

use crate::app_config::AppConfig;
use crate::device::{Device, DeviceEffect};
use crate::packets::uci::{self, *};
use crate::session::{PeerKey, Session};
use crate::{DeviceEntry, Handle, MacAddress, Pica, RangingMeasurement, UciPacket};

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
        slot_index: u8,
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
                    slot_index,
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
                    slot_index,
                    rssi,
                })
            }
        }
    }

    fn timeout(mac_address: &MacAddress, slot_index: u8) -> Self {
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
                slot_index,
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
                    slot_index,
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
    measurements: Vec<(PeerKey, TwoWayRangingMeasurement)>,
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
        uci::SessionInfoNtfConfig::Enable => {
            (true, measurements.into_iter().map(|(_, m)| m).collect())
        }
        uci::SessionInfoNtfConfig::EnableProximityTrigger
        | uci::SessionInfoNtfConfig::EnableAoaTrigger
        | uci::SessionInfoNtfConfig::EnableProximityAoaTrigger => {
            let filtered: Vec<_> = measurements
                .into_iter()
                .filter_map(|(_, m)| m.is_within_bounds(&session.app_config).then_some(m))
                .collect();
            (!filtered.is_empty(), filtered)
        }
        uci::SessionInfoNtfConfig::EnableProximityEdgeTrigger
        | uci::SessionInfoNtfConfig::EnableAoaEdgeTrigger
        | uci::SessionInfoNtfConfig::EnableProximityAoaEdgeTrigger => {
            session
                .in_range_peers
                .retain(|key| measurements.iter().any(|(k, _)| k == key));
            let emitted: Vec<_> = measurements
                .into_iter()
                .filter_map(|(key, m)| {
                    let changed = if m.is_within_bounds(&session.app_config) {
                        session.in_range_peers.insert(key)
                    } else {
                        session.in_range_peers.remove(&key)
                    };
                    changed.then_some(m)
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
    session.advance_ccc_last_sts_index_used();
    if session.has_reached_max_measurements() {
        device.stop_active_session(session_id, ReasonCode::MaxNumberOfMeasurementsReached)
    } else {
        Vec::new()
    }
}

impl Pica {
    fn zero_mac(mode: uci::MacAddressMode) -> Option<MacAddress> {
        match mode {
            uci::MacAddressMode::Mode0 => Some(MacAddress::Short([0; 2])),
            uci::MacAddressMode::Mode2 => Some(MacAddress::Extended([0; 8])),
            uci::MacAddressMode::Mode1 => None,
        }
    }

    fn is_peer_suspended(device: &Device, session_id: u32) -> bool {
        device.session(session_id).is_some_and(|peer_session| {
            peer_session.app_config.suspend_ranging_rounds() == uci::SuspendRangingRounds::Enable
        })
    }

    fn two_way_measurement(
        &self,
        app_config: &AppConfig,
        device_handle: Handle,
        peer_handle: Option<Handle>,
        mac_address: &MacAddress,
        slot_index: u8,
    ) -> TwoWayRangingMeasurement {
        peer_handle
            .and_then(|peer_handle| {
                let local = self
                    .ranging_estimator
                    .estimate(&device_handle, &peer_handle)?;
                let remote = self
                    .ranging_estimator
                    .estimate(&peer_handle, &device_handle)?;
                Some(TwoWayRangingMeasurement::new(
                    app_config,
                    mac_address,
                    local,
                    remote,
                    slot_index,
                ))
            })
            .unwrap_or_else(|| TwoWayRangingMeasurement::timeout(mac_address, slot_index))
    }

    fn ccc_selected_initiator(
        &self,
        responder_handle: Handle,
        responder_session: &Session,
        session_id: u32,
    ) -> (Option<&DeviceEntry>, bool) {
        let mut initiator_entry: Option<&DeviceEntry> = None;
        let mut any_suspended = false;
        for peer_entry in self.devices.values().filter(|peer_entry| {
            peer_entry.device.handle != responder_handle
                && peer_entry
                    .device
                    .can_start_ranging(responder_session, session_id)
        }) {
            if Self::is_peer_suspended(&peer_entry.device, session_id) {
                any_suspended = true;
            } else if initiator_entry.is_none_or(|e| peer_entry.device.handle < e.device.handle) {
                initiator_entry = Some(peer_entry);
            }
        }
        (initiator_entry, any_suspended)
    }

    fn ccc_active_responder_slots(
        &self,
        initiator_handle: Handle,
        initiator_session: &Session,
        session_id: u32,
    ) -> Vec<(u8, Handle)> {
        let max_slot = if initiator_session.app_config.multi_node_mode()
            == Some(uci::MultiNodeMode::OneToMany)
        {
            initiator_session.app_config.number_of_controlees()
        } else {
            1
        };
        let slots_per_rr = initiator_session.app_config.slots_per_rr();
        let mut by_slot: BTreeMap<u8, Vec<Handle>> = BTreeMap::new();
        for (slot, handle) in self
            .devices
            .values()
            .filter(|peer| {
                peer.device.handle != initiator_handle
                    && peer.device.can_start_ranging(initiator_session, session_id)
                    && !Self::is_peer_suspended(&peer.device, session_id)
            })
            .filter_map(|peer| {
                let peer_session = peer.device.session(session_id)?;
                let (selected, _) =
                    self.ccc_selected_initiator(peer.device.handle, peer_session, session_id);
                if selected.map(|entry| entry.device.handle) != Some(initiator_handle) {
                    return None;
                }
                let slot = peer_session.app_config.responder_slot_index();
                (slot <= max_slot && slot < slots_per_rr).then_some((slot, peer.device.handle))
            })
        {
            by_slot.entry(slot).or_default().push(handle);
        }
        by_slot
            .into_iter()
            .filter_map(|(slot, handles)| match handles.as_slice() {
                &[handle] => Some((slot, handle)),
                _ => None,
            })
            .collect()
    }

    fn collect_ccc_twr_measurements(
        &self,
        device_handle: Handle,
        session_id: u32,
        session: &Session,
    ) -> Option<Vec<(PeerKey, TwoWayRangingMeasurement)>> {
        let Some(mac_address) = Self::zero_mac(session.app_config.mac_address_mode()) else {
            return Some(Vec::new());
        };
        match session.app_config.device_role() {
            Some(uci::DeviceRole::Initiator) => {
                let measurements = self
                    .ccc_active_responder_slots(device_handle, session, session_id)
                    .into_iter()
                    .map(|(slot, peer_handle)| {
                        (
                            PeerKey::Handle(peer_handle),
                            self.two_way_measurement(
                                &session.app_config,
                                device_handle,
                                Some(peer_handle),
                                &mac_address,
                                slot,
                            ),
                        )
                    })
                    .collect();
                Some(measurements)
            }
            Some(uci::DeviceRole::Responder) => {
                let (selected, any_suspended) =
                    self.ccc_selected_initiator(device_handle, session, session_id);
                let Some(initiator_entry) = selected else {
                    return (!any_suspended).then(Vec::new);
                };
                let initiator_handle = initiator_entry.device.handle;
                let has_active_slot =
                    initiator_entry
                        .device
                        .session(session_id)
                        .is_some_and(|initiator_session| {
                            self.ccc_active_responder_slots(
                                initiator_handle,
                                initiator_session,
                                session_id,
                            )
                            .iter()
                            .any(|&(_, h)| h == device_handle)
                        });
                let measurements = has_active_slot
                    .then(|| {
                        (
                            PeerKey::Handle(initiator_handle),
                            self.two_way_measurement(
                                &session.app_config,
                                device_handle,
                                Some(initiator_handle),
                                &mac_address,
                                0,
                            ),
                        )
                    })
                    .into_iter()
                    .collect();
                Some(measurements)
            }
            _ => Some(Vec::new()),
        }
    }

    fn collect_twr_measurements(
        &self,
        device_handle: Handle,
        session_id: u32,
        session: &Session,
    ) -> Option<Vec<(PeerKey, TwoWayRangingMeasurement)>> {
        if session.app_config.is_ccc_or_aliro() {
            return self.collect_ccc_twr_measurements(device_handle, session_id, session);
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
                if Self::is_peer_suspended(&peer_entry.device, session_id) {
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
            measurements.push((
                PeerKey::Mac(mac_address),
                self.two_way_measurement(
                    &session.app_config,
                    device_handle,
                    peer_handle,
                    &mac_address,
                    0,
                ),
            ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        FiraSessionConfig, FixedEstimator, PairwiseEstimator, add_reset_device, send_cmd,
        start_fira_session,
    };

    const TWR_SESSION_ID: u32 = 17;
    const AOA_SESSION_ID: u32 = 23;
    const VIRTUAL_ANCHOR_SESSION_ID: u32 = 29;
    const EDGE_TRIGGER_SESSION_ID: u32 = 31;
    const RANGING_DURATION_MS: u32 = 100;
    const CONTROLLER_SHORT_MAC: [u8; 2] = [0x1A, 0x2B];
    const CONTROLEE_SHORT_MAC: [u8; 2] = [0x3C, 0x4D];
    const CONTROLEE_SHORT_ADDR: u16 = 0x4D3C;
    const CONTROLLER_EXT_MAC: [u8; 8] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    const CONTROLEE_EXT_MAC: [u8; 8] = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x12, 0x34];
    const CONTROLEE_EXT_ADDR: u64 = 0x3412_FFEE_DDCC_BBAA;
    const DEFAULT_RSSI_Q7_1: u8 = 100;
    const AOA_FOM_FULL: u8 = 100;
    const STANDARD_MEASUREMENT: RangingMeasurement = RangingMeasurement {
        range: 250,
        azimuth: 10,
        elevation: -5,
    };
    const EXTENDED_MEASUREMENT: RangingMeasurement = RangingMeasurement {
        range: 320,
        azimuth: 25,
        elevation: -12,
    };
    const VIRTUAL_ANCHOR_MEASUREMENT: RangingMeasurement = RangingMeasurement {
        range: 185,
        azimuth: -18,
        elevation: 8,
    };
    const AOA_MEASUREMENT: RangingMeasurement = RangingMeasurement {
        range: 50,
        azimuth: 5,
        elevation: 10,
    };
    const AZIMUTH_5_DEG_Q9_7: u16 = 0x0280;
    const ELEVATION_10_DEG_Q9_7: u16 = 0x0500;
    const AZIMUTH_25_DEG_Q9_7: u16 = 0x0C80;
    const ELEVATION_NEG_12_DEG_Q9_7: u16 = 0xFA00;
    // -10..=10 deg azimuth, 20..=45 deg elevation in Q9.7 little-endian.
    const AOA_BOUNDS_AZ_NEG10_TO_10_EL_20_TO_45_LE: [u8; 8] =
        [0x00, 0xFB, 0x00, 0x05, 0x00, 0x0A, 0x80, 0x16];
    const NEAR_PROXIMITY_100_CM_LE: [u8; 2] = [0x64, 0x00];
    const FAR_PROXIMITY_200_CM_LE: [u8; 2] = [0xC8, 0x00];
    const IN_RANGE_CM: u16 = 150;
    const OUT_OF_RANGE_CM: u16 = 250;

    fn start_twr_pair(
        pica: &mut Pica,
        session_id: u32,
        mac_mode: MacAddressMode,
        local_mac: &[u8],
        dst_mac: &[u8],
        controller_extra: Vec<AppConfigTlv>,
    ) -> (Handle, Handle) {
        let controller = add_reset_device(pica);
        let controlee = add_reset_device(pica);

        start_fira_session(
            pica,
            controller,
            &FiraSessionConfig {
                session_id,
                session_type: SessionType::FiraRangingSession,
                role: DeviceRole::Initiator,
                dev_type: DeviceType::Controller,
                mac_mode,
                multi_node_mode: MultiNodeMode::OneToOne,
                local_mac,
                dst_mac: Some(dst_mac),
                ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
                ranging_duration_ms: RANGING_DURATION_MS,
                extra_tlvs: controller_extra,
            },
        );
        start_fira_session(
            pica,
            controlee,
            &FiraSessionConfig {
                session_id,
                session_type: SessionType::FiraRangingSession,
                role: DeviceRole::Responder,
                dev_type: DeviceType::Controlee,
                mac_mode,
                multi_node_mode: MultiNodeMode::OneToOne,
                local_mac: dst_mac,
                dst_mac: Some(local_mac),
                ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
                ranging_duration_ms: RANGING_DURATION_MS,
                extra_tlvs: vec![],
            },
        );
        (controller, controlee)
    }

    #[test]
    fn twr_mode0_reports_distance_and_default_zero_rssi() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let (controller, _controlee) = start_twr_pair(
            &mut pica,
            TWR_SESSION_ID,
            MacAddressMode::Mode0,
            &CONTROLLER_SHORT_MAC,
            &CONTROLEE_SHORT_MAC,
            vec![],
        );

        pica.ranging(controller, TWR_SESSION_ID);
        let pkt = pica.drain_packets().next().unwrap().1;
        let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
        assert_eq!(ntf.two_way_ranging_measurements[0].status, uci::Status::Ok);
        assert_eq!(
            ntf.two_way_ranging_measurements[0].distance,
            STANDARD_MEASUREMENT.range
        );
        assert_eq!(ntf.two_way_ranging_measurements[0].rssi, 0);
    }

    #[test]
    fn twr_suspend_suppresses_and_resumes_both_peers() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let (controller, controlee) = start_twr_pair(
            &mut pica,
            TWR_SESSION_ID,
            MacAddressMode::Mode0,
            &CONTROLLER_SHORT_MAC,
            &CONTROLEE_SHORT_MAC,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::SuspendRangingRounds,
                v: vec![SuspendRangingRounds::Enable as u8],
            }],
        );

        pica.ranging(controller, TWR_SESSION_ID);
        pica.ranging(controlee, TWR_SESSION_ID);
        assert!(pica.drain_packets().next().is_none());

        let resume = SessionSetAppConfigCmd {
            session_token: TWR_SESSION_ID,
            tlvs: vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::SuspendRangingRounds,
                v: vec![SuspendRangingRounds::Disable as u8],
            }],
        }
        .encode_to_vec()
        .unwrap();
        pica.handle_packet(controller, &resume, std::time::Instant::now());
        let _ = pica.drain_packets().count();

        pica.ranging(controller, TWR_SESSION_ID);
        pica.ranging(controlee, TWR_SESSION_ID);
        let recipients: Vec<_> = pica
            .drain_packets()
            .filter_map(|(h, bytes)| {
                ShortMacTwoWaySessionInfoNtf::decode_full(&bytes)
                    .ok()
                    .map(|_| h)
            })
            .collect();
        assert_eq!(recipients, vec![controller, controlee]);
    }

    #[test]
    fn twr_rssi_reporting_enable_reports_default_rssi() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let (controller, _controlee) = start_twr_pair(
            &mut pica,
            TWR_SESSION_ID,
            MacAddressMode::Mode0,
            &CONTROLLER_SHORT_MAC,
            &CONTROLEE_SHORT_MAC,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::RssiReporting,
                v: vec![RssiReporting::Enable as u8],
            }],
        );

        pica.ranging(controller, TWR_SESSION_ID);
        let pkt = pica.drain_packets().next().unwrap().1;
        let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
        assert_eq!(ntf.two_way_ranging_measurements[0].rssi, DEFAULT_RSSI_Q7_1);
    }

    #[test]
    fn twr_removed_peer_reports_rx_timeout() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let (controller, controlee) = start_twr_pair(
            &mut pica,
            TWR_SESSION_ID,
            MacAddressMode::Mode0,
            &CONTROLLER_SHORT_MAC,
            &CONTROLEE_SHORT_MAC,
            vec![],
        );

        assert!(pica.remove_device(controlee));
        pica.ranging(controller, TWR_SESSION_ID);
        let pkt = pica.drain_packets().next().unwrap().1;
        let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
        assert_eq!(
            ntf.two_way_ranging_measurements[0].status,
            uci::Status::RangingRxTimeout
        );
        assert_eq!(ntf.two_way_ranging_measurements[0].rssi, 0);
    }

    #[test]
    fn twr_mode2_reports_extended_mac_address() {
        let mut pica = Pica::new(Box::new(FixedEstimator(EXTENDED_MEASUREMENT)));
        let (controller, _controlee) = start_twr_pair(
            &mut pica,
            TWR_SESSION_ID,
            MacAddressMode::Mode2,
            &CONTROLLER_EXT_MAC,
            &CONTROLEE_EXT_MAC,
            vec![],
        );

        pica.ranging(controller, TWR_SESSION_ID);
        let pkt = pica.drain_packets().next().unwrap().1;
        let ntf = ExtendedMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
        let m = &ntf.two_way_ranging_measurements[0];
        assert_eq!(m.status, uci::Status::Ok);
        assert_eq!(m.mac_address, CONTROLEE_EXT_ADDR);
        assert_eq!(m.distance, EXTENDED_MEASUREMENT.range);
        assert_eq!(m.aoa_azimuth, AZIMUTH_25_DEG_Q9_7);
        assert_eq!(m.aoa_elevation, ELEVATION_NEG_12_DEG_Q9_7);
    }

    #[test]
    fn twr_virtual_anchor_reports_ranging_without_device_session() {
        let mut pica = Pica::new(Box::new(FixedEstimator(VIRTUAL_ANCHOR_MEASUREMENT)));
        pica.create_anchor(MacAddress::Short(CONTROLEE_SHORT_MAC))
            .unwrap();
        let controller = add_reset_device(&mut pica);
        start_fira_session(
            &mut pica,
            controller,
            &FiraSessionConfig {
                session_id: VIRTUAL_ANCHOR_SESSION_ID,
                session_type: SessionType::FiraRangingSession,
                role: DeviceRole::Initiator,
                dev_type: DeviceType::Controller,
                mac_mode: MacAddressMode::Mode0,
                multi_node_mode: MultiNodeMode::OneToOne,
                local_mac: &CONTROLLER_SHORT_MAC,
                dst_mac: Some(&CONTROLEE_SHORT_MAC),
                ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
                ranging_duration_ms: RANGING_DURATION_MS,
                extra_tlvs: vec![],
            },
        );

        pica.ranging(controller, VIRTUAL_ANCHOR_SESSION_ID);
        let pkt = pica.drain_packets().next().unwrap().1;
        let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
        let m = &ntf.two_way_ranging_measurements[0];
        assert_eq!(m.status, uci::Status::Ok);
        assert_eq!(m.mac_address, CONTROLEE_SHORT_ADDR);
        assert_eq!(m.distance, VIRTUAL_ANCHOR_MEASUREMENT.range);
    }

    #[test]
    fn twr_aoa_result_req_populates_requested_fom_fields() {
        for (req, exp_az, exp_az_fom, exp_el, exp_el_fom) in [
            (AoaResultReq::AoaDisabled, 0, 0, 0, 0),
            (
                AoaResultReq::AoaEnabledAzimuthOnly,
                AZIMUTH_5_DEG_Q9_7,
                AOA_FOM_FULL,
                0,
                0,
            ),
            (
                AoaResultReq::AoaEnabledElevationOnly,
                0,
                0,
                ELEVATION_10_DEG_Q9_7,
                AOA_FOM_FULL,
            ),
            (
                AoaResultReq::AoaEnabled,
                AZIMUTH_5_DEG_Q9_7,
                AOA_FOM_FULL,
                ELEVATION_10_DEG_Q9_7,
                AOA_FOM_FULL,
            ),
        ] {
            let mut pica = Pica::new(Box::new(FixedEstimator(AOA_MEASUREMENT)));
            let (controller, _controlee) = start_twr_pair(
                &mut pica,
                AOA_SESSION_ID,
                MacAddressMode::Mode0,
                &CONTROLLER_SHORT_MAC,
                &CONTROLEE_SHORT_MAC,
                vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::AoaResultReq,
                    v: vec![req as u8],
                }],
            );

            pica.ranging(controller, AOA_SESSION_ID);
            let pkt = pica.drain_packets().next().unwrap().1;
            let m = &ShortMacTwoWaySessionInfoNtf::decode_full(&pkt)
                .unwrap()
                .two_way_ranging_measurements[0];
            assert_eq!(
                (
                    m.aoa_azimuth,
                    m.aoa_azimuth_fom,
                    m.aoa_elevation,
                    m.aoa_elevation_fom
                ),
                (exp_az, exp_az_fom, exp_el, exp_el_fom)
            );
            assert_eq!(
                (
                    m.aoa_destination_azimuth_fom,
                    m.aoa_destination_elevation_fom
                ),
                (AOA_FOM_FULL, AOA_FOM_FULL)
            );
        }
    }

    #[test]
    fn twr_aoa_bounds_trigger_emits_in_bounds_and_suppresses_out_of_bounds() {
        for (azimuth, elevation, expect_ntf) in [(5, 30, true), (30, 30, false), (5, 10, false)] {
            let mut pica = Pica::new(Box::new(FixedEstimator(RangingMeasurement {
                range: AOA_MEASUREMENT.range,
                azimuth,
                elevation,
            })));

            let (controller, _controlee) = start_twr_pair(
                &mut pica,
                AOA_SESSION_ID,
                MacAddressMode::Mode0,
                &CONTROLLER_SHORT_MAC,
                &CONTROLEE_SHORT_MAC,
                vec![
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::AoaResultReq,
                        v: vec![AoaResultReq::AoaEnabled as u8],
                    },
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                        v: vec![SessionInfoNtfConfig::EnableAoaTrigger as u8],
                    },
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::AoaBoundConfig,
                        v: AOA_BOUNDS_AZ_NEG10_TO_10_EL_20_TO_45_LE.to_vec(),
                    },
                ],
            );

            pica.ranging(controller, AOA_SESSION_ID);
            let emitted = pica.drain_packets().next();
            assert_eq!(
                emitted.is_some(),
                expect_ntf,
                "unexpected notification presence for azimuth={azimuth}, elevation={elevation}"
            );
        }
    }

    struct DynamicEstimator(
        std::sync::Arc<std::sync::atomic::AtomicU16>,
        std::sync::Arc<std::sync::atomic::AtomicI16>,
        i8,
    );

    impl crate::RangingEstimator for DynamicEstimator {
        fn estimate(&self, _left: &Handle, _right: &Handle) -> Option<RangingMeasurement> {
            use std::sync::atomic::Ordering;
            Some(RangingMeasurement {
                range: self.0.load(Ordering::Relaxed),
                azimuth: self.1.load(Ordering::Relaxed),
                elevation: self.2,
            })
        }
    }

    #[test]
    fn twr_proximity_edge_trigger_emits_only_on_entry_and_exit() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicI16, AtomicU16, Ordering};

        let range = Arc::new(AtomicU16::new(IN_RANGE_CM));
        let azimuth = Arc::new(AtomicI16::new(AOA_MEASUREMENT.azimuth));
        let mut pica = Pica::new(Box::new(DynamicEstimator(
            Arc::clone(&range),
            azimuth,
            AOA_MEASUREMENT.elevation,
        )));
        let (controller, _controlee) = start_twr_pair(
            &mut pica,
            EDGE_TRIGGER_SESSION_ID,
            MacAddressMode::Mode0,
            &CONTROLLER_SHORT_MAC,
            &CONTROLEE_SHORT_MAC,
            vec![
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                    v: vec![SessionInfoNtfConfig::EnableProximityEdgeTrigger as u8],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::NearProximityConfig,
                    v: NEAR_PROXIMITY_100_CM_LE.to_vec(),
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::FarProximityConfig,
                    v: FAR_PROXIMITY_200_CM_LE.to_vec(),
                },
            ],
        );

        // Enters 100..=200 window -> emits.
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_some());

        // Remains in window -> suppressed.
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_none());

        // Exits window -> emits.
        range.store(OUT_OF_RANGE_CM, Ordering::Relaxed);
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_some());

        // Remains outside window -> suppressed.
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_none());
    }

    #[test]
    fn twr_proximity_aoa_edge_trigger_tracks_combined_bounds() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicI16, AtomicU16, Ordering};

        let range = Arc::new(AtomicU16::new(OUT_OF_RANGE_CM));
        let azimuth = Arc::new(AtomicI16::new(AOA_MEASUREMENT.azimuth));
        let mut pica = Pica::new(Box::new(DynamicEstimator(
            Arc::clone(&range),
            Arc::clone(&azimuth),
            30,
        )));
        let (controller, _controlee) = start_twr_pair(
            &mut pica,
            EDGE_TRIGGER_SESSION_ID,
            MacAddressMode::Mode0,
            &CONTROLLER_SHORT_MAC,
            &CONTROLEE_SHORT_MAC,
            vec![
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::AoaResultReq,
                    v: vec![AoaResultReq::AoaEnabled as u8],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                    v: vec![SessionInfoNtfConfig::EnableProximityAoaEdgeTrigger as u8],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::NearProximityConfig,
                    v: NEAR_PROXIMITY_100_CM_LE.to_vec(),
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::FarProximityConfig,
                    v: FAR_PROXIMITY_200_CM_LE.to_vec(),
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::AoaBoundConfig,
                    v: AOA_BOUNDS_AZ_NEG10_TO_10_EL_20_TO_45_LE.to_vec(),
                },
            ],
        );

        // Starts outside proximity bounds -> suppressed.
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_none());

        // Enters both proximity and AoA bounds -> emits once, then suppresses.
        range.store(IN_RANGE_CM, Ordering::Relaxed);
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_some());
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_none());

        // Exits AoA bounds -> emits once on exit.
        azimuth.store(30, Ordering::Relaxed);
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_some());
    }

    fn start_ccc_or_aliro_session(
        pica: &mut Pica,
        session_id: u32,
        session_type: SessionType,
        role: DeviceRole,
        multi_node: MultiNodeMode,
        extra: Vec<AppConfigTlv>,
    ) -> Handle {
        let handle = add_reset_device(pica);
        let _ = send_cmd(
            pica,
            handle,
            SessionInitCmd {
                session_id,
                session_type,
            },
        );
        let tlvs: Vec<AppConfigTlv> = [
            AppConfigTlv {
                cfg_id: AppConfigTlvType::DeviceRole,
                v: vec![role as u8],
            },
            AppConfigTlv {
                cfg_id: AppConfigTlvType::MultiNodeMode,
                v: vec![multi_node as u8],
            },
            AppConfigTlv {
                cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                v: vec![SessionInfoNtfConfig::Enable as u8],
            },
        ]
        .into_iter()
        .chain(extra)
        .collect();
        let cfg = send_cmd(
            pica,
            handle,
            SessionSetAppConfigCmd {
                session_token: session_id,
                tlvs,
            },
        );
        assert_eq!(
            SessionSetAppConfigRsp::decode_full(&cfg[0]).unwrap().status,
            uci::Status::Ok
        );
        let start = send_cmd(pica, handle, SessionStartCmd { session_id });
        assert_eq!(
            SessionStartRsp::decode_full(&start[0]).unwrap().status,
            uci::Status::Ok
        );
        handle
    }

    #[test]
    fn ccc_and_aliro_pair_without_mac_addresses() {
        for (session_type, extra) in [
            (SessionType::Ccc, vec![]),
            (
                SessionType::Aliro,
                vec![
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::AliroMacMode,
                        v: vec![0x41],
                    },
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::SessionKey,
                        v: vec![0xAB; 16],
                    },
                ],
            ),
        ] {
            let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
            let initiator = start_ccc_or_aliro_session(
                &mut pica,
                TWR_SESSION_ID,
                session_type,
                DeviceRole::Initiator,
                MultiNodeMode::OneToOne,
                extra.clone(),
            );
            let responder = start_ccc_or_aliro_session(
                &mut pica,
                TWR_SESSION_ID,
                session_type,
                DeviceRole::Responder,
                MultiNodeMode::OneToOne,
                extra,
            );

            for (handle, expected_slot) in [(initiator, 1u8), (responder, 0u8)] {
                pica.ranging(handle, TWR_SESSION_ID);
                let pkt = pica.drain_packets().next().unwrap().1;
                let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
                assert_eq!(ntf.two_way_ranging_measurements.len(), 1);
                let m = &ntf.two_way_ranging_measurements[0];
                assert_eq!(m.status, uci::Status::Ok);
                assert_eq!(m.mac_address, 0);
                assert_eq!(m.distance, STANDARD_MEASUREMENT.range);
                assert_eq!(m.slot_index, expected_slot);
            }
        }
    }

    #[test]
    fn ccc_and_aliro_reject_incompatible_peer() {
        let cases = [
            (
                "ccc vs aliro session type",
                SessionType::Ccc,
                SessionType::Aliro,
                vec![],
                vec![],
            ),
            (
                "mismatched CccHopModeKey",
                SessionType::Ccc,
                SessionType::Ccc,
                vec![],
                vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::CccHopModeKey,
                    v: vec![1, 2, 3, 4],
                }],
            ),
            (
                "mismatched AliroMacMode",
                SessionType::Aliro,
                SessionType::Aliro,
                vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::AliroMacMode,
                    v: vec![0x41],
                }],
                vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::AliroMacMode,
                    v: vec![0x42],
                }],
            ),
            (
                "mismatched Aliro SessionKey",
                SessionType::Aliro,
                SessionType::Aliro,
                vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionKey,
                    v: vec![0xAB; 16],
                }],
                vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionKey,
                    v: vec![0xCD; 16],
                }],
            ),
        ];

        for (label, init_type, resp_type, init_extra, resp_extra) in cases {
            let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
            let initiator = start_ccc_or_aliro_session(
                &mut pica,
                TWR_SESSION_ID,
                init_type,
                DeviceRole::Initiator,
                MultiNodeMode::OneToOne,
                init_extra,
            );
            let _responder = start_ccc_or_aliro_session(
                &mut pica,
                TWR_SESSION_ID,
                resp_type,
                DeviceRole::Responder,
                MultiNodeMode::OneToOne,
                resp_extra,
            );

            pica.ranging(initiator, TWR_SESSION_ID);
            let pkt = pica.drain_packets().next().unwrap().1;
            let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
            assert!(
                ntf.two_way_ranging_measurements.is_empty(),
                "expected no pairing for {label}"
            );
        }
    }

    #[test]
    fn ccc_ranging_advances_last_sts_index_used() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let initiator = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Initiator,
            MultiNodeMode::OneToOne,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::StsIndex,
                v: 100u32.to_le_bytes().to_vec(),
            }],
        );
        let read_last_sts = |pica: &mut Pica| -> u32 {
            let pkts = send_cmd(
                pica,
                initiator,
                SessionGetAppConfigCmd {
                    session_token: TWR_SESSION_ID,
                    app_cfg: vec![AppConfigTlvType::CccLastIndexUsed],
                },
            );
            let rsp = SessionGetAppConfigRsp::decode_full(&pkts[0]).unwrap();
            u32::from_le_bytes(rsp.tlvs[0].v.as_slice().try_into().unwrap())
        };

        pica.ranging(initiator, TWR_SESSION_ID);
        let _ = pica.drain_packets().count();
        assert_eq!(read_last_sts(&mut pica), 100);

        pica.ranging(initiator, TWR_SESSION_ID);
        let _ = pica.drain_packets().count();
        assert_eq!(read_last_sts(&mut pica), 101);
    }

    #[test]
    fn ccc_same_role_sessions_do_not_pair() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let resp1 = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToOne,
            vec![],
        );
        let resp2 = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToOne,
            vec![],
        );

        for handle in [resp1, resp2] {
            pica.ranging(handle, TWR_SESSION_ID);
            let pkt = pica.drain_packets().next().unwrap().1;
            let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
            assert!(ntf.two_way_ranging_measurements.is_empty());
        }
    }

    #[test]
    fn ccc_initiator_suspend_pauses_responder() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let _initiator = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Initiator,
            MultiNodeMode::OneToOne,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::SuspendRangingRounds,
                v: vec![SuspendRangingRounds::Enable as u8],
            }],
        );
        let responder = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToOne,
            vec![],
        );

        pica.ranging(responder, TWR_SESSION_ID);
        assert!(pica.drain_packets().next().is_none());
    }

    #[test]
    fn ccc_responder_suspend_filters_only_suspended_responder() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let initiator = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Initiator,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::NumberOfControlees,
                v: vec![2],
            }],
        );
        let _active_resp = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![1],
            }],
        );
        let _suspended_resp = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::ResponderSlotIndex,
                    v: vec![2],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::SuspendRangingRounds,
                    v: vec![SuspendRangingRounds::Enable as u8],
                },
            ],
        );

        pica.ranging(initiator, TWR_SESSION_ID);
        let pkt = pica.drain_packets().next().unwrap().1;
        let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
        assert_eq!(ntf.two_way_ranging_measurements.len(), 1);
        assert_eq!(ntf.two_way_ranging_measurements[0].slot_index, 1);
    }

    #[test]
    fn ccc_multi_anchor_reports_slots_within_controlee_limit() {
        let estimator = PairwiseEstimator::default();
        let mut pica = Pica::new(Box::new(estimator.clone()));
        let initiator = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Initiator,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::NumberOfControlees,
                v: vec![2],
            }],
        );
        let resp_slot1 = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![1],
            }],
        );
        let resp_slot2 = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![2],
            }],
        );
        let _resp_slot3_over_limit = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![3],
            }],
        );
        for (peer, range) in [(resp_slot1, 120u16), (resp_slot2, 240u16)] {
            let m = RangingMeasurement {
                range,
                azimuth: 0,
                elevation: 0,
            };
            estimator.insert(initiator, peer, m);
            estimator.insert(peer, initiator, m);
        }

        pica.ranging(initiator, TWR_SESSION_ID);
        let pkt = pica.drain_packets().next().unwrap().1;
        let ntf = ShortMacTwoWaySessionInfoNtf::decode_full(&pkt).unwrap();
        let slots_and_ranges: Vec<(u8, u16)> = ntf
            .two_way_ranging_measurements
            .iter()
            .map(|m| (m.slot_index, m.distance))
            .collect();
        assert_eq!(slots_and_ranges, vec![(1, 120), (2, 240)]);
    }

    #[test]
    fn ccc_multi_anchor_drops_colliding_slot_for_both_sides() {
        let mut pica = Pica::new(Box::new(FixedEstimator(STANDARD_MEASUREMENT)));
        let initiator = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Initiator,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::NumberOfControlees,
                v: vec![2],
            }],
        );
        let _resp_slot1 = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![1],
            }],
        );
        let resp_slot2_a = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![2],
            }],
        );
        let _resp_slot2_b = start_ccc_or_aliro_session(
            &mut pica,
            TWR_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![2],
            }],
        );

        pica.ranging(initiator, TWR_SESSION_ID);
        let init_ntf =
            ShortMacTwoWaySessionInfoNtf::decode_full(&pica.drain_packets().next().unwrap().1)
                .unwrap();
        assert_eq!(init_ntf.two_way_ranging_measurements.len(), 1);
        assert_eq!(init_ntf.two_way_ranging_measurements[0].slot_index, 1);

        pica.ranging(resp_slot2_a, TWR_SESSION_ID);
        let resp_ntf =
            ShortMacTwoWaySessionInfoNtf::decode_full(&pica.drain_packets().next().unwrap().1)
                .unwrap();
        assert!(resp_ntf.two_way_ranging_measurements.is_empty());
    }

    #[test]
    fn ccc_multi_anchor_edge_trigger_tracks_peers_independently_by_handle() {
        let estimator = PairwiseEstimator::default();
        let mut pica = Pica::new(Box::new(estimator.clone()));
        let initiator = start_ccc_or_aliro_session(
            &mut pica,
            EDGE_TRIGGER_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Initiator,
            MultiNodeMode::OneToMany,
            vec![
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::NumberOfControlees,
                    v: vec![2],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                    v: vec![SessionInfoNtfConfig::EnableProximityEdgeTrigger as u8],
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::NearProximityConfig,
                    v: NEAR_PROXIMITY_100_CM_LE.to_vec(),
                },
                AppConfigTlv {
                    cfg_id: AppConfigTlvType::FarProximityConfig,
                    v: FAR_PROXIMITY_200_CM_LE.to_vec(),
                },
            ],
        );
        let resp1 = start_ccc_or_aliro_session(
            &mut pica,
            EDGE_TRIGGER_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![1],
            }],
        );
        let resp2 = start_ccc_or_aliro_session(
            &mut pica,
            EDGE_TRIGGER_SESSION_ID,
            SessionType::Ccc,
            DeviceRole::Responder,
            MultiNodeMode::OneToMany,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::ResponderSlotIndex,
                v: vec![2],
            }],
        );
        let set_range = |peer: Handle, range: u16| {
            let m = RangingMeasurement {
                range,
                azimuth: 0,
                elevation: 0,
            };
            estimator.insert(initiator, peer, m);
            estimator.insert(peer, initiator, m);
        };

        set_range(resp1, IN_RANGE_CM);
        set_range(resp2, OUT_OF_RANGE_CM);
        pica.ranging(initiator, EDGE_TRIGGER_SESSION_ID);
        let ntf1 =
            ShortMacTwoWaySessionInfoNtf::decode_full(&pica.drain_packets().next().unwrap().1)
                .unwrap();
        assert_eq!(ntf1.two_way_ranging_measurements.len(), 1);
        assert_eq!(ntf1.two_way_ranging_measurements[0].slot_index, 1);

        // Second responder entering window must emit independently despite sharing 0x0000 MAC.
        set_range(resp2, IN_RANGE_CM);
        pica.ranging(initiator, EDGE_TRIGGER_SESSION_ID);
        let ntf2 =
            ShortMacTwoWaySessionInfoNtf::decode_full(&pica.drain_packets().next().unwrap().1)
                .unwrap();
        assert_eq!(ntf2.two_way_ranging_measurements.len(), 1);
        assert_eq!(ntf2.two_way_ranging_measurements[0].slot_index, 2);
    }

    #[test]
    fn fira_multicast_remove_and_readd_retriggers_edge() {
        let mut pica = Pica::new(Box::new(FixedEstimator(RangingMeasurement {
            range: IN_RANGE_CM,
            azimuth: 0,
            elevation: 0,
        })));
        let controller = add_reset_device(&mut pica);
        let controlee = add_reset_device(&mut pica);
        start_fira_session(
            &mut pica,
            controller,
            &FiraSessionConfig {
                session_id: EDGE_TRIGGER_SESSION_ID,
                session_type: SessionType::FiraRangingSession,
                role: DeviceRole::Initiator,
                dev_type: DeviceType::Controller,
                mac_mode: MacAddressMode::Mode0,
                multi_node_mode: MultiNodeMode::OneToMany,
                local_mac: &CONTROLLER_SHORT_MAC,
                dst_mac: Some(&CONTROLEE_SHORT_MAC),
                ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
                ranging_duration_ms: RANGING_DURATION_MS,
                extra_tlvs: vec![
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::SessionInfoNtfConfig,
                        v: vec![SessionInfoNtfConfig::EnableProximityEdgeTrigger as u8],
                    },
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::NearProximityConfig,
                        v: NEAR_PROXIMITY_100_CM_LE.to_vec(),
                    },
                    AppConfigTlv {
                        cfg_id: AppConfigTlvType::FarProximityConfig,
                        v: FAR_PROXIMITY_200_CM_LE.to_vec(),
                    },
                ],
            },
        );
        start_fira_session(
            &mut pica,
            controlee,
            &FiraSessionConfig {
                session_id: EDGE_TRIGGER_SESSION_ID,
                session_type: SessionType::FiraRangingSession,
                role: DeviceRole::Responder,
                dev_type: DeviceType::Controlee,
                mac_mode: MacAddressMode::Mode0,
                multi_node_mode: MultiNodeMode::OneToMany,
                local_mac: &CONTROLEE_SHORT_MAC,
                dst_mac: Some(&CONTROLLER_SHORT_MAC),
                ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
                ranging_duration_ms: RANGING_DURATION_MS,
                extra_tlvs: vec![],
            },
        );

        // Add a dummy second controlee so removing controlee does not deactivate the session.
        let dummy_mac = [0x9A, 0xBC];
        let _ = send_cmd(
            &mut pica,
            controller,
            SessionUpdateControllerMulticastListCmd {
                session_token: EDGE_TRIGGER_SESSION_ID,
                action: UpdateMulticastListAction::AddControlee,
                payload: SessionUpdateControllerMulticastListCmdPayload {
                    controlees: vec![Controlee {
                        short_address: dummy_mac,
                        subsession_id: 1,
                    }],
                }
                .encode_to_vec()
                .unwrap(),
            },
        );

        // Initial entry emits; second round suppressed.
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_some());
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_none());

        // Remove controlee, tick once to clear stale in_range_peers entry, then re-add.
        let _ = send_cmd(
            &mut pica,
            controller,
            SessionUpdateControllerMulticastListCmd {
                session_token: EDGE_TRIGGER_SESSION_ID,
                action: UpdateMulticastListAction::RemoveControlee,
                payload: SessionUpdateControllerMulticastListCmdPayload {
                    controlees: vec![Controlee {
                        short_address: CONTROLEE_SHORT_MAC,
                        subsession_id: 0,
                    }],
                }
                .encode_to_vec()
                .unwrap(),
            },
        );
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        let _ = pica.drain_packets().count();

        // Re-start controlee (stopped by in-band termination on removal) and re-add to multicast list.
        let _ = send_cmd(
            &mut pica,
            controlee,
            SessionStartCmd {
                session_id: EDGE_TRIGGER_SESSION_ID,
            },
        );
        let _ = send_cmd(
            &mut pica,
            controller,
            SessionUpdateControllerMulticastListCmd {
                session_token: EDGE_TRIGGER_SESSION_ID,
                action: UpdateMulticastListAction::AddControlee,
                payload: SessionUpdateControllerMulticastListCmdPayload {
                    controlees: vec![Controlee {
                        short_address: CONTROLEE_SHORT_MAC,
                        subsession_id: 0,
                    }],
                }
                .encode_to_vec()
                .unwrap(),
            },
        );
        pica.ranging(controller, EDGE_TRIGGER_SESSION_ID);
        assert!(pica.drain_packets().next().is_some());
    }
}
