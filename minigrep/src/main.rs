
use std::env;

use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> =  env::args().collect();
//   let    config =   parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Couldn't parse config{:?}",err);
        process::exit(1); 
    });
    println!("{:?}",args);
    println!("The query is {}", config.query);
    println!("The filename is {}", config.filename);
    if let Err(e) = minigrep::run(config){
        println!("Application error { }", e);
        process::exit(1);
    }
 
}

