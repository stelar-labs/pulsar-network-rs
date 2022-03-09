
use std::collections::HashMap;
use std::net::SocketAddr;

pub fn add_peer(
    peers: &mut HashMap<String, HashMap<u8, SocketAddr>>,
    public_key: [u8; 32],
    peer_address: SocketAddr,
    peer_key: [u8; 32]
) {

    let node_id_bits: Vec<char> = public_key
        .iter()
        .fold(String::new(), |acc, x| format!("{}{:08b}", acc, x))
        .chars()
        .collect();
    
    let peer_id_bits: Vec<char> = peer_key
        .iter()
        .fold(String::new(), |acc, x| format!("{}{:08b}", acc, x))
        .chars()
        .collect();
        
    let mut current_prefix: String = String::new();
    
    for (i, x) in peer_id_bits.iter().enumerate() {
        
        current_prefix.push(*x);
        
        if node_id_bits[i] != *x {
                
            match peers.get(&current_prefix) {
        
                Some(r) => {

                    let list_len = r.len() as u8;

                    if list_len < 20 {

                        let mut list = r.clone();

                        list.insert(list_len, peer_address);

                        peers.insert(current_prefix.clone(), list);

                        break

                    }

                },

                None => {
                    peers.insert(current_prefix.clone(), HashMap::from([(1, peer_address)]));
                    break
                }
            }
        }
    }
}