use fides::{merkle_root, hash};

use super::Envelope;

impl Envelope {

    pub fn hash(&self) -> [u8; 32] {

        merkle_root(
            vec![
                hash(&self.kind.to_bytes()),
                hash(&self.message),
                hash(&self.nonce.to_bytes()),
                hash(&self.sender),
                hash(&self.time.to_bytes())
            ]
        )

    }

}