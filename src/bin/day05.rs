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

    fn diff(&self) -> Coor {
        self.end - self.start
    }

    fn walk(&self, points: &mut HashMap<Coor, usize>) {
        let diff = self.diff();
        let direction = Coor::new(diff.x.signum(), diff.y.signum());
        let mut point = self.start;
        while point != self.end {
            *points.entry(point).or_insert(0) += 1;
            point += direction;
        }
        *points.entry(point).or_insert(0) += 1;
    }

    fn hor_or_vert(&self) -> bool {
        let diff = self.diff();
        diff.x == 0 || diff.y == 0
    }
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s.split_once(" -> ").context(format!("no arrow: `{}`", s))?;
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

fn count_overlaps(points: &HashMap<Coor, usize>) -> usize {
    points.values().filter(|&v| *v > 1).count()
}

fn part1(input: &str) -> Result<usize> {
    let mut points = HashMap::new();
    let lines = parse(input)?;
    lines
        .into_iter()
        .filter(Line::hor_or_vert)
        .for_each(|l| l.walk(&mut points));
    Ok(count_overlaps(&points))
}

fn part2(input: &str) -> Result<usize> {
    let mut points = HashMap::new();
    let lines = parse(input)?;
    lines.iter().for_each(|line| line.walk(&mut points));
    Ok(count_overlaps(&points))
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
