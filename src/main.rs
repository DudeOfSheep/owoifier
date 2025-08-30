pub mod owo;
pub mod setup;

use crate::owo::owo::Owoifier;
use crate::setup::configuration::env_config::{Config, ConfigType};
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if 2 > args.len() {
        panic!(
            "Expected 1-3 arguments, got {}! owoifier requires arguments \'string/filepath:\"\" intensity:1..3 return_output:bool\'",
            args.len() - 1
        );
    }

    let config: Config = Config::parse_args(&args)?;

    match config.get_format() {
        ConfigType::BARE(n) => run(n, &config),
        ConfigType::FILE(n) => run(n, &config), // Read the file, error checking, then convert
    }

    Ok(())
}

fn run(text: &String, config: &Config) {
    let mut file = fs::read_to_string("src\\pattern_map").expect("Failed to read pattern file");

    let translator = Owoifier::new(if let Ok(n) = config.get_intensity_pattern(&mut file) {
        n
    } else {
        panic!("Pattern file could not be parsed!")
    });

    println!("{}", translator.convert_string(&text))
}
