
use std::sync::Arc;

use tauri::{command, AppHandle, State};

use crate::{models::{LoadResult, Settings}, services::{app_startup_latch, AppStartupLatch}};

#[command]
pub fn load(
    app_startup_latch: State<'_, Arc<AppStartupLatch>>,
    app_handle: AppHandle,
    ) -> LoadResult {
    app_startup_latch.mark_ready();

    let version = app_handle.package_info().version.to_string();

    LoadResult {
        version,
        github_url: "https://github.com/Jozefpodlecki/packet-sniffer".to_string(),
    }
}
