#[cfg(test)]
mod tests {
    use crate::logic::*;
    use crate::DAY;
    use common_2023::*;

    #[test]
    fn part_1_mock() {
        test(DAY, "mock", part_1, 8);
    }

    #[test]
    fn part_1_real() {
        test(DAY, "real", part_1, 3059);
    }

    #[test]
    fn part_2_mock() {
        test(DAY, "mock", part_2, 2286);
    }

    #[test]
    fn part_2_real() {
        test(DAY, "real", part_2, 65371);
    }
}
