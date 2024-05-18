package main

import (
	"bytes"
	"fmt"
	"math/rand"
	"net"

	"golang.design/x/clipboard"
)

func main() {
	// Connect to the proxy server.
	conn, err := net.Dial("tcp", "localhost:12080")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	defer conn.Close()

	handle(conn)
}

func randomID() string {
	const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	b := make([]byte, 10)
	for i := range b {
		b[i] = charset[rand.Intn(len(charset))]
	}
	return string(b)
}

func readMessage(conn net.Conn, buffer *bytes.Buffer) (Message, error) {
	var msg Message
	tmp_buffer := make([]byte, 1024)

	n, err := conn.Read(tmp_buffer)
	if err != nil {
		return msg, err
	}

	// Attach the chunk to the buffer.
	buffer.Write(tmp_buffer[:n])
	messageQueue, incompleteBuffer, err := extractMessagesFromBuffer(buffer.String())
	if err != nil {
		return msg, err
	}

	// Reset the buffer in case the message(s) is successfully extracted.
	buffer.Reset()
	if incompleteBuffer != "" {
		// If any remaining incomplete buffer data attach it again.
		// So that next chunk can be appended to it.
		buffer.Write([]byte(incompleteBuffer))
	}

	// Return the first message from the queue.
	return messageQueue[0], nil
}

func sendMessage(conn net.Conn, msg Message) error {
	data, err := Serialize(msg)
	if err != nil {
		return err
	}

	_, err = conn.Write(data)
	return err
}

func makeMessage(msgType string, senderID string, receiverID string) Message {
	return Message{
		Type:       msgType,
		SenderID:   senderID,
		ReceiverID: receiverID,
		MsgID:      randomID(),
	}
}

func handle(conn net.Conn) {
	defer conn.Close()
	err := clipboard.Init()
	if err != nil {
		panic(err)
	}

	buffer := bytes.NewBuffer([]byte{})

	// Initialize data structures for BFS algorithm to create topology.
	queue := make([]string, 0)
	topology := make(map[string][]string)
	visited := make(map[string]bool)
	my_id := ""

	// Handle the init message to get the ID of our node.
	for {
		msg, err := readMessage(conn, buffer)

		if err != nil {
			// Buffer is incomplete yet.
			PrintError(err)
			continue
		}

		if msg.Type == "init" {
			InfoLog("Init message received")
			my_id = msg.ReceiverID
			fmt.Println("My ID: ", my_id)
			break
		}
	}

	// Start the BFS algorithm to create the topology.
	queue = append(queue, my_id)
	initQuery := makeMessage("query", my_id, my_id)
	visited[my_id] = true
	sendMessage(conn, initQuery)

	for len(queue) > 0 {
		msg, err := readMessage(conn, buffer)

		if err != nil {
			// Buffer is incomplete yet.
			clipboard.Write(clipboard.FmtText, buffer.Bytes())
			continue
		}

		fmt.Println("Received num of neigbours: ", len(msg.N))
		fmt.Println("Receiver: ", msg.ReceiverID)
		fmt.Println("Sender: ", msg.SenderID)

		node_id := queue[0]
		for _, neighbor := range msg.N {
			topology[node_id] = append(topology[node_id], neighbor)
		}

		// Equivalent to pop operation in queues in C/C++.
		queue = queue[1:]

		for _, neighbor := range msg.N {
			if !visited[neighbor] {
				queue = append(queue, neighbor)
				queryRPC := makeMessage("query", my_id, neighbor)

				// Print RPC query
				fmt.Println("Sender: ", queryRPC.SenderID)
				fmt.Println("Receiver: ", queryRPC.ReceiverID)

				visited[neighbor] = true
				sendMessage(conn, queryRPC)
			}
		}
	}

	fmt.Println("Toplogy is created.")

	finalMsg := Message{
		Type:       "topology",
		SenderID:   my_id,
		ReceiverID: "",
		MsgID:      randomID(),
		Topology:   topology,
	}

	// Save the topology in the file by sending it to the proxy server.
	sendMessage(conn, finalMsg)
}
