use crate::{Client, Message, Topic};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

impl Client {

    pub fn listen(&self) {

        let bootstrap = self.bootstrap;

        let chain = self.chain.clone();

        let peers_clone = Arc::clone(&self.peers);

        let validators_clone = Arc::clone(&self.validators);

        let seeders_clone = Arc::clone(&self.seeders);

        let incoming_queue_clone = Arc::clone(&self.incoming_queue);

        let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);

        let route = self.route.clone();

        let socket_clone = Arc::clone(&self.socket);

        let ping_request = Message::new(&route.to_bytes(), &chain, &Topic::JoinRequest);
        
        let ping_request_bytes = ping_request.to_bytes();

        thread::spawn(move || {

            match socket_clone.lock() {

                Ok(socket) => {

                    let mut now = Instant::now();
            
                    let seeders = seeders_clone.lock().unwrap().clone();

                    let join_request = Message::new(&route.to_bytes(), &chain, &Topic::JoinRequest);

                    let join_request_bytes = join_request.to_bytes();
                    
                    if !bootstrap {
                        for seeder in &seeders {
                            let _ = socket.send_to(&join_request_bytes, seeder);
                        }
                    }

                    loop {

                        if now.elapsed().as_secs() > 300 {

                            let peers = peers_clone.lock().unwrap();

                            let validators = validators_clone.lock().unwrap();

                            for mut rt in [peers, validators] {
                                
                                for (_, bucket) in rt.iter() {
                                    for (_, address) in bucket.iter() {
                                        let _ = socket.send_to(&ping_request_bytes, address);
                                    }
                                }

                                rt.clear();
                        
                                drop(rt);

                            };

                            now = Instant::now();

                        } else {

                            let mut buffer = [0; 32000];

                            match socket.recv_from(&mut buffer) {

                                Ok((data_length, source)) => {

                                    let buffer = &mut buffer[..data_length];

                                    match incoming_queue_clone.lock() {

                                        Ok(mut incoming_queue) => incoming_queue.push((buffer.to_vec(), source)),

                                        Err(_) => ()
                                    
                                    }

                                },
                                Err(_) => (),
                            }

                            let mut outgoing_queue = outgoing_queue_clone.lock().unwrap();
                            
                            if !outgoing_queue.is_empty() {

                                let (outgoing_message, outgoing_address) = outgoing_queue[0].clone();

                                outgoing_queue.remove(0);

                                let _ = socket.send_to(&outgoing_message, outgoing_address);
                                
                            }
                        }
                    }
                

                },
                Err(_) => ()
            }
        });
    }
}
