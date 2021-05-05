use std::fs;
use std::process;
use std::error::Error;
use std::env;

    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool
    }

    impl Config {
        // original 12 signature
        // pub fn new(args: &[String]) -> Result<Config, &str> {
        //https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html
        // the lifetime elision mention and why static in the result
        // chp13 fixes
        pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
            // if args.len() < 3 {
            //     // panic!("not enough arguments"); changed to Result Return
            //     return Err("not enough arguments")
            // }

            // 13
            // the binary name
            args.next();

            // next is the first arg
            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string ")
            };

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name")
            };

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config { query, filename, case_sensitive })
        }
    }

// Extracting Logic from main
// Returning errors from the run function
// trait object Box<dyn Error>  means the function will return a type that implements
// the Error trait also that we imported use std::error::Error
// empty successes or Ok(()) is idiomatic way to indicate that we're calling run
// for side effects only. It doesn't return a value we need.
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("{}", line);
        }

        Ok(())
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        // let mut results = Vec::new();
        // for line in contents.lines() {
        //     if line.contains(query) {
        //        results.push(line);
        //     }
        // }
        // results
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        // let mut results = Vec::new();

        // let query = query.to_lowercase();
        // for line in contents.lines() {
        //     if line.to_lowercase().contains(&query) {
        //         results.push(line);
        //     }
        // }
        // results

        // chp 13 updates
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
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
Pick three.";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents))
        }

        #[test]
        fn case_insensitive() {
            let query = "rUsT";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

            assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
        }
    }
