#[cfg(test)]
mod tests {

    use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Read, Write}};

    use pretty_assertions::assert_eq;
    use smp4_common::{
        codec::cbor::{CborDecoder, CborEncoder},
        tail::tail::{TailDecode, TailEncoder, TailFields},
    };
    use tempfile::tempdir;

    #[test]
    fn tail_missing_fields() {
        let mut tail = TailEncoder::new(CborEncoder::default());
        tail.add_entry("unused entry", "test");

        assert_eq!(tail.encode().is_err(), true);
    }

    #[test]
    fn tail_fill_fields() {
        let mut tail = TailEncoder::new(CborEncoder::default());
        tail.add_entry(TailFields::METADATA_START, "100");
        tail.add_entry(TailFields::SIGNATURE_START, "100");
        tail.add_entry(TailFields::HASH_ALGS, "SHA3");
        tail.add_entry(TailFields::SIGN_ALG, "ECDSA");

        assert_eq!(tail.encode().is_err(), false)
    }

    #[test]
    fn tail_fill_fields_in_a_file() {
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert(TailFields::METADATA_START.to_string(), "100".to_string());
        expected.insert(TailFields::SIGNATURE_START.to_string(), "100".to_string());
        expected.insert(TailFields::HASH_ALGS.to_string(), "SHA3".to_string());
        expected.insert(TailFields::SIGN_ALG.to_string(), "ECDSA".to_string());

        let mut tail = TailEncoder::new(CborEncoder::default());
        for (k, v) in expected.iter() {
            tail.add_entry(k.clone(), v.clone());
        }

        let encoded = tail.encode().expect("Encode err");

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
        file_document.read_to_end(&mut buffer_read).expect("Read error");

        let mut decoder = TailDecode::new(CborDecoder::default());

        let decoded = decoder.decode(&buffer_read).expect("Cannot decode the content");

        assert_eq!(expected, decoded);
    }
}
