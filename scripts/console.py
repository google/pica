#!/usr/bin/env python3

import inspect
import readline
import socket
import sys
import time
import asyncio
from concurrent.futures import ThreadPoolExecutor

MAX_PAYLOAD_SIZE = 1024

def encode_position(x: str, y: str, z: str, azimuth: str, elevation: str) -> bytes:
    return (int(x).to_bytes(2, byteorder='little')
        + int(y).to_bytes(2, byteorder='little')
        + int(z).to_bytes(2, byteorder='little')
        + int(azimuth).to_bytes(2, byteorder='little')
        + int(elevation).to_bytes(1, byteorder='little'))

def encode_session_id(session_id: str) -> bytes:
    return int(session_id).to_bytes(4, byteorder='little')

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
    def __init__(self, reader, writer):
        self.reader = reader
        self.writer = writer

    def _send_command(self, cmd: bytes):
        self.writer.write(cmd)

    def device_reset(self, **kargs):
        """Reset the UWBS."""
        self._send_command(bytes([0x20, 0x0, 0x0, 0x1, 0x0]))

    def get_device_info(self, **kargs):
        """Retrieve the device information like (UCI version and other vendor specific info)."""
        self._send_command(bytes([0x20, 0x2, 0x0, 0x0]))

    def get_caps_info(self, **kargs):
        """Get the capability of the UWBS."""
        self._send_command(bytes([0x20, 0x3, 0x0, 0x0]))

    def session_init(self, session_id: str = '0', **kargs):
        """Initialize the session"""
        self._send_command(bytes([0x21, 0x0, 0x0, 0x5]) +
            int(session_id).to_bytes(4, byteorder='little') + bytes([0x0]))

    def session_deinit(self, session_id: str = '0', **kargs):
        """Deinitialize the session"""
        self._send_command(bytes([0x21, 0x1, 0x0, 0x4]) + encode_session_id(session_id))

    def session_set_app_config(
        self,
        session_id: str = '0',
        ranging_interval: str = '200',
        dst_mac_addresses: str = '',
        **kargs):
        """set APP Configuration Parameters for the requested UWB session."""
        encoded_dst_mac_addresses = bytes()
        for mac_address in dst_mac_addresses.split(','):
            if len(mac_address) > 0:
                encoded_dst_mac_addresses += int(mac_address).to_bytes(8, byteorder='little')

        configs = TLV()
        configs.append(0x9, int(ranging_interval).to_bytes(4, byteorder='little'))
        configs.append(0x7, encoded_dst_mac_addresses)

        self._send_command(
            bytes([0x21, 0x3, 0x0, 4 + configs.len()]) +
            encode_session_id(session_id) +
            configs.encode())

    def session_get_app_config(self, session_id: str = '0', **kargs):
        """retrieve the current APP Configuration Parameters of the requested UWB session."""
        self._send_command(
            bytes([0x21, 0x4, 0x0, 0x5]) +
            encode_session_id(session_id) +
            bytes([0x0]))

    def session_get_count(self, **kargs):
        """Retrieve number of UWB sessions in the UWBS."""
        self._send_command(bytes([0x21, 0x5, 0x0, 0x0]))

    def session_get_state(self, session_id: str = '0', **kargs):
        """Query the current state of the UWB session."""
        self._send_command(bytes([0x21, 0x6, 0x0, 0x4]) + encode_session_id(session_id))

    def range_start(self, session_id: str = '0', **kargs):
        """start a UWB session."""
        self._send_command(bytes([0x22, 0x0, 0x0, 0x4]) + encode_session_id(session_id))

    def range_stop(self, session_id: str = '0', **kargs):
        """Stop a UWB session."""
        self._send_command(bytes([0x22, 0x1, 0x0, 0x4]) + encode_session_id(session_id))

    def get_ranging_count(self, session_id: str = '0', **kargs):
        """Get the number of times ranging has been attempted during the ranging session.."""
        self._send_command(bytes([0x22, 0x3, 0x0, 0x4]) + encode_session_id(session_id))

    def pica_create_beacon(
        self,
        mac_address: str = '0',
        x: str = '0',
        y: str= '0',
        z: str = '0',
        azimuth: str = '0',
        elevation: str = '0',
        **kargs):
        """Create a Pica beacon"""
        self._send_command(
            bytes([0x29, 0x2, 0x0, 17]) +
            encode_mac_address(mac_address) +
            encode_position(int(x), int(y), int(z), int(azimuth), int(elevation)))


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
            buffer =  bytes()
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
                uci_packet = uci_packets.UciPacket.parse(packet)
                pprint.pprint(uci_packet, compact=False)
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
        'device_reset': device.device_reset,
        'get_device_info': device.get_device_info,
        'get_caps_info': device.get_caps_info,
        'session_init': device.session_init,
        'session_deinit': device.session_deinit,
        'session_set_app_config': device.session_set_app_config,
        'session_get_app_config': device.session_get_app_config,
        'session_get_count': device.session_get_count,
        'session_get_state': device.session_get_state,
        'range_start': device.range_start,
        'range_stop': device.range_stop,
        'get_ranging_count': device.get_ranging_count,
        'pica_create_beacon': device.pica_create_beacon,
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
            results = [name + '=' for name in names if name.startswith(tokens[-1])]

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

async def main():
    # Connect to Pica
    try:
        reader, writer = await asyncio.open_connection('127.0.0.1', 7000)
    except Exception as exn:
        print(
            'Failed to connect to Pica server\n' +
            'Make sure the server is running locally on port 7000')
        exit(1)

    # Start input and receive loops
    device = Device(reader, writer)
    loop = asyncio.get_event_loop()
    loop.create_task(device.read_responses_and_notifications())
    await command_line(device)

if __name__ == '__main__':
    asyncio.run(main())
