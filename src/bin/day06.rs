use anyhow::{Context, Result};
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<Vec<usize>> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().context("invalid number"))
        .collect()
}

fn run(initial: Vec<usize>, rounds: usize) -> u64 {
    let mut counts = [0; 9];
    for n in initial {
        counts[n] += 1;
    }
    for _ in 0..rounds {
        let zeros = counts[0];
        for i in 0..8 {
            counts[i] = counts[i + 1]
        }
        counts[6] += zeros;
        counts[8] = zeros;
    }
    counts.iter().sum()
}

fn part1(input: &str) -> Result<u64> {
    let initial = parse(input)?;
    Ok(run(initial, 80))
}

fn part2(input: &str) -> Result<u64> {
    let initial = parse(input)?;
    Ok(run(initial, 256))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 5934);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 26984457539);
        Ok(())
    }
}
