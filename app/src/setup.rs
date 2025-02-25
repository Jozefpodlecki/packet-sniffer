use std::error::Error;
use tauri::{App, Manager};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {

    let app_handle = app.handle();
    let resource_dir = app.path().resource_dir()?;
    let version = app_handle.package_info().version.to_string();

    Ok(())
}