use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config)-> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut src = String::new();
    f.read_to_string(&mut src)?;
    let mml = md_to_mml(src);
    println!("{}", mml.mml);
    let smf = mml_to_smf(mml);

    println!("{}", smf);
    Ok(())
}

struct MDMML {
    mml: String,
}

fn md_to_mml(src: String)->MDMML {
    let mml= MDMML{mml:src};
    mml
}

fn mml_to_smf(mml: MDMML)->u8{
    0
}
