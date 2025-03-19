use clap::ArgMatches;
use crate::cmd_args::ctrl::ctrl_clean::control_clean;
use crate::cmd_args::ctrl::sha_img_ctrl::sha_img_control;
use crate::core::hmac_demo::chat_demo::chat_demo;
use crate::core::sha_demo::collision_demo_rng::collision_demo_rng;
use crate::core::sha_demo::collision_demo_str::collision_demo_str;
use crate::core::sha_demo::sensibility_demo::sensibility_demo;


pub fn control(arg_matches: ArgMatches) {
    match arg_matches.subcommand() {
        Some(("sha-img", sub_m)) => { sha_img_control(sub_m) },
        Some(("hmac-chat", _)) => { chat_demo() },
        Some(("clean", _)) => { control_clean() },
        Some(("sha-col-rng", sub_m)) => {
            let low = sub_m.value_of("low").unwrap().parse::<i64>().unwrap();
            let high = sub_m.value_of("high").unwrap().parse::<i64>().unwrap();
            collision_demo_rng(low, high)
        },
        Some(("sha-col-str", _)) => { collision_demo_str() },
        Some(("sha-sens", sub_m)) => {
            let s = sub_m.value_of("s").unwrap();
            let t = sub_m.value_of("t").unwrap();
            sensibility_demo(s, t)
        },
        _ => { eprintln!("Unknown subcommand"); }
    }
}