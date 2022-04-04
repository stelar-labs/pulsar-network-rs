use astro_notation::list;
use crate::merkle_tree_hash;
use fides::hash;
use opis::Int;
use std::convert::TryInto;
use std::error::Error;
use std::str;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub enum Context {
    JoinRequest,
    JoinResponse,
    PingRequest,
    PingResponse,
    Encrypted
}

impl Context {

    pub fn from_bytes(byte: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        match byte {
            vec![1_u8] => Ok(Context::JoinRequest),
            vec![2_u8] => Ok(Context::JoinResponse),
            vec![3_u8] => Ok(Context::PingRequest),
            vec![4_u8] => Ok(Context::PingResponse),
            vec![5_u8] => Ok(Context::Encrypted),
            _ => Err("Kind from byte error!")?
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Context::JoinRequest => vec![1_u8],
            Context::JoinResponse => vec![2_u8],
            Context::PingRequest => vec![3_u8],
            Context::PingResponse => vec![4_u8],
            Context::Encrypted => vec![5_u8]
        }
    }

}

#[derive(Clone, Debug)]
pub struct Envelope {
    pub context: Context,
    pub message: Vec<u8>,
    pub nonce: Int,
    pub sender: [u8; 32],
    pub time: u64
}

impl Envelope {

    pub fn from(context: Context, message: Vec<u8>, public_key: [u8; 32]) -> Self {

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let envelope = Envelope {
            context: context,
            message: message,
            nonce: Int::zero(),
            sender: public_key,
            time: time
        };

        difficulty(envelope)

    }

    pub fn hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.context.to_bytes()),
            hash(&self.message),
            hash(&self.nonce.to_bytes()),
            hash(&self.sender.to_vec()),
            hash(&self.time.to_be_bytes().to_vec())
        ])
    }

    pub fn from_astro(astro: &str) -> Result<Self, Box<dyn Error>> {

        let details: Vec<Vec<u8>> = list::as_bytes(astro);
        
        if details.len() == 5 {

            match Context::from_bytes(&details[0]) {

                Ok(c) => {

                    if details[3].len() == 8 && details[4].len() == 8 {
                        
                        let envelope = Envelope {
                            context: c,
                            message: details[1],
                            nonce: Int::from_bytes(&details[2]),
                            sender: details[3].try_into().unwrap(),
                            time: u64::from_be_bytes(details[4].try_into().unwrap()),
                        };

                        let current_time: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

                        if current_time >= envelope.time && current_time - envelope.time < 86400 {

                            let envelope_hash: [u8; 32] = envelope.hash();
                
                            if envelope_hash[0] == 0 {
                                
                                Ok(envelope)

                            } else {
                                Err("Message too easy!")?
                            }
                        } else {
                            Err("Message too old!")?
                        }
                    } else {
                        Err("Sender & Time not 8 bytes!")?
                    }
                },
                Err(e) => Err(e)?
            }            
        } else {
            Err("Message inputs must be 4!")?
        }
    }

    pub fn to_astro(&self) -> String {

        list::from_bytes(vec![
            self.context.to_bytes(),
            self.message,
            self.nonce.to_bytes(),
            self.sender.to_vec(),
            self.time.to_be_bytes().to_vec()
        ])

    }
}

fn difficulty(mut envelope: Envelope) -> Envelope {

    let mut envelope_hash = envelope.hash();
    
    while envelope_hash[0] != 0 {

        envelope.nonce += Int::one();

        envelope_hash = envelope.hash();

    }

    envelope

}
