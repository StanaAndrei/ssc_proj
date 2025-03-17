use clap::ArgMatches;
use crate::cmd_args::ctrl::ctrl_clean::control_clean;
use crate::cmd_args::ctrl::sha_img_ctrl::sha_img_control;
use crate::core::hmac_demo::chat_demo::chat_demo;
use crate::core::sha_demo::collision_demo_rng::collision_demo_rng;
use crate::core::sha_demo::collision_demo_str::collision_demo_str;


pub fn control(arg_matches: ArgMatches) {
    match arg_matches.subcommand() {
        Some(("sha-img", sub_m)) => { sha_img_control(sub_m) },
        Some(("hmac-chat", _)) => { chat_demo() },
        Some(("clean", _)) => { control_clean() },
        Some(("sha-col-rng", _)) => { collision_demo_rng() },
        Some(("sha-col-str", _)) => { collision_demo_str() },
        _ => { eprintln!("Unknown subcommand"); }
    }
}