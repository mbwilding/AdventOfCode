#[cfg(test)]
mod tests {
    use crate::logic::*;
    use crate::DAY;
    use common_2023::*;

    #[test]
    fn part_1_mock() {
        test(DAY, "mock", part_1, 288);
    }

    #[test]
    fn part_1_real() {
        test(DAY, "real", part_1, 393120);
    }

    #[test]
    fn part_2_mock() {
        test(DAY, "mock", part_2, 71503);
    }

    #[test]
    fn part_2_real() {
        test(DAY, "real", part_2, 36872656);
    }
}
