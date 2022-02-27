use fides::{chacha20poly1305, hash, x25519};
use rand::Rng;
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Instant;
use std::thread;

#[derive(Clone, Debug)]
pub enum MessageKind {
    Block,
    CancelTransaction,
    NextBlock,
    Transaction
}

impl MessageKind {
    
    pub fn from_byte(byte: &u8) -> Self {
        match byte {
            1 => MessageKind::Block,
            2 => MessageKind::CancelTransaction,
            3 => MessageKind::NextBlock,
            4 => MessageKind::Transaction,
            _ => panic!("{} is not a supported message kind!", byte)
        }
    }

    pub fn into_byte(&self) -> u8 {
        match self {
            MessageKind::Block => 1_u8,
            MessageKind::CancelTransaction => 2_u8,
            MessageKind::NextBlock => 3_u8,
            MessageKind::Transaction => 4_u8
        }
    }

}

#[derive(Clone, Debug)]
pub struct Message {
    pub body: Vec<u8>,
    pub kind: MessageKind,
    nonce: [u8; 32],
}

impl Message {

    pub fn new(body: Vec<u8>, kind: MessageKind) -> Self {
        Message {
            body: body,
            kind: kind,
            nonce: [0_u8; 32]
        }
    }

    pub fn expiry(self, _days: u8) -> Self {
        self
    }

    pub fn from_bytes(input: &Vec<u8>) -> Self {
        Message {
            body: input[33..].to_vec(),
            kind: MessageKind::from_byte(&input[1]),
            nonce: input[1..33].try_into().unwrap()
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        vec![vec![self.kind.into_byte()], self.nonce.to_vec(), self.body].concat()
    }

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
    
    pub fn add_peer(mut self, node_id: [u8;32], peer: Peer) -> Self {
        
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

        self

    }

    pub fn broadcast(self, message: Message, port: u16, public_key: [u8;32], private_key: [u8;32]) {
        
        for (_, list) in self.buckets {

            for (_, peer) in list {

                let shared_key = x25519::shared_key(&private_key, &peer.public_key);

                let cipher = chacha20poly1305::encrypt(&shared_key, &message.clone().into_bytes());

                let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();

                let msg = [vec![3], public_key.to_vec(), cipher].concat();

                socket.send_to(&msg, &peer.address).unwrap();

            }
        }
    }
}

#[derive(Debug)]
pub struct Network {
    private_key: [u8;32],
    public_key: [u8;32],
    routes: Routes,
    route: Arc<Mutex<Route>>,
    port: u16
}

impl Network {

    pub fn config(routes: Routes) -> Network {

        let priv_key: [u8; 32] = x25519::private_key();

        let pub_key: [u8; 32] = x25519::public_key(&priv_key);

        let port: u16 = rand::thread_rng().gen_range(49152..65535);

        Network {
            private_key: priv_key,
            public_key: pub_key,
            routes: routes,
            route: Arc::new(Mutex::new(Route { buckets: HashMap::new() })),
            port: port
        }

    }

    fn update(&self) {

        let port: u16 = self.port;

        let public_key: [u8; 32] = self.public_key;

        let routes: Routes = self.routes.clone();

        let route_clone = Arc::clone(&self.route);

        thread::spawn(move || { 

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 300 {

                    match route_clone.lock() {
                        
                        Ok(mut r) => {

                            let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();

                            let clone_route = r.clone();

                            *r = Route { buckets: HashMap::new() };

                            drop(r);

                            for (_, list) in clone_route.buckets {

                                for (_, peer) in list {

                                    let ping: Vec<u8> = match routes {
                                        Routes::MainValidation => [vec![1], vec![1], public_key.to_vec()].concat(),
                                        Routes::TestValidation => [vec![1], vec![2], public_key.to_vec()].concat()
                                    };
                    
                                    socket.send_to(&ping, &peer.address).unwrap();
                    
                                }

                            }

                            now = Instant::now();

                        },

                        Err(_) => ()

                    }

                }

            }

        });

    }

    pub fn connect(&self) -> Receiver<(Message, Peer)> {

        let (sender, receiver): (Sender<(Message, Peer)>, Receiver<(Message, Peer)>) = mpsc::channel();

        let priv_key = self.private_key;

        let pub_key = self.public_key;

        let route_clone = Arc::clone(&self.route);

        let routes = self.routes.clone();

        let port = self.port;

        thread::spawn(move || {

            let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();

            loop {
            
                let mut raw = [0; 256002];

                let (amt, src) = socket.recv_from(&mut raw).unwrap();

                let raw = &mut raw[..amt];

                match raw[0] {
                    
                    // Ping Request 
                    1 => {

                        let response: Vec<u8> = match routes {
                            Routes::MainValidation => [vec![2], vec![1], pub_key.clone().to_vec()].concat(),
                            Routes::TestValidation => [vec![2], vec![2], pub_key.clone().to_vec()].concat()
                        };

                        socket.send_to(&response, &src).unwrap();

                    },
                    
                    // Ping Response 
                    2 => {

                        let peer_route: Routes = match raw[1] {
                            1 => Routes::MainValidation,
                            2 => Routes::TestValidation,
                            _ => panic!("{} is not a support route!", raw[1])
                        };
                        
                        if routes == peer_route {
                            
                            let peer_key: [u8; 32] = raw[2..34].try_into().unwrap();
                        
                            let peer: Peer = Peer { address: src.to_string(), public_key: peer_key };

                            let mut route = route_clone.lock().unwrap();
                            
                            *route = route.clone().add_peer(pub_key, peer);

                            drop(route);
                        
                        }

                    },
                    
                    // Standard
                    3 => {
                        
                        let peer_key: [u8; 32] = raw[1..33].try_into().unwrap();

                        let peer: Peer = Peer { address: src.to_string(), public_key: peer_key };
                        
                        let shared_key = x25519::shared_key(&priv_key, &peer_key);
     
                        let plain = chacha20poly1305::decrypt(&shared_key, &raw[33..].to_vec());
                        
                        let _plain_hash = hash(&plain);
                        
                        let msg = Message::from_bytes(&plain.to_vec());
                        
                        sender.send((msg, peer)).unwrap()

                    },
                    
                    _ => panic!(" {} is not a supported message type!", raw[0])
                    
                }

            }

        });

        self.update();

        receiver

    }

    pub fn broadcast(self, message: Message) {
        
        let route_clone = Arc::clone(&self.route);

        let validation_route = route_clone.lock().unwrap().clone();

        validation_route.broadcast(message, self.port, self.private_key, self.public_key)
        
    }

    pub fn send(self, message: Message, peer: Peer) {
        
        let priv_key = self.private_key;
        
        let shared_key = x25519::shared_key(&priv_key, &peer.public_key);
        
        let cipher = chacha20poly1305::encrypt(&shared_key, &message.into_bytes());
        
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        
        let msg = [vec![3], self.public_key.to_vec(), cipher].concat();
        
        socket.send_to(&msg, &peer.address).unwrap();

    }
    
}
