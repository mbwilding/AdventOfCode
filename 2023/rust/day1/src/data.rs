use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref NUMBER_LUT: HashMap<&'static str, u32> = {
        HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ])
    };
}
