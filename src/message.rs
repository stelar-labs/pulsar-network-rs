use astro_format::arrays;
use crate::{ Message, Context };
use std::error::Error;

impl Context {

    pub fn from_bytes(byte: &[u8]) -> Result<Self, Box<dyn Error>> {
        match byte[0] {
            1_u8 => Ok(Context::Block),
            2_u8 => Ok(Context::BlockRequest),
            3_u8 => Ok(Context::CancelTransaction),
            4_u8 => Ok(Context::Transaction),
            _ => Err("Context from byte error!")?
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Context::Block => vec![1_u8],
            Context::BlockRequest => vec![2_u8],
            Context::CancelTransaction => vec![3_u8],
            Context::Transaction => vec![4_u8],
        }
    }

}

impl Message {

    pub fn new(context: Context, body: &[u8]) -> Self {

        Message {
            body: body.to_vec(),
            context: context,
        }

    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        let details = arrays::decode(bytes)?;
        
        if details.len() == 2 {

            match Context::from_bytes(details[1]) {

                Ok(c) => {
                    
                    Ok(Message {
                        body: details[0].to_vec(),
                        context: c
                    })
                },
                
                Err(e) => Err(e)?
            }            
        } else {
            Err("Message from bytes error!")?
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {

        arrays::encode(&[
            &self.body,
            &self.context.to_bytes(),
        ])
        
    }

}
