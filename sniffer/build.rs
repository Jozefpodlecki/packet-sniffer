use std::{env, fs};
use std::path::{Path, PathBuf};

fn main() {
    let dll_name = "sniffer_lib.dll"; 

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);
    let dll_src = out_dir.parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .join(dll_name);
    
    let dll_dest = env::current_dir().unwrap();
    let dll_dest = dll_dest.parent().unwrap().join("sniffer-consumer").join("sniffer_lib.dll");

    match fs::copy(&dll_src, &dll_dest) {
        Ok(_) => {
            println!("cargo:warning=Copied {:?} to {:?}", &dll_src, &dll_dest);
        },
        Err(err) => {
            println!("cargo:warning={}", err);
        },
    }
}