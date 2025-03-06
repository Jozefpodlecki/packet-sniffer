use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, JoinHandle}};
use anyhow::{Result, Ok};
use tauri::{AppHandle, Emitter};
use tokio::{io::Join, runtime::Runtime, time::{sleep, Duration}};

use crate::models::Payload;

pub struct BackgroundWorker {
    app_handle: AppHandle,
    handle: Option<JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
}

impl BackgroundWorker {
    pub fn new(app_handle: AppHandle) -> Self {
        BackgroundWorker {
            app_handle,
            handle: None,
            is_running: Arc::new(AtomicBool::new(false)), 
        }
    }

    pub fn run(&mut self) {
        let app_handle = self.app_handle.clone();
        self.is_running.store(true, Ordering::Relaxed);
        let is_running = Arc::clone(&self.is_running);

        let handle = thread::spawn(move || {
            let mut rt = Runtime::new().unwrap();
            
            while is_running.load(Ordering::Relaxed) {
                let payload = Payload { id: 1 };
                app_handle.emit("update", payload).unwrap();

                rt.block_on(async {
                    sleep(Duration::from_secs(1)).await;
                });
            }
        });

        self.handle = Some(handle);
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(handle)  = self.handle.take() {
            self.is_running.store(false, Ordering::Relaxed);
            handle.join().map_err(|e| anyhow::anyhow!("Thread panicked: {:?}", e))?;
            self.handle = None;
        }

        Ok(())
    }
}

