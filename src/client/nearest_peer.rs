use opis::Int;

use crate::{Client, Peer};
use std::sync::Arc;

impl Client {

    pub fn nearest_peer(&self) -> Peer {

        let tables_clone = Arc::clone(&self.peers);

        let tables = tables_clone.lock().unwrap();
        
        let mut peers = Vec::new();

        for (_, table) in tables.iter() {
            for (_, peer) in table {
                peers.push(peer);
            }
        }

        peers.sort_by_key(|k| Int::from_bytes(&self.public_key) ^ Int::from_bytes(&k.public_key));

        peers[0].clone()

    }
    
}