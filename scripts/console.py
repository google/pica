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
import uci_packets

def encode_position(x: int, y: int, z: int, yaw: int, pitch: int, roll: int) -> bytes:
    return (struct.pack('<h', x)
            + struct.pack('<h', y)
            + struct.pack('<h', z)
            + struct.pack('<h', yaw)
            + struct.pack('<b', pitch)
            + struct.pack('<h', roll))


def encode_session_id(session_id: str) -> bytes:
    return int(session_id).to_bytes(4, byteorder='little')


def encode_short_mac_address(mac_address: str) -> bytes:
    return int(mac_address).to_bytes(2, byteorder='little')


def encode_mac_address(mac_address: str) -> bytes:
    return int(mac_address).to_bytes(8, byteorder='little')


def encode_tlv(typ: int, value: bytes):
    return bytes([typ, len(value)]) + value if len(value) > 0 else bytes([])


class TLV:
    def __init__(self):
        self.count = 0
        self.buffer = bytes()

    def append(self, tag: int, value: bytes):
        if len(value) > 0:
            self.count += 1
            self.buffer += encode_tlv(tag, value)

    def len(self) -> int:
        return len(self.buffer) + 1

    def encode(self) -> bytes:
        return bytes([self.count]) + self.buffer


class Device:
    def __init__(self, reader, writer, http_address):
        self.reader = reader
        self.writer = writer
        self.http_address = http_address

    def pica_get_state(
            self,
            **kargs):
        """List the UCI devices and anchors currently existing within the Pica
        virtual environment"""
        r = requests.get(f'{self.http_address}/get-state')
        print(f'{r.status_code}:\n{json.dumps(r.json(), indent=2)}')

    def pica_init_uci_device(
            self,
            x: str = "0",
            y: str = "0",
            z: str = "0",
            yaw: str = "0",
            pitch: str = "0",
            roll: str = "0",
            **kargs):
        """Init Pica device"""
        r = requests.post(f'{self.http_address}/init-uci-device/{mac_address}',
            data=json.dumps({
                'x': int(x), 'y': int(y), 'z': int(z),
                'yaw': int(yaw), 'pitch': int(pitch), 'roll': int(roll)
            }))
        print(f'{r.status_code}: {r.text}')

    def pica_create_anchor(
            self,
            mac_address: str = "00:00",
            x: str = "0",
            y: str = "0",
            z: str = "0",
            yaw: str = "0",
            pitch: str = "0",
            roll: str = "0",
            **kargs):
        """Create a Pica anchor"""
        r = requests.post(f'{self.http_address}/create-anchor/{mac_address}',
            data=json.dumps({
                'x': int(x), 'y': int(y), 'z': int(z),
                'yaw': int(yaw), 'pitch': int(pitch), 'roll': int(roll)
            }))
        print(f'{r.status_code}: {r.text}')

    def pica_destroy_anchor(
            self,
            mac_address: str = "00:00",
            **kargs):
        """Destroy a Pica anchor"""
        r = requests.post(f'{self.http_address}/destroy-anchor/{mac_address}')
        print(f'{r.status_code}: {r.text}')

    def pica_set_position(
            self,
            mac_address: str = "00:00",
            x: str = "0",
            y: str = "0",
            z: str = "0",
            yaw: str = "0",
            pitch: str = "0",
            roll: str = "0",
            **kargs):
        """Set Pica UCI device or anchor position"""
        r = requests.post(f'{self.http_address}/set-position/{mac_address}',
            data=json.dumps({
                'x': int(x), 'y': int(y), 'z': int(z),
                'yaw': int(yaw), 'pitch': int(pitch), 'roll': int(roll)
            }))
        print(f'{r.status_code}: {r.text}')

    def _send_command(self, group_id: int, opcode_id: int, payload: bytes):
        """Sends a UCI command without fragmentation"""
        command = bytes([0x20 | group_id, opcode_id,
                        0, len(payload)]) + payload
        self.writer.write(command)

    def raw(self,
            group_id: str = "0",
            opcode_id: str = "0",
            payload_size: str = "0",
            **kargs):
        """Send raw commands with random payload."""
        self._send_command(
            int(group_id), int(opcode_id),
            random.randbytes(int(payload_size)))

    def device_reset(self, **kargs):
        """Reset the UWBS."""
        self._send_command(0, 0, bytes([0x0]))

    def get_device_info(self, **kargs):
        """Retrieve the device information like (UCI version and other vendor specific info)."""
        self._send_command(0, 2, bytes([]))

    def get_caps_info(self, **kargs):
        """Get the capability of the UWBS."""
        self._send_command(0, 3, bytes([]))

    def set_config(self, low_power_mode: str = '0', **kargs):
        """Set the configuration parameters on the UWBS."""
        self._send_command(0, 4, bytes([1, 1, 1, int(low_power_mode)]))

    def get_config(self, **kargs):
        """Retrieve the current configuration parameter(s) of the UWBS."""
        self._send_command(0, 5, bytes([2, 0, 1]))

    def session_init(self, session_id: str = '0', **kargs):
        """Initialize the session"""
        self._send_command(1, 0,
                           int(session_id).to_bytes(4, byteorder='little') + bytes([0x0]))

    def session_deinit(self, session_id: str = '0', **kargs):
        """Deinitialize the session"""
        self._send_command(1, 1, encode_session_id(session_id))

    def session_set_app_config(
            self,
            session_id: str = '0',
            ranging_interval: str = '200',
            dst_mac_addresses: str = '',
            **kargs):
        """set APP Configuration Parameters for the requested UWB session."""
        encoded_dst_mac_addresses = bytes()
        dst_mac_addresses_count = 0
        for mac_address in dst_mac_addresses.split(','):
            if len(mac_address) > 0:
                dst_mac_addresses_count += 1
                encoded_dst_mac_addresses += int(
                    mac_address).to_bytes(8, byteorder='little')

        configs = TLV()
        configs.append(0x26, bytes([0x2]))  # MAC Address Mode #2
        configs.append(0x5, bytes([dst_mac_addresses_count]))
        configs.append(0x9, int(ranging_interval).to_bytes(
            4, byteorder='little'))
        configs.append(0x7, encoded_dst_mac_addresses)

        self._send_command(1, 3,
                           encode_session_id(session_id) +
                           configs.encode())

    def session_get_app_config(self, session_id: str = '0', **kargs):
        """retrieve the current APP Configuration Parameters of the requested UWB session."""
        self._send_command(1, 4,
                           encode_session_id(session_id) +
                           bytes([1, 0x9]))

    def session_get_count(self, **kargs):
        """Retrieve number of UWB sessions in the UWBS."""
        self._send_command(1, 5, bytes([]))

    def session_get_state(self, session_id: str = '0', **kargs):
        """Query the current state of the UWB session."""
        self._send_command(1, 6, encode_session_id(session_id))

    def session_update_controller_multicast_list(
            self,
            session_id: str = '0',
            action: str = 'add',
            mac_address: str = '0',
            subsession_id: str = '0',
            **kargs):
        """Update the controller multicast list."""

        if action == 'add':
            encoded_action = bytes([0])
        elif action == 'remove':
            encoded_action = bytes([1])
        else:
            print(f"Unexpected action: '{action}', expected add or remove")
            return

        self._send_command(1, 7,
                           encode_session_id(session_id) +
                           encoded_action +
                           bytes([1]) +
                           encode_short_mac_address(mac_address) +
                           encode_session_id(subsession_id))

    def range_start(self, session_id: str = '0', **kargs):
        """start a UWB session."""
        self._send_command(2, 0, encode_session_id(session_id))

    def range_stop(self, session_id: str = '0', **kargs):
        """Stop a UWB session."""
        self._send_command(2, 1, encode_session_id(session_id))

    def get_ranging_count(self, session_id: str = '0', **kargs):
        """Get the number of times ranging has been attempted during the ranging session.."""
        self._send_command(2, 3, encode_session_id(session_id))

    async def read_responses_and_notifications(self):
        def chunks(l, n):
            for i in range(0, len(l), n):
                yield l[i:i + n]

        packet = bytes()
        buffer = bytes()
        expect = 4
        have_header = False

        while True:
            chunk = await self.reader.read(expect)

            # Disconnected from Pica
            if len(chunk) == 0:
                break

            # Waiting for more bytes
            buffer += chunk
            if len(buffer) < expect:
                continue

            packet += buffer
            buffer = bytes()
            if not have_header:
                have_header = True
                expect = packet[3]
            else:
                # Format and print raw response data
                txt = '\n  '.join([
                    ' '.join(['{:02x}'.format(b) for b in shard]) for
                    shard in chunks(packet, 16)])

                command_buffer = readline.get_line_buffer()
                print('\r', end='')
                print(f'Received UCI packet [{len(packet)}]:')
                print(f'  {txt}')
                uci_packet = uci_packets.UciPacket.parse_all(packet)
                uci_packet.show()
                print(f'--> {command_buffer}', end='', flush=True)

                packet = bytes()
                have_header = False
                expect = 4


async def ainput(prompt: str = ''):
    with ThreadPoolExecutor(1, 'ainput') as executor:
        return (await asyncio.get_event_loop().run_in_executor(executor, input, prompt)).rstrip()


async def get_stream_reader(pipe) -> asyncio.StreamReader:
    loop = asyncio.get_event_loop()
    reader = asyncio.StreamReader(loop=loop)
    protocol = asyncio.StreamReaderProtocol(reader)
    await loop.connect_read_pipe(lambda: protocol, pipe)
    return reader


async def command_line(device: Device):
    commands = {
        'pica_get_state': device.pica_get_state,
        'pica_init_uci_device': device.pica_init_uci_device,
        'pica_create_anchor': device.pica_create_anchor,
        'pica_destroy_anchor': device.pica_destroy_anchor,
        'pica_set_position': device.pica_set_position,
        'device_reset': device.device_reset,
        'get_device_info': device.get_device_info,
        'get_config': device.get_config,
        'set_config': device.set_config,
        'get_caps_info': device.get_caps_info,
        'session_init': device.session_init,
        'session_deinit': device.session_deinit,
        'session_set_app_config': device.session_set_app_config,
        'session_get_app_config': device.session_get_app_config,
        'session_get_count': device.session_get_count,
        'session_get_state': device.session_get_state,
        'session_update_controller_multicast_list': device.session_update_controller_multicast_list,
        'range_start': device.range_start,
        'range_stop': device.range_stop,
        'get_ranging_count': device.get_ranging_count,
        'raw': device.raw,
    }

    def usage():
        for (cmd, func) in commands.items():
            print(f'  {cmd.ljust(32)}{func.__doc__}')

    def complete(text, state):
        tokens = readline.get_line_buffer().split()
        if not tokens or readline.get_line_buffer()[-1] == ' ':
            tokens.append('')

        # Writing a command name, complete to ' '
        if len(tokens) == 1:
            results = [cmd + ' ' for cmd in commands.keys() if
                       cmd.startswith(text)]

        # Writing a keyword argument, no completion
        elif '=' in tokens[-1]:
            results = []

        # Writing a keyword name, but unknown command, no completion
        elif tokens[0] not in commands:
            results = []

        # Writing a keyword name, complete to '='
        else:
            sig = inspect.signature(commands[tokens[0]])
            names = [name for (name, p) in sig.parameters.items()
                     if (p.kind == inspect.Parameter.POSITIONAL_OR_KEYWORD or
                         p.kind == inspect.Parameter.KEYWORD_ONLY)]
            results = [
                name + '=' for name in names if name.startswith(tokens[-1])]

        results += [None]
        return results[state]

    # Configure readline
    readline.parse_and_bind("tab: complete")
    readline.set_completer(complete)

    while True:
        cmd = await ainput('--> ')
        [cmd, *params] = cmd.split(' ')
        args = []
        kargs = dict()
        for param in params:
            if len(param) == 0:
                continue
            elif '=' in param:
                [key, value] = param.split('=')
                kargs[key] = value
            else:
                args.append(param)

        if cmd in ['quit', 'q']:
            break
        if cmd not in commands:
            print(f'Undefined command {cmd}')
            usage()
            continue
        commands[cmd](*args, **kargs)


async def run(address: str, uci_port: int, http_port: int):
    try:
        # Connect to Pica
        reader, writer = await asyncio.open_connection(address, uci_port)
    except Exception as exn:
        print(
            f'Failed to connect to Pica server at address {address}:{uci_port}\n' +
            'Make sure the server is running')
        exit(1)

    # Start input and receive loops
    device = Device(reader, writer, f'http://{address}:{http_port}')
    loop = asyncio.get_event_loop()
    loop.create_task(device.read_responses_and_notifications())
    await command_line(device)


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
