use clap::ArgMatches;
use crate::cmd_args::ctrl::ctrl_clean::control_clean;
use crate::cmd_args::ctrl::sha_img_ctrl::sha_img_control;

pub fn control(arg_matches: ArgMatches) {
    match arg_matches.subcommand() {
        Some(("sha-img", sub_m)) => { sha_img_control(sub_m) },
        Some(("clean", _)) => { control_clean() },
        _ => { eprintln!("Unknown subcommand"); }
    }
}