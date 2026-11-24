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

use crate::app_config::{AppConfig, AppConfigError};
use crate::device::Device;
use crate::packets::uci::{self, *};
use crate::ranging::{CHIPS_PER_RSTU, RSTU_PER_MS, encode_aoa, reported_rssi};
use crate::session::Session;
use crate::{Handle, MacAddress, Pica, RangingMeasurement, UciPacket};

/// Ranging counter ticks per UWB chip (`2^7 = 128`, `~15.65 ps` per tick).
const TICKS_PER_CHIP: u64 = 128;
const TICKS_PER_RSTU: u64 = CHIPS_PER_RSTU * TICKS_PER_CHIP;
const TICKS_PER_MS: u64 = RSTU_PER_MS * TICKS_PER_RSTU;
const TICKS_PER_S: u64 = TICKS_PER_MS * 1000;
/// Speed of light in air (`299,702,547 m/s`, FiRa UCI Table 33).
const SPEED_OF_LIGHT_M_PER_S: u64 = 299_702_547;
const SPEED_OF_LIGHT_CM_PER_S: u64 = SPEED_OF_LIGHT_M_PER_S * 100;

fn cm_to_tof_ticks(cm: u16) -> u64 {
    (u64::from(cm) * TICKS_PER_S) / SPEED_OF_LIGHT_CM_PER_S
}

/// UWBS capability limit (`DT_ANCHOR_MAX_ACTIVE_RR` / `DT_TAG_MAX_ACTIVE_RR`); at 16 rounds,
/// the 15 non-current anchor rounds fit in `DlTdoaRangingMeasurement`'s 4-bit count field.
pub const MAX_ACTIVE_RANGING_ROUNDS: usize = 16;

/// FiRa UCI Table 35 sentinel (`0x8000` in signed Q6.10) when CFO is omitted or unsupported.
const INVALID_CFO: u16 = 0x8000;
const TIMESTAMP_40_BIT_MASK: u64 = (1u64 << 40) - 1;
const MAX_RESPONDERS_PER_ROUND: usize = 8;

const DEFAULT_DL_TDOA_TX_TIMESTAMP_CONF: uci::DlTdoaTxTimestampConf = uci::DlTdoaTxTimestampConf {
    timestamp_type: uci::DlTdoaTxTimestampType::CommonTimeBase,
    timestamp_length: uci::DlTdoaTimestampLength::SixtyFourBit,
};
const DEFAULT_DL_TDOA_RANGING_METHOD: uci::DlTdoaRangingMethod = uci::DlTdoaRangingMethod::DsTwr;
const DEFAULT_DL_TDOA_BLOCK_SKIPPING: u8 = 0;
const DEFAULT_DL_TDOA_ANCHOR_CFO: uci::DlTdoaAnchorCfo = uci::DlTdoaAnchorCfo::AnchorCfoIncluded;
const DEFAULT_DL_TDOA_TX_ACTIVE_RANGING_ROUNDS: bool = false;
const DEFAULT_DL_TDOA_TIME_REFERENCE_ANCHOR: bool = false;
const DEFAULT_DL_TDOA_RESPONDER_TOF: bool = false;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DlTdoaAnchorLocation {
    Wgs84(uci::Wgs84Location),
    Relative(uci::RelativeLocation),
}

impl DlTdoaAnchorLocation {
    pub fn parse(value: &[u8]) -> Result<Option<Self>, AppConfigError> {
        let id = uci::AppConfigTlvType::DlTdoaAnchorLocation;
        let Some(&byte) = value.first() else {
            return Err(AppConfigError::InvalidLength {
                id,
                expected: 1,
                actual: 0,
            });
        };
        let (header, rest) = uci::DlTdoaAnchorLocationHeader::decode(value)
            .map_err(|_| AppConfigError::InvalidEnumValue { id, value: byte })?;
        match (header.presence, header.location_type) {
            (uci::DlTdoaAnchorLocationPresence::Absent, uci::DlTdoaAnchorLocationType::Wgs84) => {
                if !rest.is_empty() {
                    return Err(AppConfigError::InvalidLength {
                        id,
                        expected: 1,
                        actual: value.len(),
                    });
                }
                Ok(None)
            }
            (uci::DlTdoaAnchorLocationPresence::Absent, _) => {
                Err(AppConfigError::InvalidEnumValue { id, value: byte })
            }
            (uci::DlTdoaAnchorLocationPresence::Present, uci::DlTdoaAnchorLocationType::Wgs84) => {
                uci::Wgs84Location::decode_full(rest)
                    .map(|loc| Some(Self::Wgs84(loc)))
                    .map_err(|_| AppConfigError::InvalidLength {
                        id,
                        expected: 13,
                        actual: value.len(),
                    })
            }
            (
                uci::DlTdoaAnchorLocationPresence::Present,
                uci::DlTdoaAnchorLocationType::Relative,
            ) => uci::RelativeLocation::decode_full(rest)
                .map(|loc| Some(Self::Relative(loc)))
                .map_err(|_| AppConfigError::InvalidLength {
                    id,
                    expected: 11,
                    actual: value.len(),
                }),
        }
    }
}

impl AppConfig {
    pub fn is_dl_tdoa(&self) -> bool {
        self.ranging_round_usage() == Some(uci::RangingRoundUsage::OneWayRangingDlTdoa)
    }

    pub fn dl_tdoa_role(&self) -> Option<uci::DeviceRole> {
        if self.is_dl_tdoa() {
            self.device_role()
        } else {
            None
        }
    }

    pub fn dl_tdoa_anchor_location(&self) -> Option<DlTdoaAnchorLocation> {
        self.get(uci::AppConfigTlvType::DlTdoaAnchorLocation)
            .and_then(|bytes| DlTdoaAnchorLocation::parse(bytes).ok().flatten())
    }

    pub fn dl_tdoa_includes_active_ranging_rounds(&self) -> bool {
        self.get_bool(uci::AppConfigTlvType::DlTdoaTxActiveRangingRounds)
            .unwrap_or(DEFAULT_DL_TDOA_TX_ACTIVE_RANGING_ROUNDS)
    }

    pub fn dl_tdoa_tx_timestamp_conf(&self) -> uci::DlTdoaTxTimestampConf {
        self.get(uci::AppConfigTlvType::DlTdoaTxTimestampConf)
            .and_then(|bytes| uci::DlTdoaTxTimestampConf::decode_full(bytes).ok())
            .unwrap_or(DEFAULT_DL_TDOA_TX_TIMESTAMP_CONF)
    }

    pub fn dl_tdoa_ranging_method(&self) -> uci::DlTdoaRangingMethod {
        self.get_enum(uci::AppConfigTlvType::DlTdoaRangingMethod)
            .unwrap_or(DEFAULT_DL_TDOA_RANGING_METHOD)
    }

    pub fn dl_tdoa_includes_responder_tof(&self) -> bool {
        self.get_bool(uci::AppConfigTlvType::DlTdoaResponderTof)
            .unwrap_or(DEFAULT_DL_TDOA_RESPONDER_TOF)
    }

    pub fn dl_tdoa_includes_anchor_cfo(&self) -> bool {
        self.get_enum(uci::AppConfigTlvType::DlTdoaAnchorCfo)
            .unwrap_or(DEFAULT_DL_TDOA_ANCHOR_CFO)
            == uci::DlTdoaAnchorCfo::AnchorCfoIncluded
    }

    pub fn dl_tdoa_is_time_reference_anchor(&self) -> bool {
        self.get_bool(uci::AppConfigTlvType::DlTdoaTimeReferenceAnchor)
            .unwrap_or(DEFAULT_DL_TDOA_TIME_REFERENCE_ANCHOR)
    }

    pub fn dl_tdoa_block_skipping(&self) -> u8 {
        self.get_u8(uci::AppConfigTlvType::DlTdoaBlockSkipping)
            .unwrap_or(DEFAULT_DL_TDOA_BLOCK_SKIPPING)
    }

    pub fn is_compatible_for_dl_tdoa(&self, tag_config: &Self) -> bool {
        self.dl_tdoa_role() == Some(uci::DeviceRole::DtAnchor)
            && self.suspend_ranging_rounds() == uci::SuspendRangingRounds::Disable
            && self.channel_number() == tag_config.channel_number()
            && self.mac_address_mode() == tag_config.mac_address_mode()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DlTdoaRangingRounds {
    Tag(Vec<u8>),
    Anchor(Vec<uci::DtAnchorRangingRound>),
}

impl Session {
    pub fn dt_tag_active_rounds(&self) -> &[u8] {
        match &self.dl_tdoa_rounds {
            Some(DlTdoaRangingRounds::Tag(rounds)) => rounds.as_slice(),
            _ => &[],
        }
    }

    pub fn dt_anchor_ranging_rounds(&self) -> &[uci::DtAnchorRangingRound] {
        match &self.dl_tdoa_rounds {
            Some(DlTdoaRangingRounds::Anchor(rounds)) => rounds.as_slice(),
            _ => &[],
        }
    }

    pub fn set_dt_tag_ranging_rounds(&mut self, round_indexes: Vec<u8>) {
        self.dl_tdoa_rounds = Some(DlTdoaRangingRounds::Tag(round_indexes));
    }

    pub fn set_dt_anchor_ranging_rounds(&mut self, rounds: Vec<uci::DtAnchorRangingRound>) {
        self.dl_tdoa_rounds = Some(DlTdoaRangingRounds::Anchor(rounds));
    }

    pub fn dl_tdoa_invalid_start_reason(&self) -> Option<ReasonCode> {
        match self.app_config.dl_tdoa_role()? {
            uci::DeviceRole::DtAnchor if self.dt_anchor_ranging_rounds().is_empty() => {
                Some(ReasonCode::ErrorDtAnchorRangingRoundsNotConfigured)
            }
            uci::DeviceRole::DtTag if self.dt_tag_active_rounds().is_empty() => {
                Some(ReasonCode::ErrorDtTagRangingRoundsNotConfigured)
            }
            _ => None,
        }
    }

    pub fn encode_dl_tdoa_session_info_ntf(
        &self,
        measurements: Vec<DlTdoaMeasurement>,
    ) -> Option<UciPacket> {
        match self.app_config.mac_address_mode() {
            MacAddressMode::Mode0 => Some(
                ShortMacDlTDoASessionInfoNtf {
                    sequence_number: self.sequence_number,
                    session_token: self.id,
                    rcr_indicator: 0,
                    current_ranging_interval: self.ranging_interval_ms(),
                    dl_tdoa_measurements: measurements
                        .into_iter()
                        .filter_map(|m| match m.mac_address {
                            MacAddress::Short(addr) => Some(ShortAddressDlTdoaRangingMeasurement {
                                mac_address: u16::from_le_bytes(addr),
                                measurement: m.measurement,
                            }),
                            MacAddress::Extended(_) => None,
                        })
                        .collect(),
                }
                .encode_to_vec()
                .expect("failed to encode ShortMacDlTDoASessionInfoNtf"),
            ),
            MacAddressMode::Mode2 => Some(
                ExtendedMacDlTDoASessionInfoNtf {
                    sequence_number: self.sequence_number,
                    session_token: self.id,
                    rcr_indicator: 0,
                    current_ranging_interval: self.ranging_interval_ms(),
                    dl_tdoa_measurements: measurements
                        .into_iter()
                        .filter_map(|m| match m.mac_address {
                            MacAddress::Extended(addr) => {
                                Some(ExtendedAddressDlTdoaRangingMeasurement {
                                    mac_address: u64::from_le_bytes(addr),
                                    measurement: m.measurement,
                                })
                            }
                            MacAddress::Short(_) => None,
                        })
                        .collect(),
                }
                .encode_to_vec()
                .expect("failed to encode ExtendedMacDlTDoASessionInfoNtf"),
            ),
            mode => {
                tracing::warn!("Unsupported MAC address mode {mode:?}");
                None
            }
        }
    }
}

/// Per-round anchor state unifying static virtual anchors (from `Pica::create_anchor`)
/// and active `DtAnchor` UCI sessions.
#[derive(Debug)]
pub struct DlTdoaAnchor {
    pub mac_address: MacAddress,
    pub location: Option<DlTdoaAnchorLocation>,
    pub round_index: u8,
    pub acting_role: Option<uci::DtAnchorActingRole>,
    pub ranging_method: uci::DlTdoaRangingMethod,
    pub active_ranging_rounds: Vec<u8>,
    pub tx_timestamp_conf: uci::DlTdoaTxTimestampConf,
    pub includes_responder_tof: bool,
    pub includes_anchor_cfo: bool,
    pub is_time_reference: bool,
}

impl DlTdoaAnchor {
    /// Creates a virtual anchor (from `Pica::create_anchor`), enabling Responder ToF by default so
    /// multi-anchor simulations report inter-anchor ToF without per-anchor `AppConfig` setup.
    pub fn virtual_anchor(mac_address: MacAddress, default_round_index: u8) -> Self {
        Self {
            mac_address,
            location: None,
            round_index: default_round_index,
            acting_role: None,
            ranging_method: DEFAULT_DL_TDOA_RANGING_METHOD,
            active_ranging_rounds: Vec::new(),
            tx_timestamp_conf: DEFAULT_DL_TDOA_TX_TIMESTAMP_CONF,
            includes_responder_tof: true,
            includes_anchor_cfo: true,
            is_time_reference: false,
        }
    }

    fn from_session(
        session: &Session,
        mac_address: MacAddress,
        round: &uci::DtAnchorRangingRound,
    ) -> Self {
        let config = &session.app_config;
        let active_ranging_rounds = if config.dl_tdoa_includes_active_ranging_rounds() {
            session
                .dt_anchor_ranging_rounds()
                .iter()
                .map(|r| r.round_index)
                .filter(|&idx| idx != round.round_index)
                .collect()
        } else {
            Vec::new()
        };
        Self {
            mac_address,
            location: config.dl_tdoa_anchor_location(),
            round_index: round.round_index,
            acting_role: Some(round.role),
            ranging_method: config.dl_tdoa_ranging_method(),
            active_ranging_rounds,
            tx_timestamp_conf: config.dl_tdoa_tx_timestamp_conf(),
            includes_responder_tof: config.dl_tdoa_includes_responder_tof(),
            includes_anchor_cfo: config.dl_tdoa_includes_anchor_cfo(),
            is_time_reference: config.dl_tdoa_is_time_reference_anchor(),
        }
    }
}

impl Device {
    pub fn active_dl_tdoa_anchors(
        &self,
        peer_session: &Session,
        session_id: u32,
    ) -> Vec<DlTdoaAnchor> {
        let Some(session) = self.session(session_id) else {
            return Vec::new();
        };
        let config = &session.app_config;
        if session.session_state() != SessionState::SessionStateActive
            || !config.is_compatible_for_dl_tdoa(&peer_session.app_config)
        {
            return Vec::new();
        }
        let Some(mac_address) = config.device_mac_address() else {
            return Vec::new();
        };
        let tag_rounds = peer_session.dt_tag_active_rounds();
        session
            .dt_anchor_ranging_rounds()
            .iter()
            .filter(|round| tag_rounds.contains(&round.round_index))
            .map(|round| DlTdoaAnchor::from_session(session, mac_address, round))
            .collect()
    }

    fn configured_dl_tdoa_session_mut(
        &mut self,
        session_token: u32,
        expected_role: uci::DeviceRole,
    ) -> Result<&mut Session, uci::Status> {
        let Some(session) = self.session_mut(session_token) else {
            return Err(uci::Status::ErrorSessionNotExist);
        };
        let is_configured = session.state != uci::SessionState::SessionStateInit
            && session.app_config.dl_tdoa_role() == Some(expected_role);
        if !is_configured {
            return Err(uci::Status::Rejected);
        }
        Ok(session)
    }

    pub fn session_update_dt_tag_ranging_rounds(
        &mut self,
        cmd: SessionUpdateDtTagRangingRoundsCmd,
    ) -> SessionUpdateDtTagRangingRoundsRsp {
        let status =
            match self.configured_dl_tdoa_session_mut(cmd.session_token, uci::DeviceRole::DtTag) {
                Ok(_) if cmd.ranging_round_indexes.is_empty() => uci::Status::InvalidParam,
                Ok(_) if cmd.ranging_round_indexes.len() > MAX_ACTIVE_RANGING_ROUNDS => {
                    uci::Status::ErrorNumberOfActiveRangingRoundsExceeded
                }
                Ok(session) => {
                    session.set_dt_tag_ranging_rounds(cmd.ranging_round_indexes);
                    uci::Status::Ok
                }
                Err(status) => status,
            };
        SessionUpdateDtTagRangingRoundsRsp {
            status,
            ranging_round_indexes: Vec::new(),
        }
    }

    pub fn session_update_dt_anchor_ranging_rounds(
        &mut self,
        cmd: SessionUpdateDtAnchorRangingRoundsCmd,
    ) -> SessionUpdateDtAnchorRangingRoundsRsp {
        let status = match self
            .configured_dl_tdoa_session_mut(cmd.session_token, uci::DeviceRole::DtAnchor)
        {
            Ok(_) if cmd.ranging_rounds.is_empty() => uci::Status::InvalidParam,
            Ok(_) if cmd.ranging_rounds.len() > MAX_ACTIVE_RANGING_ROUNDS => {
                uci::Status::ErrorNumberOfActiveRangingRoundsExceeded
            }
            Ok(session) => {
                session.set_dt_anchor_ranging_rounds(cmd.ranging_rounds);
                uci::Status::Ok
            }
            Err(status) => status,
        };
        SessionUpdateDtAnchorRangingRoundsRsp {
            status,
            ranging_round_indexes: Vec::new(),
        }
    }
}

/// Carries the peer's `MacAddress` alongside `DlTdoaRangingMeasurement`, which omits the
/// address in PDL so short and extended MAC notifications can share the inner layout.
#[derive(Debug)]
pub struct DlTdoaMeasurement {
    pub mac_address: MacAddress,
    pub measurement: uci::DlTdoaRangingMeasurement,
}

impl DlTdoaMeasurement {
    fn with_response_timing(
        self,
        init_to_resp: RangingMeasurement,
        includes_tof: bool,
        slot_offset_ticks: u64,
        final_slot_offset_ticks: Option<u64>,
    ) -> Self {
        let tof_ticks = cm_to_tof_ticks(init_to_resp.range);
        let responder_reply_time =
            u32::try_from(slot_offset_ticks.saturating_sub(tof_ticks)).unwrap_or(u32::MAX);
        let initiator_reply_time = final_slot_offset_ticks.map_or(0, |final_offset| {
            let rx_offset = slot_offset_ticks.saturating_add(tof_ticks);
            u32::try_from(final_offset.saturating_sub(rx_offset)).unwrap_or(u32::MAX)
        });
        let initiator_responder_tof = if includes_tof {
            u16::try_from(tof_ticks).unwrap_or(u16::MAX)
        } else {
            0
        };
        Self {
            mac_address: self.mac_address,
            measurement: uci::DlTdoaRangingMeasurement {
                initiator_reply_time,
                responder_reply_time,
                initiator_responder_tof,
                ..self.measurement
            },
        }
    }

    fn new(
        app_config: &AppConfig,
        anchor: &DlTdoaAnchor,
        local: RangingMeasurement,
        message_type: uci::DlTdoaMessageType,
        block_index: u16,
        tx_timestamp_length: uci::DlTdoaTimestampLength,
        tx_timestamp: u64,
    ) -> Self {
        let (aoa_azimuth, aoa_azimuth_fom, aoa_elevation, aoa_elevation_fom) =
            encode_aoa(local, app_config.aoa_result_req());
        let rssi = reported_rssi(app_config);
        let rx_timestamp = tx_timestamp.wrapping_add(cm_to_tof_ticks(local.range));
        let is_final = message_type == uci::DlTdoaMessageType::FinalDtm;
        let (wgs84_location, relative_location) = if is_final {
            (None, None)
        } else {
            match &anchor.location {
                Some(DlTdoaAnchorLocation::Wgs84(loc)) => (Some(loc.clone()), None),
                Some(DlTdoaAnchorLocation::Relative(loc)) => (None, Some(loc.clone())),
                None => (None, None),
            }
        };
        let active_ranging_rounds = if is_final {
            Vec::new()
        } else {
            anchor.active_ranging_rounds.clone()
        };
        let (tx_timestamp_40, tx_timestamp_64) = match tx_timestamp_length {
            uci::DlTdoaTimestampLength::FortyBit => {
                (Some(tx_timestamp & TIMESTAMP_40_BIT_MASK), None)
            }
            uci::DlTdoaTimestampLength::SixtyFourBit => (None, Some(tx_timestamp)),
        };

        Self {
            mac_address: anchor.mac_address,
            measurement: uci::DlTdoaRangingMeasurement {
                status: uci::Status::Ok,
                message_type,
                tx_timestamp_type: anchor.tx_timestamp_conf.timestamp_type,
                block_index,
                round_index: anchor.round_index,
                nlos: 0,
                aoa_azimuth,
                aoa_azimuth_fom,
                aoa_elevation,
                aoa_elevation_fom,
                rssi,
                tx_timestamp_40,
                tx_timestamp_64,
                rx_timestamp_40: None,
                rx_timestamp_64: Some(rx_timestamp),
                anchor_cfo: if message_type == uci::DlTdoaMessageType::ResponseDtm
                    && !anchor.includes_anchor_cfo
                {
                    INVALID_CFO
                } else {
                    0
                },
                cfo: 0,
                initiator_reply_time: 0,
                responder_reply_time: 0,
                initiator_responder_tof: 0,
                wgs84_location,
                relative_location,
                active_ranging_rounds,
            },
        }
    }
}

impl Pica {
    /// Groups active anchors by `round_index` with each round's initiator placed first.
    fn dl_tdoa_anchors(
        &self,
        device_handle: Handle,
        session_id: u32,
        session: &Session,
    ) -> Vec<(Handle, DlTdoaAnchor)> {
        let mac_mode = session.app_config.mac_address_mode();
        let active_rounds = session.dt_tag_active_rounds();
        let mut candidates: Vec<(Handle, DlTdoaAnchor)> = self
            .anchors
            .iter()
            .filter(|(_, mac)| {
                matches!(
                    (mac_mode, *mac),
                    (uci::MacAddressMode::Mode0, MacAddress::Short(_))
                        | (uci::MacAddressMode::Mode2, MacAddress::Extended(_))
                )
            })
            .flat_map(|(&handle, &mac)| {
                active_rounds.iter().copied().map(move |round_index| {
                    (handle, DlTdoaAnchor::virtual_anchor(mac, round_index))
                })
            })
            .chain(
                self.devices
                    .values()
                    .filter(|entry| entry.device.handle != device_handle)
                    .flat_map(|entry| {
                        entry
                            .device
                            .active_dl_tdoa_anchors(session, session_id)
                            .into_iter()
                            .map(move |anchor| (entry.device.handle, anchor))
                    }),
            )
            .collect();
        candidates.sort_by_key(|(handle, anchor)| {
            let role_rank = match anchor.acting_role {
                Some(uci::DtAnchorActingRole::Initiator) => 0,
                None => 1,
                Some(uci::DtAnchorActingRole::Responder) => 2,
            };
            (
                anchor.round_index,
                role_rank,
                !anchor.is_time_reference,
                *handle,
            )
        });
        candidates
    }

    /// Assigns slot `0` to `PollDtm`, slots `1..=K` to `ResponseDtm`s, and slot `K + 1` to
    /// `FinalDtm` (under `DsTwr`) within each active round of the current ranging block.
    fn dl_tdoa_measurements(
        &self,
        device_handle: Handle,
        session_id: u32,
        session: &Session,
    ) -> Vec<DlTdoaMeasurement> {
        let block_stride = session.app_config.block_stride();
        let total_blocks = u64::from(session.ranging_count) * u64::from(block_stride);
        let block_index = total_blocks as u16;
        let block_ticks = u64::from(session.app_config.ranging_duration()) * TICKS_PER_MS;
        let slot_ticks = u64::from(session.app_config.slot_duration()) * TICKS_PER_RSTU;
        let slots_per_rr = session.app_config.slots_per_rr();
        let round_ticks = u64::from(slots_per_rr) * slot_ticks;
        let anchors = self.dl_tdoa_anchors(device_handle, session_id, session);

        anchors
            .chunk_by(|(_, a), (_, b)| a.round_index == b.round_index)
            .filter_map(|round_chunk| {
                let ((init_handle, init_anchor), responders) = round_chunk.split_first()?;
                if init_anchor.acting_role == Some(uci::DtAnchorActingRole::Responder) {
                    return None;
                }
                let init_local = self
                    .ranging_estimator
                    .estimate(&device_handle, init_handle)?;
                let tx_timestamp_length = init_anchor.tx_timestamp_conf.timestamp_length;
                let round_base_ticks = total_blocks
                    .wrapping_mul(block_ticks)
                    .wrapping_add(u64::from(init_anchor.round_index) * round_ticks);

                let reserved_slots = match init_anchor.ranging_method {
                    uci::DlTdoaRangingMethod::SsTwr => 1,
                    uci::DlTdoaRangingMethod::DsTwr => 2,
                };
                let max_responders = usize::from(slots_per_rr.saturating_sub(reserved_slots))
                    .min(MAX_RESPONDERS_PER_ROUND);
                let active_responders: Vec<_> = responders
                    .iter()
                    .filter(|(_, anchor)| {
                        anchor.acting_role != Some(uci::DtAnchorActingRole::Initiator)
                    })
                    .filter_map(|(resp_handle, resp_anchor)| {
                        let init_to_resp =
                            self.ranging_estimator.estimate(init_handle, resp_handle)?;
                        Some((resp_handle, resp_anchor, init_to_resp))
                    })
                    .take(max_responders)
                    .collect();

                let final_slot_offset_ticks = (init_anchor.ranging_method
                    == uci::DlTdoaRangingMethod::DsTwr
                    && !active_responders.is_empty())
                .then(|| (active_responders.len() as u64 + 1) * slot_ticks);

                let poll_dtm = DlTdoaMeasurement::new(
                    &session.app_config,
                    init_anchor,
                    init_local,
                    uci::DlTdoaMessageType::PollDtm,
                    block_index,
                    tx_timestamp_length,
                    round_base_ticks,
                );

                let response_dtms: Vec<_> = active_responders
                    .into_iter()
                    .zip(1u64..)
                    .filter_map(
                        move |((resp_handle, resp_anchor, init_to_resp), slot_idx)| {
                            let local = self
                                .ranging_estimator
                                .estimate(&device_handle, resp_handle)?;
                            let includes_tof = init_anchor.ranging_method
                                == uci::DlTdoaRangingMethod::SsTwr
                                || resp_anchor.includes_responder_tof;
                            let slot_offset_ticks = slot_idx * slot_ticks;
                            Some(
                                DlTdoaMeasurement::new(
                                    &session.app_config,
                                    resp_anchor,
                                    local,
                                    uci::DlTdoaMessageType::ResponseDtm,
                                    block_index,
                                    tx_timestamp_length,
                                    round_base_ticks.wrapping_add(slot_offset_ticks),
                                )
                                .with_response_timing(
                                    init_to_resp,
                                    includes_tof,
                                    slot_offset_ticks,
                                    final_slot_offset_ticks,
                                ),
                            )
                        },
                    )
                    .collect();

                let final_dtm = final_slot_offset_ticks
                    .filter(|_| !response_dtms.is_empty())
                    .map(|final_offset| {
                        DlTdoaMeasurement::new(
                            &session.app_config,
                            init_anchor,
                            init_local,
                            uci::DlTdoaMessageType::FinalDtm,
                            block_index,
                            tx_timestamp_length,
                            round_base_ticks.wrapping_add(final_offset),
                        )
                    });

                Some(
                    std::iter::once(poll_dtm)
                        .chain(response_dtms)
                        .chain(final_dtm),
                )
            })
            .flatten()
            .take(usize::from(u8::MAX))
            .collect()
    }

    pub fn dl_tdoa_ranging(&mut self, device_handle: Handle, session_id: u32) {
        let Some(session) = self
            .get_device(device_handle)
            .and_then(|d| d.session(session_id))
        else {
            return;
        };
        let send_ntf = session.app_config.dl_tdoa_role() == Some(uci::DeviceRole::DtTag)
            && session.app_config.effective_session_info_ntf_config()
                != uci::SessionInfoNtfConfig::Disable;
        let ntf_packet = if send_ntf {
            let measurements = self.dl_tdoa_measurements(device_handle, session_id, session);
            session.encode_dl_tdoa_session_info_ntf(measurements)
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
        if ntf_packet.is_some() {
            session.sequence_number += 1;
        }

        session.ranging_count += 1;
        let effects = if session.has_reached_max_measurements() {
            device.stop_active_session(session_id, ReasonCode::MaxNumberOfMeasurementsReached)
        } else {
            Vec::new()
        };

        if let Some(packet) = ntf_packet {
            self.send_packet(device_handle, packet);
        }
        self.apply_device_effects(device_handle, effects);
    }
}
