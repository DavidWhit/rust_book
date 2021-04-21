use std::fs;
use std::process;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("not enough arguments"); changed to Result Return
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename})
    }
}

// Extracting Logic from main
// Returning errors from the run function
// trait object Box<dyn Error>  means the function will return a type that implements
// the Error trait also that we imported use std::error::Error
// empty successes or Ok(()) is idiomatic way to indicate that we're calling run
// for side effects only. It doesn't return a value we need.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
       println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
           results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
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

        let query= "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query,contents))
    }

    #[test]
    fn case_insensitive() {

        let query= "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust", "Trust me."], search_case_insensitive(query,contents))
    }

}
