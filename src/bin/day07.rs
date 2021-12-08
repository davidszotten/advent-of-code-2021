use anyhow::{Context, Result};
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<Vec<i32>> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().context("invalid integer"))
        .collect()
}

fn part1(input: &str) -> Result<i32> {
    let mut numbers = parse(input)?;
    numbers.sort_unstable();
    let pos = numbers[numbers.len() / 2];
    Ok(numbers.into_iter().map(|n| (n - pos).abs()).sum())
}

fn part2(input: &str) -> Result<i32> {
    let numbers = parse(input)?;

    let mean = numbers.iter().sum::<i32>() / numbers.len() as i32;

    let measure = |pos: i32| -> i32 {
        numbers
            .iter()
            .map(|n| ((*n - pos).abs()) * ((*n - pos).abs() + 1) / 2)
            .sum()
    };

    Ok(measure(mean).min(measure(mean + 1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 37);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 168);
        Ok(())
    }
}
