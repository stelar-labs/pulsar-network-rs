use crate::envelope::{ Context, Envelope };
use crate::{Client, Message};
use fides::chacha20poly1305;
use std::sync::Arc;

impl Client {

    pub fn broadcast(&self, message: Message) {

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();
        
        let peers_clone = Arc::clone(&self.peers);

        let peers = peers_clone.lock().unwrap();

        for (_, list) in peers.clone() {

            for (_, peer) in list {

                match chacha20poly1305::encrypt(&peer.shared_key, &message.to_bytes()) {

                    Ok(cipher) => {

                        let envelope = Envelope::from(Context::Encrypted, cipher, self.public_key);
                        
                        let _r = outgoing_socket.send_to(&envelope.to_bytes(), &peer.address);

                    },
                    Err(_) => ()
                }
            }
        }
    }
}
