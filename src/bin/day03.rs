use anyhow::{anyhow, bail, Error, Result};
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Bit {
    Zero,
    One,
}

impl TryFrom<char> for Bit {
    type Error = Error;
    fn try_from(c: char) -> Result<Self> {
        Ok(match c {
            '0' => Bit::Zero,
            '1' => Bit::One,
            _ => bail!("invalid char `{}`", c),
        })
    }
}

fn from_str(s: &str) -> Result<Vec<Bit>> {
    s.chars().map(|c| Bit::try_from(c)).collect()
}

fn to_decimal(number: &[Bit]) -> i32 {
    let mut decimal = 0;
    for bit in number.iter() {
        decimal *= 2;
        if *bit == Bit::One {
            decimal += 1;
        }
    }
    decimal
}

fn parse(input: &str) -> Result<Vec<Vec<Bit>>> {
    input
        .trim()
        .split('\n')
        .map(from_str)
        .collect::<Result<Vec<_>>>()
}

fn part1(input: &str) -> Result<i32> {
    let numbers = parse(input)?;
    let length = numbers.get(0).ok_or(anyhow!("no numbers found"))?.len();
    let mut counts = vec![0; length];
    for number in numbers {
        for (idx, bit) in number.iter().enumerate() {
            counts[idx] += if *bit == Bit::Zero { -1 } else { 1 };
        }
    }
    let mut gamma = 0;
    for count in counts {
        gamma *= 2;
        if count > 0 {
            gamma += 1;
        }
    }
    let epsilon = (1 << length) - 1 - gamma;
    Ok(epsilon * gamma)
}

fn reduce_with_rule(numbers: &[Vec<Bit>], rule: fn(i32) -> Bit) -> Result<i32> {
    let mut numbers = numbers.to_vec();
    let mut pos = 0;
    loop {
        let mut count = 0;
        for number in &numbers {
            count += if number[pos] == Bit::Zero { -1 } else { 1 };
        }

        let target = rule(count);

        let previous_len = numbers.len();

        numbers = numbers
            .into_iter()
            .filter(|n| n[pos] == target)
            .collect::<Vec<_>>();

        if numbers.len() == 1 {
            break;
        } else if numbers.len() == previous_len {
            bail!("nothing was filtered out");
        } else if numbers.len() == 0 {
            bail!("no numbers left");
        }

        pos += 1
    }
    Ok(to_decimal(&numbers[0]))
}

fn part2(input: &str) -> Result<i32> {
    let numbers = parse(input)?;
    let oxygen = reduce_with_rule(
        &numbers,
        |count| if count >= 0 { Bit::One } else { Bit::Zero },
    )?;
    let co2 = reduce_with_rule(
        &numbers,
        |count| if count < 0 { Bit::One } else { Bit::Zero },
    )?;
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
