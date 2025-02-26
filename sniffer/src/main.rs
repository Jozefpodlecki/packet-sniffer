use std::vec;

use anyhow::Result;
use windivert::{prelude::WinDivertFlags, WinDivert};

#[tokio::main]
async fn main() -> Result<()> {

    let port = 6041;
    let filter = format!("tcp.DstPort == {port}");
    let flags = WinDivertFlags::new().set_sniff().set_recv_only();
    let windivert = WinDivert::network(filter, 0, flags)?;

    let mut buffer = vec![0u8; 65535];

    loop {
        let packet = windivert.recv(Some(&mut buffer))?;
        let data = &packet.data;
        // println!("{:?}", packet.data);

        println!("length: {}", packet.data.len());

        // if data.len() > 40 {
        //     let payload = &data[40..]; 
        //     println!("TCP Payload: {:?}", payload);
        // } else {
        //     println!("Packet too small: {:?}", data);
        // }
    }


    Ok(())
}