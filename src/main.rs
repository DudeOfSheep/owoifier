pub mod owo;
pub mod setup;

use crate::owo::owo::Context;
use crate::setup::configuration::{
    env_config::{Config, ConfigType},
    parser,
};
use std::{collections::HashMap, env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if (1..=3).contains(&args.len()) {
        panic!("Expected 3 arguments, got {}", args.len() - 1);
    }

    let config: Config = parser::parse_args(&args)?;
    let mut pattern_map: HashMap<&str, &str> = HashMap::new();
    pattern_map.insert("l", "w");
    pattern_map.insert("r", "w");
    pattern_map.insert("v", "b");
    pattern_map.insert("puppy", "ouppy");
    pattern_map.insert("kitty", "kibdy");

    let context = Context::new(pattern_map);

    match config.get_format() {
        ConfigType::BARE(n) => {
            println!("{}", context.convert_string(n))
        } // Print converted text, easy_peasy.
        ConfigType::FILE(n) => {} // Read the file, error checking, then convert
    }

    Ok(())
}
