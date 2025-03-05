
use std::{sync::mpsc::{self, Receiver}, thread, time::Duration};

use shared::{models::Packet, simulator::Simulator};
use windivert::{prelude::WinDivertFlags, WinDivert};

#[unsafe(no_mangle)]
pub extern "C" fn start_capture_fake() -> *mut Receiver<Packet> {
    let (tx, rx) = mpsc::channel::<Packet>();

    std::thread::spawn(move || {
        let simulator = Simulator::new();

        for packet in simulator {
            thread::sleep(Duration::from_secs(2));
       
            tx.send(packet).unwrap();
        }
    });

    Box::into_raw(Box::new(rx))
}

#[unsafe(no_mangle)]
pub extern "C" fn start_capture(port: i32) -> *mut Receiver<Packet> {
    let (tx, rx) = mpsc::channel::<Packet>();

    std::thread::spawn(move || {
        let filter = format!("tcp.SrcPort == {port}");
        let flags = WinDivertFlags::new().set_recv_only().set_sniff();
        let windivert = WinDivert::network(&filter, 0, flags).unwrap();
        let mut buffer = vec![0u8; 65535];

        loop {
            let windivert_packet = windivert.recv(Some(&mut buffer)).unwrap();
            let data = &windivert_packet.data;

            let packet = to_packet(data);

            tx.send(packet).unwrap();
        }
    });

    Box::into_raw(Box::new(rx))
}

fn to_packet(data: &[u8]) -> Packet {
    Packet::None
}