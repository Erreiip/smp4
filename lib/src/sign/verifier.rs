
pub trait Verifier {
    fn create_verifier() -> impl Verifier;
    fn verify(self, signature: Vec<u8>, hash: Vec<u8>, sfile_size: u64) -> bool;
}
