#[cfg(test)]
mod tests {
    use crate::logic::*;
    use crate::DAY;
    use common_2023::*;

    #[test]
    fn mock_part_1_test() {
        test(DAY, "mock", part_1, 8);
    }

    #[test]
    fn mock_part_2_test() {
        test(DAY, "mock", part_2, 2286);
    }

    #[test]
    fn real_part_1_test() {
        test(DAY, "real", part_1, 3059);
    }

    #[test]
    fn real_part_2_test() {
        test(DAY, "real", part_2, 65371);
    }
}
