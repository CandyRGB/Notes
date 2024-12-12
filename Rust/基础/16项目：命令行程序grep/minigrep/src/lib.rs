use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(config.query, &contents)
    } else {
        search_case_insensitive(config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config {query, filename, case_sensitive})
    }
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
    let query = query.to_lowercase();

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
    fn one_result() {
        let query = "小姐";
        let contents = "\
角色：
妖精爱莉，爱莉希雅，粉色妖精小姐。
三人。";
        assert_eq!(vec!["妖精爱莉，爱莉希雅，粉色妖精小姐。"],
                    search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "to";
        let contents = "\
To you:
Yes.
Me too.";
        assert_eq!(vec!["Me too."],
        search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "to";
        let contents = "\
To you:
Yes.
Me too.";
        assert_eq!(vec!["To you:", "Me too."],
        search_case_insensitive(query, contents));
    }
}