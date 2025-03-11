use clap::{App, Arg};

mod sha_demo;
use sha_demo::sha;

mod utils;
use utils::{file, img};


fn main() {
    let matches = App::new("ssc_proj")
    .version("1.0")
    .author("Stana Andrew")
    .about("Simple demo of digital signatures and hashes.")
    .arg(Arg::with_name("input_file")
        .help("Sets the input file to use")
        .required(true)
        .index(1))
    .get_matches();

    let input_path = matches.value_of("input_file").unwrap();
    let output_path = file::add_suffix_to_filename(input_path, "changed");
    let raw_data = img::extract_raw_pixels(input_path);
    let inc_raw_pixels = img::modify_first_pixel_red_inc(&*raw_data.0);
    img::save_image_from_raw_pixels(&*raw_data.0, raw_data.1,
                                    raw_data.2, output_path.as_str()
    ).expect("failed to save image");

    let original_hash = sha::get_pixels_hash(&*raw_data.0);
    println!("Original file SHA256: {}", original_hash);

    let changed_hash = sha::get_pixels_hash(&*inc_raw_pixels);
    println!("Modified file SHA256: {}", changed_hash);

    let dec_raw_pixels = img::modify_first_pixel_red_dec(&*inc_raw_pixels);
    let restored_hash = sha::get_pixels_hash(&*dec_raw_pixels);
    println!("Restored file SHA256: {}", restored_hash);

    println!("1st pixel red ch in each img: {} {} {}",
             raw_data.0[0], inc_raw_pixels[0], dec_raw_pixels[0]);
}
