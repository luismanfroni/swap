use brotli;
use std::io::prelude::*;
use std::fs::{ File, Metadata };
use tar::{ Archive, Builder };

pub fn brotli_compress() {
    let file = File::open("foo.tar").unwrap();
    let mut a = Archive::new(file);


    let mut input = brotli::CompressorReader::new(&mut io::stdin(), 4096 /* buffer size */,
        quality as u32, lg_window_size as u32);
}

pub fn tar_compression(String path, String name) -> {
    let file = File::create(name + ".tar").unwrap();
    let mut new_tar = Builder::new(file);

    new_tar.append_path("file1.txt").unwrap();
    new_tar.append_file("file2.txt", &mut File::open("file3.txt").unwrap()).unwrap();
    new_tar.finish();
}
/*
fn tar_uncompression() {
    let file = File::open("foo.tar").unwrap();
    let mut entries = Archive::new(file).entries().unwrap();
    return entries;
    for file in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        // Inspect metadata about the file
        println!("{:?}", file.header().path().unwrap());
        println!("{}", file.header().size().unwrap());

        // files implement the Read trait
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        println!("{}", s);
    }
}
*/