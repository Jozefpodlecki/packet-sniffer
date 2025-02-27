use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let dll_name = "sniffer_lib.dll"; 
    let dll_src = format!("target/debug/{}", dll_name);
    
    let target_project = "../sniffer-consumer";
    let dll_dest = format!("{}/{}", target_project, dll_name);

    println!("cargo:warning=Copied {}", out_dir);
    println!("cargo:warning=Copied {} to {}", dll_src, dll_dest);
    fs::copy(&dll_src, &dll_dest).expect("Failed to copy DLL");
}