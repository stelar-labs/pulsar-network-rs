use crate::Peer;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

impl Peer {

    pub fn new() -> Self {
        
        Peer {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            public_key: [0_u8;32],
            shared_key: [0_u8;32]
        }

    }

}
