extern crate mdmml_rust;

use std::env;
use std::process;

use mdmml_rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mdmml_rust::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
