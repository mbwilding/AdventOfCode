use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MAX_CUBES_LUT: HashMap<&'static str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
}
