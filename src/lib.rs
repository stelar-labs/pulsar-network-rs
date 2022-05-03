mod client;
mod envelope;
mod message;
mod peer;
mod route;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::net::UdpSocket;

#[derive(Clone, Debug)]
pub struct Client {
    bootstrap: bool,
    private_key: [u8;32],
    public_key: [u8;32],
    peers: Arc<Mutex<HashMap<String, HashMap<u8, Peer>>>>,
    route: Route,
    seeders: Arc<Mutex<Vec<SocketAddr>>>,
    incoming_socket: Arc<Mutex<UdpSocket>>,
    outgoing_socket: Arc<Mutex<UdpSocket>>
}

#[derive(Clone, Debug)]
pub enum Context { Block, BlockRequest, CancelTransaction, Transaction }

#[derive(Clone, Debug)]
pub struct Message { pub body: Vec<u8>, pub context: Context }

#[derive(Clone, Debug)]
pub enum Route { Main, Test }

#[derive(Clone, Debug)]
pub struct Peer {
    pub address: SocketAddr,
    pub public_key: [u8;32],
    pub shared_key: [u8;32]
}
