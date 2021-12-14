use anyhow::{bail, Context, Error, Result};
use aoc2021::dispatch;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse_rule(s: &str) -> Result<((char, char), char)> {
    let (left, right) = s.split_once(" -> ").context("no arrow")?;
    let mut l_it = left.chars();
    let c1 = l_it.next().context("left 1 missing")?;
    let c2 = l_it.next().context("left 2 missing")?;
    if l_it.next().is_some() {
        bail!("left: too much input");
    }
    let mut r_it = right.chars();
    let c3 = r_it.next().context("left 1 missing")?;
    if r_it.next().is_some() {
        bail!("right: too much input");
    }
    Ok(((c1, c2), c3))
}

struct State {
    polymer: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
    last: char,
}

impl State {
    fn step(&mut self) -> Result<()> {
        let mut new = HashMap::new();
        for (&(c1, c2), count) in &self.polymer {
            let &middle = self.rules.get(&(c1, c2)).context("no rule")?;
            *new.entry((c1, middle)).or_insert(0) += count;
            *new.entry((middle, c2)).or_insert(0) += count;
        }
        self.polymer = new;
        Ok(())
    }

    fn quality(&self) -> Result<usize> {
        let mut counts = HashMap::from([(self.last, 1)]);
        for (&(c1, _), count) in &self.polymer {
            *counts.entry(c1).or_insert(0) += count;
        }
        let max = counts.iter().max_by_key(|(_, c)| *c).context("no max")?.1;
        let min = counts.iter().min_by_key(|(_, c)| *c).context("no min")?.1;
        Ok(max - min)
    }
}

impl FromStr for State {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut it = s.trim().split("\n\n");
        let raw_polymer = it.next().context("no polymer")?;
        let raw_rules = it.next().context("no rules")?;
        if it.next().is_some() {
            bail!("too much input");
        }
        let mut polymer = HashMap::new();
        for pair in raw_polymer.chars().zip(raw_polymer.chars().skip(1)) {
            *polymer.entry(pair).or_insert(0) += 1;
        }
        let last = raw_polymer.chars().last().context("no last")?;
        let rules = raw_rules.lines().map(parse_rule).collect::<Result<_>>()?;
        Ok(Self {
            polymer,
            rules,
            last,
        })
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut state: State = input.parse()?;
    for _ in 0..10 {
        state.step()?;
    }
    state.quality()
}

fn part2(input: &str) -> Result<usize> {
    let mut state: State = input.parse()?;
    for _ in 0..40 {
        state.step()?;
    }
    state.quality()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 1588);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 2188189693529);
        Ok(())
    }
}
