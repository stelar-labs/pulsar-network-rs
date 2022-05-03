use astro_format::arrays;
use super::Envelope;

impl Envelope {

    pub fn to_bytes(&self) -> Vec<u8> {

        arrays::encode(
            &[
                &self.kind.to_bytes(),
                &self.message,
                &self.nonce.to_bytes(),
                &self.route.to_bytes(),
                &self.sender,
                &self.time.to_bytes()
            ]
        )

    }

}
