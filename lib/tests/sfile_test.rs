#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs::{File, OpenOptions},
        io::Read,
    };

    use smp4_common::{
        hash::hash_enum::hash_enum::SHA3,
        metadata::metadata::MetadataFields,
        sfile::sfile::{build_sfile, sfile_metadata, sfile_verify, truncate_sfile},
        sign::sign_enum::sign_enum::DILITHIUM2,
    };
    use std::io::Write;
    use tempfile::tempdir;

    fn get_fullfiled_metadata() -> HashMap<String, String> {
        let mut metadata: HashMap<String, String> = HashMap::new();
        metadata.insert(MetadataFields::AUTHOR.to_string(), "me".to_string());
        metadata
    }

    #[test]
    fn smp4_encode_bad_input() {
        let build_ret = build_sfile(
            "DoesNotExist".to_string(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2,
        );

        assert_eq!(build_ret.is_err(), true);
    }

    #[test]
    fn smp4_encode_good_input() {
        let dir = tempdir().expect("Directory creation error");
        let file_path = dir.path().join("temporary.txt");
        let mut file = File::create(&file_path).expect("File creation error");
        writeln!(file, "Four").expect("Write error in file");

        let file_path_str = file_path.clone().to_string_lossy().to_string();

        let build_ret = build_sfile(
            file_path_str.clone(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2,
        );

        assert_eq!(build_ret.is_ok(), true);
        let document_path = build_ret.unwrap();
        assert_eq!(
            document_path,
            file_path_str.clone().replace(".txt", ".stxt")
        );

        /* Read the start of the file to verify that it isn't corrupted */
        let buffer: &mut [u8] = &mut [0; 4];
        let mut file_document = OpenOptions::new()
            .read(true)
            .open(document_path)
            .expect("File is Impossible to open");
        file_document
            .read_exact(buffer)
            .expect("Error during file reading");

        assert_eq!(buffer[0], 70);
        assert_eq!(buffer[1], 111);
        assert_eq!(buffer[2], 117);
        assert_eq!(buffer[3], 114);
    }

    #[test]
    fn smp4_truncate_bad_input() {
        let truncate_path = truncate_sfile("DoesNotExist".to_string());

        assert_eq!(truncate_path.is_err(), true)
    }

    #[test]
    fn smp4_truncate_good_input() {
        let dir = tempdir().expect("Directory creation error");
        let file_path = dir.path().join("temporary.txt");
        let mut file = File::create(&file_path).expect("File creation error");
        writeln!(file, "Four").expect("Write error in file");

        let file_path_str = file_path.clone().to_string_lossy().to_string();

        let build_ret = build_sfile(
            file_path_str.clone(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2,
        );

        assert_eq!(build_ret.is_ok(), true);
        let mut document_path = build_ret.unwrap();
        assert_eq!(
            document_path,
            file_path_str.clone().replace(".txt", ".stxt")
        );

        let truncate_path = truncate_sfile(document_path.to_str().unwrap().to_string());

        assert_eq!(truncate_path.is_ok(), true);
        let document_truncate_path = truncate_path.unwrap();
        document_path.set_extension("txt");
        assert_eq!(document_truncate_path, document_path);
    }

    #[test]
    fn test_signature_incorrect() {
        let signature_result = sfile_verify("DoesNotExist".to_string());

        assert_eq!(signature_result.is_err(), true);
    }

    /* TODO: There must be a signature incorrect test, like one byte modified in the file */

    #[test]
    fn test_signature_correct() {
        let dir = tempdir().expect("Directory creation error");
        let file_path = dir.path().join("temporary.txt");
        let mut file = File::create(&file_path).expect("File creation error");
        writeln!(file, "Four").expect("Write error in file");

        let file_path_str = file_path.clone().to_string_lossy().to_string();

        let build_ret = build_sfile(
            file_path_str.clone(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2,
        );

        assert_eq!(build_ret.is_ok(), true);
        let document_path = build_ret.unwrap();
        assert_eq!(
            document_path,
            file_path_str.clone().replace(".txt", ".stxt")
        );

        let signature_result = sfile_verify(document_path.to_str().unwrap().to_string());

        assert_eq!(signature_result.unwrap(), true);
    }

    #[test]
    fn test_metadata_correct() {
        let dir = tempdir().expect("Directory creation error");
        let file_path = dir.path().join("temporary.txt");
        let mut file = File::create(&file_path).expect("File creation error");
        writeln!(file, "Four").expect("Write error in file");

        let file_path_str = file_path.clone().to_string_lossy().to_string();

        let build_ret = build_sfile(
            file_path_str.clone(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2,
        );

        assert_eq!(build_ret.is_ok(), true);
        let document_path = build_ret.unwrap();
        assert_eq!(
            document_path,
            file_path_str.clone().replace(".txt", ".stxt")
        );

        let metadata = sfile_metadata(document_path.to_str().unwrap().to_string());

        assert_eq!(metadata.is_ok(), true);
        assert_eq!(metadata.unwrap().get(MetadataFields::AUTHOR), get_fullfiled_metadata().get(MetadataFields::AUTHOR));
    }
}
