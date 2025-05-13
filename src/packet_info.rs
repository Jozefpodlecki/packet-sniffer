use std::{fs::{self, File, OpenOptions}, io::{BufWriter, Write as WriteIO}, path::Path};

use anyhow::*;
use chrono::Local;


pub struct PacketInfo {
    pub op_code: u16,
    pub op_code_hex: String,
    pub min_length: u32,
    pub max_length: u32,
    pub count: u32,
    pub file: BufWriter<File>
}

impl PacketInfo {
    pub fn new(op_code: u16, length: u32) -> Self {
        if Path::new("dump").exists() {
            if let Err(e) = fs::remove_dir_all("dump") {
                eprintln!("Failed to remove dump folder: {}", e);
            }
        }
        std::fs::create_dir_all("dump").expect("Failed to create dump folder");

        let timestamp = Local::now().format("%Y%m%d%H%M%S");
        let op_code_hex = format!("{:04X}", op_code);
        let filename = format!("dump/op_{}_{}.bin", op_code_hex, timestamp);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(&filename))
            .expect("Failed to open file");

        PacketInfo {
            op_code,
            op_code_hex,
            min_length: length,
            max_length: length,
            count: 0,
            file: BufWriter::new(file),
        }
    }

    pub fn update(&mut self, length: u32, data: &[u8]) -> anyhow::Result<()> {
        self.count += 1;
        self.min_length = self.min_length.min(length);
        self.max_length = self.max_length.max(length);
        let hex_str = hex::encode_upper(&data[6..]);
        self.file.write_all(hex_str.as_bytes())?;
        self.file.write_all(b"\r\n")?;
        Ok(())
    }
}