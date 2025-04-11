use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, JoinHandle}};
use anyhow::{Result, Ok};
use log::debug;
use tauri::{AppHandle, Emitter};
use tokio::{runtime::Runtime, time::{sleep, Duration}};

use crate::models::Payload;

use super::AppStartupLatch;

pub struct BackgroundWorker {
    app_startup_latch: Arc<AppStartupLatch>,
    app_handle: AppHandle,
    handle: Option<JoinHandle<()>>,
    is_running: Arc<AtomicBool>,
}

impl BackgroundWorker {
    pub fn new(
        app_startup_latch: Arc<AppStartupLatch>,
            app_handle: AppHandle) -> Self {
        BackgroundWorker {
            app_startup_latch,
            app_handle,
            handle: None,
            is_running: Arc::new(AtomicBool::new(false)), 
        }
    }

    pub fn run(&mut self) {
        if self.is_running.load(Ordering::Relaxed) {
            return;
        }

        self.is_running.store(true, Ordering::Relaxed);
        let app_handle = self.app_handle.clone();
        let app_startup_latch = self.app_startup_latch.clone();
        
        let is_running = Arc::clone(&self.is_running);

        let handle = thread::spawn(move || {
            debug!("Waiting for app load.");
            app_startup_latch.wait_for_ready();
            debug!("Starting loop.");
            Self::thread_loop(is_running, app_handle);
        });

        self.handle = Some(handle);
    }

    fn thread_loop(is_running: Arc<AtomicBool>, app_handle: AppHandle) {
        let rt = Runtime::new().unwrap();
            
        rt.block_on(async {
            while is_running.load(Ordering::Relaxed) {
                let payload = Payload { id: 1 };
                app_handle.emit("update", payload).unwrap();

                
            }    
        });
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