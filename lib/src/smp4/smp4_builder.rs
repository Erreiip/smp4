use std::time::SystemTime;

#[derive(Debug)]
pub struct SMP4Metadata {
    pub date: SystemTime,
    pub author: String,
    pub oid: i32,
    pub description: String,
    pub email: String,
    pub license: String,
}

pub fn build_smp4(file_path: String, file_metadata: SMP4Metadata) -> String {
    println!("File path: {} | File metadata: {:?}", file_path, file_metadata);

    file_path.replace(".mp4", ".smp4")
}