use fides::asymmetric;

#[derive(Debug)]
pub struct Route {}

#[derive(Debug)]
pub struct Network {
    private_key: [u8;32],
    public_key: [u8;32],
    pub validation: Route 
}

impl Network {

    pub fn connect() -> Network {

        let priv_key = asymmetric::private_key();

        let pub_key = asymmetric::public_key(&priv_key);
        
        Network {
            private_key: priv_key,
            public_key: pub_key,
            validation: Route {},
        }

    }
    
}
