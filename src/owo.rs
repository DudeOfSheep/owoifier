pub mod owo {
    use std::{collections::HashMap, env::consts::FAMILY, io::BufRead};

    // Type dictating all patterns that should be replaced and what with.
    // Lookahed dictates how far the matching algorithim looks ahead before making a final decision.
    pub struct Context<'a> {
        pattern_map: HashMap<&'a str, &'a str>,
    }

    impl<'a> Context<'a> {
        pub fn new(pattern_map: HashMap<&'a str, &'a str>) -> Context<'a> {
            Context { pattern_map }
        }

        pub fn convert_string(&self, text: &String) -> String {
            let mut buf = text.clone();
            dbg!(&buf);

            for (k, r) in self.get_pattern_map().iter() {
                buf = buf.replace(k, r);
                dbg!(&buf);
            }

            dbg!(&buf);
            buf
        }

        fn get_pattern_map(&self) -> &HashMap<&str, &str> {
            &self.pattern_map
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn valid_context() {
            let mut pattern_map: HashMap<&str, &str> = HashMap::new();
            pattern_map.insert("l", "w");
            pattern_map.insert("r", "w");

            let hello_world = String::from("Hello, World!");

            let context = Context::new(pattern_map.clone());
            context.convert_string(&hello_world);

            assert_eq!(context.convert_string(&hello_world), "Hewwo, Wowwd!")
        }
    }
}
