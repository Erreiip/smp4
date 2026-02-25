mod tests {
    use std::any::Any;

    use lib::hash::{hash_parser::HashParser, hasher::Hasher, hashsha3::HashSha3};

    #[test]
    fn hash_parse_test_not_working() {
        let str = "SHA345";

        let hasher_opt = HashParser::parse_algo(str);

        assert_eq!(hasher_opt.is_none(), true);
    }

    #[test]
    fn hash_parse_test_sha3() {
        let str = "SHA3";

        let hasher_opt = HashParser::parse_algo(str);

        let hasher = hasher_opt.expect("Hash must be a good one");

        assert_eq!(
            hasher.type_id() == HashSha3::create_hasher().type_id(),
            true
        );
    }

    #[test]
    fn hash_parse_test_multiple_hash_in_a_string() {
        let str = "SHA3;SHA3";

        let hashers_opt = HashParser::parse_algos(str);

        let hashers = hashers_opt.expect("Hash must be a good one");

        hashers.into_iter().for_each(|hasher| {
            assert_eq!(
                hasher.type_id() == HashSha3::create_hasher().type_id(),
                true
            );
        });
    }

    #[test]
    fn hash_parse_test_multiple_hash_in_a_string_with_a_wrong_one() {
        let mut str = "SHA3;SHA355";

        let mut hashers_opt = HashParser::parse_algos(str);

        assert_eq!(hashers_opt.is_none(), true);

        str = "SHA355;SHA3";

        hashers_opt = HashParser::parse_algos(str);

        assert_eq!(hashers_opt.is_none(), true);
    }
}
