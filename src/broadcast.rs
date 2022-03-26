
use crate::Message;
use crate::Network;
use fides::chacha20poly1305;
use std::sync::Arc;

impl Network {

    pub fn broadcast(&self, message: Message) {

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();
        
        let peers_clone = Arc::clone(&self.peers);

        let peers = peers_clone.lock().unwrap();

        for (_, list) in peers.clone() {

            for (_, peer) in list {

                let cipher = chacha20poly1305::encrypt(&peer.1, &message.clone().into_bytes());

                let msg = [vec![5], self.public_key.to_vec(), cipher].concat();

                outgoing_socket.send_to(&msg, &peer.0).unwrap();

            }
        }
        
    }
}