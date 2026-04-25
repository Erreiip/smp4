pub mod sfile_config {

    pub const MAGIC_BYTES_BUF_SIZE: usize = 2;
    pub const MAGIC_BYTES_BUF: [u8; 2] = [0xAA, 0xAA];

    pub fn extension_transform(extension: &str) -> String {
        String::from("s".to_string() + extension)
    }

    pub fn s_extension_transform(s_extension: &str) -> String {
        s_extension[1..].to_string()
    }
}
