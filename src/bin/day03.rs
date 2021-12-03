use anyhow::{anyhow, Result};
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<i32> {
    let length = input.find('\n').ok_or(anyhow!("no newline found"))?;
    let mut counts = vec![0; length];
    for line in input.trim().split('\n') {
        for (idx, char) in line.trim().chars().enumerate() {
            counts[idx] += if char == '0' { -1 } else { 1 };
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for count in counts {
        gamma *= 2;
        epsilon *= 2;
        if count > 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    Ok(epsilon * gamma)
}

fn part2(input: &str) -> Result<i32> {
    let mut numbers: Vec<_> = input
        .trim()
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let mut pos = 0;
    loop {
        let mut count = 0;
        for number in &numbers {
            count += if number[pos] == '0' { -1 } else { 1 };
        }

        let target = if count >= 0 { '1' } else { '0' };

        numbers = numbers
            .into_iter()
            .filter(|n| n[pos] == target)
            .collect::<Vec<_>>();

        if numbers.len() == 1 {
            break;
        }

        pos += 1
    }

    let mut oxygen = 0;
    for bit in &numbers[0] {
        oxygen *= 2;
        if *bit == '1' {
            oxygen += 1;
        }
    }

    let mut numbers: Vec<_> = input
        .trim()
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let mut pos = 0;
    loop {
        let mut count = 0;
        for number in &numbers {
            count += if number[pos] == '0' { -1 } else { 1 };
        }

        let target = if count < 0 { '1' } else { '0' };

        numbers = numbers
            .into_iter()
            .filter(|n| n[pos] == target)
            .collect::<Vec<_>>();

        if numbers.len() == 1 {
            break;
        }

        pos += 1
    }

    let mut co2 = 0;
    for bit in &numbers[0] {
        co2 *= 2;
        if *bit == '1' {
            co2 += 1;
        }
    }
    Ok(oxygen * co2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 198);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 230);
        Ok(())
    }
}
