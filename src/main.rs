use clap::{App, Arg, SubCommand};

mod sha_demo;
use sha_demo::sha;

mod utils;
use utils::{file, img};

fn demo(input_path: &str) {
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

fn main() {
    let matches = App::new("ssc_proj")
    .version("1.0")
    .author("Stana Andrew")
    .about("Simple demo of digital signatures and hashes.")
    .arg(
        Arg::with_name("input")
            .long("input")
            .value_name("FILE")
            .help("Input file to process")
            .takes_value(true)
    ).subcommand(SubCommand::with_name("clean")
        .about("Cleans up resources"))
    .get_matches();

    if let Some(file) = matches.value_of("input") {
        println!("Processing file: {}", file);
        demo(file);
    } else if let Some(_) = matches.subcommand_matches("clean") {
        println!("Cleaning...");
    } else {
        println!("No valid command provided");
    }
}
