use std::collections::HashMap;

use crate::{codec::codec::{CodecError, KeyValueDecoder, KeyValueEncoder}, common::parser_utils::parser_utils::check_fields};

pub struct TailFields {}

impl TailFields {
    pub const METADATA_START: &'static str = "metadata_start";
    pub const SIGNATURE_START: &'static str = "signature_start";
    pub const HASH_ALGS: &'static str = "hash_algs";
    pub const SIGN_ALG: &'static str = "sign_alg";
    pub const MANDANTORY_FIELDS: [&'static str; 4] = [TailFields::METADATA_START, TailFields::SIGNATURE_START, TailFields::HASH_ALGS, TailFields::SIGN_ALG];
}

pub struct TailEncoder<E: KeyValueEncoder> {
    fields: HashMap<String, String>,
    encoder: E,
}

impl<E: KeyValueEncoder> TailEncoder<E> {


    pub fn new(encoder: E) -> Self {
        Self {
            fields: HashMap::new(),
            encoder,
        }
    }

    pub fn add_entry(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.fields.insert(key.into(), value.into());
    }

    pub fn encode(&self) -> Result<Vec<u8>, CodecError> {

        match check_fields(self.fields.clone(), TailFields::MANDANTORY_FIELDS.to_vec()) {
            true => self.encoder.encode(self.fields.clone()),
            false => Err(CodecError::Encode(String::from("Fields missing")))
        }
    }
}

pub struct TailDecode<D: KeyValueDecoder> {
    decoder: D,
}

impl<D: KeyValueDecoder> TailDecode<D> {
    pub fn new(decoder: D) -> Self {
        Self { decoder }
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<HashMap<String, String>, CodecError> {
        self.decoder.decode(data)
    }
}
