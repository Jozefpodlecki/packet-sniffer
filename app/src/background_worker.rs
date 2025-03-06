use std::thread::{self, JoinHandle};
use anyhow::{Result, Ok};
use tauri::{AppHandle, Emitter};
use tokio::{io::Join, runtime::Runtime, time::{sleep, Duration}};

use crate::models::Payload;

pub struct BackgroundWorker {
    app_handle: AppHandle,
    handle: Option<JoinHandle<()>>
}

impl BackgroundWorker {
    pub fn new(app_handle: AppHandle) -> Self {
        BackgroundWorker {
            app_handle,
            handle: None
        }
    }

    pub fn run(&mut self) {
        let app_handle = self.app_handle.clone();
        let handle = thread::spawn(move || {
            let mut rt = Runtime::new().unwrap();
            
            loop {
                let payload = Payload {
                    id: 1
                };
    
                app_handle.emit("update", payload).unwrap();
    
                rt.block_on(async move {
                    sleep(Duration::from_secs(1)).await    
                });
            }
        });

        self.handle = Some(handle);
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(handle)  = self.handle.take() {
            handle.join().unwrap();
            self.handle = None;
        }

        Ok(())
    }
}

