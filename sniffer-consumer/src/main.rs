
use anyhow::{Result, Ok};
use sniffer_consumer::Wrapper;

#[tokio::main]
async fn main() -> Result<()> {
    let mut wrapper = Wrapper::new();

    wrapper.start_capture();

    wrapper.recv();

    Ok(())
}