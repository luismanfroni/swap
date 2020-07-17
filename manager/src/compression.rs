use std::io::Read;
use tar::{ Builder, Archive, Entries };
const BROTLI_BUFFER: usize = 4096;
const BROTLI_WINDOW: u32 = 21;

pub fn compress(data: &[u8], quality: u32) -> Result<Vec<u8>, std::io::Error> {
    let mut compressor = brotli::CompressorReader::new(data,
        BROTLI_BUFFER, quality, BROTLI_WINDOW);

    let mut compressed_data: Vec<u8> = Vec::new();
    compressor.read_to_end(&mut compressed_data)?;

    Ok(compressed_data)
}
pub fn decompress(data: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let mut decompressor = brotli::Decompressor::new(data.as_slice(), BROTLI_BUFFER);
    let mut decompressed_data: Vec<u8> = Vec::new();
    decompressor.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

pub fn new_tar(file: std::fs::File) -> Builder<std::fs::File> {
    Builder::new(file)
}

pub fn from_tar(file: std::fs::File) ->  Archive<std::fs::File> {
    Archive::new(file)
}