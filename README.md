## Pulsar Network

Pulsar Network is a distributed hash table peer-to-peer communication protocol for the Astreuos Blockchain.

### Features
- send and receive messages between peers.
- a message contains the body, messagekind, nonce and time. 
- message encryption uses chacha20poly1305 and a blake3 hash, of the shared point on curve25519, as the key.
- peers can be pinged and respond with their public key and route supported.
- currently supported routes are Astreuos Blockchain main and test validation routes.
 
### API

`Connect`

```
use pulsar_network::{Network, Route};

let route = Route::TestValidation;

let seeders: Vec<SocketAddr>;

let network = Network::configure(route, seeders);

for (message, peer) in network.listen() {
    println!("Got: {}", message.body);
}

```

`Message`

```

use pulsar_network::{Message, MessageKind};

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

### Future
- add sender peer ip information to messages
- multithreaded listening
- ephemeral keys & ports
- bluetooth and wifi communication

### Contributions
Pull requests, bug reports and any kind of suggestion are welcome.

2022-03-26