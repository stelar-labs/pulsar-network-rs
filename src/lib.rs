
use neutrondb::store;

use std::sync::mpsc;
use std::net::UdpSocket;
use std::error::Error;
use std::thread;

pub struct Pulsar {
    pub receiver: mpsc::Receiver<Vec<u8>>,
    pub routes: Vec<(String, String)>
}

impl Pulsar {
    
    pub fn start() -> Result<Pulsar, Box<dyn Error>> {

        let (sender, receiver) = mpsc::channel();

        let mut pulsar = Pulsar { 
            receiver: receiver,
            routes: vec![]
        };

        let net_store = store("pulsar")?;

        let routes = net_store.get_all()?;

        match routes {

            Some(res) => pulsar.routes = res,

            None => pulsar.routes = vec![("node_id".to_string(), "node_add".to_string())]

        }

        thread::spawn(move || {
            
            loop {

                let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
        
                let mut buf = [0; 10];
        
                let (amt, src) = socket.recv_from(&mut buf).unwrap();
        
                sender.send(buf.to_vec()).unwrap();
        
            }
            
        });

        Ok(pulsar)


    }

    pub fn broadcast() {}

}
