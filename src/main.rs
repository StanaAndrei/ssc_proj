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
    .subcommand(SubCommand::with_name("clean")
        .about("Cleans up resources"))
    .subcommand(SubCommand::with_name("sha-img")
        .about("Subcommand with input and obs arguments")
        .arg(
            Arg::with_name("input")
                .long("input")
                .value_name("FILE")
                .help("Input file to process")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("obs")
                .long("obs")
                .value_name("VALUE")
                .help("Observable param(0 | 1)")
                .takes_value(true)
                .requires("input")
                .validator(|v| {
                    if v == "0" || v == "1" {
                        Ok(())
                    } else {
                        Err(String::from("--obs = 0 | 1"))
                    }
                })
        )
    )
    .get_matches();

    // Check which subcommand was used
    match matches.subcommand() {
        Some(("sha-img", sub_m)) => {
            if let (Some(file), Some(obs)) = (sub_m.value_of("input"), sub_m.value_of("obs")) {
                if obs.parse::<u8>().unwrap() == 0 {
                    demo::demo_non_obs(file);
                } else {
                    demo::demo_obs(file);
                }
            }
        },
        Some(("clean", _)) => {
            println!("Cleaning...");
            match file::delete_files_except_tree_jpg() {
                Ok(_) => println!("Cleaned!"),
                Err(err) => eprintln!("Error: {}", err),
            }
        },
        _ => { eprintln!("Unknown subcommand"); }
    }
}
