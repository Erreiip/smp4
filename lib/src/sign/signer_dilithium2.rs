use crystals_dilithium::dilithium2::Keypair;

use crate::sign::{signer::Signer};
use crate::conf::conf::SEED;

pub struct SignerDilithium2 {
    keypair: Keypair,
}

impl SignerDilithium2 {

    fn new(seed: &[u8]) -> Self {
        SignerDilithium2 {keypair: Keypair::generate(Some(&seed)).unwrap()}
    }
}

impl Signer for SignerDilithium2 {

    fn create_sign() -> impl Signer {
        SignerDilithium2::new(SEED)
    }

    fn sign(&self, hash: Vec<u8>, sfile_size: u64) -> Vec<u8> {
        let mut sfile_bytes = sfile_size.to_le_bytes().to_vec();
        let mut msg = hash.clone();
        msg.append(&mut sfile_bytes);

        let sign = self.keypair.sign(&msg);
        sign.to_vec()
    }

    fn verify(&self, signature: Vec<u8>, hash: Vec<u8>, sfile_size: u64) -> bool {
        let mut sfile_bytes = sfile_size.to_le_bytes().to_vec();
        let mut msg = hash.clone();
        msg.append(&mut sfile_bytes);

        self.keypair.verify(&msg, &signature)
    }
}
