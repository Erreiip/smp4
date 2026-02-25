pub trait Hasher {
    fn create_hasher() -> impl Hasher;
    fn update(&mut self, to_hash: Vec<u8>);
    fn finalize(&mut self) -> Vec<u8>;
}
