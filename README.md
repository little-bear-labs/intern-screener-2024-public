### Intro

Hi,
This screener is designed to test a few basic skills:
1. Reading and writing over a network connection (TCP to make things easy).
2. Serializing, deserializing message formats (JSON in this case).
3. Data structures.
4. Few shell commands.
5. Basic container usage.

You need to write a client node that wishes to join some existing network and discover the whole network. To abstract away the complications
of connecting to multiple nodes in the network, you are provided with a proxy (test executable) to manage connections and route
messages on behalf of your node.

You can run the proxy by executing the provided test executable (feel free to reverse engineer or analyse network traffic).
The proxy accepts a single TCP connection from your client and sends an initial Init message:
```JSON
{
    "sender_id": "<bootstrap_id>",
    "receiver_id": "<your_id>",
    "msg_id": "",
    "type": "init"
}
```
Assume that by the time the proxy sends you the init message, it has already made connections to other peers.
There are two accepted rpcs:
1. Query
2. Topology

The query operation is as follows:
```JSON
{
    "sender_id": "<your_id>",
    "receiver_id": "<some-node-id>",
    "msg_id": "<randomly generated string>",
    "type": "query"
}
```
This should return the following response, given the `sender_id` and `received_id` exist in the network. Do not
try to masquerade as a different node. This will make the proxy crash.
```JSON
{
    "sender_id": "<your_id>",
    "receiver_id": "<some-node-id>",
    "msg_id": "<randomly generated string>",
    "type": "neighbors"
    "n": JSON Array of strings
}
```
Each string in `n` is an ID of some node in the network to which the receiver has a direct connection.
You must use the `Query` rpc to discover all connected nodes in the network.

Once you have completed this task, you must use the `Topology` rpc to push your result to the proxy
where it may be used to optimally route future requests.
```JSON
{
    "sender_id": "<your_id>",
    "receiver_id": "<some-node-id>",
    "msg_id": "<randomly generated string>",
    "type": "topology",
    "topology": {
        "<node_id>": JSON Array of neighboring nodes,
        ...
    }
}
```

At the end of the test (once you have written the topology message and the binary has exited), a result file
called `test1.result.json` will be generated.

Hint: To know what connection the proxy has already made on your behalf, you can query the system for your own id.

### Running the proxy
Due to issues with compatibility across different platforms, we have decided to release the test as a docker image.
You are required to set up Docker on your system to pass this test. Please follow the guidelines for your platform 
provided at https://docs.docker.com/desktop/.

The docker image may be pulled from:
```docker pull ghcr.io/little-bear-labs/lbl-test-proxy:latest```

You can run the test as follows:
```docker run -p 12080:12080 ghcr.io/little-bear-labs/lbl-test-proxy:latest```
This will make the test program accessible via port 12080 on localhost.

Part of your task is to extract the generated result file from the binary.
By default, the test proxy will write the result file to it's local file system on a successful test.
This file is written to (w.r.t the container's file system): `/artifact/test1.result.json`.

Couple of hints:
1. Use `docker cp`.
2. Mount a directory into the container at the appropriate location.

### Submission

You must create a public github repository for your submission. Please check your solution code into this repository.
Please also include the generated solution file in the repository by uploading the json file as a release artifact.

Bonus points for well written commit messages.

### Example messages:

Init message from node `bootstrap`: 
```JSON
{
  "sender_id": "bootstrap",
  "receiver_id": "a",
  "msg_id": "",
  "type": "init"
}
```

Query node `b` for its neighbors: 
```JSON
{
  "sender_id": "a",
  "receiver_id": "b",
  "msg_id": "msg_id",
  "type": "query"
}
```

Neighbors response from node `b`: 
```JSON
{
  "sender_id": "b",
  "receiver_id": "a",
  "msg_id": "msg_id",
  "type": "neighbors",
  "n": [
    "c",
    "d",
    "a"
  ]
}
```

Topology: 
```JSON
{
  "sender_id": "a",
  "receiver_id": "",
  "msg_id": "msg_id",
  "type": "topology",
  "topology": {
    "a": [
      "b"
    ],
    "b": [
      "c",
      "a",
      "d"
    ]
  }
}
```
