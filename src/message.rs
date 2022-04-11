use astro_format::arrays;
use crate::{ Kind, Message };
use std::error::Error;

impl Kind {

    pub fn from_bytes(byte: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        match byte[0] {
            1_u8 => Ok(Kind::GetBlock),
            2_u8 => Ok(Kind::PostBlock),
            3_u8 => Ok(Kind::PostTransaction),
            4_u8 => Ok(Kind::CancelTransaction),
            _ => Err("Kind from byte error!")?
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Kind::GetBlock => vec![1_u8],
            Kind::PostBlock => vec![2_u8],
            Kind::PostTransaction => vec![3_u8],
            Kind::CancelTransaction => vec![4_u8],
        }
    }

}

impl Message {

    pub fn new(kind: &Kind, body: &Vec<u8>) -> Self {

        Message {
            body: body.clone(),
            kind: kind.clone(),
        }

    }

    pub fn from_bytes(buffer: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        let decoded: Vec<Vec<u8>> = arrays::decode(buffer);
        
        if decoded.len() == 2 {

            match Kind::from_bytes(&decoded[1]) {

                Ok(k) => {
                    
                    Ok(Message {
                        body: decoded[0].clone(),
                        kind: k
                    })
                },
                
                Err(e) => Err(e)?
            }            
        } else {
            Err("Message inputs must be 2!")?
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {

        arrays::encode(&vec![
            self.body.clone(),
            self.kind.to_bytes(),
        ])

    }
}
