use clap::Parser;
use clap::ValueEnum;
use anyhow::*;

use crate::data_source::DataSource;
use crate::file_data_source::FileDataSource;
use crate::opcode_tracker_handler::OpcodeTrackerHandler;
use crate::packet_handler::PacketHandler;
use crate::raw_dump_handler::RawDumpHandler;
use crate::utils::prepare_dump_folder;
use crate::windivert_source::WindivertSource;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum DataSourceType {
    Windivert,
    File,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum HandlerType {
    Raw,
    Tracker,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct CommandLineArgs {
    #[arg(long, value_enum, default_value_t = DataSourceType::Windivert)]
    source: DataSourceType,

    /// Choose which handler to use
    #[arg(long, value_enum, default_value_t = HandlerType::Tracker)]
    handler: HandlerType,

    /// Path to raw dump file (used only for raw handler)
    #[arg(long, default_value = "dump.bin")]
    file_path: String,
}

impl CommandLineArgs {
 pub fn create_data_source(&self) -> Result<Box<dyn DataSource>> {
        match self.source {
            DataSourceType::Windivert => Ok(Box::new(WindivertSource::new())),
            DataSourceType::File => {
                Ok(Box::new(FileDataSource::new(self.file_path.clone())))
            }
        }
    }

    pub fn create_handler(&self) -> Result<Box<dyn PacketHandler>> {
        match self.handler {
            HandlerType::Raw => Ok(Box::new(RawDumpHandler::new(self.file_path.clone())?)),
            HandlerType::Tracker => {
                let folder_name = prepare_dump_folder()?;
                Ok(Box::new(OpcodeTrackerHandler::new(folder_name)))
            }
        }
    }
}