use opis::Int;

use crate::Peer;
use std::collections::HashMap;

impl Peer {

    pub fn add_peer(
        self,
        peers: &mut HashMap<String, HashMap<u8, Peer>>,
        public_key: [u8; 32]
    ) {

        let node_id = Int::from_bytes(&public_key).magnitude;

        let peer_id = Int::from_bytes(&self.public_key).magnitude;
            
        let mut key_bits = Vec::new();
        
        for (i, x) in peer_id.iter().enumerate() {
            
            key_bits.push(*x);
            
            if node_id[i] != *x {

                let key = Int { magnitude: key_bits.clone(), sign: false }.to_binary();
                    
                match peers.get(&key) {
            
                    Some(r) => {

                        let list_len = r.len() as u8;

                        if list_len < 20 {

                            let mut list = r.clone();

                            list.insert(list_len, self);

                            peers.insert(key, list);

                            break

                        }

                    },

                    None => {
                        
                        peers.insert(key, HashMap::from([(1, self)]));
                        
                        break

                    }
                }
            }
        }
    }

}