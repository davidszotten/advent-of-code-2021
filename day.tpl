use anyhow::Result;
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(_input: &str) -> Result<i32> {
    Ok(0)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("")?, 0);
        Ok(())
    }
}
