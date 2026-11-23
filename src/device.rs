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

mod capabilities;
mod core;
mod session_config;
mod session_control;
mod vendor_android;

use std::collections::HashMap;
use std::time::Instant;

use pdl_runtime::Packet;

use crate::MacAddress;
use crate::packets::uci::{self, *};

use super::UciPacket;
use super::session::Session;

pub const MAX_SESSION: usize = 255;

/// cf. [UCI] 8.3 Table 29
pub const MAX_NUMBER_OF_CONTROLEES: usize = 8;

/// [UCI] 8.2 Device Configuration Parameters
pub struct DeviceConfig {
    // This config is used to enable/disable the low power mode.
    //   0x00 = Disable low power mode
    //   0x01 = Enable low power mode (default)
    low_power_mode: bool,
}

// [UCI] 6.3.1 Setting the Configuration
// All device configuration parameters within the UWBS are set to
// default values at Table 44 [..].
impl Default for DeviceConfig {
    fn default() -> Self {
        DeviceConfig {
            low_power_mode: true,
        }
    }
}

pub struct Device {
    /// Flag set when the device has received the Core Device Reset command.
    /// The first command received by the device is expected to be Core Device
    /// Reset, receiving any other command before this is indicative of a
    /// bad host state.
    is_reset: bool,
    pub handle: usize,
    config: DeviceConfig,
    /// [UCI] 5. UWBS Device State Machine
    state: DeviceState,
    pub sessions: HashMap<u32, Session>,
    country_code: [u8; 2],
    pub n_active_sessions: usize,
    boot_time: Instant,
}

/// Side effect emitted by a device state transition or command.
pub enum DeviceEffect {
    /// Transmit a UCI packet to the host.
    SendPacket(UciPacket),
    /// Signal in-band ranging termination to a controlee.
    StopControleeRanging {
        mac_address: MacAddress,
        session_id: u32,
    },
}

impl DeviceEffect {
    pub fn send_control(packet: impl Packet) -> Self {
        Self::SendPacket(
            packet
                .encode_to_vec()
                .expect("failed to encode control packet"),
        )
    }
}

pub struct CommandOutput(Vec<DeviceEffect>);

impl<T: Packet> From<T> for CommandOutput {
    fn from(response: T) -> Self {
        Self(vec![DeviceEffect::send_control(response)])
    }
}

impl CommandOutput {
    fn with_effects(
        response: impl Packet,
        effects: impl IntoIterator<Item = DeviceEffect>,
    ) -> Self {
        Self(
            std::iter::once(DeviceEffect::send_control(response))
                .chain(effects)
                .collect(),
        )
    }
}

fn command_error_response(gid: GroupId, oid: u8, status: uci::Status) -> ControlPacket {
    ControlPacket {
        mt: uci::MessageType::Response,
        gid,
        payload: vec![oid, 0, 0, status.into()],
    }
}

fn packet_error_status<T, E>(result: Result<T, E>) -> uci::Status {
    if result.is_ok() {
        uci::Status::UnknownOid
    } else {
        uci::Status::SyntaxError
    }
}

impl Device {
    pub fn new(handle: usize) -> Self {
        Device {
            handle,
            is_reset: false,
            config: Default::default(),
            state: DeviceState::DeviceStateError, // Will be overwitten
            sessions: Default::default(),
            country_code: Default::default(),
            n_active_sessions: 0,
            boot_time: Instant::now(),
        }
    }

    #[must_use]
    pub fn set_state(&mut self, device_state: DeviceState) -> Option<CoreDeviceStatusNtf> {
        if device_state == self.state {
            return None;
        }

        self.state = device_state;
        Some(CoreDeviceStatusNtf { device_state })
    }

    #[must_use]
    pub fn init(&mut self) -> Option<DeviceEffect> {
        self.set_state(DeviceState::DeviceStateReady)
            .map(DeviceEffect::send_control)
    }

    #[must_use]
    fn on_session_deactivated(&mut self) -> Option<CoreDeviceStatusNtf> {
        self.n_active_sessions = self.n_active_sessions.saturating_sub(1);
        if self.n_active_sessions == 0 {
            self.set_state(DeviceState::DeviceStateReady)
        } else {
            None
        }
    }

    #[must_use]
    pub fn stop_active_session(
        &mut self,
        session_id: u32,
        reason_code: ReasonCode,
    ) -> Vec<DeviceEffect> {
        let Some(session) = self.session_mut(session_id) else {
            return Vec::new();
        };

        if session.session_state() != SessionState::SessionStateActive {
            tracing::warn!("stop_active_session: session is not active !");
            return Vec::new();
        }

        session.stop_ranging();
        let session_ntf = session
            .set_state(SessionState::SessionStateIdle, reason_code)
            .map(DeviceEffect::send_control);
        let device_ntf = self
            .on_session_deactivated()
            .map(DeviceEffect::send_control);

        session_ntf.into_iter().chain(device_ntf).collect()
    }

    pub fn session(&self, session_id: u32) -> Option<&Session> {
        self.sessions.get(&session_id)
    }

    pub fn session_mut(&mut self, session_id: u32) -> Option<&mut Session> {
        self.sessions.get_mut(&session_id)
    }

    pub fn next_ranging_deadline(&self) -> Option<Instant> {
        self.sessions
            .values()
            .filter_map(Session::next_ranging_at)
            .min()
    }

    pub fn poll_ranging(&mut self, now: Instant) -> impl Iterator<Item = u32> + '_ {
        self.sessions
            .iter_mut()
            .filter_map(move |(&session_id, session)| {
                session.poll_ranging(now).then_some(session_id)
            })
    }

    fn receive_command(&mut self, cmd: ControlPacket) -> Vec<DeviceEffect> {
        use AndroidPacketChild::*;
        use ControlPacketChild::*;
        use CorePacketChild::*;
        use SessionConfigPacketChild::*;
        use SessionControlPacketChild::*;

        // Check whether the first command received is the Core Device
        // Reset command. The controller responds with Device Status
        // Notification with DEVICE_STATE_ERROR otherwise.
        if !self.is_reset && !cmd.is_core_device_reset_cmd() {
            return CommandOutput::from(uci::CoreDeviceStatusNtf {
                device_state: DeviceState::DeviceStateError,
            })
            .0;
        }

        let Ok(child) = cmd.specialize() else {
            let status = if cmd.payload.len() >= 3 {
                uci::Status::UnknownOid
            } else {
                uci::Status::SyntaxError
            };
            return CommandOutput::from(command_error_response(
                cmd.gid,
                cmd.payload.first().copied().unwrap_or(0),
                status,
            ))
            .0;
        };

        let output: CommandOutput = match child {
            CorePacket(cmd) => match cmd.specialize() {
                Ok(CoreDeviceResetCmd(cmd)) => self.core_device_reset(cmd),
                Ok(CoreGetDeviceInfoCmd(cmd)) => self.core_get_device_info(cmd).into(),
                Ok(CoreGetCapsInfoCmd(cmd)) => self.core_get_caps_info(cmd).into(),
                Ok(CoreSetConfigCmd(cmd)) => self.core_set_config(cmd).into(),
                Ok(CoreGetConfigCmd(cmd)) => self.core_get_config(cmd).into(),
                Ok(CoreQueryTimeStampCmd(_)) => self.core_query_timestamp().into(),
                child => command_error_response(
                    GroupId::Core,
                    cmd.oid.into(),
                    packet_error_status(child),
                )
                .into(),
            },
            SessionConfigPacket(cmd) => match cmd.specialize() {
                Ok(SessionInitCmd(cmd)) => self.session_init(cmd),
                Ok(SessionDeinitCmd(cmd)) => self.session_deinit(cmd),
                Ok(SessionGetCountCmd(cmd)) => self.session_get_count(cmd).into(),
                Ok(SessionSetAppConfigCmd(cmd)) => self.session_set_app_config(cmd),
                Ok(SessionGetAppConfigCmd(cmd)) => self.session_get_app_config(cmd).into(),
                Ok(SessionGetStateCmd(cmd)) => self.session_get_state(cmd).into(),
                Ok(SessionUpdateControllerMulticastListCmd(cmd)) => {
                    self.session_update_controller_multicast_list(cmd)
                }
                Ok(SessionQueryMaxDataSizeInRangingCmd(cmd)) => {
                    self.session_query_max_data_size_in_ranging(cmd).into()
                }
                Ok(SessionUpdateDtTagRangingRoundsCmd(cmd)) => {
                    self.session_update_dt_tag_ranging_rounds(cmd).into()
                }
                Ok(SessionUpdateDtAnchorRangingRoundsCmd(cmd)) => {
                    self.session_update_dt_anchor_ranging_rounds(cmd).into()
                }
                child => command_error_response(
                    GroupId::SessionConfig,
                    cmd.oid.into(),
                    packet_error_status(child),
                )
                .into(),
            },
            SessionControlPacket(cmd) => match cmd.specialize() {
                Ok(SessionStartCmd(cmd)) => self.session_start(cmd),
                Ok(SessionStopCmd(cmd)) => self.session_stop(cmd),
                Ok(SessionGetRangingCountCmd(cmd)) => self.session_get_ranging_count(cmd).into(),
                child => command_error_response(
                    GroupId::SessionControl,
                    cmd.oid.into(),
                    packet_error_status(child),
                )
                .into(),
            },
            AndroidPacket(cmd) => match cmd.specialize() {
                Ok(AndroidSetCountryCodeCmd(cmd)) => self.android_set_country_code(cmd).into(),
                Ok(AndroidGetPowerStatsCmd(cmd)) => self.android_get_power_stats(cmd).into(),
                child => command_error_response(
                    GroupId::VendorAndroid,
                    cmd.oid.into(),
                    packet_error_status(child),
                )
                .into(),
            },
            ControlPacketChild::None => {
                // [UCI] 4.3.2 Exception Handling for Control Messages
                // The UWBS shall respond to an unknown Command (unknown GID
                // or OID) with a Response having the same GID and OID field
                // values as the Command, followed by a Status field with the
                // value of STATUS_UNKNOWN_GID/STATUS_UNKNOWN_OID respectively
                // and no additional fields.
                tracing::error!("Unsupported gid {:?}", cmd.gid);
                command_error_response(
                    cmd.gid,
                    cmd.payload.first().copied().unwrap_or(0),
                    uci::Status::UnknownGid,
                )
                .into()
            }
        };

        output.0
    }

    pub fn receive_packet(&mut self, packet: Vec<u8>) -> Vec<DeviceEffect> {
        let Some(&[mt_gid, oid]) = packet.first_chunk::<2>() else {
            return Vec::new();
        };
        let mt = parse_message_type(mt_gid);
        match mt {
            MessageType::Data => match DataPacket::decode_full(&packet) {
                Ok(packet) => {
                    let notification = self
                        .data_message_snd(packet)
                        .expect("failed to encode data transfer notification");
                    vec![DeviceEffect::send_control(notification)]
                }
                Err(err) => {
                    tracing::error!("failed to parse incoming Data packet: {err}");
                    Vec::new()
                }
            },
            MessageType::Command => {
                match ControlPacket::decode_full(&packet) {
                    // Parsing error. Determine what error response should be
                    // returned to the host:
                    // - response and notifications are ignored, no response
                    // - if the group id is not known, STATUS_UNKNOWN_GID,
                    // - otherwise, and to simplify the code, STATUS_UNKNOWN_OID is
                    //      always returned. That means that malformed commands
                    //      get the same status code, instead of
                    //      STATUS_SYNTAX_ERROR.
                    Err(_) => {
                        let group_id = mt_gid & 0xf;
                        let opcode_id = oid & 0x3f;

                        let status = if GroupId::try_from(group_id).is_ok() {
                            uci::Status::UnknownOid
                        } else {
                            uci::Status::UnknownGid
                        };
                        // The PDL generated code cannot be used to generate
                        // responses with invalid group identifiers.
                        let response = vec![
                            (u8::from(MessageType::Response) << 5) | group_id,
                            opcode_id,
                            0,
                            1,
                            status.into(),
                        ];
                        vec![DeviceEffect::SendPacket(response)]
                    }

                    // Parsing success, ignore non command packets.
                    Ok(cmd) => self.receive_command(cmd),
                }
            }

            // Message types for notifications and responses ignored
            // by the controller.
            _ => {
                tracing::warn!("received unexpected packet of MT {mt:?}");
                Vec::new()
            }
        }
    }
}
