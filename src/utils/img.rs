pub fn modify_1st_pixel_red_ch(input_path: &str, output_path: &str, inc: i8) {
    let img = image::open(input_path).expect("Failed to open image");
    let mut modified_img = img.to_rgba8();
    let pixel = modified_img.get_pixel_mut(0, 0);
    let abs: u8 = inc.unsigned_abs();
    if inc > 0 {
        pixel[0] = pixel[0].saturating_add(abs);
    } else {
        pixel[0] = pixel[0].saturating_sub(abs);
    }
    modified_img.save(output_path).expect("Failed to save modified image");
    println!("Image modified and saved to {}", output_path);
}