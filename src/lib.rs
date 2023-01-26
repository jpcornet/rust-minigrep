use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::env;
use std::error::Error;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let f = File::open(&config.file_path)?;
    let reader = BufReader::new(f);
    let stdin = io::stdin().lock();
    let lines2 = stdin.lines();
    let lines = reader.lines();
    let results = lines2
        .map(|l| l.unwrap())
        .filter(|l| l.contains(&config.query));
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
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
                            return Err("Need query string after -e");
                        }
                    },
                    _ => {
                        query = nextarg;
                        break;
                    },
                }
            } else {
                return Err("Need search pattern");
            }
        }
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Need input file"),
        };
        Ok(Config { query, file_path, ignore_case: ignore_case || env::var("IGNORE_CASE").is_ok() })
    }
}

//pub fn search<'a>(query: &'a str, lines: std::io::Lines<BufRead>) -> impl Iterator<Item = &'a str> + 'a {
//    lines.filter(move |line| line.unwrap().contains(query))
//}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(Some("safe, fast, productive."), search(query, contents).next());
//     }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let mut result = search_case_insensitive(query, contents);
        assert_eq!(Some("Rust:"), result.next());
        assert_eq!(Some("Trust me."), result.next());
        assert_eq!(None, result.next());
    }
}
