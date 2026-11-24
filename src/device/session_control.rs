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

use std::time::Instant;

use crate::packets::uci::{self, *};

use super::{CommandOutput, Device, DeviceEffect};

impl Device {
    pub fn session_start(&mut self, cmd: SessionStartCmd, now: Instant) -> CommandOutput {
        let session_id = cmd.session_id;

        tracing::debug!("[{}:0x{session_id:x}] Session Start", self.handle);

        let Some(session) = self.sessions.get_mut(&session_id) else {
            return SessionStartRsp {
                status: uci::Status::ErrorSessionNotExist,
            }
            .into();
        };

        if session.state != SessionState::SessionStateIdle {
            return SessionStartRsp {
                status: uci::Status::ErrorSessionNotConfigured,
            }
            .into();
        }

        if let Some(reason_code) = session.invalid_start_reason() {
            return CommandOutput::with_effects(
                SessionStartRsp {
                    status: uci::Status::Ok,
                },
                [DeviceEffect::send_control(SessionStatusNtf {
                    session_token: session_id,
                    session_state: SessionState::SessionStateIdle,
                    reason_code,
                })],
            );
        }

        session.start_ranging(now);

        let session_ntf = session
            .set_state(
                SessionState::SessionStateActive,
                ReasonCode::StateChangeWithSessionManagementCommands,
            )
            .map(DeviceEffect::send_control);

        self.n_active_sessions += 1;
        let device_ntf = self
            .set_state(DeviceState::DeviceStateActive)
            .map(DeviceEffect::send_control);

        let effects = session_ntf.into_iter().chain(device_ntf);

        CommandOutput::with_effects(
            SessionStartRsp {
                status: uci::Status::Ok,
            },
            effects,
        )
    }

    pub fn session_stop(&mut self, cmd: SessionStopCmd) -> CommandOutput {
        let session_id = cmd.session_id;

        tracing::debug!("[{}:0x{session_id:x}] Session Stop", self.handle);

        let Some(session) = self.sessions.get_mut(&session_id) else {
            return SessionStopRsp {
                status: uci::Status::ErrorSessionNotExist,
            }
            .into();
        };

        if session.state != SessionState::SessionStateActive {
            return SessionStopRsp {
                status: uci::Status::Rejected,
            }
            .into();
        }

        let effects = self.stop_active_session(
            session_id,
            ReasonCode::StateChangeWithSessionManagementCommands,
        );

        CommandOutput::with_effects(
            SessionStopRsp {
                status: uci::Status::Ok,
            },
            effects,
        )
    }

    pub fn session_get_ranging_count(
        &self,
        cmd: SessionGetRangingCountCmd,
    ) -> SessionGetRangingCountRsp {
        let session_id = cmd.session_id;

        tracing::debug!(
            "[{}:0x{session_id:x}] Session Get Ranging Count",
            self.handle
        );

        let Some(session) = self.sessions.get(&session_id) else {
            return SessionGetRangingCountRsp {
                status: uci::Status::ErrorSessionNotExist,
                count: 0,
            };
        };

        SessionGetRangingCountRsp {
            status: uci::Status::Ok,
            count: session.ranging_count,
        }
    }
}
