
use crate::Message;
use crate::Network;
use crate::Peer;
use fides::chacha20poly1305;
use std::sync::Arc;

impl Network {

    pub fn send(&self, message: Message, peer: Peer) {

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();

        let cipher = chacha20poly1305::encrypt(&peer.shared_key, &message.into_bytes());
        
        let msg = [vec![5], self.public_key.to_vec(), cipher].concat();
        
        outgoing_socket.send_to(&msg, peer.address).unwrap();

    }
}