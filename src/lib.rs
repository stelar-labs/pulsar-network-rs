use fides::{ chacha20poly1305, x25519 };
use rand::Rng;
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Instant;
use std::thread;
use std::str;
use opis::Int;
mod message;

#[derive(Clone, Debug)]
pub enum MessageKind {
    Block,
    CancelTransaction,
    NextBlock,
    Transaction
}

#[derive(Clone, Debug)]
pub struct Message {
    pub body: Vec<u8>,
    pub kind: MessageKind,
    nonce: Int,
    time: u64
}

fn merkle_tree_hash(_hashes: Vec<[u8;32]>) -> [u8; 32] {
    [0_u8; 32]
}

#[derive(Clone, Debug)]
pub enum Routes {
    MainValidation,
    TestValidation
}

impl PartialEq for Routes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Routes::MainValidation, Routes::MainValidation) => true,
            (Routes::TestValidation, Routes::TestValidation) => true,
            _ => false
        }
    }
}

impl Eq for Routes {}

#[derive(Clone, Debug)]
pub struct Peer {
    address: String,
    public_key: [u8; 32]
}

#[derive(Clone, Debug)]
struct Route {
    buckets: HashMap<String, HashMap<u8, Peer>>
}

impl Route {
    
    pub fn add_peer(&mut self, node_id: [u8;32], peer: Peer) {
        
        let node_id_bits: Vec<char> = node_id.iter().fold(String::new(), |acc, x| format!("{}{:08b}", acc, x)).chars().collect();
        
        let peer_id_bits : Vec<char> = peer.public_key.iter().fold(String::new(), |acc, x| format!("{}{:08b}", acc, x)).chars().collect();
        
        let mut current_prefix: String = String::new();

        for (i, x) in peer_id_bits.iter().enumerate() {

            current_prefix.push(*x);

            if node_id_bits[i] != *x {
                
                match self.buckets.get(&current_prefix) {
            
                    Some(r) => {

                        let list_len = r.len() as u8;

                        if list_len < 20 {

                            let mut list = r.clone();

                            list.insert(list_len, peer.clone());

                            self.buckets.insert(current_prefix.clone(), list);

                            break

                        }

                    },

                    None => {
                        self.buckets.insert(current_prefix.clone(), HashMap::from([(1, peer.clone())]));
                        break
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Network {
    private_key: [u8;32],
    public_key: [u8;32],
    incoming_socket: Arc<Mutex<UdpSocket>>,
    outgoing_socket: Arc<Mutex<UdpSocket>>,
    routes: Routes,
    route: Arc<Mutex<Route>>
}

impl Network {

    pub fn config(routes: Routes) -> Network {

        println!("pulsar: configuring ...");

        let priv_key: [u8; 32] = x25519::private_key();

        let pub_key: [u8; 32] = x25519::public_key(&priv_key);

        let incoming_port: u16 = rand::thread_rng().gen_range(49152..65535);

        let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

        Network {
            private_key: priv_key,
            public_key: pub_key,
            incoming_socket: Arc::new(Mutex::new(UdpSocket::bind(format!("127.0.0.1:{}", incoming_port)).unwrap())),
            outgoing_socket: Arc::new(Mutex::new(UdpSocket::bind(format!("127.0.0.1:{}", outgoing_port)).unwrap())),
            routes: routes,
            route: Arc::new(Mutex::new(Route { buckets: HashMap::new() }))
        }

    }

    fn join(&self) {

        let join_request: Vec<u8> = match self.routes {
            Routes::MainValidation => [vec![1], vec![1], self.public_key.to_vec()].concat(),
            Routes::TestValidation => [vec![1], vec![2], self.public_key.to_vec()].concat()
        };

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();

        outgoing_socket.send_to(&join_request, "127.0.0.1:55555").unwrap();

    }

    pub fn listen(&self) -> Receiver<(Message, Peer)> {

        println!("pulsar: listening ...");

        let (sender, receiver): (Sender<(Message, Peer)>, Receiver<(Message, Peer)>) = mpsc::channel();

        let priv_key = self.private_key;

        let pub_key = self.public_key;

        let route_clone = Arc::clone(&self.route);

        let routes = self.routes.clone();
        
        let incoming_socket_clone = Arc::clone(&self.incoming_socket);
        
        thread::spawn(move || {

            let incoming_socket = incoming_socket_clone.lock().unwrap();

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 300 {

                    let mut route = route_clone.lock().unwrap();

                    let clone_route = route.clone();
                    
                    *route = Route { buckets: HashMap::new() };

                    drop(route);

                    for (_, list) in clone_route.buckets {

                        for (_, peer) in list {

                            let ping: Vec<u8> = match routes {
                                Routes::MainValidation => [vec![3], vec![1], pub_key.to_vec()].concat(),
                                Routes::TestValidation => [vec![3], vec![2], pub_key.to_vec()].concat()
                            };
            
                            incoming_socket.send_to(&ping, &peer.address).unwrap();
            
                        }

                    }

                    now = Instant::now();

                } else {
            
                    let mut raw = [0; 256002];

                    let (amt, src) = incoming_socket.recv_from(&mut raw).unwrap();

                    let raw = &mut raw[..amt];

                    match raw[0] {
                        
                        1 => {

                            println!("pulsar: join request ...");

                            let peer_route: Routes = match raw[1] {
                                1 => Routes::MainValidation,
                                2 => Routes::TestValidation,
                                _ => panic!("{} is not a support route!", raw[1])
                            };

                            if routes == peer_route {
                            
                                let ping: Vec<u8> = match routes {
                                    Routes::MainValidation => [vec![3], vec![1], pub_key.clone().to_vec()].concat(),
                                    Routes::TestValidation => [vec![3], vec![2], pub_key.clone().to_vec()].concat()
                                };
        
                                incoming_socket.send_to(&ping, &src).unwrap();
                                
                                let route = route_clone.lock().unwrap();

                                for (_, list) in &route.buckets {
                                    
                                    let peer = list.get(&1).unwrap();

                                    let join_response: Vec<u8> = [vec![2], peer.address.as_bytes().to_vec()].concat();

                                    incoming_socket.send_to(&join_response, &src).unwrap();

                                }
                            }
                        },
                        // join response 
                        2 => {

                            println!("pulsar: join response ...");

                            let address: &str = str::from_utf8(&raw[1..]).unwrap();
                            
                            let response: Vec<u8> = match routes {
                                Routes::MainValidation => [vec![3], vec![1], pub_key.to_vec()].concat(),
                                Routes::TestValidation => [vec![3], vec![2], pub_key.to_vec()].concat()
                            };

                            incoming_socket.send_to(&response, address).unwrap();

                        },
                        
                        // Ping Request 
                        3 => {

                            println!("pulsar: ping request ...");

                            let response: Vec<u8> = match routes {
                                Routes::MainValidation => [vec![4], vec![1], pub_key.clone().to_vec()].concat(),
                                Routes::TestValidation => [vec![4], vec![2], pub_key.clone().to_vec()].concat()
                            };

                            incoming_socket.send_to(&response, &src).unwrap();

                        },
                        
                        // Ping Response 
                        4 => {

                            println!("pulsar: ping response ...");

                            let peer_route: Routes = match raw[1] {
                                1 => Routes::MainValidation,
                                2 => Routes::TestValidation,
                                _ => panic!("{} is not a support route!", raw[1])
                            };
                            
                            if routes == peer_route {
                                
                                let peer_key: [u8; 32] = raw[2..34].try_into().unwrap();
                            
                                let peer: Peer = Peer { address: src.to_string(), public_key: peer_key };

                                let mut route = route_clone.lock().unwrap();
                                
                                route.add_peer(pub_key, peer);
                            
                            }
                        },
                        
                        // Standard
                        5 => {

                            println!("pulsar: standard message ...");
                            
                            let peer_key: [u8; 32] = raw[1..33].try_into().unwrap();

                            let peer: Peer = Peer { address: src.to_string(), public_key: peer_key };
                            
                            let shared_key = x25519::shared_key(&priv_key, &peer_key);
        
                            let plain = chacha20poly1305::decrypt(&shared_key, &raw[33..].to_vec());

                            match Message::from_bytes(&plain.to_vec()) {
                                Ok(msg) => sender.send((msg, peer)).unwrap(),
                                Err(_) => ()
                            }

                        },
                        
                        _ => panic!(" {} is not a supported message type!", raw[0])
                        
                    }

                }

            }

        });

        self.join();

        receiver

    }

    pub fn broadcast(&self, message: Message) {

        println!("pulsar: broadcasting ...");
        
        let route_clone = Arc::clone(&self.route);

        let route = route_clone.lock().unwrap();

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();

        for (_, list) in &route.buckets {

            for (_, peer) in list {

                let shared_key = x25519::shared_key(&self.private_key, &peer.public_key);

                let cipher = chacha20poly1305::encrypt(&shared_key, &message.clone().into_bytes());

                let msg = [vec![3], self.public_key.to_vec(), cipher].concat();

                outgoing_socket.send_to(&msg, &peer.address).unwrap();

            }
        }
        
    }

    pub fn send(&self, message: Message, peer: Peer) {

        println!("pulsar: sending ...");
        
        let priv_key = self.private_key;
        
        let shared_key = x25519::shared_key(&priv_key, &peer.public_key);
        
        let cipher = chacha20poly1305::encrypt(&shared_key, &message.into_bytes());
        
        let msg = [vec![5], self.public_key.to_vec(), cipher].concat();

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        let outgoing_socket = outgoing_socket_clone.lock().unwrap();
        
        outgoing_socket.send_to(&msg, &peer.address).unwrap();

    }
    
}
