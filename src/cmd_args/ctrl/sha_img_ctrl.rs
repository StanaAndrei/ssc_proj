use clap::ArgMatches;
use crate::sha_demo::demo;

pub fn sha_img_control(sub_m: &ArgMatches) {
    if let (Some(file), Some(obs)) = (sub_m.value_of("input"), sub_m.value_of("obs")) {
        if obs.parse::<u8>().unwrap() == 0 {
            demo::demo_non_obs(file);
        } else {
            demo::demo_obs(file);
        }
    }
}