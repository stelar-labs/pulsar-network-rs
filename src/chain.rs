use crate::Chain;
use std::error::Error;

impl Chain {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        match bytes[0] {
            1_u8 => Ok(Chain::Main),
            2_u8 => Ok(Chain::Test),
            _ => Err("Route from byte error!")?
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Chain::Main => vec![1_u8],
            Chain::Test => vec![2_u8]
        }
    }
}

impl PartialEq for Chain {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Chain::Main, Chain::Main) => true,
            (Chain::Test, Chain::Test) => true,
            _ => false
        }
    }
}

impl Eq for Chain {}
