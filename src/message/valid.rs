use crate::Message;
use opis::Int;
use std::time::SystemTime;

impl Message {

    pub fn valid(&self) -> bool {
        
        let current_time = Int::from_bytes(
            &SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_be_bytes()
        );

        if current_time >= self.time && current_time - self.clone().time < Int::from_decimal("5") {
            
            let message_hash = self.hash();

            if message_hash[0] == 0 {
                true
            } else {
                false
            }
        
        } else {
            false
        }            

    }
    
}
