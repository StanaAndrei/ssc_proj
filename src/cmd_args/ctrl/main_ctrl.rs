use clap::ArgMatches;
use crate::cmd_args::ctrl::clean_ctrl::control_clean;
use crate::cmd_args::ctrl::sha_img_ctrl::sha_img_control;
use crate::cmd_args::ctrl::sha_col_ctrl::{sha_col_rng_ctrl};
use crate::cmd_args::ctrl::sha_sens_ctrl::sha_sens_ctrl;
use crate::core::hmac_demo::chat_demo::chat_demo;
use crate::core::sha_demo::collision_demo_str::collision_demo_str;

pub fn control(arg_matches: ArgMatches) {
    match arg_matches.subcommand() {
        Some(("sha-img", sub_m)) => { sha_img_control(sub_m) },
        Some(("hmac-chat", _)) => { chat_demo() },
        Some(("clean", _)) => { control_clean() },
        Some(("sha-col-rng", sub_m)) => { sha_col_rng_ctrl(sub_m) },
        Some(("sha-col-str", _)) => { collision_demo_str() },
        Some(("sha-sens", sub_m)) => { sha_sens_ctrl(sub_m) },
        _ => { eprintln!("Unknown subcommand"); }
    }
}