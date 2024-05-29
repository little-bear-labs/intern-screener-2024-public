#![allow(unused_imports)]
use crate::messages::{InitMessage, NeighborsResponse, QueryMessage, TopologyMessage};
use indicatif::ProgressBar;
use log::{error, info};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

/// Method to establish a TCP connection to the specified address and read the Init message
pub fn connect_and_read_init(address: &str) -> std::io::Result<(TcpStream, InitMessage)> {
    let mut stream = TcpStream::connect(address)
        .map_err(|e| {
            error!("Could not connect to {}: {}", address, e);
            e
        })
        .unwrap();
    info!("Connected to proxy server at {}", address);

    // Reading the Init mesage from the stream
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];
    loop {
        let bytes_read = stream
            .read(&mut temp_buffer)
            .map_err(|e| {
                error!("Error reading from stream: {}", e);
                e
            })
            .unwrap();

        if bytes_read == 0 {
            error!("Connection close before receiving init message");
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Connection closed before receiving init message",
            ));
        }

        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
        if let Ok(init_message) = serde_json::from_slice::<InitMessage>(&buffer) {
            return Ok((stream, init_message));
        }
    }
}

/// Method to send a query message to a node and receives its neighbors' response
pub fn query_and_response(
    stream: &mut TcpStream,
    sender_id: &str,
    receiver_id: &str,
) -> std::io::Result<NeighborsResponse> {
    let msg_id = Uuid::new_v4().to_string();
    let query_message = QueryMessage {
        sender_id: sender_id.to_string(),
        receiver_id: receiver_id.to_string(),
        msg_id: msg_id.clone(),
        msg_type: "query".to_string(),
    };

    // Serializing and sent the query
    let query_message_json = serde_json::to_string(&query_message)
        .map_err(|e| {
            error!("Failed to serialize query message: {}", e);
            io::Error::new(io::ErrorKind::Other, "Serialize error")
        })
        .unwrap();
    stream
        .write_all(query_message_json.as_bytes())
        .map_err(|e| {
            error!("Failed to send query message: {}", e);
            e
        })
        .unwrap();

    // read and deserialize the response
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];
    loop {
        let bytes_read = stream
            .read(&mut temp_buffer)
            .map_err(|e| {
                error!("Error reading from stream: {}", e);
                e
            })
            .unwrap();
        if bytes_read == 0 {
            error!("Connection closed before receiving neighbors response");
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Connection closed before receiving neighbors response",
            ));
        }
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
        if let Ok(neighbors_response) = serde_json::from_slice::<NeighborsResponse>(&buffer) {
            return Ok(neighbors_response);
        }
    }
}

/// Method to discover the network topology by recursively querying nodes and gathering responses
pub fn discover_network(
    stream: &mut TcpStream,
    your_id: &str,
    pb: Arc<Mutex<ProgressBar>>,
) -> io::Result<HashMap<String, Vec<String>>> {
    let mut topology = HashMap::new();
    let mut to_query = vec![your_id.to_string()];
    let mut queried = HashSet::new();

    while let Some(current_id) = to_query.pop() {
        if queried.contains(&current_id) {
            continue;
        }

        let neighbors_response = query_and_response(stream, your_id, &current_id)?;
        queried.insert(current_id.clone());
        topology.insert(current_id.clone(), neighbors_response.n.clone());

        // Update progress bar
        update_progress_bar(&pb, &queried);

        let new_neighbors = neighbors_response.n.into_iter()
            .filter(|neighbor| !queried.contains(neighbor))
            .collect::<Vec<_>>();

        to_query.extend(new_neighbors);
    }

    finish_progress_bar(&pb, &queried);
    Ok(topology)
}

fn update_progress_bar(pb: &Arc<Mutex<ProgressBar>>, queried: &HashSet<String>) {
    let  pb = pb.lock().unwrap();
    pb.set_message(format!("Discovered {} nodes", queried.len()));
    pb.tick();
}

fn finish_progress_bar(pb: &Arc<Mutex<ProgressBar>>, queried: &HashSet<String>) {
    let  pb = pb.lock().unwrap();
    pb.finish_and_clear();
    info!("Discovery Complete");
    info!("Total nodes discovered: {}", queried.len());
    info!("Sending topology");
}

/// Method to send the network topology to the proxy server
pub fn send_topology(
    stream: &mut TcpStream,
    your_id: &str,
    topology: HashMap<String, Vec<String>>,
) -> std::io::Result<()> {
    let msg_id = Uuid::new_v4().to_string();
    let topology_message = TopologyMessage {
        sender_id: your_id.to_string(),
        receiver_id: "".to_string(),
        msg_id,
        msg_type: "topology".to_string(),
        topology,
    };

    // Serialize and send the topology message
    let topology_message_json = serde_json::to_string(&topology_message)
        .map_err(|e| {
            error!("Failed to serialize topology message: {}", e);
            io::Error::new(io::ErrorKind::Other, "Serialize error")
        })
        .unwrap();
    stream
        .write_all(topology_message_json.as_bytes())
        .map_err(|e| {
            error!("Failed to send topology message: {}", e);
            e
        })
        .unwrap();
    Ok(())
}
