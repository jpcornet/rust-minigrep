use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::env;
use std::error::Error;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = config.reader.lines();

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, input) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, input) {
            println!("{line}");
        }
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub reader: Box<dyn BufRead>,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        // skip program name
        args.next();
        let mut ignore_case = false;
        let query;
        loop {
            if let Some(nextarg) = args.next() {
                match nextarg.as_str() {
                    "-i" => ignore_case = true,
                    "-h" => {
                        eprintln!("Usage: minigrep [-i] [-e] pattern file");
                        process::exit(0);
                    },
                    "-e" => {
                        if let Some(q) = args.next() {
                            query = q;
                            break;
                        } else {
                            return Err("Need query string after -e".to_string());
                        }
                    },
                    _ => {
                        query = nextarg;
                        break;
                    },
                }
            } else {
                return Err("Need search pattern".to_string());
            }
        }
        let reader: Box<dyn BufRead> = match args.next() {
            Some(arg) => {
                let f = File::open(&arg);
                if let Err(e) = f {
                    let formattederr = format!("Cannot open {}: {}", arg, e);
                    return Err(formattederr);
                }
                Box::new(BufReader::new(f.unwrap()))
            },
            None => Box::new(io::stdin().lock()),
        };
        Ok(Config { query, reader, ignore_case: ignore_case || env::var("IGNORE_CASE").is_ok() })
    }
}

pub fn search<'a>(query: &'a str, lines: Lines<Box<dyn BufRead>>) -> impl Iterator<Item = String> + 'a {
    lines
        .map(|l| l.unwrap())
        .filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(query: &str, lines: Lines<Box<dyn BufRead>>) -> impl Iterator<Item = String> + 'a {
    let query = query.to_lowercase();
    lines
        .map(|l| l.unwrap())
        .filter(move |line| line.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_bufread<'a>(contents: &'a str) -> Box<dyn BufRead + 'a> {
        let bytes = contents.as_bytes();
        Box::new(bytes)
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let input = to_bufread(contents);
        assert_eq!(Some("safe, fast, productive.".to_string()), search(query, input.lines()).next());
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let input = to_bufread(contents);
        let mut result = search_case_insensitive(query, input.lines());
        assert_eq!(Some("Rust:".to_string()), result.next());
        assert_eq!(Some("Trust me.".to_string()), result.next());
        assert_eq!(None, result.next());
    }
}
