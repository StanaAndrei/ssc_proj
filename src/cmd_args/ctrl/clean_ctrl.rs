use crate::utils::file;

pub fn control_clean() {
    println!("Cleaning...");
    match file::delete_files_except_tree_jpg() {
        Ok(_) => println!("Cleaned!"),
        Err(err) => eprintln!("Error: {}", err),
    }
}