#[cfg(test)]
mod tests {
    use lib::{
        codec::cbor::{CborDecoder, CborEncoder},
        metadata::metadata::{MetadataDecoder, MetadataEncoder},
    };
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn test_codec_cbor() {
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert("author".to_string(), "John Doe".to_string());
        expected.insert(
            "oid".to_string(),
            "d023957e-37dd-449e-b324-8a3e499b5c46".to_string(),
        );
        expected.insert("foo".to_string(), "bar".to_string());

        let mut encoder = MetadataEncoder::new(CborEncoder::default());

        for (k, v) in expected.iter() {
            encoder.add_entry(k.clone(), v.clone());
        }

        let encoded = encoder.encode().expect("Cannot encode the content");

        let mut decoder = MetadataDecoder::new(CborDecoder::default());

        let decoded = decoder.decode(&encoded).expect("Cannot decode the content");

        assert_eq!(expected, decoded);
    }

    #[test]
    fn test_failing_decode_cbor() {
        let mut decoder = MetadataDecoder::new(CborDecoder::default());
        let invalid_cbor_encoded_data = [];

        let decoded = decoder.decode(&invalid_cbor_encoded_data);

        assert!(decoded.is_err());
    }
}
