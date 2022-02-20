## Pulsar Network

Pulsar Network is a distributed hash table peer-to-peer messaging protocol for the Astreuos Blockchain written in Rust.

### Features
- Send & Receive Messages between Peers.
- A Message contains the nonce, body, 
- Message encryption using chacha20poly1305 and a blake3 hash, of the shared point on Curve 25519, as the key.
- Peers can be pinged and respond with their public key & routes.
- Currently supported route is Astreuos Blockchain Validation.
 
### API

`Connect`

```
use pulsar_network::Network;

let network = Network::connect();

network.validation = true; // join the validation route

```

`Message`

```

use pulsar_network::Message;

let msg: &str = "Hello";

let mut message = Message::new(msg);

message.expiry = 7_u8; // the default expiry is 1

```

`Broadcast`

```

network.validation.broadcast(message);

```

### Contributions
Pull requests, bug reports and any kind of suggestion are welcome.

2022-02-20
