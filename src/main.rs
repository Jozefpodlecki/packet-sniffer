
use args::CommandLineArgs;
use clap::Parser;
use log::*;
use processor::Processor;
use simple_logger::SimpleLogger;
use utils::pause;
use anyhow::*;

mod utils;
mod consumer;
mod processor;
mod packet_info;
mod opcode_tracker_handler;
mod raw_dump_handler;
mod packet_handler;
mod args;

#[tokio::main]
async fn main() -> Result<()> {
    
    SimpleLogger::new().env().init()?;
    let args = match CommandLineArgs::try_parse() {
        std::result::Result::Ok(args) => args,
        Err(err) => {
            error!("{}", err);
            pause();
            std::process::exit(1);
        }
    };
    
    let handler = args.create_handler()?;
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