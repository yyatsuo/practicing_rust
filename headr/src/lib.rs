use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn parse_positive_int(val: &str) -> MyResult<usize> {
    println!("Got {}", &val);
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3は正の整数なのでOK
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 数値以外の文字列はエラー
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0の場合もエラー
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Yusuke Yatsuo <yyatsuo@gmail.com>")
        .about("Rust head")
        .arg(Arg::with_name("files")
            .value_name("FILE(s)")
            .help("Input file(s) [default: -]")
        )
        .arg(Arg::with_name("bytes")
            .help("Number of bytes")
            .short("c")
            .long("bytes")
            .value_name("BYTES")
            .takes_value(true))
        .arg(Arg::with_name("lines")
            .help("Number of lines [default:10]")
            .short("n")
            .long("lines")
            .value_name("LINES")
            .takes_value(true))
        .get_matches();

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.occurrences_of::<usize>("lines"),
        bytes: matches.occurrences_of::<usize>("bytes"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}