use std::{collections::HashMap, fmt::Write, io::{stdout, Cursor, Write as WriteIO}, u16};

use anyhow::*;
use byteorder::{LittleEndian, ReadBytesExt};
use log::*;
use crossterm::{
    cursor::MoveTo, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{Clear, ClearType}, QueueableCommand
};

use crate::{consumer::Consumer, packet_info::PacketInfo};


pub struct Processor {
    std_out: std::io::Stdout,
    last_redraw: std::time::Instant,
    folder_name: String,
    buffer: String,
    op_codes: HashMap<u16, PacketInfo>
}

impl Processor {
    pub fn new(folder_name: String) -> Self {
        let std_out = stdout();
        let buffer = String::with_capacity(1000);
        let last_redraw = std::time::Instant::now();

        Self {
            std_out,
            last_redraw,
            buffer,
            folder_name,
            op_codes: HashMap::new()
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        
        let mut consumer = Consumer::new();
        let mut rx = consumer.start().await?; 
    
        loop {
            tokio::select! {
                data = rx.recv() => {
                    if let Some(data) = data {
                        self.handle(data)?;
                    } else {
                        break;
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    info!("Received Ctrl+C, shutting down.");
                    break;
                }
            }
    
        }
    
        consumer.stop()?;
        self.flush_all();

        Ok(())
    }

    pub fn handle(&mut self, data: Vec<u8>) -> Result<()> {
        let now = std::time::Instant::now();
        let mut cursor = Cursor::new(&data);
        let length = cursor.read_u32::<LittleEndian>()?;
        let mut op_code = cursor.read_u16::<LittleEndian>()?;

        if length != data.len() as u32 {
            op_code = u16::MAX;
        }

        let info = self
            .op_codes
            .entry(op_code)
            .or_insert_with(|| PacketInfo::new(&self.folder_name, op_code, length));

        info.update(length, &data)?;

        self.buffer.clear();

      let mut sorted_infos: Vec<_> = self.op_codes.values().collect();

        sorted_infos.sort_by(|a, b| {
            match (a.op_code == 0xFFFF, b.op_code == 0xFFFF) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => a.count.cmp(&b.count),
            }
        });

        for info in sorted_infos {
            write!(self.buffer, "opcode: {}: size: {}..{}: count {}\n", info.op_code_hex, info.min_length, info.max_length, info.count)?;
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

    pub fn flush_all(&mut self) {
        for info in self.op_codes.values_mut() {
            let _ = info.file.flush();
        }
    }
}

