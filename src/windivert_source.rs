use std::{sync::{atomic::AtomicBool, Arc}, thread::JoinHandle};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use etherparse::PacketHeaders;
use log::*;
use tokio::{runtime::Runtime, sync::{mpsc::UnboundedSender, watch}, task::{self}};
use windivert::{prelude::WinDivertFlags, CloseAction, WinDivert};
use std::thread::spawn;

use crate::data_source::{DataSource, Receiver};

#[derive(Debug)]
pub struct WindivertSource {
    filter: String,
    handle: Option<JoinHandle<Result<()>>>,
    shutdown: Arc<AtomicBool>,
    shutdown_tx: Option<watch::Sender<()>>,
}

#[async_trait]
impl DataSource for WindivertSource {
    async fn start(&mut self) -> Result<Receiver> {
        let (shutdown_tx, shutdown_rx) = watch::channel(());
        let filter = self.filter.clone();
        self.shutdown_tx = Some(shutdown_tx);

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();

        let handle = spawn(move || Self::thread_loop(filter, tx, shutdown_rx));

        self.handle = Some(handle);

        Ok(rx)
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        if let Some(handle) = self.handle.take() {
            if let Err(err) = handle.join() {
                error!("Error stopping thread: {:?}", err);
            }
        }

        Ok(())
    }
}

impl WindivertSource {
    pub fn new(filter: String) -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));

        Self {
            filter,
            handle: None,
            shutdown,
            shutdown_tx: None
        }
    }

    pub fn thread_loop(filter: String, tx: UnboundedSender<Vec<u8>>, mut shutdown_rx: watch::Receiver<()>) -> Result<()> {
        let runtime = Runtime::new()?;
            
        runtime.block_on(async move {
            let flags = WinDivertFlags::new().set_recv_only().set_sniff();
            let windivert = WinDivert::network(&filter, 0, flags)?;

            let mut windivert = Arc::new(windivert);
            
            loop {
                let windivert = windivert.clone();
                
                let recv = task::spawn_blocking(move || {
                    let mut buffer = vec![0u8; 65535];
                    let packets = match windivert.recv_ex(Some(&mut buffer), 1).ok() {
                        Some(packet) => packet,
                        None => return None,
                    };

                    let packet = match packets.first() {
                        Some(packet) => packet,
                        None => return None,
                    };
                    let headers = match PacketHeaders::from_ip_slice(&packet.data).ok() {
                        Some(headers) => headers,
                        None => return None,
                    };

                    let data = headers.payload.slice().to_vec();

                    if data.is_empty() {
                        return None;
                    }

                    Some(data)
                });

                tokio::select! {
                    result = recv => {
                        use std::result::Result::Ok;
                        match result {
                            Ok(data) => {
                                if let Some(data) = data {
                                    tx.send(data)?;
                                }
                            },
                            Err(err) => {
                                info!("{err}");
                            },
                        }
                    }
                    _ = shutdown_rx.changed() => {
                        info!("shutdown");
                        break;
                    }
                }
            }

            // windivert.close(CloseAction::Nothing)?;
            anyhow::Ok(())
        })?;

        anyhow::Ok(())
    }
}
