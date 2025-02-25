
use tauri::{generate_context, generate_handler, AppHandle, Manager};

pub fn generate_handlers() -> Box<dyn Fn(tauri::ipc::Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![
       
    ])
}
