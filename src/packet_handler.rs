use anyhow::Result;

pub trait PacketHandler {
    fn handle(&mut self, data: Vec<u8>) -> Result<()>;
    fn flush_all(&mut self);
}