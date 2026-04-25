use std::{collections::HashMap, path::Path};

use crate::{common::parser_utils::parser_utils::check_fields, hash::hash_parser::HashParser, metadata::metadata::MetadataFields, sign::sign_parser::SignParser};


fn is_values_correct(document_path: &str, metadata: HashMap<String, String>, hash_algos: &str, sign_algo: &str) -> bool {

    if Path::new(document_path).exists() == false {
        return false;
    }

    if check_fields(metadata, MetadataFields::MANDANTORY_FIELDS.to_vec()) == false {
        return false;
    }

    if HashParser::parse_algos(hash_algos).is_none() {
        return false;
    }

    if SignParser::parse_algo(sign_algo).is_none() {
        return false;
    }

    true
}

pub fn build_smp4(file_path: String, metadata: HashMap<String, String>, hash_algos: &str, sign_algo: &str) -> String {
    println!("File path: {} | metadata: {:?}", file_path, metadata);

    match is_values_correct(&file_path, metadata, hash_algos, sign_algo) {
        true => file_path.replace(".mp4", ".smp4"),
        false => "".to_string()
    }
}
