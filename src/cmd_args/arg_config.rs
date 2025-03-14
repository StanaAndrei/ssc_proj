use clap::{App, Arg, SubCommand};

const NAME: &str = "ssc_proj";
const VERSION: &str = "1.0";
const AUTHOR: &str = "Stana Andrew";
const ABOUT: &str = "\
Simple demo of digital signatures and hashes. \
";

pub fn get_app() -> App<'static> {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .subcommand(SubCommand::with_name("clean")
            .about("Cleans up files"))
        .subcommand(SubCommand::with_name("sha-img")
            .about("Subcommand with img demo")
            .arg(
                Arg::with_name("input")
                    .long("input")
                    .value_name("FILE")
                    .help("Input file to process")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("obs")
                    .long("obs")
                    .value_name("VALUE")
                    .help("Observable param(0 | 1)")
                    .takes_value(true)
                    .requires("input")
                    .validator(|v| {
                        if v == "0" || v == "1" {
                            Ok(())
                        } else {
                            Err(String::from("--obs = 0 | 1"))
                        }
                    })
            )
        ).subcommand(SubCommand::with_name("hmac-chat")
        .about("Subcommand with hmac-chat")
    )
}