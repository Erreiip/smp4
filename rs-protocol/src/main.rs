use std::fs::{File};
use std::hash::{DefaultHasher, Hasher};
use std::io::Read;

fn main() {

    let path = "test";
    let buffer = match read_file(&path) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return;
        },
    };

    let mut hasher = hash_create();

    hasher.write(buffer.as_slice());

    println!("{:x}", hasher.finish());
}

fn hash_create() -> impl Hasher {
    return DefaultHasher::new();
}

fn read_file(path: &str) -> Result<Vec<u8>, &str> {
    let mut file = match File::open(path) {
        Ok(file_ok) => file_ok,
        Err(_e) => return Err("Wrong path"),
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {},
        Err(_e) => return Err("Impossible to read file"),
    };

    Ok(buffer)
}
