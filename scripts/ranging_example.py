#!/usr/bin/env python3

# Copyright 2022 Google LLC
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

import asyncio
import argparse

from pica import Host
from pica.packets import uci
from helper import init

async def controller(host: Host, peer: Host):
    await init(host)

    host.send_control(
        uci.SessionInitCmd(
            session_id=0,
            session_type=uci.SessionType.FIRA_RANGING_SESSION))

    await host.expect_control(
        uci.SessionInitRsp(
            status=uci.StatusCode.UCI_STATUS_OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0))

    mac_address_mode = 0x0
    ranging_duration = int(1000).to_bytes(4, byteorder='little')
    device_role_initiator = bytes([0])
    device_type_controller = bytes([1])
    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=device_role_initiator),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                    v=device_type_controller),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_MAC_ADDRESS,
                    v=host.mac_address),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MAC_ADDRESS_MODE,
                    v=bytes([mac_address_mode])),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
                    v=ranging_duration),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.NO_OF_CONTROLEE,
                    v=bytes([1])),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS,
                    v=peer.mac_address),
            ]))

    await host.expect_control(
        uci.SessionSetAppConfigRsp(
            status=uci.StatusCode.UCI_STATUS_OK,
            cfg_status=[]))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0))

    host.send_control(
        uci.SessionStartCmd(
            session_id=0))

    await host.expect_control(
        uci.SessionStartRsp(
            status=uci.StatusCode.UCI_STATUS_OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_ACTIVE,
            reason_code=0))

    await host.expect_control(
        uci.DeviceStatusNtf(
            device_state=uci.DeviceState.DEVICE_STATE_ACTIVE))

    for _ in range(1, 3):
        event = await host.expect_control(
            uci.ShortMacTwoWaySessionInfoNtf,
            timeout=2.0)
        event.show()

    host.send_control(
        uci.SessionStopCmd(
            session_id=0))

    await host.expect_control(
        uci.SessionStopRsp(
            status=uci.StatusCode.UCI_STATUS_OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0))

    await host.expect_control(
        uci.DeviceStatusNtf(
            device_state=uci.DeviceState.DEVICE_STATE_READY))

    host.send_control(
        uci.SessionDeinitCmd(
            session_token=0))

    await host.expect_control(
        uci.SessionDeinitRsp(
            status=uci.StatusCode.UCI_STATUS_OK))


async def controlee(host: Host, peer: Host):
    await init(host)

    host.send_control(
        uci.SessionInitCmd(
            session_id=0,
            session_type=uci.SessionType.FIRA_RANGING_SESSION))

    await host.expect_control(
        uci.SessionInitRsp(
            status=uci.StatusCode.UCI_STATUS_OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0))

    mac_address_mode = 0x0
    ranging_duration = int(1000).to_bytes(4, byteorder='little')
    device_role_responder = bytes([1])
    device_type_controlee = bytes([0])
    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=device_role_responder),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                    v=device_type_controlee),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_MAC_ADDRESS,
                    v=host.mac_address),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MAC_ADDRESS_MODE,
                    v=bytes([mac_address_mode])),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
                    v=ranging_duration),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.NO_OF_CONTROLEE,
                    v=bytes([1])),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS,
                    v=peer.mac_address),
            ]))

    await host.expect_control(
        uci.SessionSetAppConfigRsp(
            status=uci.StatusCode.UCI_STATUS_OK,
            cfg_status=[]))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0))

    host.send_control(
        uci.SessionStartCmd(
            session_id=0))

    await host.expect_control(
        uci.SessionStartRsp(
            status=uci.StatusCode.UCI_STATUS_OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_ACTIVE,
            reason_code=0))

    await host.expect_control(
        uci.DeviceStatusNtf(
            device_state=uci.DeviceState.DEVICE_STATE_ACTIVE))

    for _ in range(1, 3):
        event = await host.expect_control(
            uci.ShortMacTwoWaySessionInfoNtf,
            timeout=2.0)
        event.show()

    host.send_control(
        uci.SessionStopCmd(
            session_id=0))

    await host.expect_control(
        uci.SessionStopRsp(
            status=uci.StatusCode.UCI_STATUS_OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0))

    await host.expect_control(
        uci.DeviceStatusNtf(
            device_state=uci.DeviceState.DEVICE_STATE_READY))

    host.send_control(
        uci.SessionDeinitCmd(
            session_token=0))

    await host.expect_control(
        uci.SessionDeinitRsp(
            status=uci.StatusCode.UCI_STATUS_OK))


async def run(address: str, uci_port: int, http_port: int):
    try:
        host0 = await Host.connect(address, uci_port, bytes([0, 1]))
        host1 = await Host.connect(address, uci_port, bytes([0, 2]))
    except Exception:
        print(
            f'Failed to connect to Pica server at address {address}:{uci_port}\n' +
            'Make sure the server is running')
        exit(1)

    async with asyncio.TaskGroup() as tg:
        tg.create_task(controller(host0, host1))
        tg.create_task(controlee(host1, host0))

    host0.disconnect()
    host1.disconnect()

    print('Ranging test completed')


def main():
    """Start a Pica interactive console."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--address',
                        type=str,
                        default='127.0.0.1',
                        help='Select the pica server address')
    parser.add_argument('--uci-port',
                        type=int,
                        default=7000,
                        help='Select the pica TCP UCI port')
    parser.add_argument('--http-port',
                        type=int,
                        default=3000,
                        help='Select the pica HTTP port')
    asyncio.run(run(**vars(parser.parse_args())))


if __name__ == '__main__':
    main()
