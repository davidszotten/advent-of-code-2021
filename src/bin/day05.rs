use anyhow::{Context, Error, Result};
use aoc2021::coor::Coor;
use aoc2021::dispatch;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Line {
    start: Coor,
    end: Coor,
}

impl Line {
    fn new(start: Coor, end: Coor) -> Self {
        Self { start, end }
    }

    fn walk(&self, points: &mut HashMap<Coor, usize>) {
        let line_diff = self.end - self.start;
        let diff = Coor::new(line_diff.x.signum(), line_diff.y.signum());
        let mut point = self.start;
        while point != self.end {
            *points.entry(point).or_insert(0) += 1;
            point += diff;
        }
        *points.entry(point).or_insert(0) += 1;
    }

    fn hor_or_vert(&self) -> bool {
        return self.start.x == self.end.x || self.start.y == self.end.y;
    }
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s.split_once(" -> ").context("no arrow")?;
        Ok(Line::new(start.parse()?, end.parse()?))
    }
}

fn parse(input: &str) -> Result<Vec<Line>> {
    input
        .trim()
        .split('\n')
        .map(|s| s.parse())
        .collect::<Result<_>>()
}

fn part1(input: &str) -> Result<usize> {
    let mut points = HashMap::new();
    let lines = parse(input)?;
    for line in lines {
        if !line.hor_or_vert() {
            continue;
        }
        line.walk(&mut points);
    }
    Ok(points.values().filter(|&v| *v > 1).count())
}

fn part2(input: &str) -> Result<usize> {
    let mut points = HashMap::new();
    let lines = parse(input)?;
    for line in lines {
        line.walk(&mut points);
    }
    Ok(points.values().filter(|&v| *v > 1).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 5);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 12);
        Ok(())
    }
}
