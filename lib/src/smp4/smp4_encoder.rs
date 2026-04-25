use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use crate::{
    codec::cbor::CborEncoder, hash::{hash_parser::HashParser, hasher::Hasher}, metadata::metadata::MetadataEncoder, sign::{sign_parser::SignParser, signer::Signer}, smp4::smp4_config::smp4_config::{MAGIC_BYTES_BUF, MAGIC_BYTES_BUF_SIZE}, tail::tail::{TailEncoder, TailFields}
};

pub struct SMP4Encoder {}

impl SMP4Encoder {
    pub fn encode(
        document_path: &str,
        metadata: HashMap<String, String>,
        hash_algos: &str,
        sign_algo: &str,
    ) -> Result<String, String> {
        let document = Path::new(document_path);
        let mut file_document = OpenOptions::new()
            .append(true)
            .open(document)
            .expect("File is Impossible to open");

        let document_metadata = file_document
            .metadata()
            .expect("Metadata of the file can't be read");
        let document_size = document_metadata.len();
        let document_value = match fs::read(document) {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };

        let mut metadata_encoder = MetadataEncoder::new(CborEncoder::default());
        metadata_encoder.append_entry(metadata);
        let encoded_metadata = match metadata_encoder.encode() {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };
        let metadata_start = document_size;
        let metadata_size = encoded_metadata.len() as u64;

        let signer = SignParser::parse_algo(sign_algo).expect("Must be a good signature algorithm");
        let empty_signature: &[u8] = signer.empty_array();
        let signature_start = metadata_start + metadata_size;

        let mut tail_encoder = TailEncoder::new(CborEncoder::default());
        tail_encoder.add_entry(TailFields::METADATA_START, metadata_start.to_string());
        tail_encoder.add_entry(TailFields::SIGNATURE_START, signature_start.to_string());
        tail_encoder.add_entry(TailFields::HASH_ALGS, hash_algos.to_string());
        tail_encoder.add_entry(TailFields::SIGN_ALG, sign_algo.to_string());
        let encoded_tail = tail_encoder.encode().expect("Tail isn't correctly set up");
        let tail_size = encoded_tail.len() as u64;

        /*
         * Here we want to get the hash of the whole:
         * 1. document
         * 2. metadata
         * 3. signature
         * 4. 0xAAAA (MAGIC_BYTES_BUF)
         * 5. tail
         */
        let mut hash_vec: Vec<u8> = Vec::new();
        let hashers = HashParser::parse_algos(hash_algos).expect("Hash algos aren't good");
        for mut hasher in hashers {
            hasher.update(document_value.clone());
            hasher.update(encoded_metadata.clone());
            hasher.update(empty_signature.to_vec().clone());
            hasher.update(MAGIC_BYTES_BUF.to_vec());
            hasher.update(encoded_tail.clone());

            hash_vec.append(&mut hasher.finalize());
        }

        let sfile_size = document_size + metadata_size + signer.size() + MAGIC_BYTES_BUF_SIZE as u64 + tail_size;
        let signature = signer.sign(hash_vec, sfile_size);

        match file_document.write_all(&encoded_metadata) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };

        match file_document.write_all(&signature) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };

        match file_document.write_all(&MAGIC_BYTES_BUF) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };

        match file_document.write_all(&encoded_tail) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };

        match fs::rename(document_path, document_path.replace(".txt", ".stxt")) {
            Ok(_) => Ok(document_path.replace(".txt", ".stxt")),
            Err(e) => Err(e.to_string()),
        }
    }
}
