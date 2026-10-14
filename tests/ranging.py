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
    assert len(rsp.tlvs) == 58
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
    assert rsp.max_data_size == 1024

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

    event = await host.expect_control(ntf_type, timeout=2.0)
    event.show()
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

        host0.send_control(uci.CoreQueryTimeStampCmd())
        rsp1 = await host0.expect_control(uci.CoreQueryTimeStampRsp)
        assert rsp1.status == uci.Status.OK
        assert rsp1.timeStamp > 0

        host0.send_control(uci.CoreQueryTimeStampCmd())
        rsp2 = await host0.expect_control(uci.CoreQueryTimeStampRsp)
        assert rsp2.status == uci.Status.OK
        assert rsp2.timeStamp >= rsp1.timeStamp

        await verify_command_error_fallbacks(host0)
    finally:
        host0.disconnect()
        host1.disconnect()

    logging.debug("Ranging test completed")


async def verify_command_error_fallbacks(host: Host):
    host.send_control(
        uci.SessionInitCmd(session_id=1, session_type=uci.SessionType.CCC)
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
    rsp = await host.expect_control(uci.SessionConfigPacket)
    assert rsp.mt == uci.MessageType.RESPONSE
    assert rsp.oid == uci.SessionConfigOpcodeId.UPDATE_DT_TAG_RANGING_ROUNDS
    assert rsp.payload == bytes([uci.Status.UNKNOWN_OID])


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
