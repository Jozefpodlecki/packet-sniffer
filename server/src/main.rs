use log::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                info!("Client disconnected.");
                break;
            }
            Ok(size) => {
                let msg = String::from_utf8_lossy(&buffer[..size]);
                info!("Received: {}", msg);

                if let Err(e) = stream.write_all(&buffer[..size]).await {
                    error!("Failed to send data: {}", e);
                    break;
                }
            }
            Err(e) => {
                error!("Error reading from client: {}", e);
                break;
            }
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    
    info!("Server starting on port 8080...");
    
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Server running on port 8080...");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                info!("New client connected: {}", addr);
                tokio::spawn(handle_client(stream));
            }
            Err(e) => error!("Failed to accept connection: {}", e),
        }
    }
}