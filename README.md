## Pulsar Network

Pulsar Network is a distributed hash table peer-to-peer messaging protocol for the Astreuos Blockchain written in Rust.

### Features
- Send and Receive Messages between Peers.
- A Message contains the body, messagekind, nonce and time. 
- Message encryption uses chacha20poly1305 and a blake3 hash, of the shared point on curve25519, as the key.
- Peers can be pinged and respond with their public key & routes supported.
- Currently supported routes are Astreuos Blockchain Main & Test Validation.
 
### API

`Connect`

```
use pulsar_network::{ Network, Routes };

let route = Routes::TestValidation;

let network = Network::config(route);

for (message, peer) in network.listen() {
    println!("Got: {}", message.body);
}

```

`Message`

```

use pulsar_network::{ Message, MessageKind };

let mut message = Message::new(MessageKind::Block, block in bytes);

```

`Broadcast`

```

network.broadcast(message);

```

`Send`

```

network.send(message, peer)

```

### Contributions
Pull requests, bug reports and any kind of suggestion are welcome.

2022-03-08