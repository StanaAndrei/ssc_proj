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
                    .short('i')
                    .value_name("FILE")
                    .help("Input file to process")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("obs")
                    .long("obs")
                    .short('o')
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
        )
        .subcommand(SubCommand::with_name("hmac-chat")
            .about("Subcommand with hmac-chat")
        )
        .subcommand(SubCommand::with_name("sha-col-rng")
            .about("Test sha256 collisions in a range")
            .arg(Arg::with_name("low")
                .help("Lower bound of the range")
                .takes_value(true)
                .default_value("-999999")
                .validator(|s| s.parse::<i64>()
                    .map(|_| ())
                    .map_err(|_| String::from("low must be an integer"))
                )
            )
            .arg(Arg::with_name("high")
                .help("Upper bound of the range")
                .takes_value(true)
                .default_value("1000000")
                .validator(|s| s.parse::<i64>()
                    .map(|_| ())
                    .map_err(|_| String::from("high must be an integer"))
                )
            )
        )
        .subcommand(SubCommand::with_name("sha-col-str")
            .about("Test sha256 collisions with random strings")
        )
        .subcommand(SubCommand::with_name("sha-sens")
            .about("Test sha256 sensibility")
            .arg(Arg::with_name("s")
                .takes_value(true)
                .required(true)
                .help("1st string"))
            .arg(Arg::with_name("t")
                .takes_value(true)
                .required(true)
                .help("2nd string"))
        )
}