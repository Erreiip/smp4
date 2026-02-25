use std::collections::HashMap;

use serde_cbor;

use crate::codec::codec::{CodecError, KeyValueDecoder, KeyValueEncoder};

#[derive(Default)]
pub struct CborEncoder;

impl KeyValueEncoder for CborEncoder {
    fn encode(&self, entries: HashMap<String, String>) -> Result<Vec<u8>, CodecError> {
        serde_cbor::to_vec(&entries).map_err(|e| CodecError::Encode(e.to_string()))
    }
}

#[derive(Default)]
pub struct CborDecoder;

impl KeyValueDecoder for CborDecoder {
    fn decode(&self, encoded: &[u8]) -> Result<HashMap<String, String>, CodecError> {
        serde_cbor::from_slice(encoded).map_err(|e| CodecError::Decode(e.to_string()))
    }
}
