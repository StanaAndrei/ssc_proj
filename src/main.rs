use clap::{App, Arg, SubCommand};

mod utils;
mod sha_demo;
use crate::sha_demo::demo;
use crate::utils::file;

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
        demo::demo_non_obs(file);
    } else if let Some(_) = matches.subcommand_matches("clean") {
        println!("Cleaning...");
        match file::delete_files_except_tree_jpg() {
            Ok(_) => println!("Cleaned!"),
            Err(err) => eprintln!("Error: {}", err),
        }
    } else {
        println!("No valid command provided");
    }
}
