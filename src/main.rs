use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("{}", contents);
}

struct Config {
    filename: String,
}

fn parse_config(args: &[String]) -> Config{
    let filename = args[1].clone();

    Config { filename }
}
