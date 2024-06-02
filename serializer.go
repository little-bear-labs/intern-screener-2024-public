package main

import (
	"encoding/json"
	"strings"
)

type Serializeable interface{}

func Deserialize(data []byte, s Serializeable) error {
	return json.Unmarshal(data, s)
}

func Serialize(s Serializeable) ([]byte, error) {
	return json.Marshal(s)
}

func extractMessagesFromBuffer(buffer string) ([]Message, string, error) {
	var messages []Message

	validBuffer := buffer[:strings.LastIndex(buffer, "}")+1]
	invalidBuffer := buffer[strings.LastIndex(buffer, "}")+1:]

	// Debugging...

	var err error
	for _, msg := range strings.Split(validBuffer, "}{") {
		// We need to add the closing bracket back to the message

		if !strings.HasSuffix(msg, "}") {
			msg += "}"
		}

		var message Message
		err = Deserialize([]byte(msg), &message)
		if err != nil {
			break
		}

		messages = append(messages, message)
	}

	return messages, invalidBuffer, err
}
