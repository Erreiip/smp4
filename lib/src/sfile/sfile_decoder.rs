use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

use crate::{
    codec::cbor::CborDecoder, hash::{hash_parser::HashParser, hasher::Hasher}, metadata::metadata::MetadataDecoder, sfile::sfile_config::sfile_config::{
        MAGIC_BYTES_BUF, MAGIC_BYTES_BUF_SIZE, s_extension_transform,
    }, sign::{sign_parser::SignParser, signer::Signer}, tail::tail::{TailDecode, TailFields}
};

pub struct SFileDecoder {}

impl SFileDecoder {
    fn is_magic_byte_buffer(buf: &[u8; MAGIC_BYTES_BUF_SIZE]) -> bool {
        return *buf == MAGIC_BYTES_BUF;
    }

    fn find_position(document: &mut File) -> Result<i64, String> {
        let mut pos: i64 = MAGIC_BYTES_BUF_SIZE as i64;
        let buf = &mut [0; MAGIC_BYTES_BUF_SIZE];
        let buf_size = MAGIC_BYTES_BUF_SIZE as i64;

        let document_size = match document.metadata() {
            Ok(metadata) => metadata.size(),
            Err(e) => return Err(e.to_string()),
        } as i64;

        while (pos) < document_size {
            match document.seek(SeekFrom::End(-pos)) {
                Err(e) => return Err(e.to_string()),
                _ => {}
            };

            match document.read_exact(buf) {
                Err(e) => return Err(e.to_string()),
                _ => {}
            };

            if SFileDecoder::is_magic_byte_buffer(buf) {
                let tail_position = document_size - (pos - buf_size);
                return Ok(tail_position);
            }

            pos = pos + 1;
        }

        Err("No tail found".to_string())
    }

    fn extract_tail(
        document: &mut File,
        tail_position: i64,
    ) -> Result<HashMap<String, String>, String> {
        match document.seek(SeekFrom::Start(tail_position as u64)) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };

        let mut buf: Vec<u8> = Vec::new();
        match document.read_to_end(&mut buf) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };

        let mut tail_decoder = TailDecode::new(CborDecoder::default());

        return match tail_decoder.decode(&buf) {
            Ok(hashmap) => Ok(hashmap),
            Err(e) => return Err(e.to_string()),
        };
    }

    pub fn verify(document_path: &str) -> Result<bool, String> {
        let mut file_document: File = OpenOptions::new()
            .read(true)
            .open(document_path)
            .expect("File is Impossible to open");
        let document_metadata = file_document
            .metadata()
            .expect("Metadata of the file can't be read");
        let document_size = document_metadata.len();

        let tail_position = match SFileDecoder::find_position(&mut file_document) {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };

        let tail_map = match SFileDecoder::extract_tail(&mut file_document, tail_position) {
            Ok(tail_map) => tail_map,
            Err(e) => return Err(e.to_string()),
        };

        let hash_algos = match tail_map.get(TailFields::HASH_ALGS) {
            Some(algos) => algos,
            None => return Err("No field for signature algorithm".to_string()),
        };

        let signature_start = match tail_map.get(TailFields::SIGNATURE_START) {
            Some(signature_start_str) => match signature_start_str.parse::<u64>() {
                Ok(signature_start) => signature_start,
                Err(e) => return Err(e.to_string()),
            },
            None => return Err("No field for signature start".to_string()),
        };

        let sign_algo = match tail_map.get(TailFields::SIGN_ALG) {
            Some(algo) => algo,
            None => return Err("No field for signature algorithm".to_string()),
        };

        let signer = SignParser::parse_algo(sign_algo).expect("Must be a good signature algorithm");
        let empty_signature: &[u8] = signer.empty_array();
        let signature_size = signer.size();
        let mut signature_buffer: Vec<u8> = vec![0; signature_size as usize];

        let buf_size: u64 = signature_start;
        let mut document_and_metadata_value: Vec<u8> = vec![0; buf_size as usize];
        match file_document.rewind() {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };
        match file_document.read_exact(&mut document_and_metadata_value) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };
        match file_document.read_exact(&mut signature_buffer) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };
        let mut tail_buffer: Vec<u8> = Vec::new();
        match file_document.read_to_end(&mut tail_buffer) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };
        tail_buffer.remove(0);
        tail_buffer.remove(0);
        // MAGIC bytes remove

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
            hasher.update(document_and_metadata_value.clone());
            hasher.update(empty_signature.to_vec().clone());
            hasher.update(MAGIC_BYTES_BUF.to_vec());
            hasher.update(tail_buffer.clone());

            hash_vec.append(&mut hasher.finalize());
        }

        Ok(signer.verify(signature_buffer, hash_vec, document_size))
    }

    pub fn metadata(document_path: &str) -> Result<HashMap<String, String>, String> {
        let mut file_document: File = OpenOptions::new()
            .read(true)
            .open(document_path)
            .expect("File is Impossible to open");

        let tail_position = match SFileDecoder::find_position(&mut file_document) {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };

        let tail_map = match SFileDecoder::extract_tail(&mut file_document, tail_position) {
            Ok(tail_map) => tail_map,
            Err(e) => return Err(e.to_string()),
        };

        let metadata_start = match tail_map.get(TailFields::METADATA_START) {
            Some(metadata_start_str) => match metadata_start_str.parse::<u64>() {
                Ok(metadata_start) => metadata_start,
                Err(e) => return Err(e.to_string()),
            },
            None => return Err("No field for signature start".to_string()),
        };

        let signature_start = match tail_map.get(TailFields::SIGNATURE_START) {
            Some(signature_start_str) => match signature_start_str.parse::<u64>() {
                Ok(signature_start) => signature_start,
                Err(e) => return Err(e.to_string()),
            },
            None => return Err("No field for signature start".to_string()),
        };

        let metadata_size = signature_start - metadata_start;
        let mut metadata_vec_buffer: Vec<u8> = vec![0; metadata_size as usize];
        match file_document.seek(SeekFrom::Start(metadata_start)) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
        match file_document.read_exact(&mut metadata_vec_buffer) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };

        let mut decoder = MetadataDecoder::new(CborDecoder::default());
        match decoder.decode(&metadata_vec_buffer) {
            Ok(map) => Ok(map),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn truncate(document_path: &str) -> Result<PathBuf, String> {
        let mut document_pathbuf = PathBuf::from(document_path);
        let mut file_document: File = OpenOptions::new()
            .read(true)
            .open(document_path)
            .expect("File is Impossible to open");

        let tail_position = match SFileDecoder::find_position(&mut file_document) {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };

        let tail_map = match SFileDecoder::extract_tail(&mut file_document, tail_position) {
            Ok(tail_map) => tail_map,
            Err(e) => return Err(e.to_string()),
        };

        let document_end_pos = match tail_map.get(TailFields::METADATA_START) {
            Some(metadata_start) => match metadata_start.parse::<u64>() {
                Ok(document_end_pos) => document_end_pos,
                Err(e) => return Err(e.to_string()),
            },
            None => return Err("No field for metadata start".to_string()),
        };

        file_document = OpenOptions::new()
            .write(true)
            .open(document_pathbuf.clone())
            .expect("File is Impossible to open");

        let extension = match document_pathbuf.extension() {
            Some(value) => value.to_str().unwrap(),
            None => return Err("Error during extension extract of the current file".to_string()),
        };
        let new_extension = s_extension_transform(extension);

        document_pathbuf.set_extension(new_extension);

        return match file_document.set_len(document_end_pos) {
            Ok(_) => match fs::rename(document_path, document_pathbuf.clone()) {
                Ok(_) => Ok(document_pathbuf),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => return Err(e.to_string()),
        };
    }
}
