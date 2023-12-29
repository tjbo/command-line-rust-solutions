use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("tjbo")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(false)
                .help("Show byte count"),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .value_name("CHARS")
                .takes_value(false)
                .help("Show character count"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .value_name("LINES")
                .takes_value(false)
                .help("Show line count"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .value_name("WORDS")
                .takes_value(false)
                .help("Show word count"),
        )
        .get_matches();

    let is_default = !matches.is_present("words")
        && !matches.is_present("lines")
        && !matches.is_present("chars")
        && !matches.is_present("bytes");

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: is_default == true && true || matches.is_present("lines"),
        words: is_default == true && true || matches.is_present("words"),
        bytes: is_default == true && true || matches.is_present("bytes"),
        chars: matches.is_present("chars"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
