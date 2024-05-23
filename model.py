class Message:
    def __init__(self, sender_id="", receiver_id="", msg_id="", type="", n=None, topology=None):
        self.sender_id = sender_id
        self.receiver_id = receiver_id
        self.msg_id = msg_id
        self.type = type
        self.n = n or []
        self.topology = topology or {}

    def to_dict(self):
        return {
            "sender_id": self.sender_id,
            "receiver_id": self.receiver_id,
            "msg_id": self.msg_id,
            "type": self.type,
            "n": self.n,
            "topology": self.topology
        }

    @staticmethod
    def from_dict(data):
        return Message(
            sender_id=data.get("sender_id"),
            receiver_id=data.get("receiver_id"),
            msg_id=data.get("msg_id"),
            type=data.get("type"),
            n=data.get("n"),
            topology=data.get("topology")
        )
