
use crate::Network;
use crate::Route;
use fides::x25519;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

impl Network {

    pub fn configure(route: Route) -> Network {

        println!("pulsar: configuring ...");

        let private_key: [u8; 32] = x25519::private_key();

        let public_key: [u8; 32] = x25519::public_key(&private_key);

        Network {
            private_key: private_key,
            public_key: public_key,
            route: route,
            peers: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}