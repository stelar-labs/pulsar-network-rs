## Pulsar Network

Pulsar Network is the distributed hash table peer-to-peer communication protocol for the Astreuos Blockchain.

### Features
- send and receive messages between peers.
- a message contains the body, messagekind, nonce and time. 
- message encryption uses chacha20poly1305 and a blake3 hash, of the shared point on curve25519, as the key.
- peers can be pinged and respond with their public key and route supported.
- currently supported routes are Astreuos Blockchain main and test validation routes.
 
### API

`Connect`

```
use pulsar_network::{ Connection, Route };

let route: Route = Route::TestNova;

let seeders: Vec<SocketAddr>;

let bootstrap_mode: bool = false;

let network = Connection::configure(route, seeders, bootstrap);

for (message, peer) in network.listen() {
    println!("Got: {}", message.body);
}

```

`Message`

```

use pulsar_network::{ Message, Kind };

let mut message = Message::new(Kind::Block, astro_list_bytes);

```

`Broadcast`

```

network.broadcast(message);

```

`Send`

```

network.send(message, peer)

```

### Improvements

- multi-threading
- ephemeral keys
- bluetooth communication

### Contributions
Pull requests, bug reports and any kind of suggestion are welcome.

2022-04-05