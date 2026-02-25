
mod tests {
    use std::any::Any;

    use lib::hash::{hash_parser::HashParser, hasher::Hasher, hashsha3::HashSha3};

    #[test]
    fn hash_parse_test_not_working() {

        let str = "SHA345";

        let hasher_opt = HashParser::parse_algo(str);

        assert_eq!(hasher_opt.is_none(), true)
    }

    #[test]
    fn hash_parse_test_sha3() {

        let str = "SHA3";

        let hasher_opt = HashParser::parse_algo(str);

        let hasher = hasher_opt.expect("Hash must be a good one");

        assert_eq!(hasher.type_id() == HashSha3::create_hasher().type_id(), true)
    }
}

