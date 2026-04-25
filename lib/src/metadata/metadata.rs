use std::collections::HashMap;

use crate::{codec::codec::{CodecError, KeyValueDecoder, KeyValueEncoder}, common::parser_utils::parser_utils::check_fields};

pub struct MetadataFields {}

impl MetadataFields {
    pub const AUTHOR: &'static str = "author";
    pub const DATE: &'static str = "date";
    pub const OID: &'static str = "OID";
    pub const DESCRIPTION: &'static str = "description";
    pub const EMAIL: &'static str = "email";
    pub const LICENSE: &'static str = "license";
    pub const LINK_ORIGIN: &'static str = "link_origin";
    pub const MANDANTORY_FIELDS: [&'static str; 1] = [MetadataFields::AUTHOR];
}

pub struct MetadataEncoder<E: KeyValueEncoder> {
    fields: HashMap<String, String>,
    encoder: E,
}

impl<E: KeyValueEncoder> MetadataEncoder<E> {

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

        match check_fields(self.fields.clone(), MetadataFields::MANDANTORY_FIELDS.to_vec()) {
            true => self.encoder.encode(self.fields.clone()),
            false => Err(CodecError::Encode(String::from("Fields missing")))
        }
    }
}

pub struct MetadataDecoder<D: KeyValueDecoder> {
    decoder: D,
}

impl<D: KeyValueDecoder> MetadataDecoder<D> {
    pub fn new(decoder: D) -> Self {
        Self { decoder }
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<HashMap<String, String>, CodecError> {
        self.decoder.decode(data)
    }
}
