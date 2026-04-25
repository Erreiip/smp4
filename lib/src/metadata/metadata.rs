use std::collections::HashMap;

use crate::codec::codec::{CodecError, KeyValueDecoder, KeyValueEncoder};

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
        self.encoder.encode(self.fields.clone())
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
