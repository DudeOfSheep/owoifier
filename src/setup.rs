pub mod configuration {
    pub mod env_config {
        use std::fs::File;

        #[derive(Debug)]
        pub enum ConfigType {
            BARE(String),
            FILE(File),
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

            pub fn get_format(&self) -> &ConfigType {
                &self.format
            }
        }
    }

    pub mod parser {
        use super::env_config::{Config, ConfigType};

        use std::{error::Error, fmt::Display, fs};
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

        pub fn parse_args(args: &Vec<String>) -> Result<Config, Box<dyn Error>> {
            let format: ConfigType = if args[1].trim().starts_with("./") {
                //Note, this is still bad. explicitely ask if the input is a file.
                ConfigType::FILE(fs::File::open(&args[1][2..])?)
            } else {
                ConfigType::BARE(args[1].clone())
            };

            let intensity: i32 = args[2].trim().parse::<i32>()?;
            let return_output: bool = args[3].trim().parse::<bool>()?;

            Ok(Config::new(format, intensity, return_output)?)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn valid_file_parse() -> Result<(), Box<dyn Error>> {
                let valid_args: Vec<String> = vec![
                    String::from("PROGRAM PATH"),
                    String::from("./Thoughts.txt"),
                    String::from("1"),
                    String::from("false"),
                ];

                parse_args(&valid_args).and(Ok(()))
            }

            #[should_panic]
            #[test]
            fn invalid_intensity_file_parse() {
                let valid_args: Vec<String> = vec![
                    String::from("PROGRAM PATH"),
                    String::from("./Thoughts.txt"),
                    String::from("BURN"),
                    String::from("false"),
                ];

                parse_args(&valid_args).and(Ok(())).expect("SUCCESS");
            }

            // Parse likes to continue even
            #[should_panic]
            #[test]
            fn invalid_path_file_parse() {
                let valid_args: Vec<String> = vec![
                    String::from("PROGRAM PATH"),
                    String::from("./does_not_exist.txt"),
                    String::from("1"),
                    String::from("false"),
                ];

                let test = parse_args(&valid_args).expect("SUCCESS");

                // The file names are being assigned, no errors are propagated up when they should.
                dbg!(test.get_format());
            }

            #[should_panic]
            #[test]
            fn invalid_output_file_parse() {
                let valid_args: Vec<String> = vec![
                    String::from("PROGRAM PATH"),
                    String::from("./Thoughts.txt"),
                    String::from("BURN"),
                    String::from("falsy"),
                ];

                parse_args(&valid_args).and(Ok(())).expect("SUCCESS");
            }

            #[test]
            fn valid_bare_parse() -> Result<(), Box<dyn Error>> {
                let valid_args: Vec<String> = vec![
                    String::from("PROGRAM PATH"),
                    String::from("Machine I have become Pretty Princess!"),
                    String::from("1"),
                    String::from("false"),
                ];

                parse_args(&valid_args).and(Ok(()))
            }
        }
    }
}
