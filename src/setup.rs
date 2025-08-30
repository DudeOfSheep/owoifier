pub mod configuration {
    pub mod env_config {
        use std::{collections::HashMap, error::Error, fs};

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

                match args.len() as i32 {
                    1 => Ok(Config::new(format, 1, true)?),
                    2 => Ok(Config::new(format, args[2].trim().parse::<i32>()?, true)?),
                    3 => Ok(Config::new(
                        format,
                        args[2].trim().parse::<i32>()?,
                        args[3].trim().parse::<bool>()?,
                    )?),
                    _ => panic!(
                        "Expected 1-3 arguments, got {}! owoifier requires arguments \'string/filepath:\"\" intensity:1..3 return_output:bool\'",
                        args.len() - 1
                    ),
                }
            }

            pub fn get_format(&self) -> &ConfigType {
                &self.format
            }

            // Takes the intensity value from
            pub fn get_intensity_pattern<'a>(
                &self,
                file: &'a mut String,
            ) -> Result<HashMap<&'a str, &'a str>, Box<dyn Error>> {
                let mut buffer = HashMap::new();

                if let Ok(n) = file
                    .split_inclusive('}')
                    .collect::<Vec<&str>>()
                    .get((&self.intensity - 1) as usize)
                    .ok_or_else(|| false)
                {
                    for item in n.split_inclusive("\n") {
                        let (k, v) = if let Some(n) = item.trim().rsplit_once('|') {
                            n
                        } else {
                            continue;
                        };

                        buffer.insert(k, v);
                    }
                };

                Ok(buffer)
            }
        }
    }
}
