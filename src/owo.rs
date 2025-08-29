pub mod owo {
    use std::collections::HashMap;

    // Type dictating all patterns that should be replaced and what with.
    pub struct Owoifier<'a> {
        pattern_map: HashMap<&'a str, &'a str>,
    }

    impl<'a> Owoifier<'a> {
        pub fn new(pattern_map: HashMap<&'a str, &'a str>) -> Owoifier<'a> {
            Owoifier { pattern_map }
        }

        pub fn convert_string(&self, text: &String) -> String {
            let mut buf = text.clone();

            for (k, r) in self.get_pattern_map().iter() {
                buf = buf.replace(k, r);
            }

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

            let context = Owoifier::new(pattern_map.clone());
            context.convert_string(&hello_world);

            assert_eq!(context.convert_string(&hello_world), "Hewwo, Wowwd!")
        }
    }
}
