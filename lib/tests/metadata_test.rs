#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use smp4_common::{
        codec::cbor::{CborDecoder, CborEncoder},
        metadata::metadata::{MetadataDecoder, MetadataEncoder, MetadataFields},
    };
    use std::{
        collections::HashMap,
        fs::{File, OpenOptions},
        io::{Read, Write},
    };
    use tempfile::tempdir;

    #[test]
    fn test_codec_cbor() {
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert(MetadataFields::AUTHOR.to_string(), "John Doe".to_string());
        expected.insert(
            MetadataFields::OID.to_string(),
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

    #[test]
    fn test_codec_cbor_in_file() {
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert(MetadataFields::AUTHOR.to_string(), "John Doe".to_string());
        expected.insert(
            MetadataFields::OID.to_string(),
            "d023957e-37dd-449e-b324-8a3e499b5c46".to_string(),
        );
        expected.insert(MetadataFields::EMAIL.to_string(), "100".to_string());

        let mut encoder = MetadataEncoder::new(CborEncoder::default());

        for (k, v) in expected.iter() {
            encoder.add_entry(k.clone(), v.clone());
        }

        let encoded = encoder.encode().expect("Cannot encode the content");

        /* temporary file */
        let dir = tempdir().expect("Directory creation error");
        let file_path = dir.path().join("temporary.txt");
        File::create(&file_path).expect("File creation error");

        let file_path_str = file_path.clone().to_string_lossy().to_string();

        let mut file_document = OpenOptions::new()
            .write(true)
            .open(file_path_str.clone())
            .expect("File is Impossible to open");

        file_document.write_all(&encoded).expect("Write error");
        file_document.flush().expect("Flushing error");

        file_document = OpenOptions::new()
            .read(true)
            .open(file_path_str.clone())
            .expect("File is Impossible to open");

        let mut buffer_read: Vec<u8> = Vec::new();
        file_document
            .read_to_end(&mut buffer_read)
            .expect("Read error");

        let mut decoder = MetadataDecoder::new(CborDecoder::default());

        let decoded = decoder
            .decode(&buffer_read)
            .expect("Cannot decode the content");

        assert_eq!(expected, decoded);
    }
}
