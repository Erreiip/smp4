#[cfg(test)]
mod tests {
    use smp4_common::sign::{signer::Signer, signer_dilithium2::SignerDilithium2, verifier::Verifier};


    #[test]
    fn sign_test() {

        let signer = SignerDilithium2::create_sign();
        let verifier = SignerDilithium2::create_verifier();

        let hash: Vec<u8> = vec![1,2];
        let sfile_size = 10;

        let sig = signer.sign(hash.clone(), sfile_size);
        assert_eq!(verifier.verify(sig, hash.clone(), sfile_size), true)
    }
}
