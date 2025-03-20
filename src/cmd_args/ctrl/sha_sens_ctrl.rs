use clap::ArgMatches;
use crate::core::sha_demo::sensibility_demo::sensibility_demo;

pub fn sha_sens_ctrl(sub_m: &ArgMatches) {
    let s = sub_m.value_of("s").unwrap();
    let t = sub_m.value_of("t").unwrap();
    sensibility_demo(s, t)
}