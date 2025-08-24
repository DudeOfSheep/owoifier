pub mod config;

use config::config::{Config, ConfigType};
use std::{env, error::Error, fmt::Display, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if (1..=3).contains(&args.len()) {
        panic!("Expected 3 arguments, got {}", args.len() - 1);
    }

    let config: Config = parse_args(&args)?;

    match config.format {
        ConfigType::BARE(n) => {} // Print converted text, easy_peasy.
        ConfigType::FILE(n) => {} // Read the file, error checking, then convert
    }

    Ok(())
}

// TODO: map out how an owoifier should work
trait Owoifier {
    fn owoify(text: &str) -> &str;
}

#[derive(Debug)]
struct ParseError {
    path: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = &self.path;
        write!(f, "Issue at {path}")
    }
}

impl Error for ParseError {}

// TODO: Make ConfigType::File a filepath type instead of a string
fn parse_args(args: &Vec<String>) -> Result<Config, Box<dyn Error>> {
    let format: ConfigType = if "./" == &args[1].trim()[0..1] {
        // Could do an if let here, maybe
        match fs::exists(&args[1]) {
            Ok(true) => ConfigType::FILE(args[1].clone()),
            Ok(false) => {
                return Err(Box::new(ParseError {
                    path: String::from("File not found"),
                }));
            }
            Err(n) => return Err(Box::new(n)),
        }
    } else {
        ConfigType::BARE(args[1].clone())
    };

    let intensity: i32 = args[2].trim().parse::<i32>()?;
    let return_output: bool = args[3].trim().parse::<bool>()?;

    Ok(Config::new(format, intensity, return_output)?)
}
