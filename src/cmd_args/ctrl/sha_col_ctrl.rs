use clap::ArgMatches;
use crate::core::sha_demo::collision_demo_rng::collision_demo_rng;

pub fn sha_col_rng_ctrl(sub_m: &ArgMatches) {
    let low = sub_m.value_of("low").unwrap().parse::<i64>().unwrap();
    let high = sub_m.value_of("high").unwrap().parse::<i64>().unwrap();
    collision_demo_rng(low, high)
}
