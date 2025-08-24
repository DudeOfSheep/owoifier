pub mod owo {
    use std::{collections::HashMap, env::consts::FAMILY, io::BufRead};

    // Type dictating all patterns that should be replaced and what with.
    // Lookahed dictates how far the matching algorithim looks ahead before making a final decision.
    pub struct Context<'a> {
        pattern_map: HashMap<&'a str, &'a str>,
        lookahead: i32,
    }

    impl<'a> Context<'a> {
        pub fn new(pattern_map: HashMap<&'a str, &'a str>) -> Context<'a> {
            let mut lookahead: i32 = 0;

            for key in pattern_map.keys() {
                let key_len = key.len() as i32;

                // Ugly
                lookahead = if key_len > lookahead {
                    key_len
                } else {
                    lookahead
                };
            }

            Context {
                pattern_map,
                lookahead,
            }
        }

        pub fn convert_string(&self, text: &mut String) {
            for (k, r) in self.get_pattern_map().iter() {
                *text = text.replace(k, r);
            }
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

            let mut hello_world = String::from("Hello, World!");

            let context = Context::new(pattern_map.clone());
            context.convert_string(&mut hello_world);

            assert_eq!(hello_world, "Hewwo, Wowwd!")
        }
    }
}
