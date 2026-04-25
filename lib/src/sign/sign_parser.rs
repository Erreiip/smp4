use crate::sign::{
    sign_enum::sign_enum::DILITHIUM2, signer::Signer, signer_dilithium2::SignerDilithium2,
};

pub struct SignParser {}

impl SignParser {
    pub fn parse_algo(sign_str: &str) -> Option<impl Signer> {
        match sign_str {
            DILITHIUM2 => Some(SignerDilithium2::create_sign()),
            _ => None,
        }
    }
}
