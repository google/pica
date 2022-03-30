#!/usr/bin/env python3
import socket
import sys
import time
import asyncio
from concurrent.futures import ThreadPoolExecutor

MAX_PAYLOAD_SIZE = 1024

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
        self._send_command(bytes([0x20, 0x0, 0x0, 0x1, 0x0]))

    def get_device_info(self):
        self._send_command(bytes([0x20, 0x2, 0x0, 0x0]))

    def get_caps_info(self):
        self._send_command(bytes([0x20, 0x3, 0x0, 0x0]))

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
    }

    print("command_line started")

    def usage():
        for cmd in commands.keys():
            print(f'  {cmd}')

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
