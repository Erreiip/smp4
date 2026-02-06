use std::fs::{File};
use std::io::Read;

use crate::hash::HashSha3;

mod hash;
use hash::Hasher;

fn main() {

    let path = "test";
    let buffer = match read_file(&path) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return;
        },
    };

    let mut hasher = HashSha3::create_hasher();

    hasher.update(buffer);

    println!("{:?}", hasher.finalize());
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
