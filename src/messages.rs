use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DS for Init Message
#[derive(Serialize, Deserialize, Debug)]
pub struct InitMessage {
    pub sender_id: String,
    pub receiver_id: String,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
}

/// DS for Query Message
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryMessage {
    pub sender_id: String,
    pub receiver_id: String,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
}

/// DS for Neighbors Response
#[derive(Serialize, Deserialize, Debug)]
pub struct NeighborsResponse {
    pub sender_id: String,
    pub receiver_id: String,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub n: Vec<String>,
}

/// DS for Topology Message
#[derive(Serialize, Deserialize, Debug)]
pub struct TopologyMessage {
    pub sender_id: String,
    pub receiver_id: String,
    pub msg_id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub topology: HashMap<String, Vec<String>>,
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, json};

    #[test]
    fn test_serialize_init_message() {
        let init_message = InitMessage {
            sender_id: "server".to_string(),
            receiver_id: "client".to_string(),
            msg_id: "1234".to_string(),
            msg_type: "init".to_string(),
        };
        let serialized = serde_json::to_string(&init_message).unwrap();
        assert!(serialized.contains("\"sender_id\":\"server\""));
    }

    #[test]
    fn test_deserialize_init_message() {
        let json_data = r#"
        {
            "sender_id": "server",
            "receiver_id": "client",
            "msg_id": "1234",
            "type": "init"
        }"#;
        let init_message: InitMessage = serde_json::from_str(json_data).unwrap();
        assert_eq!(init_message.sender_id, "server");
        assert_eq!(init_message.receiver_id, "client");
    }

    #[test]
    fn test_serialize_query_message() {
        let query_message = QueryMessage {
            sender_id: "client".to_string(),
            receiver_id: "server".to_string(),
            msg_id: "1234".to_string(),
            msg_type: "query".to_string(),
        };
        let serialized = serde_json::to_string(&query_message).unwrap();
        assert!(serialized.contains("\"sender_id\":\"client\""));
    }

    #[test]
    fn test_deserialize_query_message() {
        let json_data = r#"
        {
            "sender_id": "client",
            "receiver_id": "server",
            "msg_id": "1234",
            "type": "query"
        }"#;
        let query_message: QueryMessage = serde_json::from_str(json_data).unwrap();
        assert_eq!(query_message.sender_id, "client");
        assert_eq!(query_message.receiver_id, "server");
    }

    #[test]
    fn test_serialize_neighbors_response() {
        let neighbors_response = NeighborsResponse {
            sender_id: "server".to_string(),
            receiver_id: "client".to_string(),
            msg_id: "1234".to_string(),
            msg_type: "neighbors".to_string(),
            n: vec!["node1".to_string(), "node2".to_string()],
        };
        let serialized = serde_json::to_string(&neighbors_response).unwrap();
        assert!(serialized.contains("\"n\":[\"node1\",\"node2\"]"));
    }

    #[test]
    fn test_deserialize_neighbors_response() {
        let json_data = r#"
        {
            "sender_id": "server",
            "receiver_id": "client",
            "msg_id": "1234",
            "type": "neighbors",
            "n": ["node1", "node2"]
        }"#;
        let neighbors_response: NeighborsResponse = serde_json::from_str(json_data).unwrap();
        assert_eq!(neighbors_response.n, vec!["node1", "node2"]);
    }

    #[test]
    fn test_serialize_topology_message() {
        let topology = HashMap::from([
            (
                "node1".to_string(),
                vec!["node2".to_string(), "node3".to_string()],
            ),
            (
                "node2".to_string(),
                vec!["node1".to_string(), "node3".to_string()],
            ),
        ]);

        let topology_message = TopologyMessage {
            sender_id: "client".to_string(),
            receiver_id: "server".to_string(),
            msg_id: "1234".to_string(),
            msg_type: "topology".to_string(),
            topology,
        };

        let expected_json = json!({
            "sender_id": "client",
            "receiver_id": "server",
            "msg_id": "1234",
            "type": "topology",
            "topology": {
                "node1": ["node2", "node3"],
                "node2": ["node1", "node3"]
            }
        });

        let serialized_message = serde_json::to_string(&topology_message).unwrap();
        let serialized_message_value: serde_json::Value =
            serde_json::from_str(&serialized_message).unwrap();

        assert_eq!(serialized_message_value, expected_json);
    }

    #[test]
    fn test_deserialize_topology_message() {
        let json_data = r#"
        {
            "sender_id": "client",
            "receiver_id": "server",
            "msg_id": "1234",
            "type": "topology",
            "topology": {}
        }"#;
        let topology_message: TopologyMessage = serde_json::from_str(json_data).unwrap();
        assert_eq!(topology_message.msg_type, "topology");
    }
}
