pub mod owo;
pub mod setup;

use crate::owo::owo::Owoifier;
use crate::setup::configuration::env_config::{Config, ConfigType};
use std::{collections::HashMap, env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if (1..=3).contains(&args.len()) {
        panic!(
            "Expected 3 arguments, got {}! owoifier requires arguments \'string/filepath:\"\" intensity:1..3 return_output:bool\'",
            args.len() - 1
        );
    }

    let config: Config = Config::parse_args(&args)?;
    let mut pattern_map: HashMap<&str, &str> = HashMap::new(); // Move this to file parse

    let mut buf = String::new();

    let context = Owoifier::new(pattern_map);

    match config.get_format() {
        ConfigType::BARE(n) => {
            println!("{}", context.convert_string(n))
        } // Print converted text, easy_peasy.
        ConfigType::FILE(n) => run(n.clone()), // Read the file, error checking, then convert
    }

    Ok(())
}

fn run(text: String) {}
