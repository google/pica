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

        host.send_control(uci.CoreGetCapsInfoCmd())
        caps_rsp = await host.expect_control(uci.CoreGetCapsInfoRsp)
        assert caps_rsp.status == uci.Status.OK
        caps = {tlv.t: list(tlv.v) for tlv in caps_rsp.tlvs}
        assert caps.get(uci.CapTlvType.SUPPORTED_RSSI_REPORTING) == [0x01]

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
        assert event.current_ranging_interval == 500
        assert len(event.two_way_ranging_measurements) == 1
        assert event.two_way_ranging_measurements[0].status == uci.Status.OK
        assert event.two_way_ranging_measurements[0].mac_address == expected_mac
        assert event.two_way_ranging_measurements[0].rssi == 0

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
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RSSI_REPORTING,
                    v=[uci.RssiReporting.ENABLE],
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
    assert event.current_ranging_interval == 500
    assert len(event.two_way_ranging_measurements) == 1
    assert event.two_way_ranging_measurements[0].status == uci.Status.OK
    assert event.two_way_ranging_measurements[0].mac_address == expected_mac
    assert event.two_way_ranging_measurements[0].rssi == 100

    event = await host.expect_control(ntf_type, timeout=2.0)
    event.show()
    assert event.sequence_number == 3
    assert event.current_ranging_interval == 500
    assert len(event.two_way_ranging_measurements) == 1
    assert event.two_way_ranging_measurements[0].status == uci.Status.RANGING_RX_TIMEOUT
    assert event.two_way_ranging_measurements[0].mac_address == expected_mac
    assert event.two_way_ranging_measurements[0].rssi == 0

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

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.STS_CONFIG,
                    v=bytes([uci.StsConfig.PROVISIONED_FOR_RESPONDER_SUB_SESSION_KEY]),
                )
            ],
        )
    )
    await host.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )

    # 8 * 38-byte controlees -> 310-byte payload, placing the 8th MAC address in the 2nd PBF fragment.
    controlees = [
        uci.Controlee_V2_0_32_Byte_Version(
            short_address=bytearray([0, i + 3]),
            subsession_id=i + 1,
            subsession_key=bytearray(32),
        )
        for i in range(8)
    ]
    host.send_control(
        uci.SessionUpdateControllerMulticastListCmd(
            session_token=0,
            action=uci.UpdateMulticastListAction.ADD_CONTROLEE_WITH_EXTENDED_SUB_SESSION_KEY,
            payload=uci.SessionUpdateControllerMulticastListCmd_2_0_32_Byte_Payload(
                controlees=controlees
            ).serialize(),
        )
    )
    await host.expect_control(
        uci.SessionUpdateControllerMulticastListRsp(
            status=uci.Status.OK,
            controlee_status=[
                uci.ControleeStatus(
                    mac_address=[0, i + 3],
                    status=uci.MulticastUpdateStatus.OK_MULTICAST_LIST_UPDATE,
                )
                for i in range(8)
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
        assert event.current_ranging_interval == 500
        assert len(event.two_way_ranging_measurements) == 1
        assert event.two_way_ranging_measurements[0].status == uci.Status.OK
        assert event.two_way_ranging_measurements[0].mac_address == expected_mac
        assert event.two_way_ranging_measurements[0].rssi == 0

    with pytest.raises(TimeoutError):
        await host.expect_control(ntf_type, timeout=0.8)

    event = await host.expect_control(ntf_type, timeout=2.0)
    event.show()
    assert event.sequence_number == 2
    assert event.current_ranging_interval == 500
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
    assert event.two_way_ranging_measurements[0].rssi == 0

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

        await verify_device_capabilities(host0)
        await verify_command_error_fallbacks(host0)
        await ccc_ranging_session(host0)
        await aliro_ranging_session(host0)
        await dl_tdoa_ranging_session(host0, host1, address, uci_port)
        await twr_block_stride_ranging(host0, host1)
        await aoa_result_req_ranging(host0, host1)
    finally:
        host0.disconnect()
        host1.disconnect()

    logging.debug("Ranging test completed")


async def verify_device_capabilities(host: Host):
    host.send_control(uci.CoreGetDeviceInfoCmd())
    device_info = await host.expect_control(uci.CoreGetDeviceInfoRsp)
    assert device_info.status == uci.Status.OK
    assert device_info.mac_version == 0x0002
    assert device_info.phy_version == 0x0002
    assert device_info.uci_version == 0x0002

    host.send_control(uci.CoreGetCapsInfoCmd())
    caps_rsp = await host.expect_control(uci.CoreGetCapsInfoRsp)
    assert caps_rsp.status == uci.Status.OK

    caps = {tlv.t: bytes(tlv.v) for tlv in caps_rsp.tlvs}
    assert len(caps) == len(caps_rsp.tlvs)

    assert caps[uci.CapTlvType.SUPPORTED_FIRA_PHY_VERSION_RANGE] == bytes([1, 1, 2, 0])
    assert caps[uci.CapTlvType.SUPPORTED_FIRA_MAC_VERSION_RANGE] == bytes([1, 1, 2, 0])

    assert caps[uci.CapTlvType.SUPPORTED_MAX_MESSAGE_SIZE] == (4096).to_bytes(
        2, "little"
    )
    assert caps[uci.CapTlvType.SUPPORTED_MAX_DATA_PACKET_PAYLOAD_SIZE] == (
        1020
    ).to_bytes(2, "little")

    assert caps[uci.CapTlvType.SUPPORTED_DEVICE_TYPE] == bytes(
        [(1 << uci.DeviceType.CONTROLEE) | (1 << uci.DeviceType.CONTROLLER)]
    )
    assert caps[uci.CapTlvType.SUPPORTED_DEVICE_ROLES] == (
        (1 << uci.DeviceRole.RESPONDER)
        | (1 << uci.DeviceRole.INITIATOR)
        | (1 << uci.DeviceRole.DT_ANCHOR)
        | (1 << uci.DeviceRole.DT_TAG)
    ).to_bytes(2, "little")
    assert caps[uci.CapTlvType.SUPPORTED_RANGING_METHOD] == (
        (1 << uci.RangingRoundUsage.SS_TWR_DEFERRED_MODE)
        | (1 << uci.RangingRoundUsage.DS_TWR_DEFERRED_MODE)
        | (1 << uci.RangingRoundUsage.SS_TWR_NON_DEFERRED_MODE)
        | (1 << uci.RangingRoundUsage.DS_TWR_NON_DEFERRED_MODE)
        | (1 << uci.RangingRoundUsage.ONE_WAY_RANGING_DL_TDOA)
    ).to_bytes(2, "little")

    assert caps[uci.CapTlvType.SUPPORTED_STS_CONFIG] == bytes(
        [
            (1 << uci.StsConfig.STATIC)
            | (1 << uci.StsConfig.DYNAMIC)
            | (1 << uci.StsConfig.DYNAMIC_FOR_RESPONDER_SUB_SESSION_KEY)
            | (1 << uci.StsConfig.PROVISIONED)
            | (1 << uci.StsConfig.PROVISIONED_FOR_RESPONDER_SUB_SESSION_KEY)
        ]
    )
    assert caps[uci.CapTlvType.SUPPORTED_MULTI_NODE_MODES] == bytes(
        [(1 << uci.MultiNodeMode.ONE_TO_ONE) | (1 << uci.MultiNodeMode.ONE_TO_MANY)]
    )
    assert caps[uci.CapTlvType.SUPPORTED_RANGING_TIME_STRUCT] == bytes(
        [1 << uci.RangingTimeStruct.BLOCK_BASED_SCHEDULING]
    )
    assert caps[uci.CapTlvType.SUPPORTED_SCHEDULED_MODE] == bytes(
        [
            (1 << uci.ScheduleMode.CONTENTION_BASED)
            | (1 << uci.ScheduleMode.TIME_SCHEDULED)
        ]
    )
    assert caps[uci.CapTlvType.SUPPORTED_HOPPING_MODE] == bytes([0x00])
    assert caps[uci.CapTlvType.SUPPORTED_BLOCK_STRIDING] == bytes([0x01])
    assert caps[uci.CapTlvType.SUPPORTED_UWB_INITIATION_TIME] == bytes([0x01])
    assert caps[uci.CapTlvType.SUPPORTED_CHANNELS] == bytes([0xFF])
    assert caps[uci.CapTlvType.SUPPORTED_RFRAME_CONFIG] == bytes(
        [
            (1 << uci.RframeConfig.SP0)
            | (1 << uci.RframeConfig.SP1)
            | (1 << uci.RframeConfig.SP3)
        ]
    )
    assert caps[uci.CapTlvType.SUPPORTED_CC_CONSTRAINT_LENGTH] == bytes([0x03])
    assert caps[uci.CapTlvType.SUPPORTED_BPRF_PARAMETER_SETS] == bytes([0x3F])
    assert caps[uci.CapTlvType.SUPPORTED_HPRF_PARAMETER_SETS] == (
        (1 << 35) - 1
    ).to_bytes(5, "little")
    assert caps[uci.CapTlvType.SUPPORTED_AOA] == bytes([0x0F])
    assert caps[uci.CapTlvType.SUPPORTED_EXTENDED_MAC_ADDRESS] == bytes([0x01])
    assert caps[uci.CapTlvType.SUPPORTED_SUSPEND_RANGING_SUPPORT] == bytes([0x01])
    assert caps[uci.CapTlvType.SUPPORTED_SESSION_KEY_LENGTH] == bytes([0x03])
    assert caps[uci.CapTlvType.SUPPORTED_DT_ANCHOR_MAX_ACTIVE_RR] == bytes([16])
    assert caps[uci.CapTlvType.SUPPORTED_DT_TAG_MAX_ACTIVE_RR] == bytes([16])
    assert caps[uci.CapTlvType.SUPPORTED_DT_TAG_BLOCK_SKIPPING] == bytes([0x01])

    assert caps[uci.CapTlvType.CCC_SUPPORTED_CHAPS_PER_SLOT] == bytes([0xFF])
    assert caps[uci.CapTlvType.CCC_SUPPORTED_SYNC_CODES] == bytes(
        [0xFF, 0xFF, 0xFF, 0xFF]
    )
    assert caps[
        uci.CapTlvType.CCC_SUPPORTED_HOPPING_CONFIG_MODES_AND_SEQUENCES
    ] == bytes([0xFF])
    assert caps[uci.CapTlvType.CCC_SUPPORTED_CHANNELS] == bytes([0xFF])
    assert caps[uci.CapTlvType.CCC_SUPPORTED_VERSIONS] == bytes([1, 0])
    assert caps[uci.CapTlvType.CCC_SUPPORTED_UWB_CONFIGS] == bytes([0])
    assert caps[uci.CapTlvType.CCC_SUPPORTED_PULSE_SHAPE_COMBOS] == bytes([0])
    assert caps[uci.CapTlvType.CCC_SUPPORTED_RAN_MULTIPLIER] == bytes([1, 0, 0, 0])

    assert caps[uci.CapTlvType.ALIRO_SUPPORTED_MAC_MODES] == bytes([0x03])
    assert caps[uci.CapTlvType.ALIRO_SUPPORTED_PROTOCOL_VERSION] == bytes([1, 0])

    assert caps[uci.CapTlvType.SUPPORTED_AOA_RESULT_REQ_ANTENNA_INTERLEAVING] == bytes(
        [0x01]
    )
    assert caps[uci.CapTlvType.SUPPORTED_RSSI_REPORTING] == bytes([0x01])


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


async def dl_tdoa_ranging_session(
    host_tag: Host, host_anchor: Host, address: str, uci_port: int
):
    anchor_mac = bytes([0x34, 0x12])
    responder_mac = bytes([0x56, 0x34])
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
        status=uci.Status.REJECTED,
    )
    host_anchor.send_control(uci.SessionStartCmd(session_id=3))
    await host_anchor.expect_control(uci.SessionStartRsp(status=uci.Status.OK))
    await host_anchor.expect_control(
        uci.SessionStatusNtf(
            session_token=3,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=0x40,
        )
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
    await start_session(host_tag, 3)

    event1 = await host_tag.expect_control(
        uci.ShortMacDlTDoASessionInfoNtf, timeout=2.0
    )
    assert event1.current_ranging_interval == 200
    assert len(event1.dl_tdoa_measurements) == 1
    m1 = event1.dl_tdoa_measurements[0]
    assert m1.mac_address == int.from_bytes(anchor_mac, "little")
    assert m1.measurement.status == uci.Status.OK
    assert m1.measurement.block_index == 0
    assert m1.measurement.round_index == 1
    assert m1.measurement.message_type == uci.DlTdoaMessageType.POLL_DTM
    assert (
        m1.measurement.tx_timestamp_type == uci.DlTdoaTxTimestampType.COMMON_TIME_BASE
    )
    assert m1.measurement.tx_timestamp_64 == 3_194_880_000
    assert m1.measurement.tx_timestamp_40 is None
    assert m1.measurement.rx_timestamp_64 == 3_194_880_000
    assert m1.measurement.rx_timestamp_40 is None
    assert m1.measurement.relative_location == uci.RelativeLocation(x=10, y=20, z=30)
    assert m1.measurement.wgs84_location is None
    assert m1.measurement.active_ranging_rounds == []

    event2 = await host_tag.expect_control(
        uci.ShortMacDlTDoASessionInfoNtf, timeout=2.0
    )
    assert event2.current_ranging_interval == 200
    assert len(event2.dl_tdoa_measurements) == 1
    m2 = event2.dl_tdoa_measurements[0]
    assert m2.measurement.block_index == 2
    assert m2.measurement.tx_timestamp_64 == 15_974_400_000
    assert m2.measurement.rx_timestamp_64 == 15_974_400_000

    await update_tag_rounds(host_tag, [0])
    empty_event = await host_tag.expect_control(
        uci.ShortMacDlTDoASessionInfoNtf, timeout=2.0
    )
    assert len(empty_event.dl_tdoa_measurements) == 0

    host_anchor.send_control(uci.SessionGetRangingCountCmd(session_id=3))
    ranging_count_rsp = await host_anchor.expect_control(uci.SessionGetRangingCountRsp)
    assert ranging_count_rsp.status == uci.Status.OK
    assert ranging_count_rsp.count > 0

    await stop_session(host_tag, 3)
    await stop_session(host_anchor, 3)

    host_responder = await Host.connect(address, uci_port, responder_mac)
    try:
        await init(host_responder)

        responder_tlvs = {
            uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION: bytes([0x01])
            + bytes(range(1, 13)),
            uci.AppConfigTlvType.DL_TDOA_TX_TIMESTAMP_CONF: bytes([0x00]),
            uci.AppConfigTlvType.DL_TDOA_TX_ACTIVE_RANGING_ROUNDS: bytes([0x00]),
            uci.AppConfigTlvType.DL_TDOA_RESPONDER_TOF: bytes([0x01]),
            uci.AppConfigTlvType.DL_TDOA_ANCHOR_CFO: bytes(
                [uci.DlTdoaAnchorCfo.ANCHOR_CFO_NOT_INCLUDED]
            ),
        }
        await init_session(
            host_responder,
            3,
            uci.DeviceRole.DT_ANCHOR,
            uci.DeviceType.CONTROLLER,
            responder_mac,
            [uci.AppConfigTlv(cfg_id=k, v=v) for k, v in responder_tlvs.items()],
        )
        await update_anchor_rounds(
            host_responder,
            [
                uci.DtAnchorRangingRound(
                    round_index=1, role=uci.DtAnchorActingRole.RESPONDER
                )
            ],
        )
        await update_anchor_rounds(
            host_anchor,
            [
                uci.DtAnchorRangingRound(
                    round_index=0, role=uci.DtAnchorActingRole.INITIATOR
                ),
                uci.DtAnchorRangingRound(
                    round_index=1, role=uci.DtAnchorActingRole.INITIATOR
                ),
            ],
        )
        await update_tag_rounds(host_tag, [1])

        await start_session(host_anchor, 3)
        await start_session(host_responder, 3)
        await start_session(host_tag, 3)

        event3 = await host_tag.expect_control(
            uci.ShortMacDlTDoASessionInfoNtf, timeout=2.0
        )
        assert len(event3.dl_tdoa_measurements) == 3
        assert all(m.measurement.rssi == 0 for m in event3.dl_tdoa_measurements)

        m_init = event3.dl_tdoa_measurements[0]
        assert m_init.mac_address == int.from_bytes(anchor_mac, "little")
        assert m_init.measurement.status == uci.Status.OK
        assert m_init.measurement.message_type == uci.DlTdoaMessageType.POLL_DTM
        assert m_init.measurement.round_index == 1
        assert m_init.measurement.tx_timestamp_64 == 3_194_880_000
        assert m_init.measurement.rx_timestamp_64 == 3_194_880_000
        assert m_init.measurement.active_ranging_rounds == [0]
        assert m_init.measurement.initiator_responder_tof == 0

        m_resp = event3.dl_tdoa_measurements[1]
        assert m_resp.mac_address == int.from_bytes(responder_mac, "little")
        assert m_resp.measurement.status == uci.Status.OK
        assert m_resp.measurement.message_type == uci.DlTdoaMessageType.RESPONSE_DTM
        assert (
            m_resp.measurement.tx_timestamp_type
            == uci.DlTdoaTxTimestampType.LOCAL_TIME_BASE
        )
        assert m_resp.measurement.tx_timestamp_40 is None
        assert m_resp.measurement.tx_timestamp_64 == 3_322_675_200
        assert m_resp.measurement.rx_timestamp_40 is None
        assert m_resp.measurement.rx_timestamp_64 == 3_322_675_200
        assert m_resp.measurement.round_index == 1
        assert m_resp.measurement.wgs84_location == uci.Wgs84Location(
            data=list(range(1, 13))
        )
        assert m_resp.measurement.relative_location is None
        assert m_resp.measurement.active_ranging_rounds == []
        assert m_resp.measurement.initiator_responder_tof == 0
        assert m_resp.measurement.anchor_cfo == 0x8000
        assert m_resp.measurement.responder_reply_time == 127_795_200
        assert m_resp.measurement.initiator_reply_time == 127_795_200

        m_final = event3.dl_tdoa_measurements[2]
        assert m_final.mac_address == int.from_bytes(anchor_mac, "little")
        assert m_final.measurement.status == uci.Status.OK
        assert m_final.measurement.message_type == uci.DlTdoaMessageType.FINAL_DTM
        assert m_final.measurement.round_index == 1
        assert m_final.measurement.tx_timestamp_64 == 3_450_470_400
        assert m_final.measurement.rx_timestamp_64 == 3_450_470_400
        assert m_final.measurement.initiator_reply_time == 0
        assert m_final.measurement.responder_reply_time == 0
        assert m_final.measurement.initiator_responder_tof == 0
        assert m_final.measurement.anchor_cfo == 0
        assert m_final.measurement.wgs84_location is None
        assert m_final.measurement.relative_location is None
        assert m_final.measurement.active_ranging_rounds == []

        await set_config(
            host_anchor,
            [
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.DL_TDOA_RANGING_METHOD,
                    v=bytes([uci.DlTdoaRangingMethod.SS_TWR]),
                )
            ],
        )
        await update_anchor_rounds(
            host_responder,
            [
                uci.DtAnchorRangingRound(
                    round_index=0, role=uci.DtAnchorActingRole.RESPONDER
                ),
                uci.DtAnchorRangingRound(
                    round_index=1, role=uci.DtAnchorActingRole.RESPONDER
                ),
            ],
        )
        await set_config(
            host_tag,
            [
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.RSSI_REPORTING,
                    v=bytes([uci.RssiReporting.ENABLE]),
                )
            ],
        )
        await update_tag_rounds(host_tag, [0, 1])

        event_fanout = await host_tag.expect_control(
            uci.ShortMacDlTDoASessionInfoNtf, timeout=2.0
        )
        assert len(event_fanout.dl_tdoa_measurements) == 4
        assert [
            m.measurement.round_index for m in event_fanout.dl_tdoa_measurements
        ] == [0, 0, 1, 1]
        assert all(
            m.measurement.initiator_reply_time == 0
            for m in event_fanout.dl_tdoa_measurements
        )
        assert all(m.measurement.rssi == 100 for m in event_fanout.dl_tdoa_measurements)

        await stop_deinit_session(host_tag, 3)
        await stop_deinit_session(host_responder, 3)
        await stop_deinit_session(host_anchor, 3)
    finally:
        host_responder.disconnect()


async def twr_block_stride_ranging(host_controller: Host, host_controlee: Host):
    sid = 0

    async def init_twr_session(
        host: Host,
        role: uci.DeviceRole,
        device_type: uci.DeviceType,
        peer: Host,
        duration_ms: int,
        stride: int,
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
        host.send_control(
            uci.SessionSetAppConfigCmd(
                session_token=sid,
                tlvs=[
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.DEVICE_ROLE,
                        v=bytes([role]),
                    ),
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.DEVICE_TYPE,
                        v=bytes([device_type]),
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
                        v=bytes([uci.RangingRoundUsage.DS_TWR_DEFERRED_MODE]),
                    ),
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
                        v=int.to_bytes(duration_ms, 4, "little"),
                    ),
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.BLOCK_STRIDE_LENGTH,
                        v=bytes([stride]),
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
                session_token=sid,
                session_state=uci.SessionState.SESSION_STATE_IDLE,
                reason_code=0,
            )
        )

    async def set_config(host: Host, tlvs: list[uci.AppConfigTlv]):
        host.send_control(uci.SessionSetAppConfigCmd(session_token=sid, tlvs=tlvs))
        await host.expect_control(
            uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
        )

    async def start_session(host: Host):
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

    async def stop_deinit_session(host: Host):
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
        host.send_control(uci.SessionDeinitCmd(session_token=sid))
        await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))
        await host.expect_control(
            uci.SessionStatusNtf(
                session_token=sid,
                session_state=uci.SessionState.SESSION_STATE_DEINIT,
                reason_code=0,
            )
        )

    await init_twr_session(
        host_controlee,
        uci.DeviceRole.RESPONDER,
        uci.DeviceType.CONTROLEE,
        host_controller,
        300,
        2,
    )
    await init_twr_session(
        host_controller,
        uci.DeviceRole.INITIATOR,
        uci.DeviceType.CONTROLLER,
        host_controlee,
        100,
        1,
    )

    await start_session(host_controlee)
    c1 = await host_controlee.expect_control(
        uci.ShortMacTwoWaySessionInfoNtf, timeout=1.0
    )
    assert c1.current_ranging_interval == 300
    await set_config(
        host_controlee,
        [
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.BLOCK_STRIDE_LENGTH,
                v=bytes([5]),
            )
        ],
    )
    c2 = await host_controlee.expect_control(
        uci.ShortMacTwoWaySessionInfoNtf, timeout=1.0
    )
    assert c2.current_ranging_interval == 300
    await set_config(
        host_controlee,
        [
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG,
                v=bytes([uci.SessionInfoNtfConfig.DISABLE]),
            )
        ],
    )

    await start_session(host_controller)
    event1 = await host_controller.expect_control(
        uci.ShortMacTwoWaySessionInfoNtf, timeout=1.0
    )
    assert event1.current_ranging_interval == 200
    assert event1.two_way_ranging_measurements[0].status == uci.Status.OK

    await set_config(
        host_controller,
        [
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.BLOCK_STRIDE_LENGTH,
                v=bytes([19]),
            )
        ],
    )
    with pytest.raises(TimeoutError):
        await host_controller.expect_control(
            uci.ShortMacTwoWaySessionInfoNtf, timeout=0.5
        )

    await set_config(
        host_controller,
        [
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.BLOCK_STRIDE_LENGTH,
                v=bytes([0]),
            )
        ],
    )
    event2 = await host_controller.expect_control(
        uci.ShortMacTwoWaySessionInfoNtf, timeout=1.0
    )
    assert event2.current_ranging_interval == 100
    assert event2.sequence_number == 1

    event3 = await host_controller.expect_control(
        uci.ShortMacTwoWaySessionInfoNtf, timeout=1.0
    )
    assert event3.current_ranging_interval == 100
    assert event3.sequence_number == 2

    await stop_deinit_session(host_controller)
    await stop_deinit_session(host_controlee)


async def aoa_result_req_ranging(host0: Host, host1: Host):
    def tlvs(cfg: dict) -> list[uci.AppConfigTlv]:
        return [uci.AppConfigTlv(cfg_id=k, v=bytes(v)) for k, v in cfg.items()]

    async def init_session(host: Host, sid: int, cfg: dict):
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
        host.send_control(uci.SessionSetAppConfigCmd(session_token=sid, tlvs=tlvs(cfg)))
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

    async def deinit_session(host: Host, sid: int):
        host.send_control(uci.SessionDeinitCmd(session_token=sid))
        await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))
        await host.expect_control(
            uci.SessionStatusNtf(
                session_token=sid,
                session_state=uci.SessionState.SESSION_STATE_DEINIT,
                reason_code=0,
            )
        )

    twr_sid = 10
    await init_session(
        host1,
        twr_sid,
        {
            uci.AppConfigTlvType.DEVICE_ROLE: [uci.DeviceRole.RESPONDER],
            uci.AppConfigTlvType.DEVICE_TYPE: [uci.DeviceType.CONTROLEE],
            uci.AppConfigTlvType.DEVICE_MAC_ADDRESS: host1.mac_address,
            uci.AppConfigTlvType.DST_MAC_ADDRESS: host0.mac_address,
            uci.AppConfigTlvType.MAC_ADDRESS_MODE: [uci.MacAddressMode.MODE_0],
            uci.AppConfigTlvType.MULTI_NODE_MODE: [uci.MultiNodeMode.ONE_TO_ONE],
            uci.AppConfigTlvType.SCHEDULE_MODE: [uci.ScheduleMode.CONTENTION_BASED],
            uci.AppConfigTlvType.RANGING_ROUND_USAGE: [0x06],
            uci.AppConfigTlvType.RANGING_DURATION: int(100).to_bytes(4, "little"),
            uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [
                uci.SessionInfoNtfConfig.DISABLE
            ],
        },
    )
    await start_session(host1, twr_sid)

    await init_session(
        host0,
        twr_sid,
        {
            uci.AppConfigTlvType.DEVICE_ROLE: [uci.DeviceRole.INITIATOR],
            uci.AppConfigTlvType.DEVICE_TYPE: [uci.DeviceType.CONTROLLER],
            uci.AppConfigTlvType.DEVICE_MAC_ADDRESS: host0.mac_address,
            uci.AppConfigTlvType.DST_MAC_ADDRESS: host1.mac_address,
            uci.AppConfigTlvType.MAC_ADDRESS_MODE: [uci.MacAddressMode.MODE_0],
            uci.AppConfigTlvType.MULTI_NODE_MODE: [uci.MultiNodeMode.ONE_TO_ONE],
            uci.AppConfigTlvType.SCHEDULE_MODE: [uci.ScheduleMode.CONTENTION_BASED],
            uci.AppConfigTlvType.RANGING_ROUND_USAGE: [0x06],
            uci.AppConfigTlvType.RANGING_DURATION: int(100).to_bytes(4, "little"),
            uci.AppConfigTlvType.NUMBER_OF_CONTROLEES: [1],
            uci.AppConfigTlvType.MAX_NUMBER_OF_MEASUREMENTS: int(1).to_bytes(
                2, "little"
            ),
        },
    )

    host0.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=twr_sid,
            tlvs=[
                uci.AppConfigTlv(
                    cfg_id=uci.AppConfigTlvType.AOA_RESULT_REQ, v=bytes([0xFF])
                ),
            ],
        )
    )
    await host0.expect_control(
        uci.SessionSetAppConfigRsp(
            status=uci.Status.INVALID_PARAM,
            cfg_status=[
                uci.AppConfigStatus(
                    cfg_id=uci.AppConfigTlvType.AOA_RESULT_REQ,
                    status=uci.Status.INVALID_PARAM,
                ),
            ],
        )
    )

    twr_modes = [
        (None, 100, 100),
        (uci.AoaResultReq.AOA_DISABLED, 0, 0),
        (uci.AoaResultReq.AOA_ENABLED_AZIMUTH_ONLY, 100, 0),
        (uci.AoaResultReq.AOA_ENABLED_ELEVATION_ONLY, 0, 100),
    ]

    for req, exp_az_fom, exp_el_fom in twr_modes:
        if req is not None:
            host0.send_control(
                uci.SessionSetAppConfigCmd(
                    session_token=twr_sid,
                    tlvs=tlvs({uci.AppConfigTlvType.AOA_RESULT_REQ: [req]}),
                )
            )
            await host0.expect_control(
                uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
            )

        await start_session(host0, twr_sid)
        event = await host0.expect_control(
            uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0
        )
        assert len(event.two_way_ranging_measurements) == 1
        m = event.two_way_ranging_measurements[0]
        assert m.status == uci.Status.OK
        assert m.aoa_azimuth_fom == exp_az_fom and m.aoa_elevation_fom == exp_el_fom
        if exp_az_fom == 0:
            assert m.aoa_azimuth == 0
        if exp_el_fom == 0:
            assert m.aoa_elevation == 0
        assert (
            m.aoa_destination_azimuth == 0
            and m.aoa_destination_elevation == 0
            and m.aoa_destination_azimuth_fom == 100
            and m.aoa_destination_elevation_fom == 100
        )
        await host0.expect_control(
            uci.SessionStatusNtf(
                session_token=twr_sid,
                session_state=uci.SessionState.SESSION_STATE_IDLE,
                reason_code=uci.ReasonCode.MAX_NUMBER_OF_MEASUREMENTS_REACHED,
            )
        )
        await host0.expect_control(
            uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
        )

    host0.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=twr_sid,
            tlvs=tlvs(
                {
                    uci.AppConfigTlvType.AOA_RESULT_REQ: [
                        uci.AoaResultReq.AOA_DISABLED
                    ],
                    uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [
                        uci.SessionInfoNtfConfig.ENABLE_AOA_TRIGGER
                    ],
                }
            ),
        )
    )
    await host0.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )
    await start_session(host0, twr_sid)
    await host0.expect_control(
        uci.SessionStatusNtf(
            session_token=twr_sid,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=uci.ReasonCode.MAX_NUMBER_OF_MEASUREMENTS_REACHED,
        )
    )
    await host0.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    host0.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=twr_sid,
            tlvs=tlvs(
                {
                    uci.AppConfigTlvType.AOA_RESULT_REQ: [
                        uci.AoaResultReq.AOA_DISABLED
                    ],
                    uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [
                        uci.SessionInfoNtfConfig.ENABLE_PROXIMITY_AOA_TRIGGER
                    ],
                }
            ),
        )
    )
    await host0.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )
    await start_session(host0, twr_sid)
    event = await host0.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0)
    assert len(event.two_way_ranging_measurements) == 1
    m = event.two_way_ranging_measurements[0]
    assert m.status == uci.Status.OK
    assert m.aoa_azimuth_fom == 0 and m.aoa_elevation_fom == 0
    await host0.expect_control(
        uci.SessionStatusNtf(
            session_token=twr_sid,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=uci.ReasonCode.MAX_NUMBER_OF_MEASUREMENTS_REACHED,
        )
    )
    await host0.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    host0.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=twr_sid,
            tlvs=tlvs(
                {
                    uci.AppConfigTlvType.AOA_RESULT_REQ: [
                        uci.AoaResultReq.AOA_ENABLED_AZIMUTH_ONLY
                    ],
                    uci.AppConfigTlvType.SESSION_INFO_NTF_CONFIG: [
                        uci.SessionInfoNtfConfig.ENABLE_AOA_TRIGGER
                    ],
                    uci.AppConfigTlvType.AOA_BOUND_CONFIG: (
                        int(-10 * 128).to_bytes(2, "little", signed=True)
                        + int(10 * 128).to_bytes(2, "little", signed=True)
                        + int(20 * 128).to_bytes(2, "little", signed=True)
                        + int(45 * 128).to_bytes(2, "little", signed=True)
                    ),
                }
            ),
        )
    )
    await host0.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )
    await start_session(host0, twr_sid)
    event = await host0.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0)
    assert len(event.two_way_ranging_measurements) == 1
    m = event.two_way_ranging_measurements[0]
    assert m.status == uci.Status.OK
    assert m.aoa_azimuth_fom == 100 and m.aoa_elevation_fom == 0
    await host0.expect_control(
        uci.SessionStatusNtf(
            session_token=twr_sid,
            session_state=uci.SessionState.SESSION_STATE_IDLE,
            reason_code=uci.ReasonCode.MAX_NUMBER_OF_MEASUREMENTS_REACHED,
        )
    )
    await host0.expect_control(
        uci.CoreDeviceStatusNtf(device_state=uci.DeviceState.DEVICE_STATE_READY)
    )

    await deinit_session(host0, twr_sid)
    await stop_session(host1, twr_sid)
    await deinit_session(host1, twr_sid)

    dl_sid = 11
    await init_session(
        host1,
        dl_sid,
        {
            uci.AppConfigTlvType.DEVICE_ROLE: [uci.DeviceRole.DT_ANCHOR],
            uci.AppConfigTlvType.DEVICE_TYPE: [uci.DeviceType.CONTROLLER],
            uci.AppConfigTlvType.DEVICE_MAC_ADDRESS: host1.mac_address,
            uci.AppConfigTlvType.MAC_ADDRESS_MODE: [uci.MacAddressMode.MODE_0],
            uci.AppConfigTlvType.MULTI_NODE_MODE: [uci.MultiNodeMode.ONE_TO_ONE],
            uci.AppConfigTlvType.SCHEDULE_MODE: [uci.ScheduleMode.CONTENTION_BASED],
            uci.AppConfigTlvType.RANGING_ROUND_USAGE: [
                uci.RangingRoundUsage.ONE_WAY_RANGING_DL_TDOA
            ],
            uci.AppConfigTlvType.RANGING_DURATION: int(100).to_bytes(4, "little"),
            uci.AppConfigTlvType.DL_TDOA_ANCHOR_LOCATION: bytes([0x03])
            + bytes(uci.RelativeLocation(x=10, y=20, z=30).serialize()),
            uci.AppConfigTlvType.DL_TDOA_TX_ACTIVE_RANGING_ROUNDS: [0x01],
        },
    )
    host1.send_control(
        uci.SessionUpdateDtAnchorRangingRoundsCmd(
            session_token=dl_sid,
            ranging_rounds=[
                uci.DtAnchorRangingRound(
                    round_index=0, role=uci.DtAnchorActingRole.INITIATOR
                )
            ],
        )
    )
    await host1.expect_control(
        uci.SessionUpdateDtAnchorRangingRoundsRsp(
            status=uci.Status.OK, ranging_round_indexes=[]
        )
    )
    await start_session(host1, dl_sid)

    await init_session(
        host0,
        dl_sid,
        {
            uci.AppConfigTlvType.DEVICE_ROLE: [uci.DeviceRole.DT_TAG],
            uci.AppConfigTlvType.DEVICE_TYPE: [uci.DeviceType.CONTROLEE],
            uci.AppConfigTlvType.DEVICE_MAC_ADDRESS: host0.mac_address,
            uci.AppConfigTlvType.MAC_ADDRESS_MODE: [uci.MacAddressMode.MODE_0],
            uci.AppConfigTlvType.MULTI_NODE_MODE: [uci.MultiNodeMode.ONE_TO_ONE],
            uci.AppConfigTlvType.SCHEDULE_MODE: [uci.ScheduleMode.CONTENTION_BASED],
            uci.AppConfigTlvType.RANGING_ROUND_USAGE: [
                uci.RangingRoundUsage.ONE_WAY_RANGING_DL_TDOA
            ],
            uci.AppConfigTlvType.RANGING_DURATION: int(100).to_bytes(4, "little"),
            uci.AppConfigTlvType.AOA_RESULT_REQ: [uci.AoaResultReq.AOA_DISABLED],
        },
    )
    host0.send_control(
        uci.SessionUpdateDtTagRangingRoundsCmd(
            session_token=dl_sid, ranging_round_indexes=[0]
        )
    )
    await host0.expect_control(
        uci.SessionUpdateDtTagRangingRoundsRsp(
            status=uci.Status.OK, ranging_round_indexes=[]
        )
    )
    await start_session(host0, dl_sid)

    event = await host0.expect_control(uci.ShortMacDlTDoASessionInfoNtf, timeout=2.0)
    assert len(event.dl_tdoa_measurements) == 1
    m = event.dl_tdoa_measurements[0].measurement
    assert m.status == uci.Status.OK
    assert m.relative_location == uci.RelativeLocation(x=10, y=20, z=30)
    assert (
        m.aoa_azimuth == 0
        and m.aoa_azimuth_fom == 0
        and m.aoa_elevation == 0
        and m.aoa_elevation_fom == 0
    )

    await stop_session(host0, dl_sid)
    host0.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=dl_sid,
            tlvs=tlvs(
                {
                    uci.AppConfigTlvType.AOA_RESULT_REQ: [
                        uci.AoaResultReq.AOA_ENABLED_AZIMUTH_ONLY
                    ]
                }
            ),
        )
    )
    await host0.expect_control(
        uci.SessionSetAppConfigRsp(status=uci.Status.OK, cfg_status=[])
    )
    await start_session(host0, dl_sid)

    event = await host0.expect_control(uci.ShortMacDlTDoASessionInfoNtf, timeout=2.0)
    assert len(event.dl_tdoa_measurements) == 1
    m = event.dl_tdoa_measurements[0].measurement
    assert m.status == uci.Status.OK
    assert m.relative_location == uci.RelativeLocation(x=10, y=20, z=30)
    assert (
        m.aoa_azimuth_fom == 100 and m.aoa_elevation == 0 and m.aoa_elevation_fom == 0
    )

    await stop_session(host0, dl_sid)
    await deinit_session(host0, dl_sid)
    await stop_session(host1, dl_sid)
    await deinit_session(host1, dl_sid)


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
