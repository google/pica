# Copyright 2023 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from pica import Host
from pica.packets import uci


async def init(host: Host):
    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    host.send_control(uci.CoreDeviceResetCmd(reset_config=uci.ResetConfig.UWBS_RESET))

    await host.expect_control(uci.CoreDeviceResetRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )
