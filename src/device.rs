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
    pub handle: usize,
    config: DeviceConfig,
    /// [UCI] 5. UWBS Device State Machine
    state: DeviceState,
    pub sessions: HashMap<u32, Session>,
    country_code: [u8; 2],
    pub n_active_sessions: usize,
    /// Timestamp set when the device receives the Core Device Reset command.
    /// Receiving any other command while `None` indicates a bad host state.
    boot_time: Option<Instant>,
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
            config: Default::default(),
            state: DeviceState::DeviceStateError, // Will be overwitten
            sessions: Default::default(),
            country_code: Default::default(),
            n_active_sessions: 0,
            boot_time: None,
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

    fn receive_command(&mut self, cmd: ControlPacket, now: Instant) -> Vec<DeviceEffect> {
        use AndroidPacketChild::*;
        use ControlPacketChild::*;
        use CorePacketChild::*;
        use SessionConfigPacketChild::*;
        use SessionControlPacketChild::*;

        // Check whether the first command received is the Core Device
        // Reset command. The controller responds with Device Status
        // Notification with DEVICE_STATE_ERROR otherwise.
        if self.boot_time.is_none() && !cmd.is_core_device_reset_cmd() {
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
                Ok(CoreDeviceResetCmd(cmd)) => self.core_device_reset(cmd, now),
                Ok(CoreGetDeviceInfoCmd(cmd)) => self.core_get_device_info(cmd).into(),
                Ok(CoreGetCapsInfoCmd(cmd)) => self.core_get_caps_info(cmd).into(),
                Ok(CoreSetConfigCmd(cmd)) => self.core_set_config(cmd).into(),
                Ok(CoreGetConfigCmd(cmd)) => self.core_get_config(cmd).into(),
                Ok(CoreQueryTimeStampCmd(_)) => self.core_query_timestamp(now).into(),
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
                Ok(SessionSetAppConfigCmd(cmd)) => self.session_set_app_config(cmd, now),
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
                Ok(SessionStartCmd(cmd)) => self.session_start(cmd, now),
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

    pub fn receive_packet(&mut self, packet: Vec<u8>, now: Instant) -> Vec<DeviceEffect> {
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
                    Ok(cmd) => self.receive_command(cmd, now),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{FiraSessionConfig, fira_session_tlvs};

    const SESSION_ID: u32 = 42;
    const TEST_MODE_SESSION_ID: u32 = 77;
    const MISSING_SESSION_ID: u32 = 99;
    const LOCAL_SHORT_MAC: [u8; 2] = [0x12, 0x34];
    const PEER_SHORT_MAC: [u8; 2] = [0x56, 0x78];
    const EXTRA_CONTROLEE_MAC: [u8; 2] = [0x9A, 0xBC];
    const RANGING_DURATION_MS: u32 = 100;
    const EXPECTED_MAX_DATA_SIZE: u16 = 1020;
    const DEVICE_STATE_READY_BYTE: u8 = 0x01;
    const DEVICE_STATE_ACTIVE_BYTE: u8 = 0x02;
    const LOW_POWER_DISABLED_BYTE: u8 = 0x00;
    const RFU_CONFIG_PARAM_ID: u8 = 0x7F;
    const UCI_RESPONSE_MT: u8 = 0x02;
    const VENDOR_RESERVED_GID: u8 = 0x0F;
    const UNASSIGNED_GID: u8 = 0x04;
    const TEST_OID: u8 = 0x05;
    const VENDOR_RESERVED_GID_CMD: [u8; 4] = [0x2F, 0x05, 0x00, 0x00];
    const UNASSIGNED_GID_CMD: [u8; 4] = [0x24, 0x05, 0x00, 0x00];
    const UCI_STATUS_UNKNOWN_GID: u8 = 0x07;
    const UCI_STATUS_UNKNOWN_OID: u8 = 0x08;
    const SUBSESSION_ID: u32 = 0x0102_0304;
    const SHORT_KEY_SUBSESSION_ID: u32 = 0x1122_3344;
    const SUBSESSION_KEY_16: [u8; 16] = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F,
    ];

    fn send_cmd_at(device: &mut Device, packet: impl Packet, now: Instant) -> Vec<Vec<u8>> {
        device
            .receive_packet(packet.encode_to_vec().unwrap(), now)
            .into_iter()
            .filter_map(|effect| match effect {
                DeviceEffect::SendPacket(bytes) => Some(bytes),
                DeviceEffect::StopControleeRanging { .. } => None,
            })
            .collect()
    }

    fn send_cmd(device: &mut Device, packet: impl Packet) -> Vec<Vec<u8>> {
        send_cmd_at(device, packet, Instant::now())
    }

    fn take_packet<T: Packet>(packets: &mut Vec<Vec<u8>>) -> T {
        let idx = packets
            .iter()
            .position(|bytes| T::decode_full(bytes).is_ok())
            .expect("expected matching UCI packet");
        T::decode_full(&packets.remove(idx)).unwrap()
    }

    fn reset_device() -> Device {
        let mut device = Device::new(0);
        let _ = device.init();
        let packets = send_cmd(
            &mut device,
            CoreDeviceResetCmd {
                reset_config: ResetConfig::UwbsReset,
            },
        );
        assert_eq!(
            CoreDeviceResetRsp::decode_full(&packets[0]).unwrap().status,
            uci::Status::Ok
        );
        assert_eq!(
            CoreDeviceStatusNtf::decode_full(&packets[1])
                .unwrap()
                .device_state,
            DeviceState::DeviceStateReady
        );
        device
    }

    fn init_and_configure(device: &mut Device, session_id: u32, multi_node_mode: MultiNodeMode) {
        let _ = send_cmd(
            device,
            SessionInitCmd {
                session_id,
                session_type: SessionType::FiraRangingSession,
            },
        );
        let tlvs = fira_session_tlvs(&FiraSessionConfig {
            session_id,
            session_type: SessionType::FiraRangingSession,
            role: DeviceRole::Initiator,
            dev_type: DeviceType::Controller,
            mac_mode: MacAddressMode::Mode0,
            multi_node_mode,
            local_mac: &LOCAL_SHORT_MAC,
            dst_mac: Some(&PEER_SHORT_MAC),
            ranging_round_usage: RangingRoundUsage::DsTwrDeferredMode,
            ranging_duration_ms: RANGING_DURATION_MS,
            extra_tlvs: vec![],
        });
        let packets = send_cmd(
            device,
            SessionSetAppConfigCmd {
                session_token: session_id,
                tlvs,
            },
        );
        assert_eq!(
            SessionSetAppConfigRsp::decode_full(&packets[0])
                .unwrap()
                .status,
            uci::Status::Ok
        );
    }

    #[test]
    fn command_before_reset_reports_device_state_error() {
        let mut device = Device::new(0);
        let packets = send_cmd(&mut device, CoreGetDeviceInfoCmd {});
        assert_eq!(
            CoreDeviceStatusNtf::decode_full(&packets[0])
                .unwrap()
                .device_state,
            DeviceState::DeviceStateError
        );
    }

    #[test]
    fn core_query_timestamp_measures_elapsed_microseconds_from_reset() {
        use std::time::Duration;

        let t0 = Instant::now();
        let mut device = Device::new(0);
        let _ = device.init();
        let _ = send_cmd_at(
            &mut device,
            CoreDeviceResetCmd {
                reset_config: ResetConfig::UwbsReset,
            },
            t0,
        );
        let ts1 = CoreQueryTimeStampRsp::decode_full(
            &send_cmd_at(&mut device, CoreQueryTimeStampCmd {}, t0)[0],
        )
        .unwrap();
        let ts2 = CoreQueryTimeStampRsp::decode_full(
            &send_cmd_at(
                &mut device,
                CoreQueryTimeStampCmd {},
                t0 + Duration::from_micros(500),
            )[0],
        )
        .unwrap();
        assert_eq!(ts1.status, uci::Status::Ok);
        assert_eq!(ts1.timeStamp, 0);
        assert_eq!(ts2.status, uci::Status::Ok);
        assert_eq!(ts2.timeStamp, 500);
    }

    #[test]
    fn core_device_reset_resets_timestamp_to_zero() {
        use std::time::Duration;

        let t0 = Instant::now();
        let mut device = Device::new(0);
        let _ = device.init();
        let _ = send_cmd_at(
            &mut device,
            CoreDeviceResetCmd {
                reset_config: ResetConfig::UwbsReset,
            },
            t0,
        );
        let t1 = t0 + Duration::from_micros(500);
        let _ = send_cmd_at(
            &mut device,
            CoreDeviceResetCmd {
                reset_config: ResetConfig::UwbsReset,
            },
            t1,
        );
        let ts = CoreQueryTimeStampRsp::decode_full(
            &send_cmd_at(&mut device, CoreQueryTimeStampCmd {}, t1)[0],
        )
        .unwrap();
        assert_eq!(ts.status, uci::Status::Ok);
        assert_eq!(ts.timeStamp, 0);
    }

    #[test]
    fn core_set_and_get_config_updates_low_power_mode() {
        let mut device = reset_device();
        let set_packets = send_cmd(
            &mut device,
            CoreSetConfigCmd {
                parameters: vec![ConfigParameter {
                    id: ConfigParameterId::LowPowerMode,
                    value: vec![LOW_POWER_DISABLED_BYTE],
                }],
            },
        );
        assert_eq!(
            CoreSetConfigRsp::decode_full(&set_packets[0])
                .unwrap()
                .status,
            uci::Status::Ok
        );

        let get_packets = send_cmd(
            &mut device,
            CoreGetConfigCmd {
                parameter_ids: vec![
                    ConfigParameterId::DeviceState,
                    ConfigParameterId::LowPowerMode,
                ],
            },
        );
        let rsp = CoreGetConfigRsp::decode_full(&get_packets[0]).unwrap();
        assert_eq!(rsp.status, uci::Status::Ok);
        assert_eq!(
            rsp.parameters,
            vec![
                ConfigParameter {
                    id: ConfigParameterId::DeviceState,
                    value: vec![DEVICE_STATE_READY_BYTE],
                },
                ConfigParameter {
                    id: ConfigParameterId::LowPowerMode,
                    value: vec![LOW_POWER_DISABLED_BYTE],
                },
            ]
        );
    }

    #[test]
    fn core_set_config_rejects_read_only_and_rfu_parameters() {
        let mut device = reset_device();
        let rfu = ConfigParameterId::try_from(RFU_CONFIG_PARAM_ID).unwrap();
        let packets = send_cmd(
            &mut device,
            CoreSetConfigCmd {
                parameters: vec![
                    ConfigParameter {
                        id: ConfigParameterId::DeviceState,
                        value: vec![DEVICE_STATE_ACTIVE_BYTE],
                    },
                    ConfigParameter {
                        id: rfu,
                        value: vec![0x01],
                    },
                ],
            },
        );
        let rsp = CoreSetConfigRsp::decode_full(&packets[0]).unwrap();
        assert_eq!(rsp.status, uci::Status::InvalidParam);
        assert_eq!(
            rsp.parameters,
            vec![
                ConfigParameterStatus {
                    id: ConfigParameterId::DeviceState,
                    status: uci::Status::ReadOnly,
                },
                ConfigParameterStatus {
                    id: rfu,
                    status: uci::Status::InvalidParam,
                },
            ]
        );
    }

    #[test]
    fn core_get_config_for_rfu_param_omits_valid_parameters() {
        let mut device = reset_device();
        let rfu = ConfigParameterId::try_from(RFU_CONFIG_PARAM_ID).unwrap();
        let packets = send_cmd(
            &mut device,
            CoreGetConfigCmd {
                parameter_ids: vec![ConfigParameterId::DeviceState, rfu],
            },
        );
        let rsp = CoreGetConfigRsp::decode_full(&packets[0]).unwrap();
        assert_eq!(rsp.status, uci::Status::InvalidParam);
        assert_eq!(
            rsp.parameters,
            vec![ConfigParameter {
                id: rfu,
                value: vec![],
            }]
        );
    }

    #[test]
    fn core_set_config_rejected_while_device_active() {
        let mut device = reset_device();
        init_and_configure(&mut device, SESSION_ID, MultiNodeMode::OneToOne);
        let _ = send_cmd(
            &mut device,
            SessionStartCmd {
                session_id: SESSION_ID,
            },
        );

        let packets = send_cmd(
            &mut device,
            CoreSetConfigCmd {
                parameters: vec![ConfigParameter {
                    id: ConfigParameterId::LowPowerMode,
                    value: vec![LOW_POWER_DISABLED_BYTE],
                }],
            },
        );
        assert_eq!(
            CoreSetConfigRsp::decode_full(&packets[0]).unwrap().status,
            uci::Status::Rejected
        );
    }

    #[test]
    fn receive_packet_ignores_packets_shorter_than_header() {
        let mut device = reset_device();
        assert!(device.receive_packet(vec![0x20], Instant::now()).is_empty());
    }

    #[test]
    fn receive_packet_ignores_non_command_packets() {
        let mut device = reset_device();
        for mt in [MessageType::Response, MessageType::Notification] {
            let pkt = [u8::from(mt) << 5, 0x02, 0x00, 0x00];
            assert!(
                device
                    .receive_packet(pkt.to_vec(), Instant::now())
                    .is_empty(),
                "expected {mt:?} frame to be ignored"
            );
        }
    }

    #[test]
    fn receive_command_rejects_vendor_reserved_gid() {
        let mut device = reset_device();
        let effects = device.receive_packet(VENDOR_RESERVED_GID_CMD.to_vec(), Instant::now());
        assert_eq!(effects.len(), 1);
        let DeviceEffect::SendPacket(bytes) = &effects[0] else {
            panic!("expected SendPacket");
        };
        assert_eq!(bytes.len(), 5);
        assert_eq!(bytes[0] >> 5, UCI_RESPONSE_MT);
        assert_eq!(bytes[0] & 0x0F, VENDOR_RESERVED_GID);
        assert_eq!(bytes[1] & 0x3F, TEST_OID);
        assert_eq!(bytes[4], UCI_STATUS_UNKNOWN_GID);
    }

    #[test]
    fn receive_packet_rejects_unassigned_gid() {
        let mut device = reset_device();
        let effects = device.receive_packet(UNASSIGNED_GID_CMD.to_vec(), Instant::now());
        assert_eq!(effects.len(), 1);
        let DeviceEffect::SendPacket(bytes) = &effects[0] else {
            panic!("expected SendPacket");
        };
        assert_eq!(bytes.len(), 5);
        assert_eq!(bytes[0] >> 5, UCI_RESPONSE_MT);
        assert_eq!(bytes[0] & 0x0F, UNASSIGNED_GID);
        assert_eq!(bytes[1] & 0x3F, TEST_OID);
        assert_eq!(bytes[4], UCI_STATUS_UNKNOWN_GID);
    }

    #[test]
    fn session_init_rejects_duplicate_session_id() {
        let mut device = reset_device();
        let first = send_cmd(
            &mut device,
            SessionInitCmd {
                session_id: SESSION_ID,
                session_type: SessionType::FiraRangingSession,
            },
        );
        assert_eq!(
            SessionInitRsp::decode_full(&first[0]).unwrap().status,
            uci::Status::Ok
        );

        let second = send_cmd(
            &mut device,
            SessionInitCmd {
                session_id: SESSION_ID,
                session_type: SessionType::FiraRangingSession,
            },
        );
        assert_eq!(
            SessionInitRsp::decode_full(&second[0]).unwrap().status,
            uci::Status::ErrorSessionDuplicate
        );
    }

    #[test]
    fn session_set_app_config_rejects_aoa_result_req_while_active() {
        let mut device = reset_device();
        init_and_configure(&mut device, SESSION_ID, MultiNodeMode::OneToOne);
        let _ = send_cmd(
            &mut device,
            SessionStartCmd {
                session_id: SESSION_ID,
            },
        );

        let packets = send_cmd(
            &mut device,
            SessionSetAppConfigCmd {
                session_token: SESSION_ID,
                tlvs: vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::AoaResultReq,
                    v: vec![AoaResultReq::AoaDisabled as u8],
                }],
            },
        );
        assert_eq!(
            SessionSetAppConfigRsp::decode_full(&packets[0])
                .unwrap()
                .status,
            uci::Status::ErrorSessionActive
        );
    }

    #[test]
    fn session_deinit_while_active_transitions_device_to_ready() {
        let mut device = reset_device();
        init_and_configure(&mut device, SESSION_ID, MultiNodeMode::OneToOne);
        let _ = send_cmd(
            &mut device,
            SessionStartCmd {
                session_id: SESSION_ID,
            },
        );

        let mut packets = send_cmd(
            &mut device,
            SessionDeinitCmd {
                session_token: SESSION_ID,
            },
        );
        assert_eq!(
            SessionDeinitRsp::decode_full(&packets[0]).unwrap().status,
            uci::Status::Ok
        );
        let mut notifications = packets.split_off(1);
        assert_eq!(
            take_packet::<SessionStatusNtf>(&mut notifications).session_state,
            SessionState::SessionStateDeinit
        );
        assert_eq!(
            take_packet::<CoreDeviceStatusNtf>(&mut notifications).device_state,
            DeviceState::DeviceStateReady
        );
        assert!(notifications.is_empty());
    }

    #[test]
    fn session_set_app_config_rejects_missing_mandatory_params() {
        let mut device = reset_device();
        let _ = send_cmd(
            &mut device,
            SessionInitCmd {
                session_id: SESSION_ID,
                session_type: SessionType::FiraRangingSession,
            },
        );
        let packets = send_cmd(
            &mut device,
            SessionSetAppConfigCmd {
                session_token: SESSION_ID,
                tlvs: vec![AppConfigTlv {
                    cfg_id: AppConfigTlvType::DeviceRole,
                    v: vec![DeviceRole::Initiator as u8],
                }],
            },
        );
        assert_eq!(
            SessionSetAppConfigRsp::decode_full(&packets[0])
                .unwrap()
                .status,
            uci::Status::Rejected
        );
    }

    #[test]
    fn device_test_mode_rejects_set_app_config() {
        let mut device = reset_device();
        let _ = send_cmd(
            &mut device,
            SessionInitCmd {
                session_id: TEST_MODE_SESSION_ID,
                session_type: SessionType::DeviceTestMode,
            },
        );
        let packets = send_cmd(
            &mut device,
            SessionSetAppConfigCmd {
                session_token: TEST_MODE_SESSION_ID,
                tlvs: vec![],
            },
        );
        assert_eq!(
            SessionSetAppConfigRsp::decode_full(&packets[0])
                .unwrap()
                .status,
            uci::Status::Rejected
        );
    }

    #[test]
    fn update_dt_tag_rounds_on_missing_session_returns_not_exist() {
        let mut device = reset_device();
        let packets = send_cmd(
            &mut device,
            SessionUpdateDtTagRangingRoundsCmd {
                session_token: MISSING_SESSION_ID,
                ranging_round_indexes: vec![],
            },
        );
        assert_eq!(
            SessionUpdateDtTagRangingRoundsRsp::decode_full(&packets[0])
                .unwrap()
                .status,
            uci::Status::ErrorSessionNotExist
        );
    }

    #[test]
    fn unknown_session_config_oid_returns_unknown_oid() {
        let mut device = reset_device();
        let packets = send_cmd(
            &mut device,
            SessionConfigPacket {
                mt: MessageType::Command,
                oid: SessionConfigOpcodeId::Status,
                payload: vec![],
            },
        );
        let rsp = SessionConfigPacket::decode_full(&packets[0]).unwrap();
        assert_eq!(rsp.mt, MessageType::Response);
        assert_eq!(rsp.oid, SessionConfigOpcodeId::Status);
        assert_eq!(rsp.payload, vec![UCI_STATUS_UNKNOWN_OID]);
    }

    #[test]
    fn session_get_app_config_for_unset_param_returns_invalid_param() {
        let mut device = reset_device();
        let _ = send_cmd(
            &mut device,
            SessionInitCmd {
                session_id: SESSION_ID,
                session_type: SessionType::FiraRangingSession,
            },
        );
        let packets = send_cmd(
            &mut device,
            SessionGetAppConfigCmd {
                session_token: SESSION_ID,
                app_cfg: vec![AppConfigTlvType::DeviceRole],
            },
        );
        let get_rsp = SessionGetAppConfigRsp::decode_full(&packets[0]).unwrap();
        assert_eq!(get_rsp.status, uci::Status::InvalidParam);
        assert_eq!(
            get_rsp.tlvs,
            vec![AppConfigTlv {
                cfg_id: AppConfigTlvType::DeviceRole,
                v: vec![],
            }]
        );
    }

    #[test]
    fn session_query_max_data_size_returns_1020() {
        let mut device = reset_device();
        init_and_configure(&mut device, SESSION_ID, MultiNodeMode::OneToOne);
        let packets = send_cmd(
            &mut device,
            SessionQueryMaxDataSizeInRangingCmd {
                session_token: SESSION_ID,
            },
        );
        let max_data = SessionQueryMaxDataSizeInRangingRsp::decode_full(&packets[0]).unwrap();
        assert_eq!(max_data.status, uci::Status::Ok);
        assert_eq!(max_data.max_data_size, EXPECTED_MAX_DATA_SIZE);
    }

    #[test]
    fn session_stop_when_idle_returns_rejected() {
        let mut device = reset_device();
        init_and_configure(&mut device, SESSION_ID, MultiNodeMode::OneToOne);
        let packets = send_cmd(
            &mut device,
            SessionStopCmd {
                session_id: SESSION_ID,
            },
        );
        assert_eq!(
            SessionStopRsp::decode_full(&packets[0]).unwrap().status,
            uci::Status::Rejected
        );
    }

    #[test]
    fn multicast_remove_last_controlee_deactivates_active_session() {
        let mut device = reset_device();
        init_and_configure(&mut device, SESSION_ID, MultiNodeMode::OneToMany);
        let _ = send_cmd(
            &mut device,
            SessionStartCmd {
                session_id: SESSION_ID,
            },
        );

        let mut packets = send_cmd(
            &mut device,
            SessionUpdateControllerMulticastListCmd {
                session_token: SESSION_ID,
                action: UpdateMulticastListAction::RemoveControlee,
                payload: SessionUpdateControllerMulticastListCmdPayload {
                    controlees: vec![uci::Controlee {
                        short_address: PEER_SHORT_MAC,
                        subsession_id: SUBSESSION_ID,
                    }],
                }
                .encode_to_vec()
                .unwrap(),
            },
        );

        let rsp = SessionUpdateControllerMulticastListRsp::decode_full(&packets[0]).unwrap();
        assert_eq!(rsp.status, uci::Status::Ok);
        let mut notifications = packets.split_off(1);
        let s_ntf: SessionStatusNtf = take_packet(&mut notifications);
        assert_eq!(s_ntf.session_state, SessionState::SessionStateIdle);
        assert_eq!(s_ntf.reason_code, ReasonCode::ErrorInvalidNumOfControlees);
        let m_ntf: SessionUpdateControllerMulticastListNtf = take_packet(&mut notifications);
        assert_eq!(m_ntf.session_token, SESSION_ID);
        let d_ntf: CoreDeviceStatusNtf = take_packet(&mut notifications);
        assert_eq!(d_ntf.device_state, DeviceState::DeviceStateReady);
        assert!(notifications.is_empty());
    }

    #[test]
    fn multicast_subsession_key_requires_provisioned_sts_config() {
        let mut device = reset_device();
        init_and_configure(&mut device, SESSION_ID, MultiNodeMode::OneToMany);

        let packets = send_cmd(
            &mut device,
            SessionUpdateControllerMulticastListCmd {
                session_token: SESSION_ID,
                action: UpdateMulticastListAction::AddControleeWithShortSubSessionKey,
                payload: SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload {
                    controlees: vec![Controlee_V2_0_16_Byte_Version {
                        short_address: EXTRA_CONTROLEE_MAC,
                        subsession_id: SHORT_KEY_SUBSESSION_ID,
                        subsession_key: SUBSESSION_KEY_16,
                    }],
                }
                .encode_to_vec()
                .unwrap(),
            },
        );
        let rsp = SessionUpdateControllerMulticastListRsp::decode_full(&packets[0]).unwrap();
        assert_eq!(rsp.status, uci::Status::Failed);
        let ntf = SessionUpdateControllerMulticastListNtf::decode_full(&packets[1]).unwrap();
        assert_eq!(
            ntf.controlee_status,
            vec![ControleeStatus {
                mac_address: EXTRA_CONTROLEE_MAC,
                status: MulticastUpdateStatus::ErrorSubSessionKeyNotApplicable,
            }]
        );
    }
}
