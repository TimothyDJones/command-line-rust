use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    squeeze_blank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut curr_line = 0;
                let mut multi_blank_flag = false;
                for line_result in file.lines() {
                    let line = line_result?;
                    if config.squeeze_blank_lines {
                        if line.is_empty() && multi_blank_flag {
                            continue
                        } else if line.is_empty() {
                            multi_blank_flag = true
                        } else if !line.is_empty() {
                            multi_blank_flag = false
                        }
                    }
                    if config.number_lines {
                        curr_line += 1;
                        println!("{:>6}\t{}", curr_line, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            curr_line += 1;
                            println!("{:>6}\t{}", curr_line, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Tim Jones <tdjones74021@yahoo.com>")
        .about("Rust implementation of Unix 'cat'.")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("squeeze_blank")
                .short("s")
                .long("squeeze-blank")
                .help("Suppress repeated multiple blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
        squeeze_blank_lines: matches.is_present("squeeze_blank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
