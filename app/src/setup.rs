use std::{error::Error, sync::{Arc, Mutex}};
use tauri::{App, Emitter, Listener, Manager};

use crate::services::{processor, AppStartupLatch, BackgroundWorker, Processor};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {

    #[cfg(debug_assertions)]
    {
        let window = app.get_webview_window("main").unwrap();
        window.open_devtools();
    }

    let app_handle = app.handle().clone();
    let resource_dir = app.path().resource_dir()?;
    let version = app_handle.package_info().version.to_string();

    // let window = app_handle.get_webview_window("main").unwrap();
    let processor = Arc::new(Processor::new());
    let app_startup_latch: Arc<AppStartupLatch> = Arc::new(AppStartupLatch::new());
    let mut background_worker = BackgroundWorker::new(
        app_startup_latch.clone(),
        processor.clone(),
        app_handle.clone());

    background_worker.run();

    let background_worker = Arc::new(Mutex::new(background_worker));    

    app.manage(app_startup_latch.clone());
    app.manage(background_worker.clone());

    Ok(())
}