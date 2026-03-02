
pub trait Signer {
    fn create_sign() -> impl Signer;
    fn sign(self, hash: Vec<u8>, sfile_size: u64) -> Vec<u8>;
}
