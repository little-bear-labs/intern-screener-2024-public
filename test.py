import socket
import json

def send_message(sock, message):
    """Send a JSON message over a socket."""
    message_str = json.dumps(message)
    sock.sendall(message_str.encode())

def receive_message(sock):
    """Receive a JSON message from a socket."""
    data = sock.recv(1024).decode()
    return json.loads(data)

def main():
    # Specify the address and port of the Docker container running the proxy
    docker_address = ('localhost', 12080)  # Adjust as needed

    # Connect to the Docker container
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            sock.connect(docker_address)

            # Send a message to the proxy
            message = {
                "sender_id": "client_id",
                "receiver_id": "proxy_id",
                "type": "query"
            }
            send_message(sock, message)

            # Receive a response from the proxy
            response = receive_message(sock)
            print("Received response from proxy:", response)

    except ConnectionRefusedError:
        print("Connection to proxy refused.")
    except Exception as e:
        print("Error:", e)

if __name__ == "__main__":
    main()
