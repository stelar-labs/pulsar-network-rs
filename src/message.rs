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

        let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        let msg = Message {
            body: body,
            kind: kind,
            nonce: Int::zero(),
            time: time
        };

        difficulty(msg)

    }

    pub fn hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.body),
            hash(&self.kind.into_bytes()),
            hash(&self.nonce.clone().to_ext_bytes(32)),
            hash(&[vec![0_u8; 24], self.time.to_be_bytes().to_vec()].concat())
        ])
    }

    pub fn from_bytes(input: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        if input.len() < 65 {

            Ok(Message {
                kind: MessageKind::from_byte(&input[1]),
                nonce: Int::from_bytes(&input[1..33].to_vec()),
                time: u64::from_be_bytes(input[33..65][24..].try_into().unwrap()),
                body: input[65..].to_vec(),
            })

        } else {
            Err("Message too short!")?
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        vec![
            self.kind.into_bytes(),
            self.nonce.to_ext_bytes(32),
            [vec![0_u8; 24], self.time.to_be_bytes().to_vec()].concat(),
            self.body
        ].concat()
    }

}

fn difficulty(mut msg: Message) -> Message {

    let mut hash = msg.hash();
    
    let mut first_byte = format!("{:08b}", hash[0]);
    
    while first_byte.chars().nth(0).unwrap() == '1' {

        msg.nonce += Int::one();

        hash = msg.hash();

        first_byte = format!("{:08b}", hash[0])

    }

    msg

}