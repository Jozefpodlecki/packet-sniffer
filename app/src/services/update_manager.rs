use tauri::AppHandle;
use tauri_plugin_updater::{UpdaterExt, Result};

pub struct UpdateManager {
	app_handle: AppHandle,
}

impl UpdateManager {
	pub fn new(app_handle: AppHandle) -> Self {
		UpdateManager { app_handle }
	}

	pub async fn check_for_updates(&self) -> Result<()> {
		// let updater = self.app_handle.updater()?;
		// let check_result = updater.check().await?;

		// if let Some(update) = check_result {
		// 	println!("Update available: {:?}", update);
		// } else {
		// 	println!("No updates available");
		// }

		Ok(())
	}
}

pub async fn update(app: AppHandle) -> Result<()> {
    let updater = app.updater()?;
	let check_result = updater.check().await?;

	if let Some(update) = check_result {
	let mut downloaded = 0;
		
	update.download_and_install(
			|chunk_length, content_length| {
			downloaded += chunk_length;
			println!("downloaded {downloaded} from {content_length:?}");
			},
			|| {
			println!("download finished");
			},
		)
		.await?;

		println!("update installed");
		app.restart();
	}

	Ok(())
  }