use anyhow::{Context, Error, Result};
use aoc2021::coor::Coor;
use aoc2021::dispatch;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

struct Map {
    levels: HashMap<Coor, i32>,
}

impl Map {
    fn incr(&mut self) {
        for (_, val) in self.levels.iter_mut() {
            *val += 1;
        }
    }

    fn flash(&mut self) -> usize {
        let mut flashing: Vec<_> = self
            .levels
            .iter()
            .filter(|&(_, v)| *v > 9)
            .map(|(c, _)| *c)
            .collect();
        let mut flashed: HashSet<_> = flashing.iter().cloned().collect();
        while !flashing.is_empty() {
            let mut new_flashing = vec![];
            for c in &flashing {
                for diff in NEIGHBOURS {
                    let neighbour = *c + diff;
                    if let Some(n_level) = self.levels.get_mut(&neighbour) {
                        *n_level += 1;
                        if *n_level > 9 && !flashed.contains(&neighbour) {
                            flashed.insert(neighbour);
                            new_flashing.push(neighbour);
                        }
                    }
                }
            }
            flashing = new_flashing;
        }
        flashed.len()
    }

    fn reset(&mut self) {
        for (_, val) in self.levels.iter_mut() {
            if *val > 9 {
                *val = 0;
            }
        }
    }

    fn step(&mut self) -> usize {
        self.incr();
        let flashes = self.flash();
        self.reset();
        flashes
    }

    fn size(&self) -> usize {
        self.levels.len()
    }
}

const NEIGHBOURS: [Coor; 8] = [
    Coor::new(-1, -1),
    Coor::new(-1, 0),
    Coor::new(-1, 1),
    Coor::new(0, -1),
    Coor::new(0, 1),
    Coor::new(1, -1),
    Coor::new(1, 0),
    Coor::new(1, 1),
];

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut levels = HashMap::new();
        for (y, line) in s.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                levels.insert(Coor::new(x as i64, y as i64), c as i32 - '0' as i32);
            }
        }
        Ok(Self { levels })
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    Ok((0..100).map(|_| map.step()).sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    (1..)
        .find(|_| map.step() == map.size())
        .context("can't happen")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 1656);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 195);
        Ok(())
    }
}
