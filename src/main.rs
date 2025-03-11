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
    img::get_image_pixel_hash(&*raw_data.0);
    img::save_image_from_raw_pixels(&*raw_data.0, raw_data.1,
                                    raw_data.2, output_path.as_str()
    ).expect("failed to save image");

    let original_hash = sha::get_file_hash(input_path);
    println!("Original file SHA256: {}", original_hash);

    let modified_hash = sha::get_file_hash(output_path.as_str());
    println!("Modified file SHA256: {}", modified_hash);
}
