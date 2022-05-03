use crate::Route;
use opis::Int;
use std::time::SystemTime;
use super::{Envelope, Kind};

impl Envelope {

    pub fn new(kind: Kind, message: &[u8], public_key: &[u8;32], route: &Route) -> Self {

        let time = Int::from_bytes(
            &SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_be_bytes()
        );

        let mut envelope = Envelope {
            kind: kind,
            message: message.to_vec(),
            nonce: Int::zero(),
            route: route.clone(),
            sender: public_key.clone(),
            time: time
        };

        envelope.apply_difficulty();

        envelope

    }

}
