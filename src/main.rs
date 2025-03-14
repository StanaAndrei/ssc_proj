mod utils;
mod sha_demo;
mod cmd_args;

use crate::sha_demo::demo;
use crate::utils::file;
use crate::cmd_args::arg_config;

fn main() {
    let app = arg_config::get_app();

    match app.get_matches().subcommand() {
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
