use std::error::Error;
use std::fs::{self};

use std::env;
// Arg struct:
pub struct ArgsConfig {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool
}

 impl ArgsConfig {
  pub fn parse_config(mut args: env::Args) -> Result<ArgsConfig, &'static str>{
    args.next();
    let query = match args.next(){
      Some(args) => args,
      None => return Err("Didn't get a query string")
    };
    let filename = match args.next(){
      Some(args) => args,
      None => return Err("Didn't get any filename")
    };

      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
      Ok(ArgsConfig {
         query, 
         filename, 
         case_sensitive 
      })
    }
  
 }

 pub fn run(config: ArgsConfig) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &content)
  } else{
    search_case_insensitive(&config.query, &content)
  };

  for line in results {
    println!("{}", line);
  }
  Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(| line | line.contains(query))
    .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  contents
  .lines()
  .filter(|line| {
    line.to_lowercase().contains(&query)
  })
  .collect()
}

