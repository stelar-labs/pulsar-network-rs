# Pulsar Network

Pulsar Network is the distributed hash table peer-to-peer communication protocol for the Astreuos Blockchain.

## Features

- send and receive messages between peers.
- messages are contained in an envelope with the context, nonce, sender and time.
- message encryption uses chacha20poly1305 and a x25519 blake3 flavor as the key.
- peers can join the network by sending join requests and valid peers returning their peer list
- peers can be pinged and respond with their public key and route supported.
- currently supported routes are Astreuos Blockchain Main and Test Nova Routes used for validation.
 
## API

### Connect

```
use pulsar_network::{ Connection, Route };

let route: Route = Route::TestNova;

let seeders: Vec<SocketAddr>;

let bootstrap_mode: bool = false;

let network = Connection::configure(route, seeders, bootstrap_mode);

for (message, peer) in network.listen() {
    println!("Got: {}", message.body);
}

```

### Message

```

use pulsar_network::{ Message, Kind };

let mut message = Message::new(Kind::PostBlock, astro_list_bytes);

```

### Broadcast

```

network.broadcast(message);

```

### Send

```

network.send(message, peer)

```

## Improvements

- multi-threading
- ephemeral keys
- bluetooth communication

## Contributions

Pull requests, bug reports and any kind of suggestion are welcome.

2022-04-05
