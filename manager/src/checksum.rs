use sha1::{Sha1, Digest};
use std::fs;

pub fn checksum(String path) {
    let mut file = fs::File::open(&path)?;
    return Sha1::digest_reader(&mut file)?
}