## Pulsar Network

Pulsar Network is a distributed hash table peer-to-peer messaging protocol for the Astreuos Blockchain written in Rust.

### Features
- Send and Receive Messages between Peers.
- A Message contains the Body, Message Kind and a Nonce. 
- Message encryption uses ChaCha20Poly1305 and a Blake3 hash, of the shared point on Curve 25519, as the key.
- Peers can be pinged and respond with their public key & routes supported.
- Currently supported routes are Astreuos Blockchain Main & Test Validation.
 
### API

`Connect`

```
use pulsar_network::{ Network, Routes };

let route = Routes::TestValidation;

let network = Network::config(route);

for (message, peer) in network.connect() {
    println!("Got: {}", message.body);
}

```

`Message`

```

use pulsar_network::{ Message, MessageKind };

let block_astro_fmt: &str = "0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00";

let mut message = Message::new(MessageKind::Block, block_astro_fmt);

message = message.expiry(7_u8); // the default expiry is 1

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

2022-02-27