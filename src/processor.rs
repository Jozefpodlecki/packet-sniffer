use std::io::Cursor;

use anyhow::*;
use byteorder::{LittleEndian, ReadBytesExt};
use log::*;

use crate::consumer::Consumer;


pub struct Processor {

}

impl Processor {
    pub fn new() -> Self {
        Self {}
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

        Ok(())
    }

    pub fn handle(&mut self, data: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(&data);
        let op_code = cursor.read_u16::<LittleEndian>().unwrap();

        Ok(())
    }
}