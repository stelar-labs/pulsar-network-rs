use crate::envelope::{ Context, Envelope };
use crate::Message;
use crate::Connection;
use fides::chacha20poly1305;
use std::sync::Arc;

impl Connection {

    pub fn broadcast(&self, message: Message) {

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();
        
        let peers_clone = Arc::clone(&self.peers);

        let peers = peers_clone.lock().unwrap();

        for (_, list) in peers.clone() {

            for (_, peer) in list {

                let cipher = chacha20poly1305::encrypt(&peer.shared_key, &message.to_astro().into_bytes());

                let envelope = Envelope::from(Context::Encrypted, cipher, self.public_key);

                outgoing_socket.send_to(&envelope.to_astro().into_bytes(), &peer.address).unwrap();

            }
        }
        
    }
}
