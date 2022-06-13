use crate::Message;
use fides::{merkle_root, hash};

impl Message {

    pub fn hash(&self) -> [u8; 32] {
        merkle_root(vec![
            hash(&self.body),
            hash(&self.chain.to_bytes()),
            hash(&self.nonce.to_bytes()),
            hash(&self.time.to_bytes()),
            hash(&self.topic.to_bytes())
        ])
    }
    
}
