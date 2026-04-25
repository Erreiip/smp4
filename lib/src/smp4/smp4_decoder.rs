use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom},
    os::unix::fs::MetadataExt,
    path::Path,
};

use crate::{
    codec::cbor::CborDecoder,
    smp4::smp4_config::smp4_config::{MAGIC_BYTES_BUF, MAGIC_BYTES_BUF_SIZE},
    tail::tail::{TailDecode, TailFields},
};

pub struct SMP4Decoder {}

impl SMP4Decoder {
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

            if SMP4Decoder::is_magic_byte_buffer(buf) {
                let tail_position = document_size - (pos - buf_size);
                return Ok(tail_position);
            }

            pos = pos + 1;
        }

        Err("No tail found".to_string())
    }

    fn extract_tail(document: &mut File, tail_position: i64) -> Result<HashMap<String, String>, String> {
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

    pub fn  truncate(document_path: &str) -> Result<String, String> {
        let document = Path::new(document_path);
        let mut file_document: File = OpenOptions::new()
            .read(true)
            .open(document)
            .expect("File is Impossible to open");

        let tail_position = match SMP4Decoder::find_position(&mut file_document) {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        };

        let tail_map = match SMP4Decoder::extract_tail(&mut file_document, tail_position) {
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
            .open(document)
            .expect("File is Impossible to open");

        return match file_document.set_len(document_end_pos) {
            Ok(_) => match fs::rename(document_path, document_path.replace(".stxt", ".txt")) {
                Ok(_) => Ok(document_path.replace(".stxt", ".txt")),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => return Err(e.to_string()),
        };
    }
}
