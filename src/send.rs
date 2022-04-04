use crate::envelope::{ Context, Envelope };
use crate::Message;
use crate::Connection;
use crate::Peer;
use fides::chacha20poly1305;
use std::sync::Arc;

impl Connection {

    pub fn send(&self, message: Message, peer: Peer) {

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();

        let cipher = chacha20poly1305::encrypt(&peer.shared_key, &message.to_astro().into_bytes());

        let envelope = Envelope::from(Context::Encrypted, cipher, self.public_key);
        
        outgoing_socket.send_to(&envelope.to_astro().into_bytes(), peer.address).unwrap();

    }
}
