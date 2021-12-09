use anyhow::{Error, Result};
use aoc2021::coor::Coor;
use aoc2021::dispatch;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

struct Map {
    heights: HashMap<Coor, i32>,
    ends_in: HashMap<Coor, Coor>,
}

const NEIGHBOURS: [Coor; 4] = [
    Coor::new(-1, 0),
    Coor::new(1, 0),
    Coor::new(0, -1),
    Coor::new(0, 1),
];

impl Map {
    fn flows_to(&self, coor: &Coor) -> Option<Coor> {
        let height = self.heights[coor];
        let mut possible_dest = vec![];
        for (idx, offset) in NEIGHBOURS.iter().enumerate() {
            let dest = *offset + *coor;
            if let Some(offset_height) = self.heights.get(&dest) {
                if *offset_height <= height {
                    possible_dest.push((*offset_height, idx, dest));
                }
            }
        }
        possible_dest.sort_by_key(|k| (k.0, k.1));
        possible_dest.get(0).map(|(_, _, c)| *c)
    }

    fn follow(&mut self, coor: Coor) -> Coor {
        let height = self.heights[&coor];
        if height == 9 {
            return coor;
        }
        let mut prev = coor;
        let mut maybe_next = Some(coor);
        while let Some(next) = maybe_next {
            if let Some(end) = self.ends_in.get(&next) {
                return *end;
            }
            prev = next;
            maybe_next = self.flows_to(&next);
        }
        self.ends_in.insert(coor, prev);
        prev
    }
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut heights = HashMap::new();
        for (y, line) in s.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                heights.insert(Coor::new(x as i64, y as i64), c as i32 - '0' as i32);
            }
        }
        Ok(Self {
            heights,
            ends_in: HashMap::new(),
        })
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut sum = 0;
    let map: Map = input.parse()?;
    for coor in map.heights.keys() {
        if map.flows_to(coor).is_none() {
            sum += map.heights[coor] + 1;
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<i32> {
    let mut map: Map = input.parse()?;
    let mut destinations = HashMap::new();
    let coors: Vec<_> = map.heights.keys().cloned().collect();
    for coor in coors {
        let dst = map.follow(coor);
        *destinations.entry(dst).or_insert(0) += 1;
    }
    let mut sizes: Vec<i32> = destinations.values().cloned().collect();
    sizes.sort_unstable();
    sizes.reverse();
    Ok(sizes.iter().take(3).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 15);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 1134);
        Ok(())
    }
}
