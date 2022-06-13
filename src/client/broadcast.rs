use crate::{Client, Message, Topic, Route};
use std::sync::Arc;

impl Client {

    pub fn broadcast(&self, body: &[u8], route: &Route, topic: &Topic) {

        let message = Message::new(body, &self.chain, topic);

        let message_bytes = message.to_bytes();

        let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);
        
        let peers_clone = Arc::clone(&self.peers);
        
        let validators_clone = Arc::clone(&self.validators);

        match outgoing_queue_clone.lock() {

            Ok(mut outgoing_queue) => {

                let peers = match route {
                    Route::Peer => peers_clone.lock(),
                    Route::Validation => validators_clone.lock()    
                };

                match peers {
                    Ok(p) => {
                        for (_, bucket) in p.iter() {
                            for (_, address) in bucket {
                                outgoing_queue.push((message_bytes.clone(), *address))
                            }
                        }
                    },
                    Err(_) => ()
                };
            },
            Err(_) => ()
        };
    }
}
