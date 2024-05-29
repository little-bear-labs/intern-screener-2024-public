use indicatif::ProgressBar;
use intern_screener_2024_public::{
    docker::extract_result_file,
    network::{discover_network, send_topology},
};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn start_docker_container() -> Child {
    Command::new("docker")
        .args([
            "run",
            "-d",
            "-p",
            "12080:12080",
            "ghcr.io/little-bear-labs/lbl-test-proxy:latest",
        ])
        .spawn()
        .expect("Failed to start Docker container")
}

fn stop_docker_container(child: &mut Child) {
    child.kill().expect("Failed to kill Docker container");
    child.wait().expect("Failed to wait on Docker container");
}

#[test]
fn test_network_discovery_and_result_extraction() {
    // Start the Docker container
    let mut child = start_docker_container();

    // Wait for the container to fully start
    thread::sleep(Duration::from_secs(5));

    // Establish TCP connection to the proxy server
    let mut stream =
        TcpStream::connect("127.0.0.1:12080").expect("Failed to connect to proxy server");

    // Read the init message from the server
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 512];

    loop {
        let bytes_read = stream
            .read(&mut temp_buffer)
            .expect("Failed to read from stream");
        if bytes_read == 0 {
            break; // Connection closed
        }
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);

        // Try to deserialize the message
        match serde_json::from_slice::<Value>(&buffer) {
            Ok(message) => {
                println!("Received init message: {:?}", message);
                break;
            }
            Err(e) => {
                if e.is_eof() {
                    // Continue reading if we hit EOF while parsing
                    continue;
                } else {
                    panic!("Failed to deserialize init message: {}", e);
                }
            }
        }
    }

    let init_message: Value =
        serde_json::from_slice(&buffer).expect("Failed to deserialize init message");

    let receiver_id = init_message["receiver_id"]
        .as_str()
        .expect("Missing receiver_id");

    let progress_bar = Arc::new(Mutex::new(ProgressBar::new(100)));

    let _ = discover_network(&mut stream, receiver_id, progress_bar.clone())
        .expect("Failed to discover network");

    let dummy_topology: HashMap<String, Vec<String>> = HashMap::new();

    send_topology(&mut stream, receiver_id, dummy_topology).expect("Failed to send topology");

    extract_result_file().expect("Failed to extract result file");

    stop_docker_container(&mut child);

    let local_path = std::env::current_dir()
        .unwrap_or_else(|e| panic!("Failed to get current directory: {}", e))
        .join("test1.result.json");
    assert!(local_path.exists(), "Result file does not exist");

    std::fs::remove_file(local_path).expect("Failed to remove test artifact");
}
