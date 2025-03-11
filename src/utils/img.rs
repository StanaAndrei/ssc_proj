use sha2::{Digest, Sha256};
use image::io::Reader as ImageReader;
use image::{ImageBuffer, RgbImage};
use std::path::Path;

pub fn extract_raw_pixels(file_path: &str) -> (Vec<u8>, u32, u32) {
    // Open and decode the image
    let img = ImageReader::open(file_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    let width = img.width();
    let height = img.height();
    let raw_pixels = img.to_rgb8().into_raw();
    (raw_pixels, width, height)
}

pub fn get_image_pixel_hash(raw_pixels: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw_pixels);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn save_image_from_raw_pixels(raw_pixels: &[u8], width: u32, height: u32, output_path: &str) -> Result<(), image::ImageError> {
    let img: RgbImage = ImageBuffer::from_raw(width, height, raw_pixels.to_vec())
        .expect("Failed to create image from raw pixels");
    img.save(Path::new(output_path))?;
    Ok(())
}