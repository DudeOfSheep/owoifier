pub mod config {
    pub enum ConfigType {
        BARE(String),
        FILE(String), // Might want to make this a filepath type
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

        pub fn get_format(&self) -> ConfigType {
            self.format
        }
    }
}
