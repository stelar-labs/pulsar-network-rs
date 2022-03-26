
use crate::Message;
use crate::Network;
use crate::Peer;
use crate::peers::add_peer;
use crate::Route;
use fides::{chacha20poly1305, x25519};
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::UdpSocket;
use std::str;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Instant;

impl Network {

    pub fn bootstrap(&self) -> Receiver<(Message, Peer)> {

        let (sender, receiver): (Sender<(Message, Peer)>, Receiver<(Message, Peer)>) = channel();

        let private_key = self.private_key;

        let public_key = self.public_key;

        let route = self.route.clone();

        let peers_clone = Arc::clone(&self.peers);

        thread::spawn(move || {

            let socket = UdpSocket::bind("192.168.100.5:55555").unwrap();
        
            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 300 {

                    let mut peers = peers_clone.lock().unwrap();

                    let copy_of_peers = peers.clone();
                    
                    *peers = HashMap::new();

                    drop(peers);

                    for (_, list) in copy_of_peers {

                        for (_, peer) in list {

                            let ping_request: Vec<u8> = [vec![3], route.to_bytes(), public_key.to_vec()].concat();
            
                            socket.send_to(&ping_request, peer.0).unwrap();
            
                        }

                    }

                    now = Instant::now();

                } else {

                    let mut buf = [0; 1000000];

                    let (amt, src) = socket.recv_from(&mut buf).unwrap();

                    let buf = &mut buf[..amt];

                    match buf[0] {
                        
                        1 => {

                            let peer_route = Route::from_byte(buf[1]);

                            if route == peer_route {
                            
                                let ping_request: Vec<u8> = [vec![3], route.to_bytes(), public_key.clone().to_vec()].concat();

                                socket.send_to(&ping_request, &src).unwrap();
                                
                                let peers = peers_clone.lock().unwrap();

                                let copy_of_peers = peers.clone();

                                drop(peers);

                                for (_, list) in copy_of_peers {

                                    // slow down messages
                                    
                                    let peer = list.get(&1).unwrap();

                                    let join_response: Vec<u8> = [vec![2], peer.0.to_string().as_bytes().to_vec()].concat();

                                    socket.send_to(&join_response, &src).unwrap();

                                }
                            }
                        },
                        
                        2 => {

                            let address = str::from_utf8(&buf[1..]).unwrap();
                            
                            let response = [vec![3], route.to_bytes(), public_key.to_vec()].concat();

                            socket.send_to(&response, address).unwrap();

                        },
                        
                        3 => {

                            let peer_route = Route::from_byte(buf[1]);
                            
                            if route == peer_route {
                                
                                let peer_key: [u8; 32] = buf[2..34].try_into().unwrap();

                                let mut peers = peers_clone.lock().unwrap();
                                
                                add_peer(&mut peers, private_key, public_key, src, peer_key);

                                let ping_response = [vec![4], route.to_bytes(), public_key.to_vec()].concat();

                                socket.send_to(&ping_response, &src).unwrap();

                            }

                        },
                         
                        4 => {

                            let peer_route = Route::from_byte(buf[1]);
                            
                            if route == peer_route {
                                
                                let peer_key: [u8; 32] = buf[2..34].try_into().unwrap();

                                let mut peers = peers_clone.lock().unwrap();
                                
                                add_peer(&mut peers, private_key, public_key, src, peer_key);
                            
                            }
                        },
                        
                        5 => {

                            let peer_key: [u8; 32] = buf[1..33].try_into().unwrap();

                            let shared_key = x25519::shared_key(&private_key, &peer_key);
                                    
                            let plain = chacha20poly1305::decrypt(&shared_key, &buf[33..].to_vec());
                            
                            let message = Message::from_bytes(&plain).unwrap();
                                
                            let peer: Peer = Peer { address: src, shared_key: shared_key };
                                
                            sender.send((message, peer)).unwrap()

                        },
                        _ => ()
                    }
                }
            }
        });

        receiver

    }
}