use blake3::{ hash, Hash };
use std::fs;

pub fn checksum_from_file(String filepath): String {
    let mut file: fs::File = fs::File::open(&path)?;
    let bytes: Vec<u8> = file.bytes().collect();
    return checksum(bytes);
}

pub fn checksum(bytes: Vec<u8>): String {
    let hashed: Vec<u8> = hash(bytes).as_bytes().into_vec();
    let hashString: String = String::from_utf8(hashed);
    return hashString;
}
