use tauri::AppHandle;
use tauri_plugin_updater::{UpdaterExt, Result};

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