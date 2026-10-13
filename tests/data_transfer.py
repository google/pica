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

MAX_DATA_PACKET_PAYLOAD_SIZE = 1020
MAX_DATA_MESSAGE_SIZE = 4096


async def controller(
    host: Host,
    peer: Host,
    file: Path,
    session_type: uci.SessionType = uci.SessionType.FIRA_RANGING_AND_IN_BAND_DATA_SESSION,
    init_device: bool = True,
    data_repetition_count: int = 0,
):
    if init_device:
        await init(host)

    host.send_control(
        uci.SessionInitCmd(
            session_id=0,
            session_type=session_type,
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
    ranging_duration = int(200).to_bytes(4, byteorder="little")

    tlvs = [
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
        uci.AppConfigTlv(
            cfg_id=uci.AppConfigTlvType.SESSION_DATA_TRANSFER_STATUS_NTF_CONFIG,
            v=bytes([uci.SessionDataTransferStatusNtfConfig.ENABLE]),
        ),
    ]
    if data_repetition_count > 0:
        tlvs.append(
            uci.AppConfigTlv(
                cfg_id=uci.AppConfigTlvType.DATA_REPETITION_COUNT,
                v=bytes([data_repetition_count]),
            )
        )

    host.send_control(
        uci.SessionSetAppConfigCmd(
            session_token=0,
            tlvs=tlvs,
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

    await data_transfer(host, peer.mac_address, file, 0)

    for rep in range(1, data_repetition_count + 1):
        await host.expect_control(
            uci.SessionDataTransferStatusNtf(
                session_token=0,
                uci_sequence_number=1,
                status=uci.DataTransferNtfStatusCode.UCI_DATA_TRANSFER_STATUS_REPETITION_OK,
                tx_count=rep,
            ),
            timeout=2.0,
        )

    await host.expect_control(
        uci.SessionDataCreditNtf(
            session_token=0,
            credit_availability=uci.CreditAvailability.CREDIT_AVAILABLE,
        ),
        timeout=2.0,
    )

    await host.expect_control(
        uci.SessionDataTransferStatusNtf(
            session_token=0,
            uci_sequence_number=1,
            status=uci.DataTransferNtfStatusCode.UCI_DATA_TRANSFER_STATUS_OK,
            tx_count=data_repetition_count + 1,
        ),
        timeout=2.0,
    )

    if session_type != uci.SessionType.FIRA_DATA_TRANSFER_SESSION:
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
    file: Path,
    session_type: uci.SessionType = uci.SessionType.FIRA_RANGING_AND_IN_BAND_DATA_SESSION,
    init_device: bool = True,
    data_repetition_count: int = 0,
):
    if init_device:
        await init(host)

    host.send_control(
        uci.SessionInitCmd(
            session_id=0,
            session_type=session_type,
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
    ranging_duration = int(200).to_bytes(4, byteorder="little")

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
        b = f.read(MAX_DATA_MESSAGE_SIZE)
        for _ in range(data_repetition_count + 1):
            event = await host.expect_data(
                uci.DataMessageRcv(
                    session_handle=0,
                    status=uci.DataRcvStatusCode.UCI_STATUS_SUCCESS,
                    source_address=int.from_bytes(peer.mac_address, "little"),
                    data_sequence_number=1,
                    application_data=list(b),
                ),
                timeout=2.0,
            )
            event.show()

        if session_type != uci.SessionType.FIRA_DATA_TRANSFER_SESSION:
            event = await host.expect_control(
                uci.ShortMacTwoWaySessionInfoNtf, timeout=2.0
            )
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

    await host.expect_control(
        uci.SessionStatusNtf(
            session_token=0,
            session_state=uci.SessionState.SESSION_STATE_DEINIT,
            reason_code=0,
        )
    )


async def data_transfer(
    host: Host, dst_mac_address: bytes, file: Path, session_id: int
):
    with open(file, "rb") as f:
        b = f.read(MAX_DATA_MESSAGE_SIZE)

        msg = uci.DataMessageSnd(
            session_handle=int(session_id),
            destination_address=int.from_bytes(dst_mac_address, "little"),
            data_sequence_number=1,
            application_data=list(b),
        )
        raw_payload = msg.serialize()[4:]

        for i in range(0, len(raw_payload), MAX_DATA_PACKET_PAYLOAD_SIZE):
            chunk = raw_payload[i : i + MAX_DATA_PACKET_PAYLOAD_SIZE]
            pbf = (
                uci.PacketBoundaryFlag.COMPLETE
                if i + MAX_DATA_PACKET_PAYLOAD_SIZE >= len(raw_payload)
                else uci.PacketBoundaryFlag.NOT_COMPLETE
            )
            host.send_data(
                uci.DataPacket(
                    dpf=uci.DataPacketFormat.DATA_SND,
                    pbf=pbf,
                    payload=list(chunk),
                )
            )

            credit_availability = (
                uci.CreditAvailability.CREDIT_NOT_AVAILABLE
                if pbf == uci.PacketBoundaryFlag.COMPLETE
                else uci.CreditAvailability.CREDIT_AVAILABLE
            )
            event = await host.expect_control(
                uci.SessionDataCreditNtf(
                    session_token=int(session_id),
                    credit_availability=credit_availability,
                )
            )
            event.show()


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
        async with asyncio.TaskGroup() as tg:
            tg.create_task(
                controller(
                    host0,
                    host1,
                    file,
                    uci.SessionType.FIRA_DATA_TRANSFER_SESSION,
                    init_device=False,
                    data_repetition_count=1,
                )
            )
            tg.create_task(
                controlee(
                    host1,
                    host0,
                    file,
                    uci.SessionType.FIRA_DATA_TRANSFER_SESSION,
                    init_device=False,
                    data_repetition_count=1,
                )
            )
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
