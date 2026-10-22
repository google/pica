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
import logging
import pytest

from pica import Host
from pica.packets import uci
from .helper import init


async def controller(
    host: Host,
    peer: Host,
    mac_address_mode: uci.MacAddressMode = uci.MacAddressMode.MODE_0,
    init_device: bool = True,
):
    if init_device:
        await init(host)

    host.send_control(
        uci.SessionInitCmd(
            session_id=0, session_type=uci.SessionType.FIRA_RANGING_SESSION
        )
    )

    await host.expect_control(uci.SessionInitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0,
        )
    )

    host.send_control(
        uci.SessionGetAppConfigCmd(
            session_token=0,
            app_cfg=[uci.AppConfigTlvType.DEVICE_ROLE],
        )
    )
    await host.expect_control(
        uci.SessionGetAppConfigRsp(
            status=uci.Status.INVALID_PARAM,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=[],
                )
            ],
        )
    )

    ranging_round_usage = 0x06
    ranging_duration = int(1000).to_bytes(4, byteorder="little")

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=bytes([uci.DeviceRole.INITIATOR]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                    v=bytes([uci.DeviceType.CONTROLLER]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_MAC_ADDRESS, v=host.mac_address
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MAC_ADDRESS_MODE,
                    v=bytes([mac_address_mode]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MULTI_NODE_MODE,
                    v=bytes([uci.MultiNodeMode.ONE_TO_ONE]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SCHEDULE_MODE,
                    v=bytes([uci.ScheduleMode.CONTENTION_BASED]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_ROUND_USAGE,
                    v=bytes([ranging_round_usage]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_DURATION, v=ranging_duration
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.NUMBER_OF_CONTROLEES, v=bytes([1])
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS, v=peer.mac_address
                ),
            ],
        )
    )

    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    updated_ranging_duration = list(int(500).to_bytes(4, byteorder="little"))
    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
                    v=updated_ranging_duration,
                ),
            ],
        )
    )

    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    host.send_control(uci.SessionGetAppConfigCmd(session_token=0, app_cfg=[]))

    rsp = await host.expect_control(uci.SessionGetAppConfigRsp)
    assert rsp.status == uci.Status.OK
    assert len(rsp.tlvs) == 61
    assert (
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
            v=updated_ranging_duration,
        )
        in rsp.tlvs
    )

    host.send_control(uci.SessionQueryMaxDataSizeInRangingCmd(session_token=0))
    rsp = await host.expect_control(uci.SessionQueryMaxDataSizeInRangingRsp)
    assert rsp.status == uci.Status.OK
    assert rsp.session_token == 0
    assert rsp.max_data_size == 1020

    host.send_control(uci.SessionStartCmd(session_id=0))

    await host.expect_control(uci.SessionStartRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_ACTIVE,
            reason_code=0,
        )
    )

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_ACTIVE)
    )

    if mac_address_mode == uci.MacAddressMode.MODE_0:
        host.send_control(uci.CoreGetDeviceInfoCmd())
        rsp = await host.expect_control(uci.CoreGetDeviceInfoRsp)
        assert rsp.status == uci.Status.OK

        host.send_control(uci.CoreSetConfigCmd(parameters=[]))
        rsp = await host.expect_control(uci.CoreSetConfigRsp)
        assert rsp.status == uci.Status.REJECTED

    ntf_type = (
        uci.ShortMacTwoWaySessionInfoNtf
        if mac_address_mode == uci.MacAddressMode.MODE_0
        else uci.ExtendedMacTwoWaySessionInfoNtf
    )
    expected_mac = int.from_bytes(peer.mac_address, byteorder="little")

    for _ in range(2):
        event = await host.expect_control(ntf_type, timeout=2.0)
        event.show()
        assert len(event.two_way_ranging_measurements) == 1
        assert event.two_way_ranging_measurements[0].status == uci.Status.OK
        assert event.two_way_ranging_measurements[0].mac_address == expected_mac

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SUSPEND_RANGING_ROUNDS,
                    v=[uci.SuspendRangingRounds.ENABLE],
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SECURE_RANGING_NEFA_LEVEL,
                    v=[0x01],
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SECURE_RANGING_CSW_LENGTH,
                    v=[0x04],
                ),
            ],
        )
    )
    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    with pytest.raises(TimeoutError):
        await host.expect_control(ntf_type, timeout=0.8)

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SUSPEND_RANGING_ROUNDS,
                    v=[uci.SuspendRangingRounds.DISABLE],
                ),
            ],
        )
    )
    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    event = await host.expect_control(ntf_type, timeout=2.0)
    event.show()
    assert event.sequence_number == 2
    assert len(event.two_way_ranging_measurements) == 1
    assert event.two_way_ranging_measurements[0].status == uci.Status.OK
    assert event.two_way_ranging_measurements[0].mac_address == expected_mac

    event = await host.expect_control(ntf_type, timeout=2.0)
    event.show()
    assert event.sequence_number == 3
    assert len(event.two_way_ranging_measurements) == 1
    assert event.two_way_ranging_measurements[0].status == uci.Status.RANGING_RX_TIMEOUT
    assert event.two_way_ranging_measurements[0].mac_address == expected_mac

    host.send_control(uci.SessionStopCmd(session_id=0))

    await host.expect_control(uci.SessionStopRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    host.send_control(uci.SessionStopCmd(session_id=0))
    await host.expect_control(uci.SessionStopRsp(status=uci.Status.REJECTED))

    host.send_control(uci.SessionDeinitCmd(session_token=0))

    await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )


async def multicast_empty_stop(host: Host, peer: Host):
    host.send_control(
        uci.SessionInitCmd(
            session_id=0, session_type=uci.SessionType.FIRA_RANGING_SESSION
        )
    )

    await host.expect_control(uci.SessionInitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0,
        )
    )

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=bytes([uci.DeviceRole.INITIATOR]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                    v=bytes([uci.DeviceType.CONTROLLER]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_MAC_ADDRESS,
                    v=host.mac_address,
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MULTI_NODE_MODE,
                    v=bytes([uci.MultiNodeMode.ONE_TO_MANY]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SCHEDULE_MODE,
                    v=bytes([uci.ScheduleMode.CONTENTION_BASED]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_ROUND_USAGE,
                    v=bytes([0x06]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.NUMBER_OF_CONTROLEES,
                    v=bytes([1]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS,
                    v=peer.mac_address,
                ),
            ],
        )
    )

    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    host.send_control(uci.SessionStartCmd(session_id=0))

    await host.expect_control(uci.SessionStartRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_ACTIVE,
            reason_code=0,
        )
    )

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_ACTIVE)
    )

    host.send_control(
        uci.SessionUpdateControllerMulticastListCmd(
            session_token=0,
            action=uci.UpdateMulticastListAction.REMOVE_CONTROLEE,
            payload=uci.SessionUpdateControllerMulticastListCmdPayload(
                controlees=[
                    uci.Controlee(
                        short_address=list(peer.mac_address),
                        subsession_id=0,
                    )
                ]
            ).serialize(),
        )
    )

    await host.expect_control(
        uci.SessionUpdateControllerMulticastListRsp(
            status=uci.Status.OK,
            controlee_status=[
                uci.ControleeStatus(
                    mac_address=list(peer.mac_address),
                    status=uci.MulticastUpdateStatus.OK_MULTICAST_LIST_UPDATE,
                )
            ],
        )
    )

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=uci.ReasonCode.ERROR_INVALID_NUM_OF_CONTROLEES,
        )
    )

    ntf = await host.expect_control(uci.SessionUpdateControllerMulticastListNtf)
    assert ntf.session_token == 0

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    host.send_control(uci.SessionDeinitCmd(session_token=0))

    await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )


async def multicast_list_ntf(host: Host, peer: Host):
    host.send_control(
        uci.SessionInitCmd(
            session_id=0, session_type=uci.SessionType.FIRA_RANGING_SESSION
        )
    )

    await host.expect_control(uci.SessionInitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0,
        )
    )

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=bytes([uci.DeviceRole.INITIATOR]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                    v=bytes([uci.DeviceType.CONTROLLER]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_MAC_ADDRESS,
                    v=host.mac_address,
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MULTI_NODE_MODE,
                    v=bytes([uci.MultiNodeMode.ONE_TO_MANY]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SCHEDULE_MODE,
                    v=bytes([uci.ScheduleMode.CONTENTION_BASED]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_ROUND_USAGE,
                    v=bytes([0x06]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.NUMBER_OF_CONTROLEES,
                    v=bytes([1]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS,
                    v=peer.mac_address,
                ),
            ],
        )
    )

    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    host.send_control(
        uci.SessionUpdateControllerMulticastListCmd(
            session_token=0,
            action=uci.UpdateMulticastListAction.ADD_CONTROLEE_WITH_SHORT_SUB_SESSION_KEY,
            payload=uci.SessionUpdateControllerMulticastListCmd_2_0_16_Byte_Payload(
                controlees=[
                    uci.Controlee_V2_0_16_Byte_Version(
                        short_address=bytearray([0, 3]),
                        subsession_id=0,
                        subsession_key=bytearray(16),
                    )
                ]
            ).serialize(),
        )
    )
    await host.expect_control(
        uci.SessionUpdateControllerMulticastListRsp(
            status=uci.Status.FAILED,
            controlee_status=[],
        )
    )

    await host.expect_control(
        uci.SessionUpdateControllerMulticastListNtf(
            session_token=0,
            controlee_status=[
                uci.ControleeStatus(
                    mac_address=[0, 3],
                    status=uci.MulticastUpdateStatus.ERROR_SUB_SESSION_KEY_NOT_APPLICABLE,
                )
            ],
        )
    )

    host.send_control(uci.SessionDeinitCmd(session_token=0))

    await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )


async def controlee(
    host: Host,
    peer: Host,
    mac_address_mode: uci.MacAddressMode = uci.MacAddressMode.MODE_0,
    init_device: bool = True,
):
    if init_device:
        await init(host)

    host.send_control(
        uci.SessionInitCmd(
            session_id=0, session_type=uci.SessionType.FIRA_RANGING_SESSION
        )
    )

    await host.expect_control(uci.SessionInitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0,
        )
    )

    ranging_round_usage = 0x06
    ranging_duration = int(500).to_bytes(4, byteorder="little")

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=bytes([uci.DeviceRole.RESPONDER]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                    v=bytes([uci.DeviceType.CONTROLEE]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_MAC_ADDRESS, v=host.mac_address
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MAC_ADDRESS_MODE,
                    v=bytes([mac_address_mode]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MULTI_NODE_MODE,
                    v=bytes([uci.MultiNodeMode.ONE_TO_ONE]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SCHEDULE_MODE,
                    v=bytes([uci.ScheduleMode.CONTENTION_BASED]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_ROUND_USAGE,
                    v=bytes([ranging_round_usage]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_DURATION, v=ranging_duration
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.NUMBER_OF_CONTROLEES, v=bytes([1])
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS, v=peer.mac_address
                ),
            ],
        )
    )

    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    host.send_control(uci.SessionStartCmd(session_id=0))

    await host.expect_control(uci.SessionStartRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_ACTIVE,
            reason_code=0,
        )
    )

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_ACTIVE)
    )

    ntf_type = (
        uci.ShortMacTwoWaySessionInfoNtf
        if mac_address_mode == uci.MacAddressMode.MODE_0
        else uci.ExtendedMacTwoWaySessionInfoNtf
    )
    expected_mac = int.from_bytes(peer.mac_address, byteorder="little")

    for _ in range(2):
        event = await host.expect_control(ntf_type, timeout=2.0)
        event.show()
        assert len(event.two_way_ranging_measurements) == 1
        assert event.two_way_ranging_measurements[0].status == uci.Status.OK
        assert event.two_way_ranging_measurements[0].mac_address == expected_mac

    with pytest.raises(TimeoutError):
        await host.expect_control(ntf_type, timeout=0.8)

    event = await host.expect_control(ntf_type, timeout=2.0)
    event.show()
    assert event.sequence_number == 2
    assert len(event.two_way_ranging_measurements) == 1
    assert event.two_way_ranging_measurements[0].status == uci.Status.OK
    assert event.two_way_ranging_measurements[0].mac_address == expected_mac

    await asyncio.sleep(0.1)

    host.send_control(uci.SessionStopCmd(session_id=0))

    await host.expect_control(uci.SessionStopRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    host.send_control(uci.SessionDeinitCmd(session_token=0))

    await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )


async def ranging_max_measurements(host: Host, peer: Host):
    peer.send_control(
        uci.SessionInitCmd(
            session_id=0, session_type=uci.SessionType.FIRA_RANGING_SESSION
        )
    )

    await peer.expect_control(uci.SessionInitRsp(status=uci.Status.OK))

    await peer.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0,
        )
    )

    peer.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=bytes([uci.DeviceRole.RESPONDER]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                    v=bytes([uci.DeviceType.CONTROLEE]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_MAC_ADDRESS,
                    v=peer.mac_address,
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MULTI_NODE_MODE,
                    v=bytes([uci.MultiNodeMode.ONE_TO_ONE]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SCHEDULE_MODE,
                    v=bytes([uci.ScheduleMode.CONTENTION_BASED]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_ROUND_USAGE,
                    v=bytes([0x06]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
                    v=int.to_bytes(200, 4, "little"),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.NUMBER_OF_CONTROLEES,
                    v=bytes([1]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS,
                    v=host.mac_address,
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG,
                    v=bytes([uci.SessionInfoNtfConfig.DISABLE]),
                ),
            ],
        )
    )

    await peer.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    await peer.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    peer.send_control(uci.SessionStartCmd(session_id=0))

    await peer.expect_control(uci.SessionStartRsp(status=uci.Status.OK))

    await peer.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_ACTIVE,
            reason_code=0,
        )
    )

    await peer.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_ACTIVE)
    )

    host.send_control(
        uci.SessionInitCmd(
            session_id=0, session_type=uci.SessionType.FIRA_RANGING_SESSION
        )
    )

    await host.expect_control(uci.SessionInitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0,
        )
    )

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                    v=bytes([uci.DeviceRole.INITIATOR]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                    v=bytes([uci.DeviceType.CONTROLLER]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DEVICE_MAC_ADDRESS,
                    v=host.mac_address,
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MULTI_NODE_MODE,
                    v=bytes([uci.MultiNodeMode.ONE_TO_ONE]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SCHEDULE_MODE,
                    v=bytes([uci.ScheduleMode.CONTENTION_BASED]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_ROUND_USAGE,
                    v=bytes([0x06]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
                    v=int.to_bytes(200, 4, "little"),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.NUMBER_OF_CONTROLEES,
                    v=bytes([1]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS,
                    v=peer.mac_address,
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG,
                    v=bytes([uci.SessionInfoNtfConfig.ENABLE_PROXIMITY_EDGE_TRIGGER]),
                ),
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.MAX_NUMBER_OF_MEASUREMENTS,
                    v=int.to_bytes(1, 2, "little"),
                ),
            ],
        )
    )

    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    host.send_control(uci.SessionStartCmd(session_id=0))

    await host.expect_control(uci.SessionStartRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_ACTIVE,
            reason_code=0,
        )
    )

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_ACTIVE)
    )

    event = await host.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0)
    assert len(event.two_way_ranging_measurements) == 1
    assert event.two_way_ranging_measurements[0].status == uci.Status.OK
    assert event.two_way_ranging_measurements[0].mac_address == int.from_bytes(
        peer.mac_address, "little"
    )

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=uci.ReasonCode.MAX_NUMBER_OF_MEASUREMENTS_REACHED,
        )
    )

    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    peer.send_control(uci.SessionStopCmd(session_id=0))

    await peer.expect_control(uci.SessionStopRsp(status=uci.Status.OK))

    await peer.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    await peer.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    peer.send_control(uci.SessionDeinitCmd(session_token=0))

    await peer.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))

    await peer.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )

    host.send_control(uci.SessionDeinitCmd(session_token=0))

    await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )


async def run(address: str, uci_port: int):
    try:
        host0 = await Host.connect(address, uci_port, bytes([0, 1]))
        host1 = await Host.connect(address, uci_port, bytes([0, 2]))
    except Exception:
        logging.debug(
            f"Failed to connect to Pica server at address {address}:{uci_port}\n"
            + "Make sure the server is running"
        )
        exit(1)

    try:
        await asyncio.gather(
            controller(host0, host1),
            controlee(host1, host0),
        )

        await multicast_empty_stop(host0, host1)
        await multicast_list_ntf(host0, host1)

        # Re-run ranging on the connected hosts using 8-byte extended MAC addresses (MODE_2).
        host0.mac_address = (1).to_bytes(8, "big")
        host1.mac_address = (2).to_bytes(8, "big")
        await asyncio.gather(
            controller(host0, host1, uci.MacAddressMode.MODE_2, init_device=False),
            controlee(host1, host0, uci.MacAddressMode.MODE_2, init_device=False),
        )

        host0.mac_address = bytes([0, 1])
        host1.mac_address = bytes([0, 2])
        await ranging_max_measurements(host0, host1)

        host0.send_control(uci.CoreQueryTimeStampCmd())
        rsp1 = await host0.expect_control(uci.CoreQueryTimeStampRsp)
        assert rsp1.status == uci.Status.OK
        assert rsp1.timeStamp > 0

        host0.send_control(uci.CoreQueryTimeStampCmd())
        rsp2 = await host0.expect_control(uci.CoreQueryTimeStampRsp)
        assert rsp2.status == uci.Status.OK
        assert rsp2.timeStamp >= rsp1.timeStamp

        await verify_command_error_fallbacks(host0)
        await ccc_ranging_session(host0)
        await aliro_ranging_session(host0)
        await dl_tdoa_ranging_session(host0, host1)
    finally:
        host0.disconnect()
        host1.disconnect()

    logging.debug("Ranging test completed")


async def verify_command_error_fallbacks(host: Host):
    host.send_control(
        uci.SessionInitCmd(session_id=1, session_type=uci.SessionType.DEVICE_TEST_MODE)
    )
    await host.expect_control(uci.SessionInitRsp(status=uci.Status.OK))
    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=1,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0,
        )
    )
    host.send_control(uci.SessionSetAppConfigCmd(session_token=1, tlvs=[]))
    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.REJECTED, cfg_status=[])
    )
    host.send_control(uci.SessionDeinitCmd(session_token=1))
    await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))
    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=1,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )

    host.send_control(
        uci.SessionUpdateDtTagRangingRoundsCmd(
            session_token=1, ranging_round_indexes=[]
        )
    )
    await host.expect_control(
        uci.SessionUpdateDtTagRangingRoundsRsp(
            status=uci.Status.ERROR_SESSION_NOT_EXIST, ranging_round_indexes=[]
        )
    )

    host.send_control(
        uci.SessionConfigPacket(
            mt=uci.MessageType.COMMAND, oid=uci.SessionConfigOpcodeId.STATUS
        )
    )
    rsp = await host.expect_control(uci.SessionConfigPacket)
    assert rsp.mt == uci.MessageType.RESPONSE
    assert rsp.oid == uci.SessionConfigOpcodeId.STATUS
    assert rsp.payload == bytes([uci.Status.UNKNOWN_OID])


async def ccc_ranging_session(host: Host):
    await ccc_or_aliro_ranging_session(host, uci.SessionType.CCC)


async def aliro_ranging_session(host: Host):
    await ccc_or_aliro_ranging_session(host, uci.SessionType.ALIRO)


async def ccc_or_aliro_ranging_session(
    host: Host, session_type: uci.SessionType = uci.SessionType.CCC
):
    host.send_control(uci.SessionInitCmd(session_id=2, session_type=session_type))
    await host.expect_control(uci.SessionInitRsp(status=uci.Status.OK))
    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=2,
            session_state=uci.SessionState.SESSION_STATE_INIT,
            reason_code=0,
        )
    )

    app_configs = [
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
            v=bytes([uci.DeviceType.CCC_CONTROLLER]),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
            v=bytes([uci.DeviceRole.INITIATOR]),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.MULTI_NODE_MODE,
            v=bytes([uci.MultiNodeMode.ONE_TO_MANY]),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.CHANNEL_NUMBER,
            v=bytes([uci.ChannelNumber.CHANNEL_NUMBER_9]),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.NUMBER_OF_CONTROLEES,
            v=bytes([1]),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.SLOT_DURATION,
            v=int(2400).to_bytes(2, "little"),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
            v=int(96).to_bytes(4, "little"),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.PREAMBLE_CODE_INDEX,
            v=bytes([9]),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.SLOTS_PER_RR,
            v=bytes([6]),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.HOPPING_MODE,
            v=bytes([uci.CccHoppingMode.EXTENSION_CONTINUOUS_AES]),
        ),
        uci.AppConfigTlv(
            cfg_id=0xA3,
            v=bytes([1, 0]),
        ),
        uci.AppConfigTlv(
            cfg_id=0xA4,
            v=int(0).to_bytes(2, "little"),
        ),
        uci.AppConfigTlv(
            cfg_id=0xA5,
            v=bytes([0]),
        ),
        uci.AppConfigTlv(
            cfg_id=0xA6,
            v=int(720).to_bytes(2, "little"),
        ),
    ]
    if session_type == uci.SessionType.ALIRO:
        app_configs.extend(
            [
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.SESSION_KEY,
                    v=bytes([1] * 16),
                ),
            ]
        )

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=2,
            tlvs=app_configs,
        )
    )
    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )
    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=2,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )

    host.send_control(uci.SessionGetAppConfigCmd(session_token=2, app_cfg=[]))
    rsp = await host.expect_control(uci.SessionGetAppConfigRsp)
    assert rsp.status == uci.Status.OK
    tlvs = {tlv.cfg_id: bytes(tlv.v) for tlv in rsp.tlvs}
    assert tlvs[0xA0] == bytes([0, 0, 0, 0])
    assert tlvs[0xA8] == bytes([0, 0, 0, 0])
    assert 0xA1 not in tlvs
    if session_type == uci.SessionType.ALIRO:
        assert tlvs[0xA9] == bytes([0])
        assert tlvs.get(uci.AppConfigTlvType.SESSION_KEY) == bytes([1] * 16)

    second_app_configs = [
        uci.AppConfigTlv(
            cfg_id=0xA0,
            v=bytes(16),
        ),
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.STS_INDEX,
            v=int(10).to_bytes(4, "little"),
        ),
    ]
    if session_type == uci.SessionType.ALIRO:
        second_app_configs.append(
            uci.AppConfigTlv(
                cfg_id=0xA9,
                v=bytes([(1 << 6) | 1]),
            )
        )
    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=2,
            tlvs=second_app_configs,
        )
    )
    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    host.send_control(uci.SessionGetAppConfigCmd(session_token=2, app_cfg=[]))
    rsp = await host.expect_control(uci.SessionGetAppConfigRsp)
    assert rsp.status == uci.Status.OK
    tlvs = {tlv.cfg_id: bytes(tlv.v) for tlv in rsp.tlvs}
    assert tlvs[0xA0] == bytes(16)
    assert tlvs[0xA8] == bytes([0, 0, 0, 0])
    assert tlvs[uci.AppConfigTlvType.STS_INDEX] == int(10).to_bytes(4, "little")
    assert 0xA1 not in tlvs
    if session_type == uci.SessionType.ALIRO:
        assert tlvs.get(0xA9) == bytes([(1 << 6) | 1])
        assert tlvs.get(uci.AppConfigTlvType.SESSION_KEY) == bytes([1] * 16)

    host.send_control(uci.SessionStartCmd(session_id=2))
    await host.expect_control(uci.SessionStartRsp(status=uci.Status.OK))
    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=2,
            session_state=uci.SessionState.SESSION_STATE_ACTIVE,
            reason_code=0,
        )
    )
    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_ACTIVE)
    )

    host.send_control(uci.SessionGetAppConfigCmd(session_token=2, app_cfg=[]))
    rsp = await host.expect_control(uci.SessionGetAppConfigRsp)
    assert rsp.status == uci.Status.OK
    tlvs = {tlv.cfg_id: bytes(tlv.v) for tlv in rsp.tlvs}
    assert tlvs[0xA0] == bytes(16)
    assert tlvs[0xA8] == bytes([0, 0, 0, 0])
    assert 0xA1 not in tlvs
    if session_type == uci.SessionType.ALIRO:
        assert tlvs.get(0xA9) == bytes([(1 << 6) | 1])
        assert tlvs.get(uci.AppConfigTlvType.SESSION_KEY) == bytes([1] * 16)

    host.send_control(uci.SessionStopCmd(session_id=2))
    await host.expect_control(uci.SessionStopRsp(status=uci.Status.OK))
    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=2,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0,
        )
    )
    await host.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    host.send_control(uci.SessionDeinitCmd(session_token=2))
    await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))
    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=2,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )


async def dl_tdoa_ranging_session(host_tag: Host, host_anchor: Host):
    anchor_mac = bytes([0x34, 0x12])
    tag_mac = bytes([0x78, 0x56])

    async def init_session(
        host: Host,
        sid: int,
        role: uci.DeviceRole,
        dev_type: uci.DeviceType,
        mac_address: bytes,
        extra_tlvs: list[uci.AppConfigTlv] | None = None,
    ):
        host.send_control(
            uci.SessionInitCmd(
                session_id=sid, session_type=uci.SessionType.FIRA_RANGING_SESSION
            )
        )
        await host.expect_control(uci.SessionInitRsp(status=uci.Status.OK))
        await host.expect_control(
            uci.SessionStatusNtf(
                session_token=sid,
                session_state=uci.SessionState.SESSION_STATE_INIT,
                reason_code=0,
            )
        )
        base_tlvs = {
            uci.AppConfigTlvType.DEVICE_ROLE: [role],
            uci.AppConfigTlvType.DEVICE_TYPE: [dev_type],
            uci.AppConfigTlvType.DEVICE_MAC_ADDRESS: mac_address,
            uci.AppConfigTlvType.MULTI_NODE_MODE: [uci.MultiNodeMode.ONE_TO_ONE],
            uci.AppConfigTlvType.SCHEDULE_MODE: [uci.ScheduleMode.CONTENTION_BASED],
            uci.AppConfigTlvType.RANGING_ROUND_USAGE: [
                uci.RangingRoundUsage.ONE_WAY_RANGING_DL_TDOA
            ],
            uci.AppConfigTlvType.RANGING_DURATION: int(100).to_bytes(4, "little"),
            uci.AppConfigTlvType.MAC_ADDRESS_MODE: [uci.MacAddressMode.MODE_0],
        }
        tlvs = [
            uci.AppConfigTlv(cfg_id=k, v=bytes(v)) for k, v in base_tlvs.items()
        ] + (extra_tlvs or [])
        host.send_control(uci.SessionSetAppConfigCmd(session_token=sid, tlvs=tlvs))
        await host.expect_control(
            uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
        )
        await host.expect_control(
            uci.SessionStatusNtf(
                session_token=sid,
                session_state=uci.SessionState.SESSION_STATE_IDLE,
                reason_code=0,
            )
        )

    async def set_config(
        host: Host,
        tlvs: list[uci.AppConfigTlv],
        status: uci.Status = uci.Status.OK,
    ):
        host.send_control(uci.SessionSetAppConfigCmd(session_token=3, tlvs=tlvs))
        cfg_status = (
            []
            if status == uci.Status.OK
            else [uci.AppConfigStatus(cfg_id=tlv.cfg_id, status=status) for tlv in tlvs]
        )
        await host.expect_control(
            uci.SessionSetAppConfigRsp(status=status, cfg_status=cfg_status)
        )

    async def update_anchor_rounds(
        host: Host,
        rounds: list[uci.DtAnchorRangingRound],
    ):
        host.send_control(
            uci.SessionUpdateDtAnchorRangingRoundsCmd(
                session_token=3, ranging_rounds=rounds
            )
        )
        await host.expect_control(
            uci.SessionUpdateDtAnchorRangingRoundsRsp(
                status=uci.Status.OK, ranging_round_indexes=[]
            )
        )

    async def update_tag_rounds(
        host: Host,
        rounds: list[int],
        status: uci.Status = uci.Status.OK,
    ):
        host.send_control(
            uci.SessionUpdateDtTagRangingRoundsCmd(
                session_token=3, ranging_round_indexes=rounds
            )
        )
        await host.expect_control(
            uci.SessionUpdateDtTagRangingRoundsRsp(
                status=status, ranging_round_indexes=[]
            )
        )

    async def start_session(host: Host, sid: int):
        host.send_control(uci.SessionStartCmd(session_id=sid))
        await host.expect_control(uci.SessionStartRsp(status=uci.Status.OK))
        await host.expect_control(
            uci.SessionStatusNtf(
                session_token=sid,
                session_state=uci.SessionState.SESSION_STATE_ACTIVE,
                reason_code=0,
            )
        )
        await host.expect_control(
            uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_ACTIVE)
        )

    async def stop_session(host: Host, sid: int):
        host.send_control(uci.SessionStopCmd(session_id=sid))
        await host.expect_control(uci.SessionStopRsp(status=uci.Status.OK))
        await host.expect_control(
            uci.SessionStatusNtf(
                session_token=sid,
                session_state=uci.SessionState.SESSION_STATE_IDLE,
                reason_code=0,
            )
        )
        await host.expect_control(
            uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
        )

    async def stop_deinit_session(host: Host, sid: int):
        await stop_session(host, sid)
        host.send_control(uci.SessionDeinitCmd(session_token=sid))
        await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))
        await host.expect_control(
            uci.SessionStatusNtf(
                session_token=sid,
                session_state=uci.SessionState.SESSION_STATE_DEINIT,
                reason_code=0,
            )
        )

    await init_session(
        host_anchor,
        3,
        uci.DeviceRole.DT_ANCHOR,
        uci.DeviceType.CONTROLLER,
        anchor_mac,
        [
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION,
                v=bytes([0x03]) + uci.RelativeLocation(x=10, y=20, z=30).serialize(),
            ),
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DL_TDOA_TX_ACTIVE_RANGING_ROUNDS,
                v=bytes([0x01]),
            ),
        ],
    )

    host_anchor.send_control(
        uci.SessionGetAppConfigCmd(
            session_token=3,
            app_cfg=[
                uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION,
                uci.AppConfigTlvType.DL_TDOA_TX_ACTIVE_RANGING_ROUNDS,
            ],
        )
    )
    rsp = await host_anchor.expect_control(uci.SessionGetAppConfigRsp)
    assert rsp.status == uci.Status.OK
    anchor_tlvs = {tlv.cfg_id: bytes(tlv.v) for tlv in rsp.tlvs}
    assert anchor_tlvs[uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION] == (
        bytes([0x03]) + bytes(uci.RelativeLocation(x=10, y=20, z=30).serialize())
    )
    assert anchor_tlvs[uci.AppConfigTlvType.DL_TDOA_TX_ACTIVE_RANGING_ROUNDS] == bytes(
        [1]
    )

    for cfg_id, bad_val in [
        (uci.AppConfigTlvType.DL_TDOA_TX_TIMESTAMP_CONF, bytes([0x04])),
        (uci.AppConfigTlvType.DL_TDOA_RESPONDER_TOF, bytes([0x02])),
        (uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION, bytes([0x01, 0x00])),
        (uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION, bytes([0x05])),
        (uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION, bytes([0x02])),
        (uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION, bytes([])),
        (uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION, bytes([0x00, 0x01])),
    ]:
        await set_config(
            host_anchor,
            [uci.AppConfigTlv(cfg_id=cfg_id, v=bad_val)],
            status=uci.Status.INVALID_PARAM,
        )

    await set_config(
        host_anchor,
        [
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DL_TDOA_HOP_COUNT,
                v=bytes([3]),
            ),
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DL_TDOA_TX_TIMESTAMP_CONF,
                v=bytes([0b1111_1011]),
            ),
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION,
                v=bytes([0x01] + [0x11] * 12),
            ),
        ],
    )
    await set_config(
        host_anchor,
        [
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DL_TDOA_TX_TIMESTAMP_CONF,
                v=bytes([0x03]),
            ),
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION,
                v=bytes([0x03])
                + bytes(uci.RelativeLocation(x=10, y=20, z=30).serialize()),
            ),
        ],
    )

    await update_tag_rounds(
        host_anchor,
        [1],
        status=uci.Status.ERROR_SESSION_NOT_CONFIGURED,
    )
    await update_anchor_rounds(
        host_anchor,
        [
            uci.DtAnchorRangingRound(
                round_index=1, role=uci.DtAnchorActingRole.INITIATOR
            )
        ],
    )
    await start_session(host_anchor, 3)

    await init_session(
        host_tag,
        3,
        uci.DeviceRole.DT_TAG,
        uci.DeviceType.CONTROLEE,
        tag_mac,
    )

    await set_config(
        host_tag,
        [
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DL_TDOA_BLOCK_SKIPPING,
                v=bytes([1]),
            )
        ],
    )

    await update_tag_rounds(
        host_tag,
        [0, 1],
    )

    await stop_deinit_session(host_anchor, 3)
    await start_session(host_tag, 3)
    await asyncio.sleep(0.25)
    await stop_deinit_session(host_tag, 3)


def main():
    """Start a Pica interactive console."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--address",
        type=str,
        default="127.0.0.1",
        help="Select the pica server address",
    )
    parser.add_argument(
        "--uci-port", type=int, default=7000, help="Select the pica TCP UCI port"
    )
    asyncio.run(run(**vars(parser.parse_args())))


if __name__ == "__main__":
    main()
