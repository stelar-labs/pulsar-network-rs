use crate::{Chain, Message, Topic};
use opis::Int;
use std::time::SystemTime;

impl Message {

    pub fn new(body: &[u8], chain: &Chain, topic: &Topic) -> Self {

        let time = Int::from_bytes(
            &SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_be_bytes()
        );

        let mut message = Message {
            body: body.to_vec(),
            chain: chain.clone(),
            nonce: Int::zero(),
            time,
            topic: topic.clone()
        };

        message.apply_difficulty();

        message

    }

}
