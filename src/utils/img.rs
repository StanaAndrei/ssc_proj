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

pub fn modify_first_pixel_red_inc(pixels: &[u8]) -> Vec<u8> {
    if pixels.is_empty() {
        return Vec::new();
    }

    let mut modified_pixels = pixels.to_vec();
    if !modified_pixels.is_empty() {
        if modified_pixels[0] < 255 {
            modified_pixels[0] += 1;
        } else {
            modified_pixels[0] = 0;
        }
    }

    modified_pixels
}

pub fn modify_first_pixel_red_dec(pixels: &[u8]) -> Vec<u8> {
    if pixels.is_empty() {
        return Vec::new();
    }

    let mut modified_pixels = pixels.to_vec();
    if !modified_pixels.is_empty() {
        if modified_pixels[0] > 0 {
            modified_pixels[0] -= 1;
        } else {
            modified_pixels[0] = 255;
        }
    }

    modified_pixels
}
