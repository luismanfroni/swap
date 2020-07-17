mod cli;
use manager::{ file, checksum, compression, packager };
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let opts = cli::app();
    match opts.command {
        cli::Command::New(path) => {
            let file: file::File = file::new_file(&path.input);
            packager::parse_swap_file(file);
        }

        cli::Command::Compress(c) => {
            println!("Input File: {}", c.input);
            println!("Compressed File: {}", c.second_input);
            let mut input_file = file::new_file(&c.input);
            let file_bytes = input_file.get_bytes().as_slice();
            if let Ok(compressed_data) = compression::compress(file_bytes, 11) {
                let mut output_file = file::new_file(&c.second_input);

                output_file.new_std_file().write_all(&compressed_data)
                    .expect("Error trying to write compressed file!");
            } else {
                println!("Error on file input: {}", c.input)
            }
        }
        cli::Command::Checksum(c) => {
            if c.debug {
                println!("Debug; {}", c.input);
            }
            let mut file: file::File = file::new_file(&c.input);
            let checksum = checksum::checksum(file.get_bytes());
            println!("{}", checksum);
        }
    }
}
