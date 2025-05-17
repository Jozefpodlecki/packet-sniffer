use anyhow::Result;
use std::fmt::Debug;

pub trait PacketHandler : Debug {
    fn handle(&mut self, data: Vec<u8>) -> Result<()>;
    fn flush_all(&mut self);
}