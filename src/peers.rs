use fides::x25519;
use std::collections::HashMap;
use std::net::SocketAddr;

pub fn add_peer(
    peers: &mut HashMap<String, HashMap<u8, (SocketAddr, [u8; 32])>>,
    private_key: [u8; 32],
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

    let shared_key = x25519::shared_key(&private_key, &peer_key);
    
    for (i, x) in peer_id_bits.iter().enumerate() {
        
        current_prefix.push(*x);
        
        if node_id_bits[i] != *x {
                
            match peers.get(&current_prefix) {
        
                Some(r) => {

                    let list_len = r.len() as u8;

                    if list_len < 20 {

                        let mut list = r.clone();

                        list.insert(list_len, (peer_address, shared_key));

                        peers.insert(current_prefix, list);

                        break

                    }

                },

                None => {
                    peers.insert(current_prefix, HashMap::from([(1, (peer_address, shared_key))]));
                    break
                }
            }
        }
    }
}
