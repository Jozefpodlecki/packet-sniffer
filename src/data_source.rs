use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;
use std::fmt::Debug;
pub type Receiver = UnboundedReceiver<Vec<u8>>;

#[async_trait]
pub trait DataSource: Debug  {
    async fn start(&mut self) -> Result<Receiver>;
    fn stop(&mut self) -> Result<()>;
}