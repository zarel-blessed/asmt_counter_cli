use std::env;
use std::fs;
use std::process;

fn main() {
     let args: Vec<String> = env::args().collect();

     let config = Config::build(&args).unwrap_or_else(|err| {
          println!("Problem parsing arguments: {err}");
          process::exit(1);
      });

     let contents = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
          println!("Error while reading file: {err}");
          process::exit(1);
     });

     println!("{}\nCharacter count: {}", contents, count_char(&contents, &config.query));
}

struct Config {
     query: String,
     file_path: String
}

impl Config {
     fn build(args: &[String]) -> Result<Config, &'static str> {
          if args.len() < 3 {
               return Err("Expected arguments are not provided!");
          }

          println!("{:?}", args);

          let query = args[1].clone();
          let file_path = args[2].clone();

          Ok(Config { query , file_path })
     }
} 
 
fn count_char(contents: &String, query: &String) -> u32 {
     let mut count = 0;

     for i in contents.to_lowercase().as_bytes().iter() {
          if (*i as char).to_string() == *query {
               count += 1;
          }
     }

     count
} 
