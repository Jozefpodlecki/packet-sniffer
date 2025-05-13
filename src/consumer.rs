use std::{fs::File, io::BufReader, net::{Ipv4Addr, Ipv6Addr}, path::Path, sync::{atomic::AtomicBool, Arc}, thread::JoinHandle};
use anyhow::{Ok, Result};
use etherparse::{NetHeaders, PacketHeaders, TransportHeader};
use log::*;
use tokio::{runtime::Runtime, sync::{mpsc::{UnboundedReceiver, UnboundedSender}, watch}, task::{self}};
use windivert::{prelude::WinDivertFlags, WinDivert};
use std::thread::spawn;

pub struct Consumer {
    handle: Option<JoinHandle<Result<()>>>,
    shutdown: Arc<AtomicBool>,
    shutdown_tx: Option<watch::Sender<()>>,
}

impl Consumer {
    pub fn new() -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));

        Self {
            handle: None,
            shutdown,
            shutdown_tx: None
        }
    }

    pub async fn start(&mut self) -> Result<UnboundedReceiver<Vec<u8>>> {
   
        let (shutdown_tx, shutdown_rx) = watch::channel(());
        self.shutdown_tx = Some(shutdown_tx);

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();

        let handle = spawn(move || Self::thread_loop(tx, shutdown_rx));

        self.handle = Some(handle);

        Ok(rx)
    }

    pub fn thread_loop(tx: UnboundedSender<Vec<u8>>, mut shutdown_rx: watch::Receiver<()>) -> Result<()> {
        let runtime = Runtime::new()?;
            
        runtime.block_on(async move {
            let filter = "inbound && tcp.SrcPort == 6040";
            let flags = WinDivertFlags::new().set_recv_only().set_sniff();
            let windivert = WinDivert::network(&filter, 0, flags)?;

            let windivert = Arc::new(windivert);
            
            loop {
                let windivert = windivert.clone();
                
                let recv = task::spawn_blocking(move || {
                    let mut buffer = vec![0u8; 65535];
                    let packet = match windivert.recv(Some(&mut buffer)).ok() {
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

            anyhow::Ok(())
        })?;

        anyhow::Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
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
