pub mod owo;
pub mod setup;

use crate::owo::owo::Owoifier;
use crate::setup::configuration::env_config::{Config, ConfigType};
use std::{env, error::Error, fs, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if 2 > args.len() {
        panic!(
            "Expected 1-3 arguments, got {}! owoifier requires arguments \'string/filepath:\"\" intensity:1..3 return_output:bool\'",
            args.len() - 1
        );
    }

    let config = Config::parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Issue parsing arguments {}", err);
        process::exit(1)
    });

    let res: String = match config.get_format() {
        ConfigType::BARE(n) => run(n, &config)?,
        ConfigType::FILE(n) => {
            println!("{}", &args[1][2..args[1].len() - 4]);
            let res = run(n, &config)?;
            fs::write(
                format!("{}Owoified.txt", &args[1][2..args[1].len() - 4]),
                &res,
            )?;
            res
        } // Read the file, error checking, then convert
    };

    if *config.may_return() {
        println!("{}", res);
    }

    Ok(())
}

fn run(text: &String, config: &Config) -> Result<String, Box<dyn Error>> {
    let mut file = fs::read_to_string("src\\pattern_map.txt").expect("Failed to read pattern file");

    let translator = Owoifier::new(if let Ok(n) = config.get_intensity_pattern(&mut file) {
        n
    } else {
        panic!("Pattern file could not be parsed!")
    });

    Ok(format!("{}", translator.convert_string(&text)))
}
