use std::{collections::HashMap, fmt::Write, io::{stdout, Cursor, Write as WriteIO}, u16};

use anyhow::*;
use byteorder::{LittleEndian, ReadBytesExt};
use log::*;
use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, QueueableCommand,
};

use crate::{consumer::Consumer, packet_info::PacketInfo};


pub struct Processor {
    std_out: std::io::Stdout,
    buffer: String,
    op_codes: HashMap<u16, PacketInfo>
}

impl Processor {
    pub fn new() -> Self {
        let std_out = stdout();
        let buffer = String::with_capacity(1000);

        Self {
            std_out,
            buffer,
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
        let mut cursor = Cursor::new(&data);
        let length = cursor.read_u32::<LittleEndian>()?;
        let mut op_code = cursor.read_u16::<LittleEndian>()?;

        if length != data.len() as u32 {
            op_code = u16::MAX;
        }

        let info = self
            .op_codes
            .entry(op_code)
            .or_insert_with(|| PacketInfo::new(op_code, length));

        info.update(length, &data)?;

        self.buffer.clear();
        for (op_code, info) in self.op_codes.iter() {
            write!(self.buffer, "{}: {}..{} - {}\n", op_code, info.min_length, info.max_length, info.count)?;
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

