
use crate::Message;
use crate::Network;
use fides::chacha20poly1305;
use rand::Rng;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;

impl Network {

    pub fn send(&self, message: Message, peer: SocketAddr) {

        println!("pulsar: sending ...");

        let port: u16 = rand::thread_rng().gen_range(49152..65535);

        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).expect("couldn't bind to address, try again!");

        let shared_keys_clone = Arc::clone(&self.shared_keys);

        let shared_keys = shared_keys_clone.lock().unwrap();
        
        let shared_key = shared_keys.get(&peer).unwrap();

        let cipher = chacha20poly1305::encrypt(&shared_key, &message.into_bytes());
        
        let msg = [vec![5], cipher].concat();
        
        socket.send_to(&msg, peer).unwrap();

    }
}