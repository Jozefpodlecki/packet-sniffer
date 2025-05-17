use std::{
    collections::HashMap,
    fmt::Write,
    io::{Cursor, Stdout, Write as WriteIO},
    time::Instant,
};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::{packet_handler::PacketHandler, packet_info::PacketInfo};

#[derive(Debug)]
pub struct OpcodeTrackerHandler {
    std_out: Stdout,
    last_redraw: Instant,
    buffer: String,
    folder_name: String,
    op_codes: HashMap<u16, PacketInfo>,
}

impl OpcodeTrackerHandler {
    pub fn new(folder_name: String) -> Self {
        Self {
            std_out: std::io::stdout(),
            last_redraw: Instant::now(),
            buffer: String::with_capacity(1000),
            folder_name,
            op_codes: HashMap::new(),
        }
    }

    fn process(&mut self, data: Vec<u8>) -> Result<()> {
        let now = Instant::now();
        let mut cursor = Cursor::new(&data);
        let length = cursor.read_u32::<LittleEndian>()?;
        let mut op_code = cursor.read_u16::<LittleEndian>()?;

        // If length mismatches, assign special opcode
        if length != data.len() as u32 {
            op_code = u16::MAX;
        }

        let info = self
            .op_codes
            .entry(op_code)
            .or_insert_with(|| PacketInfo::new(&self.folder_name, op_code, length));

        info.update(length, &data)?;

        self.buffer.clear();

        // Sort by count, special case for 0xFFFF
        let mut sorted_infos: Vec<_> = self.op_codes.values().collect();

        sorted_infos.sort_by(|a, b| {
            match (a.op_code == 0xFFFF, b.op_code == 0xFFFF) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => a.count.cmp(&b.count),
            }
        });

        for info in sorted_infos {
            write!(
                self.buffer,
                "opcode: {}: size: {}..{}: count {}\n",
                info.op_code_hex, info.min_length, info.max_length, info.count
            )?;
        }

        if now.duration_since(self.last_redraw).as_secs() >= 3 {
            self.std_out.queue(Clear(ClearType::All))?;
            self.last_redraw = now;
        }

        self.std_out
            .queue(MoveTo(0, 0))?
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(Color::Black))?
            .queue(Print(&self.buffer))?
            .queue(ResetColor)?
            .flush()?;

        Ok(())
    }

    pub fn flush(&mut self) {
        for info in self.op_codes.values_mut() {
            let _ = info.file.flush();
        }
    }
}

impl PacketHandler for OpcodeTrackerHandler {
    fn handle(&mut self, data: Vec<u8>) -> Result<()> {
        self.process(data)
    }

    fn flush_all(&mut self) {
        self.flush();
    }
}
