use std::{fs, path::Path};

use chrono::Local;
use std::io::{self, Read, Write};
use anyhow::*;

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn prepare_dump_folder() -> Result<String> {
    // Optional: clear existing folder
    // if Path::new("dump").exists() {
    //     fs::remove_dir_all("dump")?;
    // }

    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let folder_name = format!("dump_{}", timestamp);
    fs::create_dir_all(&folder_name)?;
    Ok(folder_name)
}