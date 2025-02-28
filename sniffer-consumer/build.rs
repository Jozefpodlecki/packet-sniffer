use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);

    let bin_dir = out_dir.parent().unwrap().parent().unwrap().parent().unwrap();

    let dll_source = PathBuf::from("sniffer_lib.dll");
    let dll_dest = bin_dir.join("sniffer_lib.dll");

    println!("cargo:warning={:?} to {:?}", dll_source, dll_dest);
    match fs::copy(&dll_source, &dll_dest) {
        Ok(_) => {

        },
        Err(_) => {
            
        },
    }
}