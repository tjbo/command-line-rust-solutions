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
    // let matches = App::new("wcr")
    //     .version("0.1.0")
    //     .author("tjbo")
    //     .about("Rust wc")
    //     .get_matches()

    Ok(Config {
        files: vec![],
        lines: false,
        words: false,
        bytes: false,
        chars: false,
    })
}
