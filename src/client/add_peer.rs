use fides::hash;
use opis::Int;
use std::{collections::HashMap, net::SocketAddr};

pub fn add_peer(route: &mut HashMap<String, HashMap<u8, SocketAddr>>, source: SocketAddr) {

    let peer_id = hash(source.to_string().as_bytes());

    let mut peer_bits = Int::from_bytes(&peer_id).magnitude;

    if peer_bits.len() < 256 {

        while peer_bits.len() != 256 {
            peer_bits = [vec![peer_bits[0]], peer_bits].concat()
        }

    } else {
        peer_bits = peer_bits[peer_bits.len() - 256 .. peer_bits.len()].to_vec()
    }
        
    let mut key_bits = Vec::new();
    
    for (i, x) in peer_bits.iter().enumerate() {
        
        key_bits.push(*x);
        
        if peer_bits[i] != *x {

            let key = Int { magnitude: key_bits.clone(), sign: false }.to_binary();
                
            match route.get(&key) {
        
                Some(bucket) => {

                    let bucket_length = bucket.len() as u8;

                    if bucket_length < 32 {

                        let mut bucket = bucket.clone();

                        bucket.insert(bucket_length, source);

                        route.insert(key, bucket);

                        break

                    }

                },

                None => {
                    
                    route.insert(key, HashMap::from([(1, source)]));
                    
                    break

                }
            }
        }
    }
}
