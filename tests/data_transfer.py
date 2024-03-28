#!/usr/bin/env python3

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

import asyncio
import argparse
import logging
from pica import Host
from pica.packets import uci
from .helper import init
from pathlib import Path

MAX_DATA_PACKET_PAYLOAD_SIZE = 1024


async def controller(host: Host, peer: Host, file: Path):
    await init(host)

    host.send_control(
        uci.SessionInitCmd(
            session_id=0,
            session_type=uci.SessionType.FIRA_RANGING_AND_IN_BAND_DATA_SESSION,
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
                    v=bytes([uci.MacAddressMode.MODE_0]),
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

    await data_transfer(host, peer.mac_address, file, 0)

    # START SESSION CMD
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
    event.show()

    event = await host.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0)
    event.show()

    # STOP SESSION
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

    # DEINIT
    host.send_control(uci.SessionDeinitCmd(session_token=0))

    await host.expect_control(uci.SessionDeinitRsp(status=uci.Status.OK))


async def controlee(host: Host, peer: Host, file: Path):
    await init(host)

    host.send_control(
        uci.SessionInitCmd(
            session_id=0,
            session_type=uci.SessionType.FIRA_RANGING_AND_IN_BAND_DATA_SESSION,
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
                    v=bytes([uci.MacAddressMode.MODE_0]),
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

    with file.open("rb") as f:
        application_data = list(bytearray(f.read()))
        event = await host.expect_data(
            uci.DataMessageRcv(
                session_handle=0,
                status=uci.Status.OK,
                source_address=int.from_bytes(peer.mac_address, "little"),
                data_sequence_number=0x01,
                application_data=application_data,
            ),
            timeout=2.0,
        )
        event.show()

    event = await host.expect_control(uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0)
    event.show()

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


async def data_transfer(
    host: Host, dst_mac_address: bytes, file: Path, session_id: int
):
    try:
        with open(file, "rb") as f:
            b = f.read()
            seq_num = 0

            if len(b) > MAX_DATA_PACKET_PAYLOAD_SIZE:
                for i in range(0, len(b), MAX_DATA_PACKET_PAYLOAD_SIZE):
                    chunk = b[i : i + MAX_DATA_PACKET_PAYLOAD_SIZE]

                    if i + MAX_DATA_PACKET_PAYLOAD_SIZE >= len(b):
                        host.send_data(
                            uci.DataMessageSnd(
                                session_handle=int(session_id),
                                destination_address=int.from_bytes(dst_mac_address),
                                data_sequence_number=seq_num,
                                application_data=chunk,
                            )
                        )
                    else:
                        host.send_data(
                            uci.DataMessageSnd(
                                session_handle=int(session_id),
                                pbf=uci.PacketBoundaryFlag.NOT_COMPLETE,
                                destination_address=int.from_bytes(dst_mac_address),
                                data_sequence_number=seq_num,
                                application_data=chunk,
                            )
                        )

                    seq_num += 1
                    if seq_num >= 65535:
                        seq_num = 0

                    event = await host.expect_control(
                        uci.SessionDataCreditNtf(
                            session_token=int(session_id),
                            credit_availability=uci.CreditAvailability.CREDIT_AVAILABLE,
                        )
                    )
                    event.show()
            else:
                host.send_data(
                    uci.DataMessageSnd(
                        session_handle=int(session_id),
                        destination_address=int.from_bytes(dst_mac_address),
                        data_sequence_number=seq_num,
                        application_data=b,
                    )
                )
                event = await host.expect_control(
                    uci.SessionDataCreditNtf(
                        session_token=int(session_id),
                        credit_availability=uci.CreditAvailability.CREDIT_AVAILABLE,
                    )
                )
                event.show()

    except Exception as e:
        print(e)


async def run(address: str, uci_port: int, file: Path):
    try:
        host0 = await Host.connect(address, uci_port, bytes([0x34, 0x12]))
        host1 = await Host.connect(address, uci_port, bytes([0x78, 0x56]))
    except Exception as e:
        raise Exception(
            f"Failed to connect to Pica server at address {address}:{uci_port}\n"
            + "Make sure the server is running"
        )

    try:
        async with asyncio.TaskGroup() as tg:
            tg.create_task(controller(host0, host1, file))
            tg.create_task(controlee(host1, host0, file))
    except Exception as e:
        raise e
    finally:
        host0.disconnect()
        host1.disconnect()
        logging.debug("Data transfer test completed")


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
    parser.add_argument(
        "--file", type=Path, required=True, help="Select the file to transfer"
    )
    asyncio.run(run(**vars(parser.parse_args())))


if __name__ == "__main__":
    main()
