use std::collections::{HashMap, VecDeque};
use std::error::Error;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    sender_id: String,
    receiver_id: String,
    msg_id: String,
    r#type: String,
    n: Option<Vec<String>>,
    topology: Option<HashMap<String, Vec<String>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the proxy server.
    let mut conn = TcpStream::connect("localhost:12080").await?;

    handle(&mut conn).await?;
    Ok(())
}

fn random_id() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..10).map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset.chars().nth(idx).unwrap()
    }).collect()
}

async fn read_message(conn: &mut TcpStream, buffer: &mut Vec<u8>) -> Result<Message, Box<dyn Error>> {
    let mut tmp_buffer = vec![0; 1024];
    let n = conn.read(&mut tmp_buffer).await?;
    if n == 0 {
        return Err("Connection closed".into());
    }
    buffer.extend_from_slice(&tmp_buffer[..n]);

    let (messages, incomplete_buffer) = extract_messages_from_buffer(buffer)?;

    // Reset the buffer in case the message(s) is successfully extracted.
    buffer.clear();
    buffer.extend_from_slice(&incomplete_buffer);

    // Return the first message from the queue.
    messages.into_iter().next().ok_or_else(|| "No message found".into())
}

async fn send_message(conn: &mut TcpStream, msg: Message) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_vec(&msg)?;
    conn.write_all(&data).await?;
    Ok(())
}

fn make_message(msg_type: &str, sender_id: &str, receiver_id: &str) -> Message {
    Message {
        r#type: msg_type.to_string(),
        sender_id: sender_id.to_string(),
        receiver_id: receiver_id.to_string(),
        msg_id: random_id(),
        n: None,
        topology: None,
    }
}

async fn handle(conn: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let mut buffer = Vec::new();

    // Initialize data structures for BFS algorithm to create topology.
    let mut queue = VecDeque::new();
    let mut topology: HashMap<String, Vec<String>> = HashMap::new();
    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut my_id = String::new();

    // Handle the init message to get the ID of our node.
    loop {
        let msg = read_message(conn, &mut buffer).await?;
        if msg.r#type == "init" {
            println!("Init message received");
            my_id = msg.receiver_id.clone();
            println!("My ID: {}", my_id);
            break;
        }
    }

    // Start the BFS algorithm to create the topology.
    queue.push_back(my_id.clone());
    let init_query = make_message("query", &my_id, &my_id);
    visited.insert(my_id.clone(), true);
    send_message(conn, init_query).await?;

    while let Some(node_id) = queue.pop_front() {
        let msg = match read_message(conn, &mut buffer).await {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error reading message: {}", e);
                continue;
            }
        };

        if let Some(neighbors) = &msg.n {
            println!("Received num of neighbours: {}", neighbors.len());
            println!("Receiver: {}", msg.receiver_id);
            println!("Sender: {}", msg.sender_id);

            for neighbor in neighbors {
                topology.entry(node_id.clone()).or_default().push(neighbor.clone());
            }

            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    queue.push_back(neighbor.clone());
                    let query_rpc = make_message("query", &my_id, neighbor);

                    println!("Sender: {}", query_rpc.sender_id);
                    println!("Receiver: {}", query_rpc.receiver_id);

                    visited.insert(neighbor.clone(), true);
                    send_message(conn, query_rpc).await?;
                }
            }
        }
    }

    println!("Topology is created.");

    let final_msg = Message {
        r#type: "topology".to_string(),
        sender_id: my_id.clone(),
        receiver_id: "".to_string(),
        msg_id: random_id(),
        topology: Some(topology),
        n: None,
    };

    send_message(conn, final_msg).await?;
    Ok(())
}

fn extract_messages_from_buffer(buffer: &[u8]) -> Result<(Vec<Message>, Vec<u8>), Box<dyn Error>> {
    let buffer_str = String::from_utf8_lossy(buffer);
    let valid_buffer = match buffer_str.rfind('}') {
        Some(pos) => &buffer_str[..pos + 1],
        None => return Ok((Vec::new(), buffer.to_vec())),
    };
    let invalid_buffer = &buffer_str[buffer_str.rfind('}')? + 1..];

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(valid_buffer.to_string())?;

    let mut messages = Vec::new();
    for msg in valid_buffer.split("}{") {
        let msg = if msg.ends_with('}') { msg.to_string() } else { format!("{}{}", msg, "}") };
        let message: Message = match serde_json::from_str(&msg) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to parse message: {}", e);
                continue;
            }
        };
        messages.push(message);
    }

    Ok((messages, invalid_buffer.as_bytes().to_vec()))
}
