
use tauri::command;

use crate::models::{LoadResult, Settings};

#[command]
pub fn load() -> LoadResult {
    
    LoadResult {
        settings: Settings {
            
        }
    }
}
