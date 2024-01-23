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

import argparse
import inspect
import json
import random
import readline
import socket
import sys
import time
import requests
import struct
import asyncio
from concurrent.futures import ThreadPoolExecutor

from pica import Host
from pica.packets import uci

MAX_DATA_PACKET_PAYLOAD_SIZE = 1024


def encode_short_mac_address(mac_address: str) -> bytes:
    return int(mac_address).to_bytes(2, byteorder="little")


def encode_mac_address(mac_address: str) -> bytes:
    return int(mac_address).to_bytes(8, byteorder="little")


def parse_mac_address(mac_address: str) -> bytes:
    bs = mac_address.split(":")
    return bytes(int(b, 16) for b in bs)


class Device:
    def __init__(self, reader, writer, http_address):
        self.host = Host(reader, writer, bytes([0, 1]))
        self.http_address = http_address

    def pica_get_state(self, **kargs):
        """List the UCI devices and anchors currently existing within the Pica
        virtual environment"""
        r = requests.get(f"{self.http_address}/get-state")
        print(f"{r.status_code}:\n{json.dumps(r.json(), indent=2)}")

    def pica_init_uci_device(
        self,
        mac_address: str = "00:00",
        x: str = "0",
        y: str = "0",
        z: str = "0",
        yaw: str = "0",
        pitch: str = "0",
        roll: str = "0",
        **kargs,
    ):
        """Init Pica device"""
        r = requests.post(
            f"{self.http_address}/init-uci-device/{mac_address}",
            data=json.dumps(
                {
                    "x": int(x),
                    "y": int(y),
                    "z": int(z),
                    "yaw": int(yaw),
                    "pitch": int(pitch),
                    "roll": int(roll),
                }
            ),
        )
        print(f"{r.status_code}: {r.text}")

    def pica_create_anchor(
        self,
        mac_address: str = "00:00",
        x: str = "0",
        y: str = "0",
        z: str = "0",
        yaw: str = "0",
        pitch: str = "0",
        roll: str = "0",
        **kargs,
    ):
        """Create a Pica anchor"""
        r = requests.post(
            f"{self.http_address}/create-anchor/{mac_address}",
            data=json.dumps(
                {
                    "x": int(x),
                    "y": int(y),
                    "z": int(z),
                    "yaw": int(yaw),
                    "pitch": int(pitch),
                    "roll": int(roll),
                }
            ),
        )
        print(f"{r.status_code}: {r.text}")

    def pica_destroy_anchor(self, mac_address: str = "00:00", **kargs):
        """Destroy a Pica anchor"""
        r = requests.post(f"{self.http_address}/destroy-anchor/{mac_address}")
        print(f"{r.status_code}: {r.text}")

    def pica_set_position(
        self,
        mac_address: str = "00:00",
        x: str = "0",
        y: str = "0",
        z: str = "0",
        yaw: str = "0",
        pitch: str = "0",
        roll: str = "0",
        **kargs,
    ):
        """Set Pica UCI device or anchor position"""
        r = requests.post(
            f"{self.http_address}/set-position/{mac_address}",
            data=json.dumps(
                {
                    "x": int(x),
                    "y": int(y),
                    "z": int(z),
                    "yaw": int(yaw),
                    "pitch": int(pitch),
                    "roll": int(roll),
                }
            ),
        )
        print(f"{r.status_code}: {r.text}")

    def device_reset(self, **kargs):
        """Reset the UWBS."""
        self.host.send_control(
            uci.DeviceResetCmd(reset_config=uci.ResetConfig.UWBS_RESET)
        )

    def get_device_info(self, **kargs):
        """Retrieve the device information like (UCI version and other vendor specific info)."""
        self.host.send_control(uci.GetDeviceInfoCmd())

    def get_caps_info(self, **kargs):
        """Get the capability of the UWBS."""
        self.host.send_control(uci.GetCapsInfoCmd())

    def set_config(self, low_power_mode: str = "0", **kargs):
        """Set the configuration parameters on the UWBS."""
        self.host.send_control(
            uci.SetConfigCmd(
                tlvs=[
                    uci.DeviceConfigTlv(
                        cfg_id=uci.DeviceConfigId.LOW_POWER_MODE,
                        v=bytes([int(low_power_mode)]),
                    ),
                ]
            )
        )

    def get_config(self, **kargs):
        """Retrieve the current configuration parameter(s) of the UWBS."""
        self.host.send_control(
            uci.GetConfigCmd(
                cfg_id=[
                    uci.DeviceConfigId.LOW_POWER_MODE,
                    uci.DeviceConfigId.DEVICE_STATE,
                ]
            )
        )

    def session_init(self, session_id: str = "0", **kargs):
        """Initialize the session"""
        self.host.send_control(
            uci.SessionInitCmd(
                session_id=int(session_id),
                session_type=uci.SessionType.FIRA_RANGING_AND_IN_BAND_DATA_SESSION,
            )
        )

    def session_deinit(self, session_id: str = "0", **kargs):
        """Deinitialize the session"""
        self.host.send_control(uci.SessionDeinitCmd(session_token=int(session_id)))

    def session_set_app_config(
        self,
        session_id: str = "0",
        ranging_interval: str = "200",
        dst_mac_addresses: str = "",
        **kargs,
    ):
        """set APP Configuration Parameters for the requested UWB session."""
        dst_mac_addresses = [
            parse_mac_address(a) for a in dst_mac_addresses.split(",") if a
        ]
        if any(len(a) > 2 for a in dst_mac_addresses):
            mac_address_mode = 0x2
            mac_address_len = 8
        else:
            mac_address_mode = 0x0
            mac_address_len = 2

        encoded_dst_mac_addresses = bytes()
        for mac_address in dst_mac_addresses:
            encoded_dst_mac_addresses += mac_address
            encoded_dst_mac_addresses += b"\0" * (mac_address_len - len(mac_address))

        self.host.send_control(
            uci.SessionSetAppConfigCmd(
                session_token=int(session_id),
                tlvs=[
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.MAC_ADDRESS_MODE,
                        v=bytes([mac_address_mode]),
                    ),
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.RANGING_DURATION,
                        v=int(ranging_interval).to_bytes(4, byteorder="little"),
                    ),
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.NO_OF_CONTROLEE,
                        v=bytes([len(dst_mac_addresses)]),
                    ),
                    uci.AppConfigTlv(
                        cfg_id=uci.AppConfigTlvType.DST_MAC_ADDRESS,
                        v=encoded_dst_mac_addresses,
                    ),
                ],
            )
        )

    def session_get_app_config(self, session_id: str = "0", **kargs):
        """retrieve the current APP Configuration Parameters of the requested UWB session."""
        self.host.send_control(
            uci.SessionGetAppConfigCmd(session_token=int(session_id), app_cfg=[0x9])
        )

    def session_get_count(self, **kargs):
        """Retrieve number of UWB sessions in the UWBS."""
        self.host.send_control(uci.SessionGetCountCmd())

    def session_get_state(self, session_id: str = "0", **kargs):
        """Query the current state of the UWB session."""
        self.host.send_control(uci.SessionGetStateCmd(session_token=int(session_id)))

    def session_update_controller_multicast_list(
        self,
        session_id: str = "0",
        action: str = "add",
        mac_address: str = "0",
        subsession_id: str = "0",
        **kargs,
    ):
        """Update the controller multicast list."""

        if action == "add":
            encoded_action = uci.UpdateMulticastListAction.ADD_CONTROLEE
        elif action == "remove":
            encoded_action = uci.UpdateMulticastListAction.REMOVE_CONTROLEE
        else:
            print(f"Unexpected action: '{action}', expected add or remove")
            return

        self.host.send_control(
            uci.SessionUpdateControllerMulticastListCmd(
                session_token=int(session_id),
                action=encoded_action,
                payload=uci.SessionUpdateControllerMulticastListCmdPayload(
                    controlees=[
                        uci.Controlee(
                            short_address=encode_short_mac_address(mac_address),
                            subsession_id=int(subsession_id),
                        )
                    ],
                ).serialize(),
            )
        )

    def range_start(self, session_id: str = "0", **kargs):
        """start a UWB session."""
        self.host.send_control(uci.SessionStartCmd(session_id=int(session_id)))

    def range_stop(self, session_id: str = "0", **kargs):
        """Stop a UWB session."""
        self.host.send_control(uci.SessionStopCmd(session_id=int(session_id)))

    def get_ranging_count(self, session_id: str = "0", **kargs):
        """Get the number of times ranging has been attempted during the ranging session.."""
        self.host.send_control(
            uci.SessionGetRangingCountCmd(session_id=int(session_id))
        )

    def data_transfer(
        self,
        dst_mac_address,
        file_name,
        session_id: str = "0",
    ):
        """Initiates data transfer by sending (possibly segmented) UCI data packet(s)."""

        # Does not have flow control, i.e. waiting for data credit notifications in between sending packets
        try:
            with open(file_name, "rb") as f:
                b = f.read()
                seq_num = 0
                dst_mac_address = parse_mac_address(dst_mac_address)

                if len(b) > MAX_DATA_PACKET_PAYLOAD_SIZE:
                    for i in range(0, len(b), MAX_DATA_PACKET_PAYLOAD_SIZE):
                        section = b[i : i + MAX_DATA_PACKET_PAYLOAD_SIZE]

                        if i + MAX_DATA_PACKET_PAYLOAD_SIZE >= len(b):
                            self.host.send_data(
                                uci.DataMessageSnd(
                                    session_handle=int(session_id),
                                    destination_address=int.from_bytes(dst_mac_address),
                                    data_sequence_number=seq_num,
                                    application_data=section,
                                )
                            )
                        else:
                            self.host.send_data(
                                uci.DataMessageSnd(
                                    session_handle=int(session_id),
                                    pbf=uci.PacketBoundaryFlag.NOT_COMPLETE,
                                    destination_address=int.from_bytes(dst_mac_address),
                                    data_sequence_number=seq_num,
                                    application_data=section,
                                )
                            )

                        seq_num += 1
                        if seq_num >= 65535:
                            seq_num = 0
                else:
                    self.host.send_data(
                        uci.DataMessageSnd(
                            session_handle=int(session_id),
                            destination_address=int.from_bytes(dst_mac_address),
                            data_sequence_number=seq_num,
                            application_data=b,
                        )
                    )

        except Exception as e:
            print(e)

    async def read_responses_and_notifications(self):
        def chunks(l, n):
            for i in range(0, len(l), n):
                yield l[i : i + n]

        while True:
            packet = await self.host._recv_control()

            # Format and print raw response data
            txt = "\n  ".join(
                [
                    " ".join(["{:02x}".format(b) for b in shard])
                    for shard in chunks(packet, 16)
                ]
            )

            command_buffer = readline.get_line_buffer()
            print("\r", end="")
            print(f"Received UCI packet [{len(packet)}]:")
            print(f"  {txt}")

            try:
                uci_packet = uci.ControlPacket.parse_all(packet)
                uci_packet.show()
            except Exception as exn:
                pass

            print(f"--> {command_buffer}", end="", flush=True)


async def ainput(prompt: str = ""):
    with ThreadPoolExecutor(1, "ainput") as executor:
        return (
            await asyncio.get_event_loop().run_in_executor(executor, input, prompt)
        ).rstrip()


async def get_stream_reader(pipe) -> asyncio.StreamReader:
    loop = asyncio.get_event_loop()
    reader = asyncio.StreamReader(loop=loop)
    protocol = asyncio.StreamReaderProtocol(reader)
    await loop.connect_read_pipe(lambda: protocol, pipe)
    return reader


async def command_line(device: Device):
    commands = {
        "pica_get_state": device.pica_get_state,
        "pica_init_uci_device": device.pica_init_uci_device,
        "pica_create_anchor": device.pica_create_anchor,
        "pica_destroy_anchor": device.pica_destroy_anchor,
        "pica_set_position": device.pica_set_position,
        "device_reset": device.device_reset,
        "get_device_info": device.get_device_info,
        "get_config": device.get_config,
        "set_config": device.set_config,
        "get_caps_info": device.get_caps_info,
        "session_init": device.session_init,
        "session_deinit": device.session_deinit,
        "session_set_app_config": device.session_set_app_config,
        "session_get_app_config": device.session_get_app_config,
        "session_get_count": device.session_get_count,
        "session_get_state": device.session_get_state,
        "session_update_controller_multicast_list": device.session_update_controller_multicast_list,
        "range_start": device.range_start,
        "range_stop": device.range_stop,
        "data_transfer": device.data_transfer,
        "get_ranging_count": device.get_ranging_count,
    }

    def usage():
        for cmd, func in commands.items():
            print(f"  {cmd.ljust(32)}{func.__doc__}")

    def complete(text, state):
        tokens = readline.get_line_buffer().split()
        if not tokens or readline.get_line_buffer()[-1] == " ":
            tokens.append("")

        # Writing a command name, complete to ' '
        if len(tokens) == 1:
            results = [cmd + " " for cmd in commands.keys() if cmd.startswith(text)]

        # Writing a keyword argument, no completion
        elif "=" in tokens[-1]:
            results = []

        # Writing a keyword name, but unknown command, no completion
        elif tokens[0] not in commands:
            results = []

        # Writing a keyword name, complete to '='
        else:
            sig = inspect.signature(commands[tokens[0]])
            names = [
                name
                for (name, p) in sig.parameters.items()
                if (
                    p.kind == inspect.Parameter.POSITIONAL_OR_KEYWORD
                    or p.kind == inspect.Parameter.KEYWORD_ONLY
                )
            ]
            results = [name + "=" for name in names if name.startswith(tokens[-1])]

        results += [None]
        return results[state]

    # Configure readline
    readline.parse_and_bind("tab: complete")
    readline.set_completer(complete)

    while True:
        cmd = await ainput("--> ")
        [cmd, *params] = cmd.split(" ")
        args = []
        kargs = dict()
        for param in params:
            if len(param) == 0:
                continue
            elif "=" in param:
                [key, value] = param.split("=")
                kargs[key] = value
            else:
                args.append(param)

        if cmd in ["quit", "q"]:
            break
        if cmd not in commands:
            print(f"Undefined command {cmd}")
            usage()
            continue
        commands[cmd](*args, **kargs)


async def run(address: str, uci_port: int, http_port: int):
    try:
        # Connect to Pica
        reader, writer = await asyncio.open_connection(address, uci_port)
    except Exception as exn:
        print(
            f"Failed to connect to Pica server at address {address}:{uci_port}\n"
            + "Make sure the server is running"
        )
        exit(1)

    # Start input and receive loops
    device = Device(reader, writer, f"http://{address}:{http_port}")
    loop = asyncio.get_event_loop()
    loop.create_task(device.read_responses_and_notifications())
    await command_line(device)


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
        "--http-port", type=int, default=3000, help="Select the pica HTTP port"
    )
    asyncio.run(run(**vars(parser.parse_args())))


if __name__ == "__main__":
    main()
