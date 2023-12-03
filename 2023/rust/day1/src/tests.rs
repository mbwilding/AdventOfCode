#[cfg(test)]
mod tests {
    use crate::logic::*;
    use crate::DAY;
    use common_2023::*;

    #[test]
    fn part_1_mock() {
        test(DAY, "mock1", part_1, 142);
    }

    #[test]
    fn part_1_real() {
        test(DAY, "real", part_1, 54573);
    }

    #[test]
    fn part_2_mock() {
        test(DAY, "mock2", part_2, 281);
    }

    #[test]
    fn part_2_real() {
        test(DAY, "real", part_2, 54591);
    }
}
