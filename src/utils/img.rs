use image::io::Reader as ImageReader;
use image::{ImageBuffer, RgbImage};
use std::path::Path;

pub fn extract_raw_pixels(file_path: &str) -> (Vec<u8>, u32, u32) {
    let img = ImageReader::open(file_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    let width = img.width();
    let height = img.height();
    let raw_pixels = img.to_rgb8().into_raw();
    (raw_pixels, width, height)
}

pub fn save_image_from_raw_pixels(raw_pixels: &[u8], width: u32, height: u32, output_path: &str) -> Result<(), image::ImageError> {
    let img: RgbImage = ImageBuffer::from_raw(width, height, raw_pixels.to_vec())
        .expect("Failed to create image from raw pixels");
    img.save(Path::new(output_path))?;
    Ok(())
}

pub fn modify_pixels(
    pixels: &[u8],
    width: usize,
    channel: usize,
    sub_mat: (usize, usize, usize, usize),
    delta: i16
) -> Vec<u8> {
    if pixels.is_empty() {
        return Vec::new();
    }

    let mut modified_pixels = pixels.to_vec();
    let (start_i, start_j, end_i, end_j) = sub_mat;
    let channels = 4;

    for i in start_i..=end_i {
        for j in start_j..=end_j {
            let pixel_index = (i * width + j) * channels + channel;

            if pixel_index < modified_pixels.len() {
                let current_value = modified_pixels[pixel_index] as i16;
                let new_value = (current_value + delta).rem_euclid(256);
                modified_pixels[pixel_index] = new_value as u8;
            }
        }
    }

    modified_pixels
}