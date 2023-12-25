use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("tjbo")
        .about("Rust head")
        .arg(
            Arg::with_name("file")
                .value_name("file")
                .help("Input file")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .help("print the num line instead of 10")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .help("print the first NUM of bytes")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: parse_positive_int(matches.value_of("lines").unwrap_or("")).unwrap_or(10),
        bytes: parse_positive_int(matches.value_of("bytes").unwrap_or("")).ok(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("run");
    Ok(())
}
