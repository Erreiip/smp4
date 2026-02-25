
use crate::hash::hasher::Hasher;
use crate::hash::hashsha3::HashSha3;

pub struct HashParser {}

impl HashParser {

    pub fn parse_algo(hash_str: &str) -> Option<impl Hasher> {

        match hash_str {
            "SHA3" => Some(HashSha3::create_hasher()),
            _  => None
        }
    }
}
