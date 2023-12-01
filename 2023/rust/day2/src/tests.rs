#[cfg(test)]
mod tests {
    use crate::logic::*;
    use crate::DAY;
    use common_2023::*;

    #[test]
    fn mock_part_1_test() {
        test(DAY, "mock1", part_1, 0);
    }

    #[test]
    fn mock_part_2_test() {
        test(DAY, "mock2", part_2, 0);
    }

    #[test]
    fn real_part_1_test() {
        test(DAY, "real", part_1, 0);
    }

    #[test]
    fn real_part_2_test() {
        test(DAY, "real", part_2, 0);
    }
}
