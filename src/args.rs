use clap::Parser;
use clap::ValueEnum;
use anyhow::*;

use crate::opcode_tracker_handler::OpcodeTrackerHandler;
use crate::packet_handler::PacketHandler;
use crate::raw_dump_handler::RawDumpHandler;
use crate::utils::prepare_dump_folder;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum HandlerType {
    Raw,
    Tracker,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct CommandLineArgs {
    /// Choose which handler to use
    #[arg(short, long, value_enum, default_value_t = HandlerType::Tracker)]
    handler: HandlerType,

    /// Path to raw dump file (used only for raw handler)
    #[arg(short, long, default_value = "dump.bin")]
    raw_file: String,
}

impl CommandLineArgs {
    pub fn create_handler(&self) -> Result<Box<dyn PacketHandler>> {
        match self.handler {
            HandlerType::Raw => Ok(Box::new(RawDumpHandler::new(self.raw_file.clone())?)),
            HandlerType::Tracker => {
                let folder_name = prepare_dump_folder()?;
                Ok(Box::new(OpcodeTrackerHandler::new(folder_name)))
            }
        }
    }
}