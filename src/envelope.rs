use astro_format::arrays;
use fides::merkle_root;
use opis::Int;
use std::convert::TryInto;
use std::error::Error;
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
        match byte[0] {
            1_u8 => Ok(Context::JoinRequest),
            2_u8 => Ok(Context::JoinResponse),
            3_u8 => Ok(Context::PingRequest),
            4_u8 => Ok(Context::PingResponse),
            5_u8 => Ok(Context::Encrypted),
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
    pub time: Int
}

impl Envelope {

    pub fn from(context: Context, message: Vec<u8>, public_key: [u8; 32]) -> Self {

        let time = Int::from_bytes(
            &SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_be_bytes()
                .to_vec()
        );

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
        merkle_root(&vec![
            self.context.to_bytes(),
            self.message.clone(),
            self.nonce.to_bytes(),
            self.sender.to_vec(),
            self.time.to_bytes()
        ])
    }

    pub fn from_bytes(buffer: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        let decoded = arrays::decode(buffer);
        
        if decoded.len() == 5 {

            match Context::from_bytes(&decoded[0]) {

                Ok(c) => {
                        
                    let envelope = Envelope {
                        context: c,
                        message: decoded[1].clone(),
                        nonce: Int::from_bytes(&decoded[2]),
                        sender: decoded[3].clone().try_into().unwrap(),
                        time: Int::from_bytes(&decoded[4])
                    };
                    
                    let envelope_hash: [u8; 32] = envelope.hash();
            
                    if envelope_hash[0] == 0 {
                        
                        let current_time = Int::from_bytes(
                            &SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                                .to_be_bytes()
                                .to_vec()
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

    pub fn to_bytes(&self) -> Vec<u8> {

        arrays::encode(&vec![
            self.context.to_bytes(),
            self.message.clone(),
            self.nonce.to_bytes(),
            self.sender.to_vec(),
            self.time.to_bytes()
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
