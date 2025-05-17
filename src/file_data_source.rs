use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    thread::{spawn, JoinHandle},
};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use tokio::sync::{mpsc::unbounded_channel, watch};
use log::*;
use async_trait::async_trait;

use crate::data_source::*;

#[derive(Debug)]
pub struct FileDataSource {
    file_path: PathBuf,
    handle: Option<JoinHandle<()>>,
    shutdown_tx: Option<watch::Sender<()>>,
}

impl FileDataSource {
    pub fn new<P: Into<PathBuf>>(file_path: P) -> Self {
        Self {
            file_path: file_path.into(),
            handle: None,
            shutdown_tx: None,
        }
    }
}

#[async_trait]
impl DataSource for FileDataSource {
    async fn start(&mut self) -> Result<Receiver> {
        let (tx, rx) = unbounded_channel();
        let (shutdown_tx, shutdown_rx) = watch::channel(());
        self.shutdown_tx = Some(shutdown_tx);

        let file_path = self.file_path.clone();
        let handle = spawn(move || {
            let file = match File::open(file_path) {
                Ok(f) => f,
                Err(err) => {
                    error!("Failed to open file: {err}");
                    return;
                }
            };
            let mut reader = BufReader::new(file);

            loop {
                if shutdown_rx.has_changed().unwrap_or(false) {
                    debug!("FileDataSource: shutdown signal received");
                    break;
                }

                let len = match reader.read_u32::<LittleEndian>() {
                    Ok(n) => n as usize,
                    Err(_) => break,
                };

                let mut buffer = vec![0u8; len];
                if let Err(err) = reader.read_exact(&mut buffer) {
                    warn!("Failed to read full packet: {err}");
                    break;
                }

                if tx.send(buffer).is_err() {
                    break;
                }
            }

            debug!("FileDataSource: finished reading file");
        });

        self.handle = Some(handle);
        Ok(rx)
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        Ok(())
    }
}
