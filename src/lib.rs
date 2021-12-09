
use neutrondb::Store;
use stellar_notation::{ encode, decode };
use opis::Int;

use std::sync::mpsc;
use std::net::UdpSocket;
use std::error::Error;
use std::thread;

enum MessageKind {
    Block,
    Transaction,
    Join,
    Get,
    Text
}

pub struct Message {
    pub kind: MessageKind,
    pub body: Vec<u8>,
    pub signature: Vec<u8>
}

struct Route { id: Int, ip: String }

struct Bucket { distance: Int, routes: Vec<Route> }

pub struct Config { id: String, store: Store }

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
            buckets: vec![],
            store: config.store
        };

        let routes = network.store.get_all()?;

        let mut routes_vec: Vec<Route> = vec![];

        match routes {

            Some(res) => {
                
                routes_vec = res.iter()
                    .map(|x| {
                        
                        Route {
                            id: Int::from_str(&x.0, 16).unwrap(),
                            ip: x.1.to_string()
                        }

                    })
                    .collect();

            },

            None => ()

        }

        network.buckets = (1..256)
            .map(|x| {

                let distance: Int = Int::from_str(&x.to_string(), 10).unwrap();

                let mut bucket_routes = routes_vec;

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

            let mut network: Network = self;
            
            loop {

                let socket = UdpSocket::bind("127.0.0.1:7577").unwrap();
        
                let mut buf = [0; 10];
        
                let (amt, src) = socket.recv_from(&mut buf).unwrap();

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
