use clap::{App, Arg};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;


mod utils; // Declare the `utils` module
use utils::file::add_changed_to_filename;
use utils::img::modify_image_slightly;

fn get_file_hash(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    format!("{:x}", result)
}


fn main() {
    let matches = App::new("ssc_proj")
    .version("1.0")
    .author("<NAME> <<EMAIL>>")
    .about("ssc_proj")
    .arg(Arg::with_name("input_file")
        .help("Sets the input file to use")
        .required(true)
        .index(1))
    .get_matches();

    let input_path = matches.value_of("input_file").unwrap();
    let output_path = add_changed_to_filename(input_path);
    modify_image_slightly(input_path, output_path.as_str());

    let original_hash = get_file_hash(input_path);
    println!("Original file SHA256: {}", original_hash);

    let modified_hash = get_file_hash(output_path.as_str());
    println!("Modified file SHA256: {}", modified_hash);
}
