use std::error::Error;
use tauri::{App, Manager};

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


    let mut background_worker = BackgroundWorker::new(app_handle.clone());

    background_worker.run();

    Ok(())
}