package main

type Message struct {
	SenderID   string              `json:"sender_id"`
	ReceiverID string              `json:"receiver_id"`
	MsgID      string              `json:"msg_id"`
	Type       string              `json:"type"`
	N          []string            `json:"n"`
	Topology   map[string][]string `json:"topology"`
}
