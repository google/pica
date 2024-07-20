// Copyright 2023, Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Packet parsers and serializers.

/// UCI packet parser and serializer.
pub mod uci {
    #![allow(clippy::all)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    #![allow(missing_docs)]

    include!(concat!(env!("OUT_DIR"), "/uci_packets.rs"));

    impl ControlPacket {
        pub fn is_core_device_reset_cmd(&self) -> bool {
            let Ok(core_packet) = CorePacket::try_from(self) else {
                return false;
            };
            core_packet.oid == CoreOpcodeId::DeviceReset
        }
    }

    /// Size of common UCI packet header.
    pub const COMMON_HEADER_SIZE: usize = 1;
    /// Size of UCI packet headers.
    pub const HEADER_SIZE: usize = 4;
    /// Maximum size of an UCI control packet payload.
    pub const MAX_CTRL_PACKET_PAYLOAD_SIZE: usize = 255;
    /// Maximum size of an UCI data packet payload.
    pub const MAX_DATA_PACKET_PAYLOAD_SIZE: usize = 1024;

    // Extract the message type from the first 3 bits of the passed (header) byte
    pub fn parse_message_type(byte: u8) -> MessageType {
        MessageType::try_from((byte >> 5) & 0x7).unwrap_or(MessageType::Command)
    }

    /// Read a single UCI packet from a TCP read half.
    /// This function does not reassemble segmented packets.
    pub async fn read(
        mut socket: tokio::net::tcp::OwnedReadHalf,
    ) -> Option<(Vec<u8>, tokio::net::tcp::OwnedReadHalf)> {
        use tokio::io::AsyncReadExt;

        let mut packet = vec![0; HEADER_SIZE];

        // Read the common packet header.
        socket.read_exact(&mut packet[0..HEADER_SIZE]).await.ok()?;

        let common_packet_header =
            CommonPacketHeader::decode_full(&packet[0..COMMON_HEADER_SIZE]).ok()?;

        let payload_length = match common_packet_header.mt {
            MessageType::Data => {
                let data_packet_header =
                    DataPacketHeader::decode_full(&packet[0..HEADER_SIZE]).ok()?;
                data_packet_header.payload_length as usize
            }
            _ => {
                let control_packet_header =
                    ControlPacketHeader::decode_full(&packet[0..HEADER_SIZE]).ok()?;
                control_packet_header.payload_length as usize
            }
        };

        // Read the packet payload.
        packet.resize(payload_length + HEADER_SIZE, 0);
        socket.read_exact(&mut packet[HEADER_SIZE..]).await.ok()?;

        Some((packet, socket))
    }

    /// Write a single UCI packet to a TCP write half.
    /// This function accepts segmented packets only.
    pub async fn write(
        mut socket: tokio::net::tcp::OwnedWriteHalf,
        packet: Vec<u8>,
    ) -> std::result::Result<tokio::net::tcp::OwnedWriteHalf, anyhow::Error> {
        use tokio::io::AsyncWriteExt;

        socket.write_all(&packet).await?;

        Ok(socket)
    }
}
