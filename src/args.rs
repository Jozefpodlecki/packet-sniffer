use chrono::Local;
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

    #[arg(long, default_value = Some("dump.bin"))]
    input_path: Option<String>,

    /// Path to raw dump file (used only for raw handler)
    #[arg(long, default_value = None)]
    output_path: Option<String>,
}

impl CommandLineArgs {
   fn default_output_path(&self) -> String {
        self.output_path.clone().unwrap_or_else(|| {
            let timestamp = Local::now().format("dump_%Y-%m-%d_%H%M%S.bin").to_string();
            timestamp
        })
    }

    pub fn create_data_source(&self) -> Result<Box<dyn DataSource>> {
        let filter = "inbound && tcp.SrcPort == 6040".into();

        match self.source {
            DataSourceType::Windivert => Ok(Box::new(WindivertSource::new(filter))),
            DataSourceType::File => {
                Ok(Box::new(FileDataSource::new(self.output_path.clone().unwrap())))
            }
        }
    }

    pub fn create_handler(&self) -> Result<Box<dyn PacketHandler>> {
        match self.handler {
            HandlerType::Raw => {
                let file_path = self.default_output_path();
                let handler = Box::new(RawDumpHandler::new(file_path)?);
                Ok(handler)
            },
            HandlerType::Tracker => {
                let folder_name = prepare_dump_folder()?;
                Ok(Box::new(OpcodeTrackerHandler::new(folder_name)))
            }
        }
    }
}