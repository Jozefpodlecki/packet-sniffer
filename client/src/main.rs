use std::time::Duration;

use anyhow::Result;
use tokio::{net::TcpStream, time::sleep};
use tokio::io::AsyncReadExt;
use log::{info, error};
use log4rs;
use clap::{arg, command, Parser};

/// Command-line arguments for the client
#[derive(Parser)]
#[command(name = "Rust TCP Client", about = "A simple async TCP client with logging")]
struct Args {
    /// Port to connect to (default: 6041)
    #[arg(short, long, default_value_t = 6041)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let args = Args::parse();
    let address = format!("127.0.0.1:{}", args.port);
    let retry_delay = Duration::from_secs(3); 
    
    loop {
        match TcpStream::connect(&address).await {
            Ok(mut stream) => {
                info!("Connected to server at {}", address);
    
                let mut buffer = vec![0; 512];
                loop {
                    match stream.read(&mut buffer).await {
                        Ok(0) => {
                            info!("Server closed connection.");
                            break;
                        }
                        Ok(_size) => {}
                        Err(err) => {
                            error!("Failed to connect to {}: {}", address, err);
                            error!("Retrying in {} seconds...", retry_delay.as_secs());
                            sleep(retry_delay).await;
                        }
                    }
                }
            }
            Err(e) => error!("Failed to connect to {}: {}", address, e),
        }
    }

    Ok(())
}