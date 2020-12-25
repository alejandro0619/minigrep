mod lib;
use std::env;
use std::process;
use lib::config::ArgsConfig;
use lib::config::run;
// main:
fn main(){
  let args: Vec<String> = env::args().collect();
  let config = ArgsConfig::parse_config(&args)
    .unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments {}", err);
      process::exit(1);
    });
    if let Err(e) = run(config) {
      eprintln!("App error {}", e);
      process::exit(1);
    }
}