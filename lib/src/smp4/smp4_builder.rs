use std::{collections::HashMap, path::Path};

use crate::{
    common::parser_utils::parser_utils::check_fields,
    hash::hash_parser::HashParser,
    metadata::metadata::MetadataFields,
    sign::sign_parser::SignParser,
    smp4::{smp4_decoder::SMP4Decoder, smp4_encoder::SMP4Encoder},
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

pub fn build_smp4(
    document_path: String,
    metadata: HashMap<String, String>,
    hash_algos: &str,
    sign_algo: &str,
) -> String {
    println!("File path: {} | metadata: {:?}", document_path, metadata);

    match is_values_correct(&document_path, metadata.clone(), hash_algos, sign_algo) {
        true => {
            match SMP4Encoder::encode(&document_path, metadata.clone(), hash_algos, sign_algo) {
                Ok(value) => return value,
                Err(err) => return err,
            }
        }
        false => "".to_string(),
    }
}

pub fn truncate_smp4(document_path: String) -> String {
    println!("File path: {}", document_path);

    match is_document_exist(&document_path) {
        true => {
            match SMP4Decoder::truncate(&document_path) {
                Ok(value) => return value,
                Err(e) => return e,
            }
        }
        false => "".to_string(),
    }
}
