use anyhow::Result;
use aoc2021::dispatch;
use itertools::Itertools;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1721
199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 7);
        Ok(())
    }

    #[test]
    fn test_part1_empty() -> Result<()> {
        assert_eq!(part1("")?, 0);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 5);
        Ok(())
    }
}
