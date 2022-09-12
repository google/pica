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

#![allow(clippy::unused_io_amount)]

use std::path::Path;
use std::time::Instant;
use tokio::io::AsyncWriteExt;

pub struct File {
    file: tokio::fs::File,
    start_time: Instant,
}

pub enum Direction {
    Rx,
    Tx,
}

impl File {
    pub async fn create<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
        let mut file = tokio::fs::File::create(path).await?;

        // PCAPng files must start with a Section Header Block.
        file.write(&u32::to_le_bytes(0x0A0D0D0A)).await?; // Block Type
        file.write(&u32::to_le_bytes(28)).await?; // Block Total Length
        file.write(&u32::to_le_bytes(0x1A2B3C4D)).await?; // Byte-Order Magic
        file.write(&u16::to_le_bytes(1)).await?; // Major Version
        file.write(&u16::to_le_bytes(0)).await?; // Minor Version
        file.write(&u64::to_le_bytes(0xFFFFFFFFFFFFFFFF)).await?; // Section Length (not specified)
        file.write(&u32::to_le_bytes(28)).await?; // Block Total Length

        // Write the Interface Description Block used for all
        // UCI records.
        file.write(&u32::to_le_bytes(0x00000001)).await?; // Block Type
        file.write(&u32::to_le_bytes(20)).await?; // Block Total Length
        file.write(&u16::to_le_bytes(293)).await?; // LinkType
        file.write(&u16::to_le_bytes(0)).await?; // Reserved
        file.write(&u32::to_le_bytes(0)).await?; // SnapLen (no limit)
        file.write(&u32::to_le_bytes(20)).await?; // Block Total Length

        Ok(File {
            file,
            start_time: Instant::now(),
        })
    }

    pub async fn write(&mut self, packet: &[u8], _dir: Direction) -> std::io::Result<()> {
        let packet_data_padding: usize = 4 - packet.len() % 4;
        let block_total_length: u32 = packet.len() as u32 + packet_data_padding as u32 + 32;
        let timestamp = self.start_time.elapsed().as_micros();

        // Wrap the packet inside an Enhanced Packet Block.
        self.file.write(&u32::to_le_bytes(0x00000006)).await?; // Block Type
        self.file
            .write(&u32::to_le_bytes(block_total_length))
            .await?;
        self.file.write(&u32::to_le_bytes(0)).await?; // Interface ID
        self.file
            .write(&u32::to_le_bytes((timestamp >> 32) as u32))
            .await?; // Timestamp (High)
        self.file.write(&u32::to_le_bytes(timestamp as u32)).await?; // Timestamp (Low)
        self.file
            .write(&u32::to_le_bytes(packet.len() as u32))
            .await?; // Captured Packet Length
        self.file
            .write(&u32::to_le_bytes(packet.len() as u32))
            .await?; // Original Packet Length
        self.file.write(packet).await?;
        self.file.write(&vec![0; packet_data_padding]).await?;
        self.file
            .write(&u32::to_le_bytes(block_total_length))
            .await?; // Block Total Length
        Ok(())
    }
}
