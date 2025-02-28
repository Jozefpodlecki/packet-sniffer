#![allow(warnings)]

use std::{error::Error, net::Ipv4Addr, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, vec};

use anyhow::{Result};
use windivert::{layer::NetworkLayer, packet::WinDivertPacket, prelude::WinDivertFlags, WinDivert};
use tokio::{select, signal::windows::{self, ctrl_c}, task};

#[tokio::main]
async fn main() -> Result<()> {
    let ip = "127.0.0.1"; 
    let port = 6041;
    
    let filter = format!("ip.SrcAddr == {ip} and tcp.SrcPort == {port}");
    // let filter = format!("tcp.DstPort == {port} or tcp.SrcPort == {port}");
    // let filter = format!("tcp.PayloadLength > 70 and tcp.PayloadLength < 80");
    let flags = WinDivertFlags::new().set_recv_only().set_sniff();
    let windivert = WinDivert::network(&filter, 0, flags)?;
    let windivert = Arc::new(Mutex::new(windivert));
    let mut signal = ctrl_c()?;
    let stop_signal = Arc::new(AtomicBool::new(false));

    let sniffer_task = {
        let windivert = windivert.clone();
        let stop_signal = stop_signal.clone();

        task::spawn_blocking(move || {
            let mut buffer = vec![0u8; 65535];

            while !stop_signal.load(Ordering::Relaxed) {
                let Ok(mut guard) = windivert.try_lock() else { continue };
                match guard.recv(Some(&mut buffer)) {
                    Ok(packet) => {

                        let data = &packet.data;

                        let src_ip = Ipv4Addr::new(data[12], data[13], data[14], data[15]);
                        let dst_ip = Ipv4Addr::new(data[16], data[17], data[18], data[19]);

                        let src_port = u16::from_be_bytes([data[20], data[21]]);
                        let dst_port = u16::from_be_bytes([data[22], data[23]]);
                        let payload = &data[40..];

                        println!("Packet Captured:");
                        println!("  Source IP: {} | Source Port: {}", src_ip, src_port);
                        println!("  Destination IP: {} | Destination Port: {}", dst_ip, dst_port);
                        println!("  Length: {}", data.len());
                        println!("  Data: {:?}\n", data);
                    }
                    Err(err) => {
                        eprintln!("Error receiving packet: {:?}", err);
                        break;
                    }
                }
            }
        })
    };

    signal.recv().await;
    println!("\nCtrl+C received. Exiting...");

    stop_signal.store(true, Ordering::Relaxed);

    sniffer_task.await;

    Ok(())
}