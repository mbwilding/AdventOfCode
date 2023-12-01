#[cfg(test)]
mod tests {
    use crate::logic::*;
    use common_2023::*;

    #[test]
    fn mock_part_1_test() {
        test(1, "mock1", part_1, 142);
    }

    #[test]
    fn mock_part_2_test() {
        test(1, "mock2", part_2, 281);
    }

    #[test]
    fn real_part_1_test() {
        test(1, "real", part_1, 54573);
    }

    #[test]
    fn real_part_2_test() {
        test(1, "real", part_2, 54591);
    }
}
