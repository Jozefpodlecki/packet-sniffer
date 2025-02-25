use handlers::generate_handlers;
use tauri::generate_context;

mod handlers;
mod setup;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = generate_context!();

    tauri::Builder::default()
        .setup(setup::setup_app)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .invoke_handler(generate_handlers())
        .run(context)
        .expect("error while running tauri application");
}
