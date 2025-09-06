pub mod configuration {
    pub mod env_config {
        use std::{
            collections::HashMap,
            error::{self, Error},
            fs,
        };

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

            pub fn parse_args(
                mut args: impl Iterator<Item = String>,
            ) -> Result<Config, Box<dyn Error>> {
                args.next();

                let format: ConfigType = match args.next() {
                    Some(n) => {
                        if let Ok(x) = fs::read_to_string(&n) {
                            ConfigType::FILE(x)
                        } else {
                            ConfigType::BARE(n)
                        }
                    }
                    None => {
                        panic!("Missing required text argument!"); // Find a way to construct an error. ffs.
                    }
                };

                // Ugly
                let intensity: i32 = if let Some(n) = args.next() {
                    n.parse::<i32>()?
                } else {
                    1
                };
                let return_output: bool = args.next().is_some_and(|x| !x.starts_with("./"));

                Ok(Config::new(format, intensity, return_output)?)
            }

            pub fn get_format(&self) -> &ConfigType {
                &self.format
            }

            pub fn may_return(&self) -> &bool {
                &self.return_output
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
