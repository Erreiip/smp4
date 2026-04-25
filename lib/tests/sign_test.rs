#[cfg(test)]
mod tests {
    use smp4_common::sign::{sign_enum::sign_enum::DILITHIUM2, sign_parser::SignParser, signer::Signer, signer_dilithium2::SignerDilithium2};
    use std::any::Any;

    #[test]
    fn sign_parse_test() {

        assert_eq!(SignParser::parse_algo("notgood").is_none(), true);
        assert_eq!(SignParser::parse_algo(DILITHIUM2).is_some(), true);

        let signer = SignParser::parse_algo(DILITHIUM2).unwrap();
        assert_eq!(
            signer.type_id() == SignerDilithium2::create_sign().type_id(),
            true
        );
    }

    #[test]
    fn sign_test() {

        let signer = SignerDilithium2::create_sign();

        let hash: Vec<u8> = vec![1,2];
        let sfile_size = 10;

        let sig = signer.sign(hash.clone(), sfile_size);
        assert_eq!(signer.verify(sig, hash.clone(), sfile_size), true)
    }
}
