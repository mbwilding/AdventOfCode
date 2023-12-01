#[cfg(test)]
mod tests {
    use crate::logic::*;
    use common_2023::*;
    use crate::DAY;

    #[test]
    fn mock_part_1_test() {
        test(DAY, "mock1", part_1, 142);
    }

    #[test]
    fn mock_part_2_test() {
        test(DAY, "mock2", part_2, 281);
    }

    #[test]
    fn real_part_1_test() {
        test(DAY, "real", part_1, 54573);
    }

    #[test]
    fn real_part_2_test() {
        test(DAY, "real", part_2, 54591);
    }
}
