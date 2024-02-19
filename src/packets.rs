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

    use crate::pcapng;
    use std::pin::Pin;
    use tokio::io::{AsyncRead, AsyncWrite};
    use tokio::sync::Mutex;

    /// Read UCI Control and Data packets received on the UCI transport.
    /// Performs recombination of the segmented packets.
    pub struct Reader {
        socket: Pin<Box<dyn AsyncRead + Send>>,
    }

    /// Write UCI Control and Data packets received to the UCI transport.
    /// Performs segmentation of the packets.
    pub struct Writer {
        socket: Pin<Box<dyn AsyncWrite + Send>>,
    }

    impl Reader {
        /// Create an UCI reader from an UCI transport.
        pub fn new<T: AsyncRead + Send + 'static>(rx: T) -> Self {
            Reader {
                socket: Box::pin(rx),
            }
        }

        /// Read a single UCI packet from the reader. The packet is automatically
        /// re-assembled if segmented on the UCI transport. Data segments
        /// are _not_ re-assembled but returned immediatly for credit
        /// acknowledgment.
        pub async fn read(&mut self, pcapng: &Option<pcapng::File>) -> anyhow::Result<Vec<u8>> {
            use tokio::io::AsyncReadExt;

            let mut complete_packet = vec![0; HEADER_SIZE];

            // Note on reassembly:
            // For each segment of a Control Message, the
            // header of the Control Packet SHALL contain the same MT, GID and OID
            // values. It is correct to keep only the last header of the segmented packet.
            loop {
                // Read the common packet header.
                self.socket
                    .read_exact(&mut complete_packet[0..HEADER_SIZE])
                    .await?;
                let common_packet_header =
                    CommonPacketHeader::parse(&complete_packet[0..COMMON_HEADER_SIZE])?;

                // Read the packet payload.
                let payload_length = match common_packet_header.get_mt() {
                    MessageType::Data => {
                        let data_packet_header =
                            DataPacketHeader::parse(&complete_packet[0..HEADER_SIZE])?;
                        data_packet_header.get_payload_length() as usize
                    }
                    _ => {
                        let control_packet_header =
                            ControlPacketHeader::parse(&complete_packet[0..HEADER_SIZE])?;
                        control_packet_header.get_payload_length() as usize
                    }
                };
                let mut payload_bytes = vec![0; payload_length];
                self.socket.read_exact(&mut payload_bytes).await?;
                complete_packet.extend(&payload_bytes);

                if let Some(ref pcapng) = pcapng {
                    let mut packet_bytes = vec![];
                    packet_bytes.extend(&complete_packet[0..HEADER_SIZE]);
                    packet_bytes.extend(&payload_bytes);
                    pcapng.write(&packet_bytes, pcapng::Direction::Tx)?;
                }

                if common_packet_header.get_mt() == MessageType::Data {
                    return Ok(complete_packet);
                }

                // Check the Packet Boundary Flag.
                match common_packet_header.get_pbf() {
                    PacketBoundaryFlag::Complete => return Ok(complete_packet),
                    PacketBoundaryFlag::NotComplete => (),
                }
            }
        }
    }

    impl Writer {
        /// Create an UCI writer from an UCI transport.
        pub fn new<T: AsyncWrite + Send + 'static>(rx: T) -> Self {
            Writer {
                socket: Box::pin(rx),
            }
        }

        /// Write a single UCI packet to the writer. The packet is automatically
        /// segmented if the payload exceeds the maximum size limit.
        pub async fn write(
            &mut self,
            mut packet: &[u8],
            pcapng: &Option<pcapng::File>,
        ) -> anyhow::Result<()> {
            use tokio::io::AsyncWriteExt;

            let mut header_bytes = [packet[0], packet[1], packet[2], 0];
            packet = &packet[HEADER_SIZE..];

            loop {
                let message_type = parse_message_type(header_bytes[0]);
                let chunk_length = std::cmp::min(
                    packet.len(),
                    match message_type {
                        MessageType::Data => MAX_DATA_PACKET_PAYLOAD_SIZE,
                        _ => MAX_CTRL_PACKET_PAYLOAD_SIZE,
                    },
                );
                // Update header with framing information.
                let pbf = if chunk_length < packet.len() {
                    PacketBoundaryFlag::NotComplete
                } else {
                    PacketBoundaryFlag::Complete
                };
                const PBF_MASK: u8 = 0x10;
                header_bytes[0] &= !PBF_MASK;
                header_bytes[0] |= (pbf as u8) << 4;

                match message_type {
                    MessageType::Data => {
                        let chunk_le_bytes = (chunk_length as u16).to_le_bytes();
                        header_bytes[2..4].copy_from_slice(&chunk_le_bytes);
                    }
                    _ => header_bytes[3] = chunk_length as u8,
                }

                if let Some(ref pcapng) = pcapng {
                    let mut packet_bytes = vec![];
                    packet_bytes.extend(&header_bytes);
                    packet_bytes.extend(&packet[..chunk_length]);
                    pcapng.write(&packet_bytes, pcapng::Direction::Rx)?
                }

                // Write the header and payload segment bytes.
                self.socket.write_all(&header_bytes).await?;
                self.socket.write_all(&packet[..chunk_length]).await?;
                packet = &packet[chunk_length..];

                if packet.is_empty() {
                    return Ok(());
                }
            }
        }
    }
}
