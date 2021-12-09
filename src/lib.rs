
use neutrondb::Store;
// use stellar_notation::{ encode, decode };
use opis::Int;

use std::sync::mpsc;
use std::net::UdpSocket;
use std::error::Error;
use std::thread;

#[derive(Debug, Clone)]
pub enum MessageKind {
    Block,
    Transaction,
    Join,
    Get,
    Text
}

#[derive(Debug, Clone)]
pub struct Message {
    pub kind: MessageKind,
    pub body: Vec<u8>,
    pub signature: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct Route { id: Int, ip: String }

#[derive(Debug, Clone)]
pub struct Bucket { distance: Int, routes: Vec<Route> }

#[derive(Debug, Clone)]
pub struct Config { id: String, store: Store }

#[derive(Debug)]
pub struct Network {
    pub id: Int,
    pub sender: mpsc::Sender<Message>,
    pub receiver: mpsc::Receiver<Message>,
    pub buckets: Vec<Bucket>,
    pub store: Store
}

impl Network {
    
    pub fn configure(config: Config) -> Result<Network, Box<dyn Error>> {

        let (sender, receiver) = mpsc::channel();

        let id = Int::from_str(&config.id, 16)?;

        let mut network = Network {
            id: id,
            sender: sender,
            receiver: receiver,
            buckets: Vec::new(),
            store: config.store
        };

        let routes = network.store.get_all()?;

        let mut routes_vec: Vec<Route> = Vec::new();

        match routes {

            Some(res) => {
                
                routes_vec = res.iter()
                    .map(|x| {

                        let id: Int = Int::from_str(&x.0, 16).unwrap();
                        let ip: String = x.1.to_string();
                        
                        Route {
                            id: id,
                            ip: ip
                        }

                    })
                    .collect();

            },

            None => ()

        }

        network.buckets = (1..256)
            .map(|x| {

                let distance: Int = Int::from_str(&x.to_string(), 10).unwrap();

                let mut bucket_routes = routes_vec.clone();

                bucket_routes.retain(|x| network.id.clone().sub(&x.id).is_equal(&distance));

                Bucket {
                    distance: distance,
                    routes: bucket_routes
                }

            })
            .collect();

        Ok(network)

    }

    pub fn connect(self) {

        thread::spawn(move || {

            let network: Network = self;
            
            loop {

                let socket = UdpSocket::bind("127.0.0.1:7577").unwrap();
        
                let mut buf = [0; 10];
        
                let (_amt, _src) = socket.recv_from(&mut buf).unwrap();

                let message = Message {
                    kind: MessageKind::Block,
                    body: vec![],
                    signature: vec![]
                };
        
                network.sender.send(message).unwrap();
        
            }
            
        });

    }

    pub fn broadcast() {}

}
