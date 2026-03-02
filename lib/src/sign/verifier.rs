
pub trait Verifier {
    fn create_verifier() -> impl Verifier;
    fn verify(signature: Vec<u8>, hash: Vec<u8>, sfile_size: u64) -> Vec<u8>;
}
