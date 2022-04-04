use crate::Route;
use std::error::Error;

impl Route {

    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        match bytes[0] {
            1_u8 => Ok(Route::MainNova),
            2_u8 => Ok(Route::TestNova),
            _ => Err("Route from byte error!")?
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Route::MainNova => vec![1_u8],
            Route::TestNova => vec![2_u8]
        }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Route::MainNova, Route::MainNova) => true,
            (Route::TestNova, Route::TestNova) => true,
            _ => false
        }
    }
}

impl Eq for Route {}
