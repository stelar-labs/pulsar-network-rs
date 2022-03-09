
use crate::Message;
use crate::Network;
use crate::peers::add_peer;
use crate::Route;
use fides::{chacha20poly1305, x25519};
use rand::Rng;
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Instant;

impl Network {

    pub fn listen(&self) -> Receiver<(Message, SocketAddr)> {

        println!("listening on pulsar network ...");

        let (sender, receiver): (Sender<(Message, SocketAddr)>, Receiver<(Message, SocketAddr)>) = channel();

        let private_key = self.private_key;

        let public_key = self.public_key;

        let route = self.route.clone();

        let peers_clone = Arc::clone(&self.peers);

        let shared_keys_clone = Arc::clone(&self.shared_keys);

        thread::spawn(move || {

            let port: u16 = rand::thread_rng().gen_range(49152..65535);

            let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).expect("couldn't bind to address, try again!");
        
            let mut now = Instant::now();

            let join_request = [vec![1], route.to_bytes()].concat();

            socket.send_to(&join_request, "192.168.100.8:55555").expect("couldn't not join network, try again!");

            loop {

                if now.elapsed().as_secs() > 300 {

                    let mut peers = peers_clone.lock().unwrap();

                    let copy_of_peers = peers.clone();
                    
                    *peers = HashMap::new();

                    drop(peers);

                    for (_, list) in copy_of_peers {

                        for (_, peer) in list {

                            let ping_request: Vec<u8> = [vec![3], route.to_bytes(), public_key.to_vec()].concat();
            
                            socket.send_to(&ping_request, &peer).unwrap();
            
                        }

                    }

                    now = Instant::now();

                } else {

                    let mut buf = [0; 256002];

                    let (amt, src) = socket.recv_from(&mut buf).unwrap();

                    let buf = &mut buf[..amt];

                    match buf[0] {
                        
                        1 => {

                            // join request ...

                            let peer_route = Route::from_byte(buf[1]);

                            if route == peer_route {
                            
                                let ping: Vec<u8> = [vec![3], route.to_bytes(), public_key.clone().to_vec()].concat();

                                socket.send_to(&ping, &src).unwrap();
                                
                                let peers = peers_clone.lock().unwrap();

                                let copy_of_peers = peers.clone();

                                drop(peers);

                                for (_, list) in copy_of_peers {

                                    // slow down messages
                                    
                                    let peer = list.get(&1).unwrap();

                                    let join_response: Vec<u8> = [vec![2], peer.to_string().as_bytes().to_vec()].concat();

                                    socket.send_to(&join_response, &src).unwrap();

                                }
                            }
                        },
                        
                        2 => {

                            // join response ...

                            let address = str::from_utf8(&buf[1..]).unwrap();
                            
                            let response = [vec![3], route.to_bytes(), public_key.to_vec()].concat();

                            socket.send_to(&response, address).unwrap();

                        },
                        
                        3 => {

                            // ping request ...

                            let response = [vec![4], route.to_bytes(), public_key.to_vec()].concat();

                            socket.send_to(&response, &src).unwrap();

                        },
                         
                        4 => {

                            // ping response ...

                            let peer_route = Route::from_byte(buf[1]);
                            
                            if route == peer_route {
                                
                                let peer_key: [u8; 32] = buf[2..34].try_into().unwrap();

                                let mut peers = peers_clone.lock().unwrap();
                                
                                add_peer(&mut peers, public_key, src, peer_key);

                                let shared_key = x25519::shared_key(&private_key, &peer_key);

                                let mut shared_keys = shared_keys_clone.lock().unwrap();

                                shared_keys.insert(src, shared_key);
                            
                            }
                        },
                        
                        5 => {

                            // standard message ...

                            let shared_keys = shared_keys_clone.lock().unwrap();

                            match shared_keys.get(&src) {
                                Some(key) => {
                                    
                                    let plain = chacha20poly1305::decrypt(&key, &buf[1..].to_vec());
                                    
                                    match Message::from_bytes(&plain.to_vec()) {
                                        Ok(msg) => sender.send((msg, src)).unwrap(),
                                        Err(_) => ()
                                    }
                                },
                                None => ()
                            }
                        },
                        _ => panic!(" {} is not a supported message type!", buf[0])
                    }
                }
            }
        });

        receiver

    }
}