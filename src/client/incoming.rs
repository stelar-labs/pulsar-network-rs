use crate::{Client, Message, Topic, Route};
use std::net::SocketAddr;
use std::str::{self, FromStr};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;
use super::add_peer::add_peer;

impl Client {

    pub fn incoming(&self, sender: Sender<(Message, SocketAddr)>) {

        let chain = self.chain.clone();

        let peers_clone = Arc::clone(&self.peers);

        let validators_clone = Arc::clone(&self.validators);

        let incoming_queue_clone = Arc::clone(&self.incoming_queue);

        let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);

        let route = self.route.clone();

        thread::spawn(move || {

            loop {

                match incoming_queue_clone.lock() {

                    Ok(incoming_queue) => {

                        if !incoming_queue.is_empty() {

                            let incoming_bytes = &incoming_queue[0].0;

                            let incoming_source = incoming_queue[0].1;

                            match Message::from_bytes(&incoming_bytes) {

                                Ok(message) => {
        
                                    match message.topic {
    
                                        Topic::Block => { let _ = sender.send((message, incoming_source)); },
    
                                        Topic::BlockRequest => { let _ = sender.send((message, incoming_source)); },
    
                                        Topic::CancelTransaction => { let _ = sender.send((message, incoming_source)); },
    
                                        Topic::JoinRequest => {
    
                                            match Route::from_bytes(&message.body) {
    
                                                Ok(route) => {
    
                                                    let mut peers = peers_clone.lock().unwrap();
    
                                                    let mut validators = validators_clone.lock().unwrap();
    
                                                    match route {
                                                        Route::Peer => add_peer(&mut peers, incoming_source),
                                                        Route::Validation => add_peer(&mut validators, incoming_source)
                                                    };

                                                    let mut outgoing_messages = Vec::new();
    
                                                    let ping_response = Message::new(&route.to_bytes(), &chain, &Topic::PingResponse);

                                                    outgoing_messages.push((ping_response.to_bytes(), incoming_source));

                                                    for r in [peers, validators] {

                                                        for (_, bucket) in r.iter() {

                                                            match bucket.get(&1) {

                                                                Some(a) => {
                                                                    let join_response = Message::new(a.to_string().as_bytes(), &chain, &Topic::JoinResponse);
                                                                    outgoing_messages.push((join_response.to_bytes(), incoming_source));
                                                                },

                                                                None => ()
                                                            
                                                            }
                                                        
                                                        }
                                                    
                                                    }
    
                                                    match outgoing_queue_clone.lock() {

                                                        Ok(mut outgoing_queue) => {

                                                            for outgoing_message in outgoing_messages {

                                                                outgoing_queue.push(outgoing_message);

                                                            }
                                                        },
                                                        Err(_) => todo!(),
                                                    }

                                                },
    
                                                Err(_) => ()
    
                                            }
    
                                        },
    
                                        Topic::JoinResponse => {
    
                                            match str::from_utf8(&message.body) {
                                                
                                                Ok(s) => {

                                                    let ping_request = Message::new(
                                                        &route.to_bytes(),
                                                        &chain,
                                                        &Topic::PingRequest
                                                    );

                                                    let ping_request = ping_request.to_bytes();

                                                    match SocketAddr::from_str(s) {

                                                        Ok(join_response_address) => {

                                                            match outgoing_queue_clone.lock() {

                                                                Ok(mut outgoing_queue) => outgoing_queue.push((ping_request, join_response_address)),
        
                                                                Err(_) => ()
        
                                                            }

                                                        },

                                                        Err(_) => (),
                                                    
                                                    }

                                                },

                                                Err(_) => ()

                                            }
    
                                        },
    
                                        Topic::PingRequest => {
    
                                            match Route::from_bytes(&message.body) {
    
                                                Ok(route) => {
    
                                                    let mut peers = peers_clone.lock().unwrap();
    
                                                    let mut validators = validators_clone.lock().unwrap();
    
                                                    match route {
                                                        Route::Peer => add_peer(&mut peers, incoming_source),
                                                        Route::Validation => add_peer(&mut validators, incoming_source)
                                                    };
                                            
                                                    let ping_response = Message::new(&route.to_bytes(), &chain, &Topic::PingResponse);

                                                    let ping_response_bytes = ping_response.to_bytes();

                                                    match outgoing_queue_clone.lock() {

                                                        Ok(mut outgoing_queue) => outgoing_queue.push((ping_response_bytes, incoming_source)),

                                                        Err(_) => ()

                                                    }
    
                                                },
                                                Err(_) => ()
                                            }
                                        },
    
                                        Topic::PingResponse => {
                                            
                                            match Route::from_bytes(&message.body) {
    
                                                Ok(route) => {
    
                                                    let mut peers = peers_clone.lock().unwrap();
    
                                                    let mut validators = validators_clone.lock().unwrap();
    
                                                    match route {
                                                        Route::Peer => { add_peer(&mut peers, incoming_source) },
                                                        Route::Validation => { add_peer(&mut validators, incoming_source) }
                                                    };
                                                },
                                                Err(_) => ()
                                            }
                                        },
    
                                        Topic::Transaction => { let _ = sender.send((message, incoming_source)); }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    Err(_) => break
                }
            }
        });
    }
}
