use log::*;
use processor::Processor;
use simple_logger::SimpleLogger;
use utils::pause;
use anyhow::*;

mod utils;
mod consumer;
mod processor;
mod packet_info;

#[tokio::main]
async fn main() -> Result<()> {
    
    SimpleLogger::new().env().init().unwrap();
    let mut processor = Processor::new();

    match processor.run().await {
        std::result::Result::Ok(_) => {
            info!("main:run:Ok");
        },
        Err(err) => {
            error!("{}", err);
        },
    };

    pause();

    Ok(())
}