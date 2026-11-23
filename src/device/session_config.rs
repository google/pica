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

use std::collections::hash_map::Entry;
use std::time::Instant;

use pdl_runtime::Packet;

use crate::MacAddress;
use crate::app_config::SubSessionKey;
use crate::packets::uci::{self, *};
use crate::session::Session;

use super::{CommandOutput, Device, DeviceEffect, MAX_NUMBER_OF_CONTROLEES, MAX_SESSION};

impl Device {
    pub fn session_init(&mut self, cmd: SessionInitCmd) -> CommandOutput {
        let session_id = cmd.session_id;
        let session_type = cmd.session_type;

        tracing::debug!("[{}] Session init", self.handle);
        tracing::debug!("  session_id=0x{session_id:x}");
        tracing::debug!("  session_type={session_type:?}");

        if self.sessions.len() >= MAX_SESSION {
            return SessionInitRsp {
                status: uci::Status::ErrorMaxSessionsExceeded,
            }
            .into();
        }

        match self.sessions.entry(session_id) {
            Entry::Occupied(_) => SessionInitRsp {
                status: uci::Status::ErrorSessionDuplicate,
            }
            .into(),
            Entry::Vacant(entry) => {
                let mut session = Session::new(session_id, session_type, self.handle);
                let effects = session.init().into_iter().map(DeviceEffect::send_control);
                entry.insert(session);
                CommandOutput::with_effects(
                    SessionInitRsp {
                        status: uci::Status::Ok,
                    },
                    effects,
                )
            }
        }
    }

    pub fn session_deinit(&mut self, cmd: SessionDeinitCmd) -> CommandOutput {
        let session_id = cmd.session_token;
        tracing::debug!("[{}] Session deinit", self.handle);
        tracing::debug!("  session_id=0x{session_id:x}");

        let Some(mut session) = self.sessions.remove(&session_id) else {
            return SessionDeinitRsp {
                status: uci::Status::ErrorSessionNotExist,
            }
            .into();
        };

        let was_active = session.state == SessionState::SessionStateActive;
        let session_ntf = session
            .set_state(
                SessionState::SessionStateDeinit,
                ReasonCode::StateChangeWithSessionManagementCommands,
            )
            .map(DeviceEffect::send_control);
        let device_ntf = if was_active {
            self.on_session_deactivated()
                .map(DeviceEffect::send_control)
        } else {
            None
        };
        let effects = session_ntf.into_iter().chain(device_ntf);

        CommandOutput::with_effects(
            SessionDeinitRsp {
                status: uci::Status::Ok,
            },
            effects,
        )
    }

    pub fn session_get_count(&self, _cmd: SessionGetCountCmd) -> SessionGetCountRsp {
        tracing::debug!("[{}] Session get count", self.handle);

        SessionGetCountRsp {
            status: uci::Status::Ok,
            session_count: self.sessions.len() as u8,
        }
    }

    pub fn session_query_max_data_size_in_ranging(
        &self,
        cmd: SessionQueryMaxDataSizeInRangingCmd,
    ) -> SessionQueryMaxDataSizeInRangingRsp {
        if self.session(cmd.session_token).is_some() {
            SessionQueryMaxDataSizeInRangingRsp {
                session_token: cmd.session_token,
                status: uci::Status::Ok,
                max_data_size: MAX_DATA_PACKET_PAYLOAD_SIZE as u16,
            }
        } else {
            SessionQueryMaxDataSizeInRangingRsp {
                session_token: cmd.session_token,
                status: uci::Status::ErrorSessionNotExist,
                max_data_size: 0,
            }
        }
    }

    pub fn session_set_app_config(
        &mut self,
        cmd: SessionSetAppConfigCmd,
        now: Instant,
    ) -> CommandOutput {
        let session_handle = cmd.session_token;

        tracing::debug!(
            "[{}:0x{session_handle:x}] Session Set App Config",
            self.handle
        );

        let Some(session) = self.sessions.get_mut(&session_handle) else {
            return SessionSetAppConfigRsp {
                cfg_status: Vec::new(),
                status: uci::Status::ErrorSessionNotExist,
            }
            .into();
        };

        if !session.is_supported_session_type() {
            return SessionSetAppConfigRsp {
                status: uci::Status::Rejected,
                cfg_status: vec![],
            }
            .into();
        }

        if session.state == SessionState::SessionStateActive {
            const IMMUTABLE_PARAMETERS: &[AppConfigTlvType] = &[AppConfigTlvType::AoaResultReq];
            if cmd
                .tlvs
                .iter()
                .any(|cfg| IMMUTABLE_PARAMETERS.contains(&cfg.cfg_id))
            {
                return SessionSetAppConfigRsp {
                    status: uci::Status::ErrorSessionActive,
                    cfg_status: vec![],
                }
                .into();
            }
        }

        if !matches!(
            session.state,
            SessionState::SessionStateInit
                | SessionState::SessionStateIdle
                | SessionState::SessionStateActive
        ) {
            return SessionSetAppConfigRsp {
                status: uci::Status::Rejected,
                cfg_status: Vec::new(),
            }
            .into();
        }

        let mut app_config = session.app_config.clone();
        let mut invalid_parameters = vec![];
        // Defer DeviceMacAddress and DstMacAddress so MacAddressMode and
        // NumberOfControlees take effect first regardless of TLV order in the command.
        let (deferred_tlvs, immediate_tlvs): (Vec<_>, Vec<_>) =
            cmd.tlvs.into_iter().partition(|cfg| {
                matches!(
                    cfg.cfg_id,
                    uci::AppConfigTlvType::DeviceMacAddress | uci::AppConfigTlvType::DstMacAddress
                )
            });
        for cfg in immediate_tlvs.into_iter().chain(deferred_tlvs) {
            match app_config.set(cfg.cfg_id, &cfg.v) {
                Ok(_) => (),
                Err(_) => invalid_parameters.push(AppConfigStatus {
                    cfg_id: cfg.cfg_id,
                    status: uci::Status::InvalidParam,
                }),
            }
        }

        if !app_config.has_mandatory_parameters() {
            tracing::error!(
                "[{}:0x{session_handle:x}] missing mandatory APP config parameters",
                self.handle
            );
            return SessionSetAppConfigRsp {
                status: uci::Status::Rejected,
                cfg_status: vec![],
            }
            .into();
        }

        if invalid_parameters.is_empty() {
            session.set_app_config(app_config, now);
            let effects = if session.state == SessionState::SessionStateInit {
                session
                    .set_state(
                        SessionState::SessionStateIdle,
                        ReasonCode::StateChangeWithSessionManagementCommands,
                    )
                    .map(DeviceEffect::send_control)
            } else {
                None
            };
            CommandOutput::with_effects(
                SessionSetAppConfigRsp {
                    status: uci::Status::Ok,
                    cfg_status: invalid_parameters,
                },
                effects,
            )
        } else {
            SessionSetAppConfigRsp {
                status: uci::Status::InvalidParam,
                cfg_status: invalid_parameters,
            }
            .into()
        }
    }

    pub fn session_get_app_config(&self, cmd: SessionGetAppConfigCmd) -> SessionGetAppConfigRsp {
        let session_handle = cmd.session_token;

        tracing::debug!(
            "[{}:0x{session_handle:x}] Session Get App Config",
            self.handle
        );

        let Some(session) = self.sessions.get(&session_handle) else {
            return SessionGetAppConfigRsp {
                tlvs: vec![],
                status: uci::Status::ErrorSessionNotExist,
            };
        };

        let (status, tlvs) = if cmd.app_cfg.is_empty() {
            (uci::Status::Ok, session.app_config.to_tlvs())
        } else {
            let mut valid_parameters = vec![];
            let mut invalid_parameters = vec![];
            for id in cmd.app_cfg {
                match session.app_config.get(id) {
                    Some(value) => valid_parameters.push(AppConfigTlv {
                        cfg_id: id,
                        v: value.to_vec(),
                    }),
                    None => invalid_parameters.push(AppConfigTlv {
                        cfg_id: id,
                        v: vec![],
                    }),
                }
            }

            if invalid_parameters.is_empty() {
                (uci::Status::Ok, valid_parameters)
            } else {
                (uci::Status::InvalidParam, invalid_parameters)
            }
        };

        SessionGetAppConfigRsp { status, tlvs }
    }

    pub fn session_get_state(&self, cmd: SessionGetStateCmd) -> SessionGetStateRsp {
        let session_handle = cmd.session_token;

        tracing::debug!("[{}:0x{session_handle:x}] Session Get State", self.handle);

        let Some(session) = self.sessions.get(&session_handle) else {
            return SessionGetStateRsp {
                session_state: SessionState::SessionStateInit,
                status: uci::Status::ErrorSessionNotExist,
            };
        };

        SessionGetStateRsp {
            status: uci::Status::Ok,
            session_state: session.state,
        }
    }

    pub fn session_update_controller_multicast_list(
        &mut self,
        cmd: SessionUpdateControllerMulticastListCmd,
    ) -> CommandOutput {
        let session_handle = cmd.session_token;

        tracing::debug!(
            "[{}:0x{session_handle:x}] Session Update Controller Multicast List",
            self.handle
        );

        let Some(session) = self.sessions.get_mut(&session_handle) else {
            return SessionUpdateControllerMulticastListRsp {
                status: uci::Status::ErrorSessionNotExist,
                controlee_status: vec![],
            }
            .into();
        };

        if (session.state != SessionState::SessionStateActive
            && session.state != SessionState::SessionStateIdle)
            || session.app_config.device_type() != Some(DeviceType::Controller)
            || session.app_config.multi_node_mode() != Some(MultiNodeMode::OneToMany)
        {
            return SessionUpdateControllerMulticastListRsp {
                status: uci::Status::Rejected,
                controlee_status: vec![],
            }
            .into();
        }
        let action = cmd.action;
        let mut dst_addresses: Vec<MacAddress> = session.app_config.dst_mac_address().collect();
        let new_controlees: Vec<Controlee> = match action {
            UpdateMulticastListAction::AddControlee
            | UpdateMulticastListAction::RemoveControlee => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmdPayload::decode_full(&cmd.payload)
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRsp {
                        status: uci::Status::SyntaxError,
                        controlee_status: vec![],
                    }
                    .into();
                }
            }
            UpdateMulticastListAction::AddControleeWithShortSubSessionKey => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload::decode_full(
                        &cmd.payload,
                    )
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRsp {
                        status: uci::Status::SyntaxError,
                        controlee_status: vec![],
                    }
                    .into();
                }
            }
            UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey => {
                if let Ok(packet) =
                    SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload::decode_full(
                        &cmd.payload,
                    )
                {
                    packet
                        .controlees
                        .iter()
                        .map(|controlee| controlee.into())
                        .collect()
                } else {
                    return SessionUpdateControllerMulticastListRsp {
                        status: uci::Status::SyntaxError,
                        controlee_status: vec![],
                    }
                    .into();
                }
            }
        };
        let mut controlee_status_ntf = Vec::new();
        let mut controlee_status_rsp = Vec::new();
        let mut status = uci::Status::Ok;
        let mut effects = Vec::new();

        match action {
            UpdateMulticastListAction::AddControlee
            | UpdateMulticastListAction::AddControleeWithShortSubSessionKey
            | UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey => {
                for controlee in new_controlees {
                    let mut update_status = MulticastUpdateStatus::OkMulticastListUpdate;
                    if (action == UpdateMulticastListAction::AddControleeWithShortSubSessionKey
                        || action
                            == UpdateMulticastListAction::AddControleeWithExtendedSubSessionKey)
                        && session.app_config.sts_config()
                            != uci::StsConfig::ProvisionedForResponderSubSessionKey
                    {
                        // If Action is 0x02 or 0x03 for STS_CONFIG values other than
                        // 0x04, the UWBS shall return SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF
                        // with Status set to STATUS_ERROR_SUB_SESSION_KEY_NOT_APPLICABLE for each
                        // Controlee in the Controlee List.
                        status = uci::Status::Failed;
                        update_status = MulticastUpdateStatus::ErrorSubSessionKeyNotApplicable;
                        controlee_status_ntf.push(ControleeStatus {
                            mac_address: controlee.short_address,
                            status: update_status,
                        });
                    } else {
                        let address = MacAddress::Short(controlee.short_address);
                        if !dst_addresses.contains(&address) {
                            if dst_addresses.len() == MAX_NUMBER_OF_CONTROLEES {
                                status = uci::Status::ErrorMulticastListFull;
                                update_status = MulticastUpdateStatus::ErrorMulticastListFull;
                            } else {
                                dst_addresses.push(address);
                            };
                        } else {
                            status = uci::Status::Failed;
                            update_status = MulticastUpdateStatus::ErrorAddressAlreadyPresent;
                        }
                        controlee_status_rsp.push(ControleeStatus {
                            mac_address: controlee.short_address,
                            status: update_status,
                        });
                    }
                }
            }
            UpdateMulticastListAction::RemoveControlee => {
                for controlee in new_controlees {
                    let address = MacAddress::Short(controlee.short_address);
                    let attempt_count = session.app_config.in_band_termination_attempt_count();
                    let mut update_status = MulticastUpdateStatus::OkMulticastListUpdate;
                    if !dst_addresses.contains(&address) {
                        status = uci::Status::Failed;
                        update_status = MulticastUpdateStatus::ErrorAddressNotFound;
                    } else {
                        dst_addresses.retain(|value| *value != address);
                        // If IN_BAND_TERMINATION_ATTEMPT_COUNT is not equal to 0x00, then the
                        // UWBS shall transmit the RCM with the “Stop Ranging” bit set to ‘1’
                        // for IN_BAND_TERMINATION_ATTEMPT_COUNT times to the corresponding
                        // Controlee.
                        if attempt_count != 0 {
                            effects.push(DeviceEffect::StopControleeRanging {
                                mac_address: address,
                                session_id: session_handle,
                            });
                        }
                        // Following requirements are applicable when the action is set to Delete (Action field set to 0x01):
                        // When the command is received while the Session State is SESSION_STATE_ACTIVE,
                        // For requested Controlees present in the multicast list,
                        // UWBS shall send the SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF and the
                        // corresponding Controlee status shall be set to STATUS_OK_MULTICAST_LIST_UPDATE
                        // in the Status List of SESSION_UPDATE_CONTROLLER_MULTICAST_LIST_NTF.
                        if session.state == SessionState::SessionStateActive {
                            controlee_status_ntf.push(ControleeStatus {
                                mac_address: controlee.short_address,
                                status: update_status,
                            });
                        }
                    }
                    controlee_status_rsp.push(ControleeStatus {
                        mac_address: controlee.short_address,
                        status: update_status,
                    });
                }
            }
        }

        session
            .app_config
            .set_number_of_controlees(dst_addresses.len() as u8);
        session.app_config.set_dst_mac_address(&dst_addresses);

        // If the multicast list becomes empty, the UWBS shall move the session to
        // SESSION_STATE_IDLE by sending the SESSION_STATUS_NTF with Reason Code
        // set to ERROR_INVALID_NUM_OF_CONTROLEES.
        let deactivated =
            dst_addresses.is_empty() && session.state == SessionState::SessionStateActive;
        if deactivated {
            session.stop_ranging();
            effects.extend(
                session
                    .set_state(
                        SessionState::SessionStateIdle,
                        ReasonCode::ErrorInvalidNumOfControlees,
                    )
                    .map(DeviceEffect::send_control),
            );
        }

        if !controlee_status_ntf.is_empty() {
            effects.push(DeviceEffect::send_control(
                SessionUpdateControllerMulticastListNtf {
                    controlee_status: controlee_status_ntf,
                    session_token: session_handle,
                },
            ));
        }

        if deactivated {
            effects.extend(
                self.on_session_deactivated()
                    .map(DeviceEffect::send_control),
            );
        }

        CommandOutput::with_effects(
            SessionUpdateControllerMulticastListRsp {
                status,
                controlee_status: controlee_status_rsp,
            },
            effects,
        )
    }
}

struct Controlee {
    short_address: [u8; 2],
    #[allow(dead_code)]
    sub_session_id: u32,
    #[allow(dead_code)]
    session_key: SubSessionKey,
}

impl From<&uci::Controlee> for Controlee {
    fn from(value: &uci::Controlee) -> Self {
        Controlee {
            short_address: value.short_address,
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::None,
        }
    }
}

impl From<&uci::Controlee_V2_0_16_Byte_Version> for Controlee {
    fn from(value: &uci::Controlee_V2_0_16_Byte_Version) -> Self {
        Controlee {
            short_address: value.short_address,
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::Short(value.subsession_key),
        }
    }
}

impl From<&uci::Controlee_V2_0_32_Byte_Version> for Controlee {
    fn from(value: &uci::Controlee_V2_0_32_Byte_Version) -> Self {
        Controlee {
            short_address: value.short_address,
            sub_session_id: value.subsession_id,
            session_key: SubSessionKey::Extended(value.subsession_key),
        }
    }
}
