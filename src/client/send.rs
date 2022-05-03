use crate::envelope::{Envelope, Kind};
use crate::{Client, Message};
use crate::Peer;
use fides::chacha20poly1305;
use std::sync::Arc;

impl Client {

    pub fn send(&self, message: Message, peer: Peer) {

        let outgoing_clone = Arc::clone(&self.outgoing_socket);

        let outgoing = outgoing_clone.lock().unwrap();

        match chacha20poly1305::encrypt(&peer.shared_key, &message.to_bytes()) {

            Ok(cipher) => {

                let envelope = Envelope::new(Kind::Encrypted, &cipher, &self.public_key, &self.route);
        
                let _r = outgoing.send_to(&envelope.to_bytes(), peer.address);

            },
            Err(_) => ()
        }
    }
}
