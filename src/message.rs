mod apply_difficulty;
mod from_bytes;
mod hash;
mod new;
mod to_bytes;
mod valid;
use crate::Topic;
use std::error::Error;

impl Topic {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        match bytes[0] {
            1_u8 => Ok(Topic::Block),
            2_u8 => Ok(Topic::BlockRequest),
            3_u8 => Ok(Topic::CancelTransaction),
            4_u8 => Ok(Topic::JoinRequest),
            5_u8 => Ok(Topic::JoinResponse),
            6_u8 => Ok(Topic::PingRequest),
            7_u8 => Ok(Topic::PingResponse),
            8_u8 => Ok(Topic::Transaction),
            _ => Err("Internal error!")?
        }

    }

    pub fn to_bytes(&self) -> Vec<u8> {

        match self {
            Topic::Block => vec![1_u8],
            Topic::BlockRequest => vec![2_u8],
            Topic::CancelTransaction => vec![3_u8],
            Topic::JoinRequest => vec![4_u8],
            Topic::JoinResponse => vec![5_u8],
            Topic::PingRequest => vec![6_u8],
            Topic::PingResponse => vec![7_u8],
            Topic::Transaction => vec![8_u8],
        }

    }

}
