use blake3::hash;

pub fn checksum(bytes: &[u8]) -> String {
    let hashed: Vec<u8> = hash(bytes).as_bytes().to_vec();
    let hash_string = String::from_utf8_lossy(&hashed);
    String::from(hash_string.as_ref())
}
