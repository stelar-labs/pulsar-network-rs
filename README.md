# Pulsar Network

Pulsar Network is the distributed hash table peer-to-peer communication protocol for the Astreum Blockchain.

## Author

Roy R. O. Okello

[Email](mailto:0xR3y@protonmail.com)

[Github](https://github.com/0xR3y)

[Twitter](https://twitter.com/0xR3y)

## Features

- send and receive messages between peers.
- messages contain a body, chain, nonce, time and topic.
- peers can join the network by sending join requests to other peers returning known addresses
- peers can be pinged and respond with the route.
- message topics are block, block request, cancel transaction, join request, join response, ping request, ping response and transaction
- currently supported routes are peer and validation.

## API

### Client

```text
use pulsar_network::{Chain, Client, Route, Topic};

let bootstrap = false;

let chain = Chain::Test;

let route = Route::Peer;

let seeders: Vec<SocketAddr>;

let client = Client::new(bootstrap, chain, route, seeders);

for (message, source) in client.messages() {
    println!("Got: {}", message.body);
}
```

### Message

```text
use pulsar_network::{Message, Topic};

let block_bytes;

let mut message = Message::new(&block_bytes, &chain, &Topic::Block);
```

### Broadcast

```text
let tx_bytes;

client.broadcast(&tx_bytes, &Route::Validation, &Topic::Transaction);
```

### Send

```text
network.send(&address, &block_bytes, &Topic::Block)
```

## Contributions

Pull requests, bug reports and any kind of suggestion are welcome.

[Twitter](https://twitter.com/StelarLabs)

2022-06-18
