# Documentation for the Rust Codebase

## Overview

This project consists of several Rust modules designed to interact with a Docker container, establish a TCP connection to a server, query the network, gather topology data, and extract results from the container. Below is a detailed breakdown of each component, its responsibilities, and how they interact.

## Modules

### 1. Main Module( `src/main.rs` )

This module is the main entry point of the application. It initializes the logger, establishes a TCP connection to the proxy server, discovers the network topology, sends the discovered topology to the proxy server, extracts a result file from a Docker container, and exits gracefully.

### `main() -> std::io::Result<()>`:

- Initializes the logger using `env_logger` and sets the log level to `Info`.
- Establishes a TCP connection to the proxy server at `127.0.0.1:12080`.
- Discovers the network topology by querying nodes recursively.
- Sends the discovered network topology to the proxy server.
- Waits for a short duration before exiting.
- Extracts a result file from a Docker container.
- Exits the application successfully.

### 2. Library file( `src/lib.rs` )

This file defines the public modules `docker`, `messages`, `network`, and `progress`.

### 3. Docker Module( `src/docker.rs` )

This module handles interactions with Docker, specifically retrieving the container ID and extracting files from the container.

#### `get_container_id() -> Option<String>`
- Retrieves the latest Docker container ID based on the image name.
- As the image is stopped after every run we have to get the lastest stopped image id Ex., *93ba9f52e9ea*

#### `extract_result_file()`

- Retrieves the container ID of the latest container based on a specific image.
- Copies the result file from the Docker container to the local directory.

### 4. Messages Module( `src/messages.rs` )

Defines data structures for various message types used in the network communication, and provides serialization and deserialization capabilities.

- Data Structures:

    - `InitMessage`: Represents the initial message structure.
    - `QueryMessage`: Represents a query message sent to a node.
    - `NeighborsResponse`: Represents the response message containing neighbors of a node.
    - `TopologyMessage`: Represents the message containing the network topology.

### 5. Network Module( `src/network.rs` )

This module contains functions for network-related operations, including establishing a TCP connection, querying nodes, discovering network topology, and sending the topology to the proxy server.

#### `connect_and_read_init(address: &str) -> std::io::Result<(TcpStream, InitMessage)>`

- Establishes a TCP connection to the specified address.
- Reads the initialization message from the stream.

#### `query_and_response(stream: &mut TcpStream, sender_id: &str, receiver_id: &str) -> std::io::Result<NeighborsResponse>`

- Sends a query message to a node specified by `receiver_id`.
- Receives the node's neighbors' response.

#### `discover_network(stream: &mut TcpStream, your_id: &str, pb: Arc<Mutex<ProgressBar>>) -> io::Result<HashMap<String, Vec<String>>>`

- Discovers the network topology by recursively querying nodes and gathering responses.
- Utilizes a progress bar to indicate the discovery progress.

#### `send_topology(stream: &mut TcpStream, your_id: &str, topology: HashMap<String, Vec<String>>) -> std::io::Result<()>`

- Sends the discovered network topology to the proxy server.

### 6. Progress Module( `src/progress.rs` )

Manages the progress bar used during the network discovery process.

#### `initialize_progress_bar() -> Arc<Mutex<ProgressBar>>`

- Initializes a progress bar with a spinner style.

### 7. Integration Test( `tests/integration_test.rs` )

Tests the entire flow of starting a Docker container, establishing a TCP connection, discovering the network topology, sending the topology, and extracting the result file.

#### `start_discover_container() -> Child`
- Starts the Docker container.

#### `stop_docker_container(child: &mut Child)`
- Stops the Docker container.

#### `test_network_discovery_and_result_extraction()`
- Comprehensive test function that:
    - Starts a Docker container.
    - Establishes a TCP connection to the proxy server running in the container.
    - Discovers the network topology.
    - Sends the discovered topology to the proxy server.
    - Extracts a result file from the Docker container.
    - Stops the Docker container.
    - Asserts that the result file exists and removes it after the test.

## Installation and Running the application 

### Dependencies:
1. **Rust Toolchain** - Make sure you have Rust installed. You can install it via [rustup](https://rustup.rs/).
2. **Docker** - Ensure Docker is installed and running on your system.

### Installation:
1. Clone the project repository:
    ```bash
    git clone https://github.com/AtharvaWaghchoure/intern-screener-2024-public
    ```
2. Navigate to the project directory:
    ```bash
    cd intern-screener-2024-public
    ```
3. Build the project using Cargo (Rust's package manager):
    ```bash
    cargo build --release
    ```
4. Pull the docker image for the Proxy image:
    ```bash
    docker pull ghcr.io/little-bear-labs/lbl-test-proxy:latest
    ```

### Running the Application:
1. Open a terminal and run docker image:
    ```bash
    docker run -p 12080:12080 ghcr.io/little-bear-labs/lbl-test-proxy:latest
    ```
2. In seperate terminal, run the application using the following command in the project repository:
    ```bash
    cargo run --release
    ```
3. On Completion of the program you may check or read the extracted file:
    ```bash
    $ ls
    Cargo.lock  NOTES.md   src/     test1.result.json # extracted file
    Cargo.toml  README.md  target/  tests/
    ```
    or
    ```bash
    cat test1.result.json
    ```
    **Possible Error Handling**:
    - Docker Not Running: If Docker is not running, you'll likely encounter errors related to Docker commands. Make sure Docker is up and running before executing the application.

    - Failed to Connect to TCP Server: If the application fails to connect to the TCP server, ensure that the server address and port are correctly configured. Check network connectivity and firewall settings if necessary.

    - Failed to Extract Result File: If the application fails to extract the result file from the Docker container, check Docker permissions and ensure that the container ID is correctly retrieved.

    - Dependency Errors: If you encounter any dependency-related errors during the build process, make sure you have all necessary dependencies installed and that your Rust toolchain is up to date.

    - Compilation Errors: If you encounter compilation errors during the build process, check the project's dependencies and ensure that all code is correctly formatted and up to date.

### Testing the Application:
- To run the integration tests and unit tests, use the following command:
    ```bash
    cargo test
    ```
### Additional Notes
- Ensure that the Docker image specified in the docker.rs module exists and is accessible from your Docker registry.
