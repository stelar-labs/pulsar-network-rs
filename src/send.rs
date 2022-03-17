
use crate::Message;
use crate::Network;
use crate::Peer;
use fides::chacha20poly1305;
use rand::Rng;
use std::net::UdpSocket;

impl Network {

    pub fn send(&self, message: Message, peer: Peer) {

        println!("pulsar: sending message to {} ...", peer.address);

        let port: u16 = rand::thread_rng().gen_range(49152..65535);

        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).expect("couldn't bind to address, try again!");

        let cipher = chacha20poly1305::encrypt(&peer.shared_key, &message.into_bytes());
        
        let msg = [vec![5], self.public_key.to_vec(), cipher].concat();
        
        socket.send_to(&msg, peer.address).unwrap();

    }
}