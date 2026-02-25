use std::{collections::HashMap, error::Error, fmt};

#[derive(Debug)]
pub enum CodecError {
    Encode(String),
    Decode(String),
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodecError::Encode(msg) => write!(f, "Error while encoding: {}", msg),
            CodecError::Decode(msg) => write!(f, "Error while decoding: {}", msg),
        }
    }
}

impl Error for CodecError {}

pub trait KeyValueEncoder {
    fn encode(&self, entries: HashMap<String, String>) -> Result<Vec<u8>, CodecError>;
}

pub trait KeyValueDecoder {
    fn decode(&self, encoded: &[u8]) -> Result<HashMap<String, String>, CodecError>;
}
