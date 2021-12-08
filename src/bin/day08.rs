use anyhow::{bail, Context, Error, Result};
use aoc2021::dispatch;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(PartialEq, Eq, Clone)]
struct Digit {
    segments: HashSet<char>,
}

impl std::fmt::Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut v: Vec<_> = self.segments.iter().collect();
        v.sort();
        let s = v.into_iter().collect::<String>();
        write!(f, "Digit {{ {} }}", s)
    }
}

impl Digit {
    fn intersection(&self, other: &Digit) -> Self {
        Self {
            segments: self
                .segments
                .intersection(&other.segments)
                .map(|&c| c)
                .collect(),
        }
    }

    fn share(&self, other: &Digit, count: usize) -> bool {
        self.intersection(other).len() == count
    }

    fn len(&self) -> usize {
        self.segments.len()
    }

    fn copy(&self) -> Self {
        Self {
            segments: self.segments.iter().cloned().collect(),
        }
    }
}

impl Hash for Digit {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        let mut v: Vec<_> = self.segments.iter().collect();
        v.sort();
        v.hash(h)
    }
}

impl FromStr for Digit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Digit {
            segments: s.chars().collect(),
        })
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    patterns: Vec<Digit>,
    output: Vec<Digit>,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut it = s.split(" | ");
        let raw_patterns = it.next().context("no patterns")?;
        let raw_output = it.next().context("no output")?;
        Ok(Input {
            patterns: raw_patterns
                .split(' ')
                .map(|s| s.parse())
                .collect::<Result<_>>()?,
            output: raw_output
                .split(' ')
                .map(|s| s.parse())
                .collect::<Result<_>>()?,
        })
    }
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|l| l.parse::<Input>())
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|i| {
            i.output
                .iter()
                .filter(|o| matches!(o.segments.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum())
}

/*

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

*/

fn decode(input: &Input) -> Result<usize> {
    let patterns = input.patterns.clone();

    let mut by_size = HashMap::new();
    for pattern in patterns {
        by_size
            .entry(pattern.segments.len())
            .or_insert(vec![])
            .push(pattern);
    }

    fn get_single_by_size(by_size: &mut HashMap<usize, Vec<Digit>>, size: usize) -> Result<Digit> {
        let mut digits = by_size.remove(&size).context("missing digit")?;
        assert_eq!(digits.len(), 1);
        Ok(digits.remove(0))
    }

    fn one(it: &mut dyn Iterator<Item = &Digit>) -> Result<Digit> {
        let item = it.next().context("was empty")?;
        if let Some(_) = it.next() {
            bail!("more than one");
        }
        Ok(item.copy())
    }

    let d1: Digit = get_single_by_size(&mut by_size, 2)?;
    let d4: Digit = get_single_by_size(&mut by_size, 4)?;
    let d7: Digit = get_single_by_size(&mut by_size, 3)?;
    let d8: Digit = get_single_by_size(&mut by_size, 7)?;

    let sixes = by_size.remove(&6).context("missing digit")?;
    assert_eq!(sixes.len(), 3);
    let d6: Digit = one(&mut sixes.iter().filter(|s| s.share(&d1, 1)))?;
    let d9: Digit = one(&mut sixes.iter().filter(|s| s.share(&d4, 4)))?;
    let d0: Digit = one(&mut sixes.iter().filter(|&s| s != &d6 && s != &d9))?;

    let fives = by_size.remove(&5).context("missing digit")?;
    assert_eq!(fives.len(), 3);
    let d3: Digit = one(&mut fives.iter().filter(|s| s.share(&d1, 2)))?;
    let d5: Digit = one(&mut fives.iter().filter(|&s| s != &d3 && s.share(&d6, 5)))?;
    let d2: Digit = one(&mut fives.iter().filter(|&s| s != &d3 && s.share(&d6, 4)))?;

    let map = HashMap::from([
        (d0, 0),
        (d1, 1),
        (d2, 2),
        (d3, 3),
        (d4, 4),
        (d5, 5),
        (d6, 6),
        (d7, 7),
        (d8, 8),
        (d9, 9),
    ]);
    assert_eq!(map.len(), 10);

    Ok(input.output.iter().fold(0, |acc, x| acc * 10 + map[x]))
}

fn part2(input: &str) -> Result<usize> {
    input
        .lines()
        .map(|l| l.parse::<Input>())
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(decode)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 26);
        Ok(())
    }

    #[test]
    fn test_decode() -> Result<()> {
        let first =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let input = first.parse()?;
        assert_eq!(decode(&input)?, 5353);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 61229);
        Ok(())
    }
}