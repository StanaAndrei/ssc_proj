mod utils;
mod cmd_args;
mod core;

extern crate num_cpus;

use crate::cmd_args::arg_config;
use crate::cmd_args::ctrl::main_ctrl::control;

fn main() {
    let app = arg_config::get_app();
    let arg_matches = app.get_matches();
    control(arg_matches);
}
