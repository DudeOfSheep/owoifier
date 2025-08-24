pub mod owo;
pub mod setup;

use crate::setup::configuration::{
    env_config::{Config, ConfigType},
    parser,
};
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if (1..=3).contains(&args.len()) {
        panic!("Expected 3 arguments, got {}", args.len() - 1);
    }

    let config: Config = parser::parse_args(&args)?;

    match config.get_format() {
        ConfigType::BARE(n) => {} // Print converted text, easy_peasy.
        ConfigType::FILE(n) => {} // Read the file, error checking, then convert
    }

    Ok(())
}
