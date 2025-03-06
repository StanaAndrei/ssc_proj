pub fn modify_image_slightly(input_path: &str, output_path: &str) {
    let img = image::open(input_path).expect("Failed to open image");
    let mut modified_img = img.to_rgba8();
    let pixel = modified_img.get_pixel_mut(0, 0);
    // Modify the pixel slightly (adjust the red channel by 1)
    // This change is imperceptible to humans
    pixel[0] = pixel[0].saturating_add(1);
    modified_img.save(output_path).expect("Failed to save modified image");
    println!("Image modified and saved to {}", output_path);
}