use crate::{Client, Message, Topic};
use std::{net::SocketAddr, sync::Arc};

impl Client {

    pub fn send(&self, address: &SocketAddr, body: &[u8], topic: &Topic) {
        
        let message = Message::new(body, &self.chain, topic);

        let message_bytes = message.to_bytes();

        let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);

        match outgoing_queue_clone.lock() {

            Ok(mut outgoing_queue) => {

                outgoing_queue.push((message_bytes, *address))
            },

            Err(_) => ()

        };

    }
}
