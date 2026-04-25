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
        sign::sign_enum::sign_enum::DILITHIUM2,
        smp4::smp4_builder::{build_smp4, truncate_smp4},
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
        let build_ret: String = build_smp4(
            "DoesNotExist".to_string(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2,
        );

        assert_eq!(build_ret, "".to_string())
    }

    #[test]
    fn smp4_encode_good_input() {
        let dir = tempdir().expect("Directory creation error");
        let file_path = dir.path().join("temporary.txt");
        let mut file = File::create(&file_path).expect("File creation error");
        writeln!(file, "Four").expect("Write error in file");

        let file_path_str = file_path.clone().to_string_lossy().to_string();

        let build_ret: String = build_smp4(
            file_path_str.clone(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2,
        );

        assert_eq!(build_ret, file_path_str.clone().replace(".txt", ".stxt")); // TODO: change this to change only the endfile extension

        /* Read the start of the file to verify that it isn't corrupted */
        let buffer: &mut [u8] = &mut [0; 4];
        let mut file_document = OpenOptions::new()
            .read(true)
            .open(build_ret)
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
        let truncate_path: String = truncate_smp4("DoesNotExist".to_string());

        assert_eq!(truncate_path, "".to_string())
    }

    #[test]
    fn smp4_truncate_good_input() {
        let dir = tempdir().expect("Directory creation error");
        let file_path = dir.path().join("temporary.txt");
        let mut file = File::create(&file_path).expect("File creation error");
        writeln!(file, "Four").expect("Write error in file");

        let file_path_str = file_path.clone().to_string_lossy().to_string();

        let build_ret: String = build_smp4(
            file_path_str.clone(),
            get_fullfiled_metadata(),
            SHA3,
            DILITHIUM2,
        );

        assert_eq!(build_ret, file_path_str.clone().replace(".txt", ".stxt"));

        let truncate_path: String = truncate_smp4(
            build_ret.clone(),
        );

        assert_eq!(truncate_path, build_ret.replace(".stxt", ".txt"));
    }
}
