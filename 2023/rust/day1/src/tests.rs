#[cfg(test)]
mod tests {
    use crate::logic::*;
    use anyhow::Result;
    use common_2023::*;

    #[test]
    fn mock_part_1_test() -> Result<()> {
        let lines = read_lines("../../!data/day1/mock1.txt")?;
        let result = part_1(&lines);
        assert_eq!(142, result);

        Ok(())
    }

    #[test]
    fn mock_part_2_test() -> Result<()> {
        let lines = read_lines("../../!data/day1/mock2.txt")?;
        let result = part_2(&lines);
        assert_eq!(281, result);

        Ok(())
    }

    #[test]
    fn real_part_1_test() -> Result<()> {
        let lines = read_lines("../../!data/day1/real.txt")?;
        let result = part_1(&lines);
        assert_eq!(54573, result);

        Ok(())
    }

    #[test]
    fn real_part_2_test() -> Result<()> {
        let lines = read_lines("../../!data/day1/real.txt")?;
        let result = part_2(&lines);
        assert_eq!(54591, result);

        Ok(())
    }
}
