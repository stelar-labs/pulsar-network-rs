
use crate::Route;

impl Route {

    pub fn from_byte(b: u8) -> Route {
        match b {
            1 => Route::MainValidation,
            2 => Route::TestValidation,
            _ => panic!("{} is not a support route!", b)
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Route::MainValidation => vec![1_u8],
            Route::TestValidation => vec![2_u8]
        }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Route::MainValidation, Route::MainValidation) => true,
            (Route::TestValidation, Route::TestValidation) => true,
            _ => false
        }
    }
}

impl Eq for Route {}