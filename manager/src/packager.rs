use crate::file;
use shared::package_details::Details;
use shared::package::Package;
use serde_json::{Result, Value};

pub struct Packager {
    files: Vec<file::File>,
    details: Details
}

pub fn parse_swap_file(swap_file: &mut file::File) {
    if !swap_file.get_metadata().is_file() {
        panic!("File informed isn't in the right format!");
    }
    let swapfile_content = String::from_utf8_lossy(swap_file.get_bytes());
    let swap_json: Package = serde_json::from_str(swapfile_content.as_ref()).unwrap();

    println!("{:?}", swap_json);
}