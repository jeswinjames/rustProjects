use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_name)?;
    let results = if config.case_sensitive {
        search(&config.query, &content)
    }else{
        search_case_insensitive(&config.query, &content)
    };

    for line in results{
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
}


pub struct Config{
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Less than 2 arguments");
        } 
        let query = args[1].clone();
        let file_name = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query, file_name, case_sensitive})
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_senstive(){
        let query = "duct";
        let content = "\
Rust: 
safe, fast, productive.
Pick three
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insenstive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three
Duct tape
Trust me";

    assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(&query, &content));
    }
}