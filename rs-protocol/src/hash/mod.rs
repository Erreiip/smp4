use sha3::{Digest, Sha3_256};

pub trait Hasher {
    fn create_hasher() -> impl Hasher;
    fn update(&mut self, to_hash: Vec<u8>);
    fn finalize(&mut self) -> Vec<u8>;
}

#[derive(Default)]
pub struct HashSha3 {
    sub_hasher: Sha3_256,
}

impl HashSha3 {

    fn new() -> Self {
        HashSha3 { sub_hasher: Sha3_256::new() }
    }
}

impl Hasher for HashSha3 {

    fn create_hasher() -> impl Hasher {
        HashSha3::new()
    }

    fn update(&mut self, to_hash: Vec<u8>) {

        self.sub_hasher.update(to_hash);
    }

    fn finalize(&mut self) -> Vec<u8> {

        self.sub_hasher.finalize_reset().to_vec()
    }
}
