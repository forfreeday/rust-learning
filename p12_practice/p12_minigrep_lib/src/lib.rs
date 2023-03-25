use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text: \n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    // for 写法
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // 等价写法
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{ query, filename })
    }
}
