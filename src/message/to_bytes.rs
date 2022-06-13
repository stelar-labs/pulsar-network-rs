use astro_format::arrays;
use crate::Message;

impl Message {

    pub fn to_bytes(&self) -> Vec<u8> {
        arrays::encode(&[
            &self.body,
            &self.chain.to_bytes(),
            &self.nonce.to_bytes(),
            &self.time.to_bytes(),
            &self.topic.to_bytes()
        ])
    }
    
}
