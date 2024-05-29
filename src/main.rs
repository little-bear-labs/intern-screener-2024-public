mod docker;
mod messages;
mod network;
mod progress;

use crate::docker::extract_result_file;
use crate::network::{connect_and_read_init, discover_network, send_topology};
use crate::progress::initialize_progress_bar;
use env_logger::Builder;
use log::{error, info};
use std::{io::Read, sync::Arc, time::Duration};

fn main() -> std::io::Result<()> {
    // Initialize logger
    Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // TCP Connection and Init Message
    let (mut stream, init_message) = match connect_and_read_init("127.0.0.1:12080") {
        Ok(result) => {
            info!("Received init message");
            result
        }
        Err(e) => {
            error!("Failed to connect and read init message: {}", e);
            return Err(e);
        }
    };

    // Initialize the progress bar
    let pb = initialize_progress_bar();

    // Discover the entire network
    let topology = match discover_network(&mut stream, &init_message.receiver_id, Arc::clone(&pb)) {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to discover the network: {}", e);
            return Err(e);
        }
    };

    // Send the topology to the proxy
    match send_topology(&mut stream, &init_message.receiver_id, topology) {
        Ok(_) => match stream.read(&mut [0; 1]) {
            Ok(0) => info!("Sent topology successfully"),
            Err(e) => error!("Error reading from stream: {}", e),
            _ => error!("Unexpected result while waiting for server to close the connection"),
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                error!("Server closed the connection. Exiting gracefully.");
                return Err(e);
            } else {
                error!("Failed to send the topology: {}", e);
                return Err(e);
            }
        }
    };

    // Wait for a short duration before exiting
    std::thread::sleep(Duration::from_secs(2));
    info!("Server closed the connection. Exiting gracefully.");

    // Extract result file from Docker Container
    match extract_result_file() {
        Ok(_) => {
            info!("File Extracted!");
        }
        Err(e) => {
            error!("Failed to extract the file");
            return Err(e);
        }
    };

    info!("Application exiting successfully");

    Ok(())
}
