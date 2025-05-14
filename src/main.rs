use std::{fs, path::Path};

use chrono::Local;
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

    // if Path::new("dump").exists() {
    //     if let Err(e) = fs::remove_dir_all("dump") {
    //         eprintln!("Failed to remove dump folder: {}", e);
    //     }
    // }
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let folder_name = format!("dump_{}", timestamp);
    std::fs::create_dir_all(&folder_name).expect("Failed to create dump folder");

    let mut processor = Processor::new(folder_name);

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