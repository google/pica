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

use super::capabilities::DEFAULT_CAPS_INFO;
use super::{CommandOutput, Device};

const UCI_VERSION: u16 = 0x0002; // Version 2.0
const MAC_VERSION: u16 = 0x0002; // Version 2.0
const PHY_VERSION: u16 = 0x0002; // Version 2.0
const TEST_VERSION: u16 = 0x1001; // Version 1.1

impl Device {
    // The fira norm specify to send a response, then reset, then
    // send a notification once the reset is done
    pub fn core_device_reset(&mut self, cmd: CoreDeviceResetCmd, now: Instant) -> CommandOutput {
        let reset_config = cmd.reset_config;
        tracing::debug!("[{}] DeviceReset", self.handle);
        tracing::debug!("  reset_config={reset_config:?}");

        *self = Device::new(self.handle);
        self.boot_time = Some(now);
        let effects = self.init();

        CommandOutput::with_effects(
            CoreDeviceResetRsp {
                status: uci::Status::Ok,
            },
            effects,
        )
    }

    pub fn core_get_device_info(&self, _cmd: CoreGetDeviceInfoCmd) -> CoreGetDeviceInfoRsp {
        tracing::debug!("[{}] GetDeviceInfo", self.handle);
        CoreGetDeviceInfoRsp {
            status: uci::Status::Ok,
            uci_version: UCI_VERSION,
            mac_version: MAC_VERSION,
            phy_version: PHY_VERSION,
            uci_test_version: TEST_VERSION,
            vendor_spec_info: Vec::new(),
        }
    }

    pub fn core_get_caps_info(&self, _cmd: CoreGetCapsInfoCmd) -> CoreGetCapsInfoRsp {
        tracing::debug!("[{}] GetCapsInfo", self.handle);

        CoreGetCapsInfoRsp {
            status: uci::Status::Ok,
            tlvs: DEFAULT_CAPS_INFO.to_tlvs().into(),
        }
    }

    pub fn core_set_config(&mut self, cmd: CoreSetConfigCmd) -> CoreSetConfigRsp {
        tracing::debug!("[{}] SetConfig", self.handle);
        // UCI 6.3
        if self.state != DeviceState::DeviceStateReady {
            return CoreSetConfigRsp {
                status: uci::Status::Rejected,
                parameters: vec![],
            };
        }

        // [UCI] 6.3.1 Setting the Configuration
        // The UWBS shall respond with CORE_SET_CONFIG_RSP setting the Status
        // field of STATUS_INVALID_PARAM and including one or more invalid
        // Parameter ID(s) If the Host tries to set a parameter that is not
        // available in the UWBS. All other configuration parameters should
        // have been set to the new values within the UWBS.
        let mut invalid_parameters = vec![];
        for parameter in cmd.parameters {
            match parameter.id {
                uci::ConfigParameterId::DeviceState => {
                    invalid_parameters.push(uci::ConfigParameterStatus {
                        id: parameter.id,
                        status: uci::Status::ReadOnly,
                    })
                }
                uci::ConfigParameterId::LowPowerMode => {
                    self.config.low_power_mode = parameter.value.first().copied().unwrap_or(1) != 0;
                }
                uci::ConfigParameterId::Rfu(id) => {
                    tracing::warn!("unknown config parameter id 0x{:02x}", *id);
                    invalid_parameters.push(uci::ConfigParameterStatus {
                        id: parameter.id,
                        status: uci::Status::InvalidParam,
                    })
                }
            }
        }

        CoreSetConfigRsp {
            status: if invalid_parameters.is_empty() {
                uci::Status::Ok
            } else {
                uci::Status::InvalidParam
            },
            parameters: invalid_parameters,
        }
    }

    pub fn core_get_config(&self, cmd: CoreGetConfigCmd) -> CoreGetConfigRsp {
        tracing::debug!("[{}] GetConfig", self.handle);

        // [UCI] 6.3.2 Retrieve the Configuration
        // If the Host tries to retrieve any Parameter(s) that are not available
        // in the UWBS, the UWBS shall respond with a CORE_GET_CONFIG_RSP with
        // a Status field of STATUS_INVALID_PARAM, containing each unavailable
        // Device Configuration Parameter Type with Length field is zero. In
        // this case, the CORE_GET_CONFIG_RSP shall not include any parameter(s)
        // that are available in the UWBS.
        let mut valid_parameters = vec![];
        let mut invalid_parameters = vec![];
        for id in cmd.parameter_ids {
            match id {
                ConfigParameterId::DeviceState => valid_parameters.push(ConfigParameter {
                    id,
                    value: vec![self.state.into()],
                }),
                ConfigParameterId::LowPowerMode => valid_parameters.push(ConfigParameter {
                    id,
                    value: vec![self.config.low_power_mode.into()],
                }),
                ConfigParameterId::Rfu(_) => {
                    invalid_parameters.push(ConfigParameter { id, value: vec![] })
                }
            }
        }

        if invalid_parameters.is_empty() {
            CoreGetConfigRsp {
                status: uci::Status::Ok,
                parameters: valid_parameters,
            }
        } else {
            CoreGetConfigRsp {
                status: uci::Status::InvalidParam,
                parameters: invalid_parameters,
            }
        }
    }

    pub fn core_query_timestamp(&self, now: Instant) -> CoreQueryTimeStampRsp {
        let Some(boot) = self.boot_time else {
            return CoreQueryTimeStampRsp {
                status: uci::Status::Rejected,
                timeStamp: 0,
            };
        };
        CoreQueryTimeStampRsp {
            status: uci::Status::Ok,
            timeStamp: now.saturating_duration_since(boot).as_micros() as u64,
        }
    }
}
