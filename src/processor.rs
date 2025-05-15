use crate::packet_handler::PacketHandler;

pub struct Processor {
    handler: Box<dyn PacketHandler>,
}

impl Processor {
    pub fn new(handler: Box<dyn PacketHandler>) -> Self {
        Self { handler }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut consumer = crate::consumer::Consumer::new();
        let mut rx = consumer.start().await?;

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

        consumer.stop()?;
        self.handler.flush_all();

        Ok(())
    }
}
