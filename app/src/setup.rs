use std::{error::Error, sync::{Arc, Mutex}};
use tauri::{App, Listener, Manager};

use crate::{background_worker::{self, BackgroundWorker}, updater};

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
    {
        let app_handle = app_handle.clone();
        // tokio::task::spawn(async move {
        //     match updater::update(app_handle).await {
        //         Ok(_) => {},
        //         Err(err) => {
        //             println!("{:?}", err);
        //         },
        //     }
        // });

        tauri::async_runtime::spawn(async move {
            match updater::update(app_handle).await {
                Ok(_) => {},
                Err(err) => {
                    println!("{:?}", err);
                },
            }
        });
    }

    let background_worker = Arc::new(Mutex::new(BackgroundWorker::new(app_handle.clone())));

    {
        let background_worker = background_worker.clone();
        app_handle.listen_any("restart", move |event| {
            let mut background_worker = background_worker.lock().unwrap();
            background_worker.stop().unwrap();
        });
    }
    

    let mut background_worker = background_worker.lock().unwrap();
    background_worker.run();

    Ok(())
}