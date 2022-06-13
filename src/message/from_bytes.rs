use crate::{Chain, Message, Topic};
use std::error::Error;
use astro_format::arrays;
use opis::Int;

impl Message {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        let details = arrays::decode(bytes)?;
        
        if details.len() == 5 {
            
            Ok(Message {
                body: details[0].to_vec(),
                chain: Chain::from_bytes(details[1])?,
                nonce: Int::from_bytes(details[2]),
                time: Int::from_bytes(details[3]),
                topic: Topic::from_bytes(details[4])?
            })           
        
        } else {
            Err("Internal error!")?
        }

    }
    
}
