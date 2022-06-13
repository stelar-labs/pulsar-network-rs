use crate::Route;
use std::error::Error;

impl Route {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        match bytes[0] {
            1_u8 => Ok(Route::Peer),
            2_u8 => Ok(Route::Validation),
            _ => Err("Internal error!")?
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Route::Peer => vec![1_u8],
            Route::Validation => vec![2_u8]
        }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Route::Peer, Route::Peer) => true,
            (Route::Validation, Route::Validation) => true,
            _ => false
        }
    }
}

impl Eq for Route {}
