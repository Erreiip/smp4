use std::collections::HashMap;

use crate::codec::codec::{CodecError, KeyValueDecoder, KeyValueEncoder};

pub struct TailEncoder<E: KeyValueEncoder> {
    fields: HashMap<String, String>,
    encoder: E,
}

impl<E: KeyValueEncoder> TailEncoder<E> {
    pub const METADATA_START: &'static str = "metadata_start";
    pub const SIGNATURE_START: &'static str = "signature_start";
    pub const HASH_ALGS: &'static str = "hash_algs";
    pub const SIGN_ALG: &'static str = "sign_alg";

    const MANDANTORY_FIELDS: [&'static str; 4] = [
        TailEncoder::<E>::METADATA_START,
        TailEncoder::<E>::SIGNATURE_START,
        TailEncoder::<E>::HASH_ALGS,
        TailEncoder::<E>::SIGN_ALG,
    ];

    pub fn new(encoder: E) -> Self {
        Self {
            fields: HashMap::new(),
            encoder,
        }
    }

    pub fn add_entry(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.fields.insert(key.into(), value.into());
    }

    fn check_fields(&self) -> bool {
        let mut has_all_fields = true;
        for field in TailEncoder::<E>::MANDANTORY_FIELDS {
            if self.fields.contains_key(field) == false {
                has_all_fields = false;
            }
        }

        has_all_fields
    }

    pub fn encode(&self) -> Result<Vec<u8>, CodecError> {
        match self.check_fields() {
            true => self.encoder.encode(self.fields.clone()),
            false => Err(CodecError::Encode(String::from("Fields missing"))),
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
