use opis::Int;

use super::Envelope;

impl Envelope {
    
    pub fn apply_difficulty(&mut self) {

        let mut envelope_hash = self.hash();
        
        while envelope_hash[0] != 0 {

            self.nonce += Int::one();

            envelope_hash = self.hash();

        }

    }

}