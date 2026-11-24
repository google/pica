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

use crate::packets::uci::{self, *};

use super::Device;

impl Device {
    pub fn android_set_country_code(
        &mut self,
        cmd: AndroidSetCountryCodeCmd,
    ) -> AndroidSetCountryCodeRsp {
        tracing::debug!("[{}] Set country code", self.handle);
        tracing::debug!(
            "  country_code={},{}",
            char::from(cmd.country_code[0]),
            char::from(cmd.country_code[1])
        );

        self.country_code = cmd.country_code;
        AndroidSetCountryCodeRsp {
            status: uci::Status::Ok,
        }
    }

    pub fn android_get_power_stats(
        &mut self,
        _cmd: AndroidGetPowerStatsCmd,
    ) -> AndroidGetPowerStatsRsp {
        tracing::debug!("[{}] Get power stats", self.handle);

        // TODO
        AndroidGetPowerStatsRsp {
            stats: PowerStats {
                status: uci::Status::Ok,
                idle_time_ms: 0,
                tx_time_ms: 0,
                rx_time_ms: 0,
                total_wake_count: 0,
            },
        }
    }
}
