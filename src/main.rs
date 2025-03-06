use clap::{App, Arg};

mod sha_demo;
use sha_demo::sha;

mod utils;
use utils::{file, img};


fn main() {
    let matches = App::new("ssc_proj")
    .version("1.0")
    .author("<NAME> <<EMAIL>>")
    .about("ssc_proj")
    .arg(Arg::with_name("input_file")
        .help("Sets the input file to use")
        .required(true)
        .index(1))
    .get_matches();

    let input_path = matches.value_of("input_file").unwrap();
    let output_path = file::add_suffix_to_filename(input_path, "changed");
    img::modify_1st_pixel_red_ch(input_path, output_path.as_str(), 1);

    let original_hash = sha::get_file_hash(input_path);
    println!("Original file SHA256: {}", original_hash);

    let modified_hash = sha::get_file_hash(output_path.as_str());
    println!("Modified file SHA256: {}", modified_hash);
}
