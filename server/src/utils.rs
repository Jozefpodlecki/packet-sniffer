use std::{net::SocketAddr, time::Duration};

use log::*;
use shared::DamagePacket;
use tokio::{io::AsyncWriteExt, net::TcpStream, time};

use crate::simulator::{self, Simulator, SimulatorOptions};

pub async fn send_to_client(stream: &mut TcpStream, addr: SocketAddr) {
    let mut interval = time::interval(Duration::from_secs(2));
    let mut simulator = Simulator::new();
    let options = SimulatorOptions {};
    simulator.setup(options);

    loop {
        interval.tick().await;
        
        let encoded = simulator.generate_packet();

        if let Err(e) = stream.write_all(&encoded).await {
            error!("Failed to send message to {}: {}", addr, e);
            break;
        }

    }
}
