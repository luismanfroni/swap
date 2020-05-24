use std::fs;
use std::io;

pub struct File {
    path: String,
    metadata: Option<fs::Metadata>,
    bytes: Option<Vec<u8>>
}

pub fn new_file(path: &str) -> File {
    File {
        path: String::from(path),
        bytes: None, metadata: None
    }
}

impl File {
    pub fn get_bytes(&mut self) -> &Vec<u8> {
        let path = self.path.clone();
        self.bytes.get_or_insert_with(|| {
            match fs::read(&path) {
                Ok(bytes) => {
                    bytes
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::PermissionDenied {
                        eprintln!("please run again with appropriate permissions.");
                    }
                    panic!("{}", e);
                }
            }
        })
    }

    pub fn get_metadata(&mut self) -> &fs::Metadata {
        let path = self.path.clone();
        self.metadata.get_or_insert_with(|| {
            match fs::metadata(&path) {
                Ok(metadata) => {
                    metadata
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        })
    }
}
