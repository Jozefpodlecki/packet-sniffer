use std::{
    fs::File,
    io::Write,
    path::PathBuf,
};

use anyhow::Result;

use crate::packet_handler::PacketHandler;

#[derive(Debug)]
pub struct RawDumpHandler {
    file: File,
}

impl RawDumpHandler {
    pub fn new(file_path: impl Into<PathBuf>) -> Result<Self> {
        let file = File::create(file_path.into())?;
        Ok(Self { file })
    }
}

impl PacketHandler for RawDumpHandler {
    fn handle(&mut self, data: Vec<u8>) -> Result<()> {
        let length = data.len() as u32;
        self.file.write_all(&length.to_le_bytes())?;

        self.file.write_all(&data)?;
        Ok(())
    }

    fn flush_all(&mut self) {
        let _ = self.file.flush();
    }
}
