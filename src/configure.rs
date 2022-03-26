
use crate::Network;
use crate::Route;
use fides::x25519;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;
use std::net::{SocketAddr, UdpSocket};

impl Network {

    pub fn configure(route: Route, seeders: Vec<SocketAddr>) -> Network {

        let private_key: [u8; 32] = x25519::private_key();

        let public_key: [u8; 32] = x25519::public_key(&private_key);

        let incoming_port: u16 = rand::thread_rng().gen_range(49152..65535);

        let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

        Network {
            private_key: private_key,
            public_key: public_key,
            route: route,
            peers: Arc::new(Mutex::new(HashMap::new())),
            incoming_socket: Arc::new(Mutex::new(UdpSocket::bind(format!("127.0.0.1:{}", incoming_port)).unwrap())),
            outgoing_socket: Arc::new(Mutex::new(UdpSocket::bind(format!("127.0.0.1:{}", outgoing_port)).unwrap())),
            seeders: Arc::new(Mutex::new(seeders))
        }
    }
}