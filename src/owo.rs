pub mod owo {
    use std::{collections::HashMap, str::pattern::Pattern};
    trait Owoifier {
        fn replace_pass(text: &str) -> &str;
        fn extension_pass(text: &str) -> &str;
        fn insertion_pass(text: &str) -> &str;
    }

    struct Context<'a> {
        pattern_map: HashMap<&'a str, &'a str>,
        lookahead: i32,
    }

    impl<'a> Context<'a> {
        fn new(pattern_map: HashMap<&'a str, &'a str>) -> Context<'a> {
            let mut lookahead: i32 = 0;

            for key in pattern_map.keys() {
                let key_len = key.len() as i32;
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
    }

    pub fn basic_convert(text: &mut String) {
        let buffer: String = text.drain(..).collect();

        for item in buffer.chars() {
            if item.is_alphabetic() {
                match item {
                    'r' => text.push('w'),
                    'l' => text.push('w'),
                    n => text.push(n),
                }
            }
        }
    }
}
