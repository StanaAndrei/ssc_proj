use crate::sha_demo::sha;
use crate::utils::{file, img};

pub fn demo_non_obs(input_path: &str) {
    let output_path = file::add_suffix_to_filename(input_path, "mod_non_obs");
    let raw_data = img::extract_raw_pixels(input_path);
    let d: i16 = 1;
    let inc_raw_pixels = img::modify_pixels(
        &*raw_data.0, raw_data.1.try_into().unwrap(),
        1, (0, 0, 0, 0), d
    );


    let original_hash = sha::get_pixels_hash(&*raw_data.0);
    println!("Original file SHA256: {}", original_hash);

    img::save_image_from_raw_pixels(&inc_raw_pixels, raw_data.1,
                                    raw_data.2, output_path.as_str()
    ).expect("failed to save image");
    let changed_hash = sha::get_pixels_hash(&*inc_raw_pixels);
    println!("Modified file SHA256: {}", changed_hash);


    let dec_raw_pixels = img::modify_pixels(
        &*inc_raw_pixels, raw_data.1.try_into().unwrap(),
        1, (0, 0, 0, 0), -d
    );
    let restored_hash = sha::get_pixels_hash(&*dec_raw_pixels);
    println!("Restored file SHA256: {}", restored_hash);

    println!("1st pixel red ch in each img: {} {} {}",
             raw_data.0[0], inc_raw_pixels[0], dec_raw_pixels[0]);
}

pub fn demo_obs(input_path: &str) {
    let output_path = file::add_suffix_to_filename(input_path, "mod_obs");
    let raw_data = img::extract_raw_pixels(input_path);
    let half_width: usize = (raw_data.1 as usize) / 2;
    let half_height: usize = (raw_data.2 as usize)  / 2;
    let offset_width: usize = half_width / 2;
    let offset_height: usize = half_height / 2;
    let d: i16 = 100;
    let inc_raw_pixels = img::modify_pixels(
        &*raw_data.0, raw_data.1.try_into().unwrap(), 1,
        (offset_width, offset_height, offset_width + half_width, offset_height + half_height), d
    );


    let original_hash = sha::get_pixels_hash(&*raw_data.0);
    println!("Original file SHA256: {}", original_hash);

    img::save_image_from_raw_pixels(&inc_raw_pixels, raw_data.1,
                                    raw_data.2, output_path.as_str()
    ).expect("failed to save image");
    let changed_hash = sha::get_pixels_hash(&*inc_raw_pixels);
    println!("Modified file SHA256: {}", changed_hash);

    let dec_raw_pixels = img::modify_pixels(
        &*inc_raw_pixels, raw_data.1.try_into().unwrap(), 1,
        (offset_width, offset_height, offset_width + half_width, offset_height + half_height), -d
    );
    let restored_hash = sha::get_pixels_hash(&*dec_raw_pixels);
    println!("Restored file SHA256: {}", restored_hash);
}