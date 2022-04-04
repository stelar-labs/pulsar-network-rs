use crate::envelope::{ Context, Envelope };
use crate::Message;
use crate::Connection;
use crate::Peer;
use crate::peers::add_peer;
use crate::Route;
use fides::{chacha20poly1305, x25519};
use std::collections::HashMap;
use std::str;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Instant;

impl Connection {

    pub fn listen(&self) -> Receiver<(Message, Peer)> {

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

            let join_request = Envelope::from(Context::JoinRequest, route.to_bytes(), public_key);
            
            if !bootstrap {

                for seeder in &seeders {
                    
                    let _res = incoming_socket.send_to(&join_request.to_astro().into_bytes(), seeder);

                }
            
            }

            loop {

                if now.elapsed().as_secs() > 300 {

                    let mut peers = peers_clone.lock().unwrap();

                    let copy_of_peers = peers.clone();
                    
                    *peers = HashMap::new();

                    drop(peers);

                    for (_, list) in &copy_of_peers {

                        for (_, peer) in list {

                            let ping_request = Envelope::from(Context::PingRequest, route.to_bytes(), public_key);
            
                            incoming_socket.send_to(&ping_request.to_astro().into_bytes(), peer.0).unwrap();
            
                        }

                    }

                    if !bootstrap && copy_of_peers.len() == 1 {

                        for seeder in &seeders {
                
                            let _res = incoming_socket.send_to(&join_request.to_astro().into_bytes(), seeder);
            
                        }
                    }

                    now = Instant::now();

                } else {

                    let mut buf = [0; 1000000];

                    let (amt, src) = incoming_socket.recv_from(&mut buf).unwrap();

                    let buf = &mut buf[..amt];

                    match str::from_utf8(buf) {
                        
                        Ok(s) => {
                            
                            match Envelope::from_astro(s) {

                                Ok(e) => {

                                    match e.context {

                                        Context::JoinRequest => {
                                            
                                            match Route::from_bytes(&e.message) {

                                                Ok(r) => {

                                                    if route == r {

                                                        let ping_request = Envelope::from(Context::PingRequest, e.message, public_key);

                                                        incoming_socket.send_to(&ping_request.to_astro().into_bytes(), &src).unwrap();
                                                        
                                                        let peers = peers_clone.lock().unwrap();

                                                        let copy_of_peers = peers.clone();

                                                        drop(peers);

                                                        for (_, list) in copy_of_peers {

                                                            // slow down messages
                                                            
                                                            let peer = list.get(&1).unwrap();

                                                            let join_response = Envelope::from(Context::JoinResponse, peer.0.to_string().into_bytes(), public_key);

                                                            incoming_socket.send_to(&join_response.to_astro().into_bytes(), &src).unwrap();

                                                        }
                                                    }
                                                },
                                                Err(_) => ()
                                            }
                                        },
                        
                                        Context::JoinResponse => {

                                            match str::from_utf8(&e.message) {
                        
                                                Ok(s) => {

                                                    let ping_request = Envelope::from(Context::PingResponse, route.to_bytes(), public_key);
                                            
                                                    incoming_socket.send_to(&ping_request.to_astro().into_bytes(), s).unwrap();

                                                },
                                                Err(_) => ()
                                            }
                                        },
                                        
                                        Context::PingRequest => {

                                            match Route::from_bytes(&e.message) {

                                                Ok(r) => {
                                                    
                                                    if route == r {

                                                        let mut peers = peers_clone.lock().unwrap();
                                                        
                                                        add_peer(&mut peers, private_key, public_key, src, e.sender);
                                                        
                                                        let ping_response = Envelope::from(Context::PingResponse, route.to_bytes(), public_key);

                                                        incoming_socket.send_to(&ping_response.to_astro().into_bytes(), &src).unwrap();

                                                    }

                                                },
                                                Err(_) => ()
                                            }
                                        },
                                        
                                        Context::PingResponse => {

                                            match Route::from_bytes(&e.message) {

                                                Ok(r) => {
                                                    
                                                    if route == r {
                                                        
                                                        let mut peers = peers_clone.lock().unwrap();
                                                
                                                        add_peer(&mut peers, private_key, public_key, src, e.sender);

                                                    }

                                                },
                                                Err(_) => ()
                                            }
                                        },
                                        
                                        Context::Encrypted => {

                                            let shared_key = x25519::shared_key(&private_key, &e.sender);
                                                    
                                            let plain = chacha20poly1305::decrypt(&shared_key, &e.message);

                                            match str::from_utf8(&plain) {

                                                Ok(message_str) => {

                                                    match Message::from_astro(message_str) {

                                                        Ok(message) => {
                                                            
                                                            let peer: Peer = Peer { address: src, shared_key: shared_key };
                                                            
                                                            sender.send((message, peer)).unwrap()

                                                        },
                                                        Err(_) => ()
                                                    }
                                                },
                                                Err(_) => ()
                                            }
                                        }
                                    }
                                },
                                _ => ()
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