mod broadcast;
mod configure;
mod envelope;
mod listen;
mod message;
mod peers;
mod route;
mod send;
use fides::hash;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::net::UdpSocket;

#[derive(Clone, Debug)]
pub struct Connection {
    bootstrap: bool,
    private_key: [u8;32],
    public_key: [u8;32],
    route: Route,
    peers: Arc<Mutex<HashMap<String, HashMap<u8, (SocketAddr, [u8; 32])>>>>,
    incoming_socket: Arc<Mutex<UdpSocket>>,
    outgoing_socket: Arc<Mutex<UdpSocket>>,
    seeders: Arc<Mutex<Vec<SocketAddr>>>
}

#[derive(Clone, Debug)]
pub enum Kind {
    GetBlock,
    PostBlock,
    PostTransaction,
    CancelTransaction
}

#[derive(Clone, Debug)]
pub struct Message {
    body: Vec<u8>,
    kind: Kind,
}

#[derive(Clone, Debug)]
pub enum Route {
    MainNova,
    TestNova
}

#[derive(Clone, Debug)]
pub struct Peer {
    pub address: SocketAddr,
    shared_key: [u8; 32]
}

fn merkle_tree_hash(mut hashes: Vec<[u8;32]>) -> [u8; 32] {

    if hashes.is_empty() {
        
        [0_u8; 32]
    
    } else {

        if hashes.len() % 2 != 0 { hashes.push([0_u8; 32]) };

        while hashes.len() > 1 {

            let mut cache: Vec<[u8; 32]> = Vec::new();

            let mut intermediate: Vec<[u8; 32]> = Vec::new();

            for h in &hashes {
                
                intermediate.push(*h);
                
                if intermediate.len() == 2 {
                    
                    cache.push(hash(&[
                        intermediate[0].to_vec(),
                        intermediate[1].to_vec()
                    ].concat()));

                    intermediate.clear()

                }

            }

            hashes = cache
        
        };

        hashes[0]

    }

}
