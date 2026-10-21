// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! PCAPng packet capture encoding for UCI packets (link type 293).

use std::io::Write;
use std::time::Duration;

/// 48-byte PCAPng file header (Section Header Block and Interface Description
/// Block declaring UCI link type 293).
pub const HEADER: [u8; 48] = [
    // Section Header Block (28 bytes)
    0x0A, 0x0D, 0x0D, 0x0A, // Block Type (0x0A0D_0D0A)
    28, 0, 0, 0, // Block Total Length (28)
    0x4D, 0x3C, 0x2B, 0x1A, // Byte-Order Magic (0x1A2B_3C4D)
    1, 0, // Major Version (1)
    0, 0, // Minor Version (0)
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // Section Length (not specified)
    28, 0, 0, 0, // Block Total Length (28)
    // Interface Description Block (20 bytes)
    0x01, 0x00, 0x00, 0x00, // Block Type (0x0000_0001)
    20, 0, 0, 0, // Block Total Length (20)
    0x25, 0x01, // LinkType (293)
    0, 0, // Reserved (0)
    0, 0, 0, 0, // SnapLen (no limit)
    20, 0, 0, 0, // Block Total Length (20)
];

const PADDING: &[u8] = &[0; 3];

/// A non-allocating PCAPng Enhanced Packet Block wrapping a packet slice.
#[derive(Clone, Copy, Debug)]
pub struct Packet<'a> {
    header: [u8; 28],
    payload: &'a [u8],
    padding: &'static [u8],
    footer: [u8; 4],
}

impl<'a> Packet<'a> {
    /// Wraps `payload` in a PCAPng Enhanced Packet Block with the given `timestamp`.
    pub fn new(payload: &'a [u8], timestamp: Duration) -> Self {
        let padding_len = (4 - (payload.len() % 4)) % 4;
        let block_total_length = (payload.len() + padding_len + 32) as u32;
        let timestamp_us = timestamp.as_micros();
        let packet_len = payload.len() as u32;

        let [bt0, bt1, bt2, bt3] = 0x0000_0006_u32.to_le_bytes();
        let [bl0, bl1, bl2, bl3] = block_total_length.to_le_bytes();
        let [th0, th1, th2, th3] = ((timestamp_us >> 32) as u32).to_le_bytes();
        let [tl0, tl1, tl2, tl3] = (timestamp_us as u32).to_le_bytes();
        let [pl0, pl1, pl2, pl3] = packet_len.to_le_bytes();

        Self {
            header: [
                bt0, bt1, bt2, bt3, // Block Type (0x0000_0006)
                bl0, bl1, bl2, bl3, // Block Total Length
                0, 0, 0, 0, // Interface ID (0)
                th0, th1, th2, th3, // Timestamp (High)
                tl0, tl1, tl2, tl3, // Timestamp (Low)
                pl0, pl1, pl2, pl3, // Captured Packet Length
                pl0, pl1, pl2, pl3, // Original Packet Length
            ],
            payload,
            padding: PADDING.get(..padding_len).unwrap_or_default(),
            footer: [bl0, bl1, bl2, bl3],
        }
    }

    /// Returns the four constituent byte slices of this Enhanced Packet Block
    /// for zero-allocation or vectored I/O writes.
    pub const fn as_slices(&self) -> [&[u8]; 4] {
        [&self.header, self.payload, self.padding, &self.footer]
    }

    /// Writes the Enhanced Packet Block to a synchronous [`Write`] implementor.
    pub fn write_to(&self, mut writer: impl Write) -> std::io::Result<()> {
        for slice in self.as_slices() {
            writer.write_all(slice)?;
        }
        Ok(())
    }
}
