use crate::envelope::{ Envelope, Kind };
use crate::{Client, Message};
use fides::chacha20poly1305;
use std::sync::Arc;

impl Client {

    pub fn broadcast(&self, message: Message) {

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();
        
        let tables_clone = Arc::clone(&self.peers);

        let tables = tables_clone.lock().unwrap();

        for (_, table) in tables.clone() {

            for (_, peer) in table {

                match chacha20poly1305::encrypt(&peer.shared_key, &message.to_bytes()) {

                    Ok(cipher) => {

                        let envelope = Envelope::new(Kind::Encrypted, &cipher, &self.public_key, &self.route);
                        
                        let _r = outgoing_socket.send_to(&envelope.to_bytes(), &peer.address);

                    },
                    
                    Err(_) => ()

                }

            }

        }

    }

}
