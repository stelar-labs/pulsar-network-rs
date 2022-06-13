use crate::Message;
use opis::Int;

impl Message {
    
    pub fn apply_difficulty(&mut self) {

        let mut message_hash = self.hash();
        
        while message_hash[0] != 0 {

            self.nonce += Int::one();

            message_hash = self.hash();

        }

    }

}