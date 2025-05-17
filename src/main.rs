
use args::CommandLineArgs;
use clap::Parser;
use flexi_logger::{Duplicate, FileSpec, Logger};
use log::*;
use processor::Processor;
use utils::pause;
use anyhow::*;

mod utils;
mod windivert_source;
mod file_data_source;
mod processor;
mod packet_info;
mod opcode_tracker_handler;
mod raw_dump_handler;
mod packet_handler;
mod data_source;
mod args;

fn get_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[tokio::main]
async fn main() -> Result<()> {
    
    Logger::try_with_str("debug")?
        // .log_to_file(FileSpec::default())
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;
    let args = match CommandLineArgs::try_parse() {
        std::result::Result::Ok(args) => args,
        Err(err) => {
            error!("{}", err);
            pause();
            return Ok(());
        }
    };
    
    let source = args.create_data_source()?;
    let handler = args.create_handler()?;
    let mut processor = Processor::new(source, handler);

    match processor.run().await {
        std::result::Result::Ok(_) => {
            debug!("main:run:Ok");
        },
        Err(err) => {
            error!("{}", err);
        },
    };

    pause();

    Ok(())
}