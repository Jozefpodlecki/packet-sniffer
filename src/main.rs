
use log::*;
use opcode_tracker_handler::OpcodeTrackerHandler;
use packet_handler::PacketHandler;
use processor::Processor;
use raw_dump_handler::RawDumpHandler;
use simple_logger::SimpleLogger;
use utils::{pause, prepare_dump_folder};
use anyhow::*;

mod utils;
mod consumer;
mod processor;
mod packet_info;
mod opcode_tracker_handler;
mod raw_dump_handler;
mod packet_handler;

#[tokio::main]
async fn main() -> Result<()> {
    
    SimpleLogger::new().env().init()?;

    
    let file_path = "dump.bin";
    let handler: Box<dyn PacketHandler> = Box::new(RawDumpHandler::new(file_path)?);

    let folder_name = prepare_dump_folder()?;
    let handler: Box<dyn PacketHandler> = Box::new(OpcodeTrackerHandler::new(folder_name));
    let mut processor = Processor::new(handler);

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