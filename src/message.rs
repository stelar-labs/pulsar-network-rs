use std::convert::TryInto;
use opis::Int;
use std::time::SystemTime;
use crate::merkle_tree_hash;
use crate::Message;
use crate::MessageKind;
use fides::hash;
use std::error::Error;

impl MessageKind {
    
    pub fn from_byte(byte: &u8) -> Self {
        match byte {
            1 => MessageKind::Block,
            2 => MessageKind::CancelTransaction,
            3 => MessageKind::NextBlock,
            4 => MessageKind::Transaction,
            _ => panic!("{} is not a supported message kind!", byte)
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        match self {
            MessageKind::Block => vec![1_u8],
            MessageKind::CancelTransaction => vec![2_u8],
            MessageKind::NextBlock => vec![3_u8],
            MessageKind::Transaction => vec![4_u8]
        }
    }

}

impl Message {

    pub fn new(kind: MessageKind, body: Vec<u8>) -> Self {

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let msg = Message {
            body: body,
            kind: kind,
            nonce: Int::zero(),
            time: time
        };

        difficulty(msg)

    }

    pub fn message_hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.body),
            hash(&self.kind.into_bytes()),
            hash(&self.nonce.clone().to_ext_bytes(32)),
            hash(&self.time.to_be_bytes().to_vec())
        ])
    }

    pub fn from_bytes(input: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        if input.len() > 42 {

            let message = Message {
                kind: MessageKind::from_byte(&input[0]),
                nonce: Int::from_bytes(&input[1..33].to_vec()),
                time: u64::from_be_bytes(input[33..41].try_into().unwrap()),
                body: input[41..].to_vec(),
            };

            let current_time: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

            if current_time >= message.time && current_time - message.time < 86400 {

                let msg_hash: [u8; 32] = message.message_hash();

                let first_byte_msg_hash = format!("{:08b}", msg_hash[0]);
    
                if first_byte_msg_hash.chars().nth(0).unwrap() == '0' {
                    
                    Ok(message)

                } else {
                    Err("Message too easy!")?
                }
            } else {
                Err("Message too old!")?
            }
        } else {
            Err("Message too short!")?
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        vec![
            self.kind.into_bytes(),
            self.nonce.to_ext_bytes(32),
            self.time.to_be_bytes().to_vec(),
            self.body
        ].concat()
    }

}

fn difficulty(mut msg: Message) -> Message {

    let init_h = msg.message_hash();
    
    let mut first_byte = format!("{:08b}", init_h[0]);
    
    while first_byte.chars().nth(0).unwrap() == '1' {

        msg.nonce += Int::one();

        let new_hash = msg.message_hash();

        first_byte = format!("{:08b}", new_hash[0])

    }

    msg

}