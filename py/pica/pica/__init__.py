import asyncio
from typing import Union, Type, TypeVar
from .packets import uci

UciPacket = TypeVar("UciPacket", uci.DataPacket, uci.ControlPacket)


class Host:
    def __init__(self, reader, writer, mac_address: bytes):
        self.reader = reader
        self.writer = writer
        self.control_queue = asyncio.Queue()
        self.data_queue = asyncio.Queue()
        self.mac_address = mac_address

        loop = asyncio.get_event_loop()
        self.reader_task = loop.create_task(self._read_packets())

    @staticmethod
    async def connect(address: str, port: int, mac_address: bytes) -> "Host":
        reader, writer = await asyncio.open_connection(address, port)
        return Host(reader, writer, mac_address)

    def disconnect(self):
        self.writer.close()
        self.reader_task.cancel()

    async def _read_exact(self, expected_len: int) -> bytes:
        """Read an exact number of bytes from the socket.

        Raises an exception if the socket gets disconnected."""
        received = bytes()
        while len(received) < expected_len:
            chunk = await self.reader.read(expected_len - len(received))
            received += chunk
        return received

    async def _read_packet(self) -> bytes:
        """Read a single UCI packet from the socket.

        The packet is automatically re-assembled if segmented on
        the UCI transport."""

        complete_packet_bytes = bytes()

        # Note on reassembly:
        # For each segment of a Control Message, the
        # header of the Control Packet SHALL contain the same MT, GID and OID
        # values. It is correct to keep only the last header of the
        # segmented packet.
        while True:
            # Read the common packet header.
            header_bytes = await self._read_exact(4)
            header = uci.ControlPacketHeader.parse_all(header_bytes)

            # Read the packet payload.
            payload_bytes = await self._read_exact(header.payload_length)
            complete_packet_bytes += payload_bytes

            # Check the Packet Boundary Flag.
            match header.pbf:
                case uci.PacketBoundaryFlag.COMPLETE:
                    return header_bytes + complete_packet_bytes
                case uci.PacketBoundaryFlag.NOT_COMPLETE:
                    pass

    async def _read_packets(self):
        """Loop reading UCI packets from the socket.
        Receiving packets are added to the control queue."""
        try:
            while True:
                packet = await self._read_packet()
                await self.control_queue.put(packet)
        except Exception as exn:
            print(f"reader task closed")

    async def _recv_control(self) -> bytes:
        return await self.control_queue.get()

    def send_control(self, packet: uci.ControlPacket):
        # TODO packet fragmentation.
        packet = bytearray(packet.serialize())
        packet[3] = len(packet) - 4
        self.writer.write(packet)

    def send_data(self, packet: uci.DataPacket):
        packet = bytearray(packet.serialize())
        size = len(packet) - 4
        size_bytes = size.to_bytes(2, byteorder="little")
        packet[2] = size_bytes[0]
        packet[3] = size_bytes[1]
        self.writer.write(packet)

    async def expect_data(
        self,
        expected: Union[Type[uci.DataPacket], uci.DataPacket],
        timeout: float = 1.0,
    ) -> uci.DataPacket:
        """Wait for a data packet being sent from the controller.

        Raises ValueError if the packet is not well formatted.
        Raises ValueError if the packet does not match the expected type or value.
        Raises TimeoutError if no packet is received after `timeout` seconds.
        Returns the received packet on success.
        """

        return await self._expect_packet(expected, uci.DataPacket, timeout)

    async def expect_control(
        self,
        expected: Union[Type[uci.ControlPacket], uci.ControlPacket],
        timeout: float = 1.0,
    ) -> uci.ControlPacket:
        """Wait for a control packet being sent from the controller.

        Raises ValueError if the packet is not well formatted.
        Raises ValueError if the packet does not match the expected type or value.
        Raises TimeoutError if no packet is received after `timeout` seconds.
        Returns the received packet on success.
        """

        return await self._expect_packet(expected, uci.ControlPacket, timeout)

    async def _expect_packet(
        self,
        expected: Union[Type[UciPacket], UciPacket],
        expected_packet_type: Type[UciPacket],
        timeout: float = 1.0,
    ) -> UciPacket:
        """Wait for a packet being sent from the controller.

        Raises ValueError if the packet is not well formatted.
        Raises ValueError if the packet does not match the expected type or value.
        Raises TimeoutError if no packet is received after `timeout` seconds.
        Returns the received packet on success.
        """

        packet = await asyncio.wait_for(self._recv_control(), timeout=timeout)
        received = expected_packet_type.parse_all(packet)

        if isinstance(expected, type) and not isinstance(received, expected):
            raise ValueError(
                f"received unexpected packet {received.__class__.__name__},"
                + f" expected {expected.__name__}"
            )

        if isinstance(expected, expected_packet_type) and received != expected:
            raise ValueError(
                f"received unexpected packet {received.__class__.__name__},"
                + f" expected {expected.__class__.__name__}"
            )

        return received
