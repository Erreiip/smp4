use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    common::parser_utils::parser_utils::check_fields,
    hash::hash_parser::HashParser,
    metadata::metadata::MetadataFields,
    sfile::{sfile_decoder::SFileDecoder, sfile_encoder::SFileEncoder},
    sign::sign_parser::SignParser,
};

fn is_document_exist(document_path: &str) -> bool {
    let document = Path::new(document_path);
    if document.exists() == false || document.is_file() == false {
        return false;
    }

    true
}

fn is_values_correct(
    document_path: &str,
    metadata: HashMap<String, String>,
    hash_algos: &str,
    sign_algo: &str,
) -> bool {
    is_document_exist(document_path)
        && check_fields(metadata, MetadataFields::MANDANTORY_FIELDS.to_vec())
        && HashParser::parse_algos(hash_algos).is_some()
        && SignParser::parse_algo(sign_algo).is_some()
}

pub fn build_sfile(
    document_path: String,
    metadata: HashMap<String, String>,
    hash_algos: &str,
    sign_algo: &str,
) -> Result<PathBuf, String> {
    println!("File path: {} | metadata: {:?}", document_path, metadata);

    match is_values_correct(&document_path, metadata.clone(), hash_algos, sign_algo) {
        true => {
            match SFileEncoder::encode(&document_path, metadata.clone(), hash_algos, sign_algo) {
                Ok(value) => return Ok(value),
                Err(err) => return Err(err),
            }
        }
        false => Err("Values aren't properly passed".to_string()),
    }
}

pub fn truncate_sfile(document_path: String) -> Result<PathBuf, String> {
    println!("File path: {}", document_path);

    match is_document_exist(&document_path) {
        true => match SFileDecoder::truncate(&document_path) {
            Ok(value) => return Ok(value),
            Err(e) => return Err(e),
        },
        false => Err("Values aren't properly passed".to_string()),
    }
}

pub fn sfile_verify(document_path: String) -> Result<bool, String> {
    println!("File path: {}", document_path);

    match is_document_exist(&document_path) {
        true => match SFileDecoder::verify(&document_path) {
            Ok(value) => return Ok(value),
            Err(e) => return Err(e),
        },
        false => Err("Values aren't properly passed".to_string()),
    }
}
