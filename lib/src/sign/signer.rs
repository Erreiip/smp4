pub trait Signer {
    fn create_sign() -> impl Signer;
    fn sign(&self, hash: Vec<u8>, sfile_size: u64) -> Vec<u8>;
    fn verify(&self, signature: Vec<u8>, hash: Vec<u8>, sfile_size: u64) -> bool;
    fn size(&self) -> u64;
    fn empty_array(&self) -> &[u8];
}
