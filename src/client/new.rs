use crate::{Client, Chain, Route};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use rand::Rng;
use std::net::{UdpSocket, SocketAddr};

impl Client {

    pub fn new(bootstrap: bool, chain: Chain, route: Route, seeders: Vec<SocketAddr>) -> Result<Client, Box<dyn Error>> {

        let port: u16 =
        
            if bootstrap {

                55555
            
            } else {
                
                rand::thread_rng().gen_range(49152..65535)
            
            };

        let address = format!("127.0.0.1:{}", port);

        let socket = UdpSocket::bind(address)?;

        Ok(Client {
            bootstrap,
            chain,
            peers: Arc::new(Mutex::new(HashMap::new())),
            validators: Arc::new(Mutex::new(HashMap::new())),
            seeders: Arc::new(Mutex::new(seeders)),
            socket: Arc::new(Mutex::new(socket)),
            route,
            incoming_queue: Arc::new(Mutex::new(Vec::new())),
            outgoing_queue: Arc::new(Mutex::new(Vec::new()))
            
        })

    }
    
}
