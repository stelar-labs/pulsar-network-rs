use crate::Route;
use std::error::Error;

impl Route {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        match bytes[0] {
            1_u8 => Ok(Route::Main),
            2_u8 => Ok(Route::Test),
            _ => Err("Route from byte error!")?
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Route::Main => vec![1_u8],
            Route::Test => vec![2_u8]
        }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Route::Main, Route::Main) => true,
            (Route::Test, Route::Test) => true,
            _ => false
        }
    }
}

impl Eq for Route {}
