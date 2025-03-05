use clap::{App, Arg};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read};

fn get_file_hash(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    format!("{:x}", result)
}

fn modify_image_slightly(input_path: &str, output_path: &str) {
    let img = image::open(input_path).expect("Failed to open image");
    let mut modified_img = img.to_rgba8();
    let pixel = modified_img.get_pixel_mut(0, 0);
    // Modify the pixel slightly (adjust the red channel by 1)
    // This change is imperceptible to humans
    pixel[0] = pixel[0].saturating_add(1);
    modified_img.save(output_path).expect("Failed to save modified image");
    println!("Image modified and saved to {}", output_path);
}

fn add_changed_to_filename(path: &str) -> String {
    let path = std::path::Path::new(path);

    let dir = path.parent().unwrap_or(std::path::Path::new(""));
    let filename = path.file_name().unwrap_or_default().to_str().unwrap_or("");

    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            return dir.join(format!("{}_changed.{}", stem, ext))
                .to_str()
                .unwrap_or(path.to_str().unwrap_or(""))
                .to_string();
        }
    }

    dir.join(format!("{}_changed", filename))
        .to_str()
        .unwrap_or(path.to_str().unwrap_or(""))
        .to_string()
}

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
    let output_path = add_changed_to_filename(input_path);
    modify_image_slightly(input_path, output_path.as_str());

    let original_hash = get_file_hash(input_path);
    println!("Original file SHA256: {}", original_hash);

    let modified_hash = get_file_hash(output_path.as_str());
    println!("Modified file SHA256: {}", modified_hash);
}
