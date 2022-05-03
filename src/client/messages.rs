use crate::envelope::{Envelope, Kind};
use crate::{Client, Message, Peer};
use fides::{chacha20poly1305, x25519};
use opis::Int;
use std::collections::HashMap;
use std::str;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Instant;

impl Client {

    pub fn messages(&self) -> Receiver<(Message, Peer)> {

        let (sender, receiver): (Sender<(Message, Peer)>, Receiver<(Message, Peer)>) = channel();

        let private_key = self.private_key;

        let public_key = self.public_key;

        let route = self.route.clone();

        let peers_clone = Arc::clone(&self.peers);

        let incoming_socket_clone = Arc::clone(&self.incoming_socket);

        let seeders_clone = Arc::clone(&self.seeders);

        let bootstrap: bool = self.bootstrap;

        thread::spawn(move || {

            let incoming_socket = incoming_socket_clone.lock().unwrap();
        
            let mut now = Instant::now();
            
            let seeders = seeders_clone.lock().unwrap().clone();

            let join_request = Envelope::new(Kind::JoinRequest, &route.to_bytes(), &public_key, &route);
            
            if !bootstrap {

                for seeder in &seeders {
                    
                    let _res = incoming_socket.send_to(&join_request.to_bytes(), seeder);

                }
            
            }

            loop {

                if now.elapsed().as_secs() > 300 {

                    let mut peers = peers_clone.lock().unwrap();

                    let copy_of_peers = peers.clone();
                    
                    *peers = HashMap::new();

                    drop(peers);

                    let ping_request = Envelope::new(Kind::PingRequest, &[], &public_key, &route);

                    for (_, list) in &copy_of_peers {

                        for (_, peer) in list {

                            incoming_socket.send_to(&ping_request.to_bytes(), peer.address).unwrap();
            
                        }

                    }

                    if !bootstrap && copy_of_peers.len() == 1 {

                        for seeder in &seeders {
                
                            let _res = incoming_socket.send_to(&join_request.to_bytes(), seeder);
            
                        }
                    }

                    now = Instant::now();

                } else {

                    let mut buf = [0; 32000];

                    let (amt, src) = incoming_socket.recv_from(&mut buf).unwrap();

                    let buf = &mut buf[..amt];
                            
                    match Envelope::from_bytes(&buf.to_vec()) {

                        Ok(e) => {

                            if e.route == route {

                                match e.kind {

                                    Kind::JoinRequest => {
                                        
                                        let ping_request = Envelope::new(Kind::PingRequest, &[], &public_key, &route);

                                        incoming_socket.send_to(&ping_request.to_bytes(), &src).unwrap();
                                                    
                                        let tables = peers_clone.lock().unwrap();

                                        for (_, table) in tables.iter() {
                                            
                                            let mut peers = Vec::new();
                                            
                                            for (_, peer) in table {
                                                peers.push(peer);
                                            }

                                            peers.sort_by_key(|k| Int::from_bytes(&public_key) ^ Int::from_bytes(&k.public_key));

                                            let join_response = Envelope::new(Kind::JoinResponse, peers[0].address.to_string().as_bytes(), &public_key, &route);

                                            incoming_socket.send_to(&join_response.to_bytes(), &src).unwrap();

                                        }

                                    },
                    
                                    Kind::JoinResponse => {

                                        match str::from_utf8(&e.message) {
                    
                                            Ok(s) => {

                                                let ping_request = Envelope::new(Kind::PingResponse, &[], &public_key, &route);
                                        
                                                incoming_socket.send_to(&ping_request.to_bytes(), s).unwrap();

                                            },

                                            Err(_) => ()

                                        }

                                    },
                                    
                                    Kind::PingRequest => {
                                        
                                        let mut peers = peers_clone.lock().unwrap();

                                        let peer = Peer {
                                            address: src,
                                            public_key: e.sender,
                                            shared_key: x25519::shared_key(&private_key, &e.sender)
                                        };
                                        
                                        peer.add_peer(&mut peers, public_key);
                                        
                                        let ping_response = Envelope::new(Kind::PingResponse, &[], &public_key, &route);
                                        
                                        incoming_socket.send_to(&ping_response.to_bytes(), &src).unwrap();
                                        
                                    },
                                    
                                    Kind::PingResponse => {

                                        let peer = Peer {
                                            address: src,
                                            public_key: e.sender,
                                            shared_key: x25519::shared_key(&private_key, &e.sender)
                                        };
                                        
                                        let mut peers = peers_clone.lock().unwrap();
                                            
                                        peer.add_peer(&mut peers, public_key);

                                    },
                                    
                                    Kind::Encrypted => {

                                        let shared_key = x25519::shared_key(&private_key, &e.sender);
                                                
                                        match chacha20poly1305::decrypt(&shared_key, &e.message) {

                                            Ok(plain) => {
                                                match Message::from_bytes(&plain) {

                                                    Ok(message) => {
                                                        
                                                        let peer = Peer {
                                                            address: src,
                                                            public_key: e.sender,
                                                            shared_key: shared_key
                                                        };
                                                        
                                                        sender.send((message, peer)).unwrap()

                                                    },
                                                    Err(_) => ()
                                                }

                                            },

                                            Err(_) => ()

                                        }

                                    }

                                }
                            
                            }

                        },

                        _ => ()

                    }

                }

            }

        });

        receiver

    }
}
