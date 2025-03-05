use std::{error::Error, thread};
use tauri::{App, Emitter, Listener, Manager};
use tokio::{runtime::Runtime, time::{sleep, Duration}};
use serde::{Deserialize, Serialize};

use crate::updater;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Payload {
    id: i32
}

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

    thread::spawn(move || {
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
    

    Ok(())
}