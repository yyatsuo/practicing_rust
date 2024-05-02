use std::fmt::{self, Formatter, Display};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "files:{:?}\nnumber_lines:{}\nnumber_nonblank_lines:{}",
        self.files, self.number_lines, self.number_nonblank_lines
    )
    }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Yusuke Yatsuo <yyatsuo@gmail.com>")
        .about("Rust catr")
        .arg(
            Arg::with_name("file")
            .value_name("FILE")
            .help("Input files")
            .multiple(true)
            .default_value("-"),
        )
        .arg(
            Arg::with_name("number_line")
            .short("n")
            .long("--number")
            .help("number all output lines")
            .takes_value(false)
            .conflicts_with("number_nonblank_lines")
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
            .short("b")
            .long("--number-nonblank")
            .help("number nonempty output lines")
            .takes_value(false)
        )
        .get_matches();

    Ok(Config{
        files:  matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number_line"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open{}: {}", filename, err),
            Ok(file) => {
                //println!("Opened {}", filename);
                let mut line_num = 1;
                for line in file.lines() {
                    let linestr = line.unwrap();
                    if config.number_lines {
                        print!("     {}\t",line_num);
                        line_num += 1;
                    }
                    else if config.number_nonblank_lines {
                        if linestr != "" {
                            print!("     {}\t",line_num);
                            line_num += 1;
                        }
                    }
                    println!("{}", linestr);
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}