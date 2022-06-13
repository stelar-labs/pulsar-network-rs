mod chain;
mod client;
mod message;
mod route;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::net::UdpSocket;
use opis::Int;

#[derive(Debug)]
pub struct Client {
    bootstrap: bool,
    incoming_queue: Arc<Mutex<Vec<(Vec<u8>, SocketAddr)>>>,
    outgoing_queue: Arc<Mutex<Vec<(Vec<u8>, SocketAddr)>>>,
    socket: Arc<Mutex<UdpSocket>>,
    peers: Arc<Mutex<HashMap<String, HashMap<u8, SocketAddr>>>>,
    validators: Arc<Mutex<HashMap<String, HashMap<u8, SocketAddr>>>>,
    chain: Chain,
    route: Route,
    seeders: Arc<Mutex<Vec<SocketAddr>>>
}

#[derive(Clone, Debug)]
pub enum Topic {
    Block,
    BlockRequest,
    CancelTransaction,
    JoinRequest,
    JoinResponse,
    PingRequest,
    PingResponse,
    Transaction
}

#[derive(Clone, Debug)]
pub struct Message {
    pub body: Vec<u8>,
    pub chain: Chain,
    pub nonce: Int,
    pub time: Int,
    pub topic: Topic
}

#[derive(Clone, Debug)]
pub enum Chain {
    Main,
    Test
}

#[derive(Clone, Debug)]
pub enum Route {
    Peer,
    Validation
}
