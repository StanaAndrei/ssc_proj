use sha2::{Digest, Sha256};

pub fn get_pixels_hash(pixels: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pixels);
    let result = hasher.finalize();
    format!("{:x}", result)
}