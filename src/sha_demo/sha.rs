use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};

pub fn get_file_hash(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    format!("{:x}", result)
}