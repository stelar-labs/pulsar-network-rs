## Pulsar Network

Pulsar Network is a distributed hash table peer-to-peer messaging protocol for the Astreuos Blockchain written in Rust.

### Features
- Send and Receive Messages between Peers.
- A Message contains the Body, Message Kind, a Nonce and the Sender Peer. 
- Message encryption using ChaCha20Poly1305 and a Blake3 hash, of the shared point on Curve 25519, as the key.
- Peers can be pinged and respond with their public key & routes supported.
- Currently supported route is Astreuos Blockchain Validation.
 
### API

`Connect`

```
use pulsar_network::Network;

let network = Network::config();

network.validation = true; // join the validation route

for message in network.messages {
    println!("Got: {}", message);
}

```

`Message`

```

use pulsar_network::{Message, MessageKind};

let message_body: &str = "Hello";

let mut message = Message::new(MessageKind::Block, message_body);

message = message.expiry(7_u8); // the default expiry is 1

```

`Broadcast`

```

use pulsar_network::Routes;

network.broadcast(message, Routes::Validation);

```

`Send`

```

network.send(message, incoming_message.sender)

```

### Contributions
Pull requests, bug reports and any kind of suggestion are welcome.

2022-02-23