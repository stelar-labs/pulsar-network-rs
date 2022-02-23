use astro_notation::list;
use fides::{asymmetric, hash, symmetric};
use rand::Rng;
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::UdpSocket;
use std::str;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Instant;
use std::thread;

#[derive(Clone, Debug)]
pub enum MessageKind {
    CancelTransaction,
    LatestBlock,
    NewBlock,
    NewTransaction,
    StorageRequest,
    StorageResponse
}

impl MessageKind {
    
    pub fn from_byte(byte: &u8) -> Self {
        match byte {
            1 => MessageKind::CancelTransaction,
            2 => MessageKind::LatestBlock,
            3 => MessageKind::NewBlock,
            4 => MessageKind::NewTransaction,
            5 => MessageKind::StorageRequest,
            6 => MessageKind::StorageResponse,
            _ => panic!("{} is not a supported MessageKind!", byte)
        }
    }

    pub fn into_byte(&self) -> u8 {
        match self {
            MessageKind::CancelTransaction => 1_u8,
            MessageKind::LatestBlock => 2_u8,
            MessageKind::NewBlock => 3_u8,
            MessageKind::NewTransaction => 4_u8,
            MessageKind::StorageRequest => 5_u8,
            MessageKind::StorageResponse => 6_u8,
        }
    }

}

#[derive(Clone, Debug)]
pub struct Message {
    body: String,
    kind: MessageKind,
    nonce: String,
    pub sender: Peer
}

impl Message {

    pub fn new(body: &str, kind: MessageKind) -> Self {
        Message {
            body: body.to_string(),
            kind: kind,
            nonce: "0x00".to_string(),
            sender: Peer::default()
        }
    }

    pub fn from_bytes(input: &Vec<u8>) -> Self {
        
        let astro_list = list::as_bytes(str::from_utf8(&input).unwrap());

        Message {
            body: str::from_utf8(&astro_list[0]).unwrap().to_string(),
            kind: MessageKind::from_byte(&astro_list[1][0]),
            nonce: str::from_utf8(&astro_list[2]).unwrap().to_string(),
            sender: Peer::default()
        }

    }

    pub fn into_bytes(self) -> Vec<u8> {
        
        let bytes: Vec<Vec<u8>> = vec![
            self.body.into_bytes(),
            vec![self.kind.into_byte()],
            self.nonce.into_bytes()
        ];

        let astro_str: String = list::from_bytes(bytes);

        astro_str.into_bytes()

    }

}

#[derive(Debug)]
pub enum Routes {
    Validation
}

#[derive(Clone, Debug)]
pub struct Peer {
    address: String,
    public_key: [u8;32]
}

impl Peer {

    pub fn default() -> Self {
        Peer {
            address: String::new(),
            public_key: [0_u8;32]
        }
    }
}

#[derive(Clone, Debug)]
struct Route { buckets: HashMap<String, HashMap<u8, Peer>> }

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

                let shared_key = asymmetric::shared_key(&private_key, &peer.public_key);

                let cipher = symmetric::encrypt(&shared_key, &message.clone().into_bytes());

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
    pub validation: bool,
    validation_route: Arc<Mutex<Route>>,
    port: u16
}

impl Network {

    pub fn config() -> Network {

        let priv_key = asymmetric::private_key();

        let pub_key = asymmetric::public_key(&priv_key);

        let port: u16 = rand::thread_rng().gen_range(49152..65535);

        Network {
            private_key: priv_key,
            public_key: pub_key,
            validation: false,
            validation_route: Arc::new(Mutex::new(Route{ buckets: HashMap::new() })),
            port: port
        }

    }

    pub fn messages(self) -> Receiver<Message> {

        let priv_key = self.private_key;
            
        let pub_key = self.public_key;
        
        let port = self.port;

        let validation = self.validation;

        let validation_route_clone = Arc::clone(&self.validation_route);

        let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        thread::spawn(move || {

            let validation_route_clone = Arc::clone(&self.validation_route);

            let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();

            loop {
            
                let mut raw = [0; 256002];

                let (amt, src) = socket.recv_from(&mut raw).unwrap();

                let raw = &mut raw[..amt];

                let mut validation_route = validation_route_clone.lock().unwrap();

                match raw[0] {
                    // Ping Request 
                    1 => {

                        let mut response = [vec![2], pub_key.to_vec()].concat();

                        if validation {
                            response = [response, vec![1]].concat();
                        } else {
                            response = [response, vec![0]].concat();
                        }

                        socket.send_to(&response, &src).unwrap();

                    },
                    // Ping Response 
                    2 => {
                        
                        if raw[33] == 1 {
                            
                            let peer_key: [u8; 32] = raw[1..33].try_into().unwrap();
                        
                            let peer: Peer = Peer { address: src.to_string(), public_key: peer_key };
                            
                            *validation_route = validation_route.clone().add_peer(pub_key, peer);
                        
                        }

                    },
                    // Standard
                    3 => {
                        
                        let peer_key: [u8; 32] = raw[1..33].try_into().unwrap();

                        let peer: Peer = Peer { address: src.to_string(), public_key: peer_key };
                        
                        let shared_key = asymmetric::shared_key(&priv_key, &peer_key);
     
                        let plain = symmetric::decrypt(&shared_key, &raw[33..].to_vec());
                        
                        let _plain_hash = hash(&plain);
                        
                        let mut msg = Message::from_bytes(&plain.to_vec());

                        msg.sender = peer;
                        
                        sender.send(msg).unwrap()

                    },
                    _ => panic!(" {} is not a supported message type!", raw[0])
                }

            }

        });

        thread::spawn(move || {

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 300 {

                    now = Instant::now();

                    let mut validation_route = validation_route_clone.lock().unwrap();

                    let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();

                    for (_, list) in validation_route.clone().buckets {

                        for (_, peer) in list {
            
                            let msg = [vec![1], pub_key.to_vec()].concat();
            
                            socket.send_to(&msg, &peer.address).unwrap();
            
                        }

                    }

                    *validation_route = Route { buckets: HashMap::new() }


                }
            }

        });

        receiver

    }

    pub fn broadcast(self, message: Message, route: Routes) {

        match route {
            Routes::Validation => { 

                let validation_route_clone = Arc::clone(&self.validation_route);

                let validation_route = validation_route_clone.lock().unwrap().clone();

                validation_route.broadcast(message, self.port, self.public_key, self.private_key)
                
            }
        }
        
    }

    pub fn send(self, message: Message, peer: Peer) {
        
        let priv_key = self.private_key;
        
        let shared_key = asymmetric::shared_key(&priv_key, &peer.public_key);
        
        let cipher = symmetric::encrypt(&shared_key, &message.into_bytes());
        
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        
        let msg = [vec![3], self.public_key.to_vec(), cipher].concat();
        
        socket.send_to(&msg, &peer.address).unwrap();

    }
    
}
