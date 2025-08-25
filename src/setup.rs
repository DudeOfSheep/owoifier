pub mod configuration {
    pub mod env_config {
        use std::{error::Error, fs};

        #[derive(Debug)]
        pub enum ConfigType {
            BARE(String),
            FILE(String),
        }

        pub struct Config {
            format: ConfigType,
            intensity: i32,
            return_output: bool,
        }

        impl Config {
            pub fn new(
                format: ConfigType,
                intensity: i32,
                return_output: bool,
            ) -> Result<Config, &'static str> {
                if (1..=3).contains(&intensity) {
                    Ok(Config {
                        format,
                        intensity,
                        return_output,
                    })
                } else {
                    Err("Intensity must be within range 1 to 3")
                }
            }

            pub fn parse_args(args: &Vec<String>) -> Result<Config, Box<dyn Error>> {
                let format: ConfigType = if args[1].trim().starts_with("./") {
                    //Note, this is still bad. explicitely ask if the input is a file.
                    ConfigType::FILE(fs::read_to_string(&args[1][2..])?)
                } else {
                    ConfigType::BARE(args[1].clone())
                };

                let intensity: i32 = args[2].trim().parse::<i32>()?;
                let return_output: bool = args[3].trim().parse::<bool>()?;

                Ok(Config::new(format, intensity, return_output)?)
            }

            pub fn get_format(&self) -> &ConfigType {
                &self.format
            }
        }
    }
}
