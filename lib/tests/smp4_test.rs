#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use smp4_common::{hash::hash_enum::hash_enum::SHA3, metadata::metadata::MetadataFields, sign::sign_enum::sign_enum::DILITHIUM2, smp4::smp4_builder::build_smp4};

    fn get_fullfiled_metadata() -> HashMap<String, String> {

        let mut metadata: HashMap<String, String> = HashMap::new();
        metadata.insert(MetadataFields::AUTHOR.to_string(), "me".to_string());
        metadata
    }

    #[test]
    fn smp4_wrong_input_test() {

        let build_ret: String = build_smp4(
            "DoesNotExist".to_string(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2
        );

        assert_eq!(build_ret, "".to_string())
    }

    #[test]
    fn smp4_good_input_test() {

        let build_ret: String = build_smp4(
            "/".to_string(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2
        );

        assert_eq!(build_ret, "/".to_string()) // TODO: change this, create a tempfile for the test
    }
}
