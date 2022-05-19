use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}