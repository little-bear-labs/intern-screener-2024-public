import json
import pyperclip
from model import Message

def serialize(obj):
    return json.dumps(obj.to_dict()).encode()

def deserialize(data, cls):
    return cls.from_dict(json.loads(data))

def extract_messages_from_buffer(buffer):
    buffer_str = buffer.decode()
    valid_buffer = buffer_str[:buffer_str.rfind("}") + 1]
    invalid_buffer = buffer_str[buffer_str.rfind("}") + 1:]
    
    pyperclip.copy(valid_buffer)
    
    messages = []
    for msg_str in valid_buffer.split("}{"):
        msg_str = msg_str if msg_str.endswith("}") else msg_str + "}"
        msg = deserialize(msg_str.encode(), Message)
        messages.append(msg)
    
    return messages, invalid_buffer.encode()
