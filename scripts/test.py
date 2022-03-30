#!/usr/bin/env python3
import socket
import sys
import time
import asyncio
from concurrent.futures import ThreadPoolExecutor

MAX_PAYLOAD_SIZE = 1024

def encode_position(x: int, y: int, z: int, azimuth: int, elevation: int):
  return (x.to_bytes(2, byteorder='little')
    + y.to_bytes(2, byteorder='little')
    + z.to_bytes(2, byteorder='little')
    + azimuth.to_bytes(2, byteorder='little')
    + elevation.to_bytes(1, byteorder='little'))

class Device:
    def __init__(self, reader, writer):
        self.reader = reader
        self.writer = writer

        # self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        # self.socket.connect((addr, port))

    def _send_command(self, cmd: bytes):
        # self.socket.send(cmd)
        self.writer.write(cmd)

    def device_reset(self):
        """Reset the UWBS."""
        self._send_command(bytes([0x20, 0x0, 0x0, 0x1, 0x0]))

    def get_device_info(self):
        """Retrieve the device information like (UCI version and other vendor specific info)."""
        self._send_command(bytes([0x20, 0x2, 0x0, 0x0]))

    def get_caps_info(self):
        """Get the capability of the UWBS."""
        self._send_command(bytes([0x20, 0x3, 0x0, 0x0]))

    def session_init(self, session_id: str = '0'):
        """Initialize the session"""
        self._send_command(bytes([0x21, 0x0, 0x0, 0x5]) +
            int(session_id).to_bytes(4, byteorder='little') + bytes([0x0]))

    def session_deinit(self, session_id: str = '0'):
        """Deinitialize the session"""
        self._send_command(bytes([0x21, 0x1, 0x0, 0x4]) + int(session_id).to_bytes(4, byteorder='little'))

    def session_get_count(self):
        """Retrieve number of UWB sessions in the UWBS."""
        self._send_command(bytes([0x21, 0x5, 0x0, 0x0]))

    def session_get_state(self, session_id: str = '0'):
        """Query the current state of the UWB session."""
        self._send_command(bytes([0x21, 0x6, 0x0, 0x4]) + int(session_id).to_bytes(4, byteorder='little'))

    def pica_create_beacon(
      self,
      mac_address: str = '0',
      x: str = '0',
      y: str= '0',
      z: str = '0',
      azimuth: str = '0',
      elevation: str = '0'):
        """Create a Pica beacon"""
        self._send_command(bytes([0x29, 0x2, 0x0, 17]) + int(mac_address).to_bytes(8, byteorder='little')
          + encode_position(int(x), int(y), int(z), int(azimuth), int(elevation)))


    async def read_responses_and_notifications(self):
        def chunks(l, n):
            for i in range(0, len(l), n):
                yield l[i:i + n]

        while True:
            packet = await self.reader.read(1024)
            # Disconnected from Pica
            if len(packet) == 0:
                break
            # Format and print raw response data
            txt = '\n  '.join([
                ' '.join(['{:02x}'.format(b) for b in shard]) for
                shard in chunks(packet, 16)])
            print(f'Received UCI packet [{len(packet)}]:')
            print(f'  {txt}')

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
        'session_get_count': device.session_get_count,
        'session_get_state': device.session_get_state,
        'pica_create_beacon': device.pica_create_beacon,
    }

    def usage():
        for (cmd, func) in commands.items():
            print(f'  {cmd.ljust(32)}{func.__doc__}')

    while True:
        cmd = await ainput('--> ')
        [cmd, *args] = cmd.split(' ')
        if cmd in ['quit', 'q']:
            break
        if cmd not in commands:
            print(f'Undefined command {cmd}')
            usage()
            continue
        commands[cmd](*args)

async def main():
    reader, writer = await asyncio.open_connection('127.0.0.1', 7000)
    device = Device(reader, writer)
    loop = asyncio.get_event_loop()
    loop.create_task(device.read_responses_and_notifications())
    await command_line(device)

if __name__ == '__main__':
    asyncio.run(main())
