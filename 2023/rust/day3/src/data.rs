use lazy_static::lazy_static;

lazy_static! {
    pub static ref DIRECTIONS: [(i16, i16); 8] = {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
    };
}
