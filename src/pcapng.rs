use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub struct File {
    file: std::fs::File,
    start_time: Instant,
}

pub enum Direction {
    Rx,
    Tx,
}

impl File {
    pub fn create<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
        let mut file = std::fs::File::create(path)?;

        // PCAPng files must start with a Section Header Block.
        file.write(&u32::to_le_bytes(0x0A0D0D0A))?; // Block Type
        file.write(&u32::to_le_bytes(28))?; // Block Total Length
        file.write(&u32::to_le_bytes(0x1A2B3C4D))?; // Byte-Order Magic
        file.write(&u16::to_le_bytes(1))?; // Major Version
        file.write(&u16::to_le_bytes(0))?; // Minor Version
        file.write(&u64::to_le_bytes(0xFFFFFFFFFFFFFFFF))?; // Section Length (not specified)
        file.write(&u32::to_le_bytes(28))?; // Block Total Length

        // Write the Interface Description Block used for all
        // UCI records.
        file.write(&u32::to_le_bytes(0x00000001))?; // Block Type
        file.write(&u32::to_le_bytes(20))?; // Block Total Length
        file.write(&u16::to_le_bytes(293))?; // LinkType
        file.write(&u16::to_le_bytes(0))?; // Reserved
        file.write(&u32::to_le_bytes(0))?; // SnapLen (no limit)
        file.write(&u32::to_le_bytes(20))?; // Block Total Length

        Ok(File {
            file,
            start_time: Instant::now(),
        })
    }

    pub fn write(&mut self, packet: &[u8], _dir: Direction) -> std::io::Result<()> {
        let packet_data_padding: usize = 4 - packet.len() % 4;
        let block_total_length: u32 = packet.len() as u32 + packet_data_padding as u32 + 32;
        let timestamp = self.start_time.elapsed().as_micros();

        // Wrap the packet inside an Enhanced Packet Block.
        self.file.write(&u32::to_le_bytes(0x00000006))?; // Block Type
        self.file.write(&u32::to_le_bytes(block_total_length))?;
        self.file.write(&u32::to_le_bytes(0))?; // Interface ID
        self.file
            .write(&u32::to_le_bytes((timestamp >> 32) as u32))?; // Timestamp (High)
        self.file.write(&u32::to_le_bytes(timestamp as u32))?; // Timestamp (Low)
        self.file.write(&u32::to_le_bytes(packet.len() as u32))?; // Captured Packet Length
        self.file.write(&u32::to_le_bytes(packet.len() as u32))?; // Original Packet Length
        self.file.write(packet)?;
        self.file.write(&vec![0; packet_data_padding])?;
        self.file.write(&u32::to_le_bytes(block_total_length))?; // Block Total Length
        Ok(())
    }
}
