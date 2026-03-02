use std::{error::Error, fs::OpenOptions, io::Write, path::Path};

use sha3::{Digest, Sha3_256};

use crate::{
    codec::cbor::CborEncoder,
    metadata::metadata::MetadataEncoder,
    sign::{signer::Signer, signer_dilithium2::SignerDilithium2},
    tail::tail::TailEncoder,
};

pub struct SFileEncoder {
    pub metadata_encoder: MetadataEncoder<CborEncoder>,
    pub tail_encoder: TailEncoder<CborEncoder>,
    // signer: SignerDilithium2,
}

impl SFileEncoder {
    pub fn new() -> Self {
        Self {
            metadata_encoder: MetadataEncoder::new(CborEncoder::default()),
            tail_encoder: TailEncoder::new(CborEncoder::default()),
            // signer: SignerDilithium2::create_sign(),
        }
    }

    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata_encoder.add_entry(key, value);
    }

    pub fn add_tail(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.tail_encoder.add_entry(key, value);
    }

    pub fn encode<P: AsRef<Path>>(&self, target_path: P) -> Result<(), Box<dyn Error>> {
        let meta_bytes = self.metadata_encoder.encode()?;
        let tail_bytes = self.tail_encoder.encode()?;

        const MAGIC_BITS_PAD_SIZE: usize = 4;
        const MAGICS: [u8; MAGIC_BITS_PAD_SIZE] = [0xAAu8; MAGIC_BITS_PAD_SIZE];

        let mut payload_without_sig = Vec::new();
        payload_without_sig.extend_from_slice(&MAGICS);
        payload_without_sig.extend_from_slice(&tail_bytes);

        let mut hash_input = Vec::new();
        hash_input.extend_from_slice(&meta_bytes);
        hash_input.extend_from_slice(&payload_without_sig);

        let hash = { Sha3_256::digest(&hash_input).to_vec() };

        const SIGNATURE_LEN: usize = 2048;
        let file_size_without_sig =
            (meta_bytes.len() + SIGNATURE_LEN + payload_without_sig.len()) as u64;

        let signer = SignerDilithium2::create_sign();

        let signature = signer.sign(hash, file_size_without_sig);

        let mut final_buffer =
            Vec::with_capacity(meta_bytes.len() + signature.len() + payload_without_sig.len());
        final_buffer.extend_from_slice(&meta_bytes);
        final_buffer.extend_from_slice(&signature);
        final_buffer.extend_from_slice(&payload_without_sig);

        let mut file = OpenOptions::new().append(true).open(&target_path)?;
        file.write_all(&final_buffer)?;

        Ok(())
    }
}
