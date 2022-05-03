use std::{error::Error, time::SystemTime};

use astro_format::arrays;
use opis::Int;

use crate::Route;

use super::{Envelope, Kind};

impl Envelope {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        let details = arrays::decode(bytes)?;
        
        if details.len() == 6 {

            match Kind::from_bytes(details[0]) {

                Ok(k) => {
                        
                    let envelope = Envelope {
                        kind: k,
                        message: details[1].to_vec(),
                        nonce: Int::from_bytes(details[2]),
                        route: Route::from_bytes(details[3])?,
                        sender: details[4].try_into()?,
                        time: Int::from_bytes(details[5])
                    };
                    
                    let envelope_hash = envelope.hash();
            
                    if envelope_hash[0] == 0 {
                        
                        let current_time = Int::from_bytes(
                            &SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                                .to_be_bytes()
                        );

                        if current_time >= envelope.time && current_time - envelope.clone().time < Int::from_decimal("86400") {
                            
                            Ok(envelope)
                        
                        } else {
                            
                            Err("Message too old!")?
                        
                        }

                    } else {
                        
                        Err("Message too easy!")?
                    
                    }
                
                },
                
                Err(e) => Err(e)?
            
            }            
        
        } else {

            Err("Parameter error!")?
        
        }

    }

}
