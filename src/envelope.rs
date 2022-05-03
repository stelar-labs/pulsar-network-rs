mod apply_difficulty;
mod from_bytes;
mod hash;
mod new;
mod to_bytes;
use opis::Int;
use std::error::Error;

use crate::Route;

#[derive(Clone, Debug)]
pub enum Kind { JoinRequest, JoinResponse, PingRequest, PingResponse, Encrypted }

impl Kind {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        match bytes[0] {
            1_u8 => Ok(Kind::JoinRequest),
            2_u8 => Ok(Kind::JoinResponse),
            3_u8 => Ok(Kind::PingRequest),
            4_u8 => Ok(Kind::PingResponse),
            5_u8 => Ok(Kind::Encrypted),
            _ => Err("Kind from byte error!")?
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Kind::JoinRequest => vec![1_u8],
            Kind::JoinResponse => vec![2_u8],
            Kind::PingRequest => vec![3_u8],
            Kind::PingResponse => vec![4_u8],
            Kind::Encrypted => vec![5_u8]
        }
    }

}

#[derive(Clone, Debug)]
pub struct Envelope {
    pub kind: Kind,
    pub message: Vec<u8>,
    pub nonce: Int,
    pub route: Route,
    pub sender: [u8; 32],
    pub time: Int
}
