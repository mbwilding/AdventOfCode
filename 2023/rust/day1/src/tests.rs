#[cfg(test)]
mod tests {
    use crate::logic::*;
    use anyhow::Result;
    use common_2023::*;

    #[test]
    fn mock_part_1_test() -> Result<()> {
        test(1, "mock1", part_1, 142)?;

        Ok(())
    }

    #[test]
    fn mock_part_2_test() -> Result<()> {
        test(1, "mock2", part_2, 281)?;

        Ok(())
    }

    #[test]
    fn real_part_1_test() -> Result<()> {
        test(1, "real", part_1, 54573)?;

        Ok(())
    }

    #[test]
    fn real_part_2_test() -> Result<()> {
        test(1, "real", part_2, 54591)?;

        Ok(())
    }
}
