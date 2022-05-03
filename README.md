# Pulsar Network

Pulsar Network is the distributed hash table peer-to-peer communication protocol for the Astreuos Blockchain.

## Features

- send and receive messages between peers.
- messages are contained in an envelope with the context, nonce, sender, route and time.
- message encryption uses chacha20poly1305 and a x25519 blake3 flavor as the key.
- peers can join the network by sending join requests and valid peers returning the nearest peers
- peers can be pinged and respond with their public key and route supported.
- currently supported routes are Astreuos blockchain main and test routes used for validation.

## API

### Client

```text
use pulsar_network::{Client, Route};

let bootstrap = false;

let route = Route::Test

let seeders: Vec<SocketAddr>;

let client = Client::new(bootstrap, route, seeders);

for (message, peer) in client.messages() {
    println!("Got: {}", message.body);
}
```

### Message

```text
use pulsar_network::{Message, Kind};

let mut message = Message::new(Kind::Block, message_bytes);
```

### Broadcast

```text
network.broadcast(message);
```

### Send

```text
network.send(message, peer)
```

## Improvements

- multi-threading
- ephemeral keys
- bluetooth communication

## Contributions

Pull requests, bug reports and any kind of suggestion are welcome.

2022-05-03
