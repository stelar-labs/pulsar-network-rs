use crate::{Config, Route};

impl Config {

    pub fn new() -> Self {
        Config {
            bootstrap: false,
            route: Route::Test,
            seeders: Vec::new()
        }
    }

}