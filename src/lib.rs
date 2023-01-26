use std::fs;
use std::env;
use std::error::Error;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        let query = config.query.to_lowercase();
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
    } else {
        contents
            .lines()
            .filter(|line| line.contains(&config.query))
    };

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
