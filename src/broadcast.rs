
use crate::Message;
use crate::Network;
use fides::chacha20poly1305;
use rand::Rng;
use std::net::UdpSocket;
use std::sync::Arc;

impl Network {

    pub fn broadcast(&self, message: Message) {

        println!("pulsar: broadcasting ...");

        let port: u16 = rand::thread_rng().gen_range(49152..65535);

        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).expect("couldn't bind to address, try again!");
        
        let peers_clone = Arc::clone(&self.peers);

        let peers = peers_clone.lock().unwrap();

        for (_, list) in peers.clone() {

            for (_, peer) in list {

                let cipher = chacha20poly1305::encrypt(&peer.1, &message.clone().into_bytes());

                let msg = [vec![5], self.public_key.to_vec(), cipher].concat();

                socket.send_to(&msg, &peer.0).unwrap();

            }
        }
        
    }
}