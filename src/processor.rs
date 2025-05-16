use crate::{data_source::DataSource, packet_handler::PacketHandler};

pub struct Processor {
    source: Box<dyn DataSource>,
    handler: Box<dyn PacketHandler>,
}

impl Processor {
    pub fn new(
        source: Box<dyn DataSource>,
        handler: Box<dyn PacketHandler>) -> Self {
        Self { source, handler }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
    
        let mut rx = self.source.start().await?;

        loop {
            tokio::select! {
                data = rx.recv() => {
                    if let Some(data) = data {
                        self.handler.handle(data)?;
                    } else {
                        break;
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    break;
                }
            }
        }

        self.source.stop()?;
        self.handler.flush_all();

        Ok(())
    }
}
