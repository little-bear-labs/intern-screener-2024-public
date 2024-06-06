import socket
import random
import string
import pyperclip
from model import Message
from serializer import serialize,extract_messages_from_buffer
from savefile import makefile

def random_id():
    charset = string.ascii_letters + string.digits
    return ''.join(random.choice(charset) for _ in range(10))

def read_message(conn, buffer):
    tmp_buffer = conn.recv(1024)
    if not tmp_buffer:
        raise ConnectionError("Connection closed by the server")
    
    buffer.extend(tmp_buffer)
    message_queue, incomplete_buffer = extract_messages_from_buffer(buffer)
    
    # Clear buffer and store incomplete part
    buffer.clear()
    buffer.extend(incomplete_buffer)
    
    return message_queue[0]

def send_message(conn, msg):
    data = serialize(msg)
    conn.sendall(data)

def make_message(msg_type, sender_id, receiver_id):
    return Message(
        type=msg_type,
        sender_id=sender_id,
        receiver_id=receiver_id,
        msg_id=random_id()
    )

def handle(conn):
    pyperclip.copy("")
    
    buffer = bytearray()
    queue = []
    topology = {}
    visited = {}
    my_id = ""

    # Handle the init message to get the ID of our node.
    while True:
        try:
            msg = read_message(conn, buffer)
        except Exception as e:
            print("Error:", e)
            continue
        
        if msg.type == "init":
            print("Init message received")
            my_id = msg.receiver_id
            print("My ID:", my_id)
            break
    
    queue.append(my_id)
    init_query = make_message("query", my_id, my_id)
    visited[my_id] = True
    send_message(conn, init_query)
    
    while queue:
        try:
            msg = read_message(conn, buffer)
        except Exception as e:
            print("Error:", e)
            pyperclip.copy(buffer.decode())
            continue
        
        print("Received num of neighbors:", len(msg.n))
        print("Receiver:", msg.receiver_id)
        print("Sender:", msg.sender_id)

        node_id = queue.pop(0)
        topology[node_id] = msg.n
        
        for neighbor in msg.n:
            if not visited.get(neighbor):
                queue.append(neighbor)
                query_rpc = make_message("query", my_id, neighbor)
                
                print("Sender:", query_rpc.sender_id)
                print("Receiver:", query_rpc.receiver_id)
                
                visited[neighbor] = True
                send_message(conn, query_rpc)
    
    print("Topology is created.")
    
    final_msg = Message(
        type="topology",
        sender_id=my_id,
        receiver_id="",
        msg_id=random_id(),
        topology=topology
    )
    
    send_message(conn, final_msg)

def main():
    # for docker linux conntinter connections
    # host = '172.17.0.1'
    host = 'localhost'
    port = 12080
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as conn:
        conn.connect((host, port))
        handle(conn)

if __name__ == "__main__":
    main()
    makefile()
