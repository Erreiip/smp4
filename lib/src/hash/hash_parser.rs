use crate::conf::conf::DELIMITER;
use crate::hash::hash_enum::hash_enum::SHA3;
use crate::hash::hasher::Hasher;
use crate::hash::hashsha3::HashSha3;

pub struct HashParser {}

impl HashParser {
    pub fn parse_algos(hashs_str: &str) -> Option<Vec<impl Hasher>> {
        let split = hashs_str.split(DELIMITER);

        let mut hashers = Vec::new();
        let mut is_valid = true;
        split.for_each(|hash_algo| {
            let hasher_opt = HashParser::parse_algo(hash_algo);
            match hasher_opt {
                Some(hasher) => hashers.push(hasher),
                None => is_valid = false,
            }
        });

        match is_valid {
            true => Some(hashers),
            false => None,
        }
    }

    pub fn parse_algo(hash_str: &str) -> Option<impl Hasher> {
        match hash_str {
            SHA3 => Some(HashSha3::create_hasher()),
            _ => None,
        }
    }
}
