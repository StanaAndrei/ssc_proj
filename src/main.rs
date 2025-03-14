mod utils;
mod sha_demo;
mod cmd_args;

use crate::cmd_args::arg_config;
use crate::cmd_args::ctrl::main_ctrl::control;

fn main() {
    let app = arg_config::get_app();
    let arg_matches = app.get_matches();
    control(arg_matches);
}
