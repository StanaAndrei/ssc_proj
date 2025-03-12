use crate::sha_demo::sha;
use crate::utils::{file, img};

pub fn demo_non_obs(input_path: &str) {
    let output_path = file::add_suffix_to_filename(input_path, "changed");
    let raw_data = img::extract_raw_pixels(input_path);
    let inc_raw_pixels = img::modify_pixels(
        &*raw_data.0, raw_data.1.try_into().unwrap(),
        0, (0, 0, 0, 0), 1
    );
    img::save_image_from_raw_pixels(&*raw_data.0, raw_data.1,
                                    raw_data.2, output_path.as_str()
    ).expect("failed to save image");

    let original_hash = sha::get_pixels_hash(&*raw_data.0);
    println!("Original file SHA256: {}", original_hash);

    let changed_hash = sha::get_pixels_hash(&*inc_raw_pixels);
    println!("Modified file SHA256: {}", changed_hash);

    let dec_raw_pixels = img::modify_pixels(
        &*inc_raw_pixels, raw_data.1.try_into().unwrap(),
        0, (0, 0, 0, 0), -1
    );
    let restored_hash = sha::get_pixels_hash(&*dec_raw_pixels);
    println!("Restored file SHA256: {}", restored_hash);

    println!("1st pixel red ch in each img: {} {} {}",
             raw_data.0[0], inc_raw_pixels[0], dec_raw_pixels[0]);
}