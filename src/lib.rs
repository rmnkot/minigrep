use std::env;
use std::error::Error;
use std::fs;

const ARGS_LENGTH: usize = 2;
const IGNORE_CASE_VAR: &str = "IGNORE_CASE";
const IGNORE_CASE_FLAG: &str = "--ignore-case";

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, String> {
        if args.len() < 1 + ARGS_LENGTH {
            return Err("not enough arguments".to_string());
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case_arg = args.get(3);

        let ignore_case = match ignore_case_arg {
            Some(arg) => {
                let parts: Vec<&str> = arg.split("=").collect();

                if parts[0] != IGNORE_CASE_FLAG {
                    return Err(format!(
                        "expected \"{}\" but get {}",
                        IGNORE_CASE_FLAG, parts[0]
                    ));
                }

                parts[1].parse::<bool>().unwrap_or(false)
            }
            None => env::var(IGNORE_CASE_VAR).is_ok(),
        };

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
