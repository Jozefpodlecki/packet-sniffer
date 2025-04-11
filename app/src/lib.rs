use handlers::generate_handlers;
use log::error;
use tauri::generate_context;
use tauri_plugin_log::{Target, TargetKind};

mod handlers;
mod setup;
mod updater;
mod background_worker;
mod models;
mod hook;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


    let context = generate_context!();

    tauri::Builder::default()
            .plugin(tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Debug)
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                })
            ])
            .build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(generate_handlers())
        .setup(setup::setup_app)
        .run(context)
        .expect("error while running tauri application");
}
