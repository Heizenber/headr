use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Roman Popov <example@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("Files")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
                .required(true)
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("Count")
                .help("Number of lines")
                .short('n')
                .long("count")
                .default_value("10")
                .takes_value(true)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("Bytes")
                .help("Number of bytes")
                .short('b')
                .long("bytes")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.value_of_lossy("lines").unwrap().parse::<usize>()?,
        bytes: Some(matches.value_of_lossy("bytes").unwrap().parse::<usize>()?),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    unimplemented!();
}