use anyhow::{bail, Context, Error, Result};
use aoc2021::coor::Coor;
use aoc2021::dispatch;
use std::collections::HashSet;
use std::fmt::Write;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse_fold(s: &str) -> Result<(char, i64)> {
    // fold along x=5";
    let (_, equation) = s.rsplit_once(' ').context("no equation")?;
    let (dir_s, raw_value) = equation.split_once('=').context("no equal sign")?;
    let dir = dir_s.chars().next().context("no chars?")?;
    Ok((dir, raw_value.parse()?))
}

struct Instructions {
    dots: HashSet<Coor>,
    folds: Vec<(char, i64)>,
}

fn fold_val(val: i64, line: i64) -> i64 {
    if val < line {
        val
    } else {
        2 * line - val
    }
}

impl Instructions {
    fn fold(&mut self) {
        let fold = self.folds.remove(0);
        self.dots = match fold.0 {
            'y' => self
                .dots
                .iter()
                .map(|d| Coor::new(d.x, fold_val(d.y, fold.1)))
                .collect(),
            'x' => self
                .dots
                .iter()
                .map(|d| Coor::new(fold_val(d.x, fold.1), d.y))
                .collect(),
            _ => panic!("unexcpected fold line {:?}", fold),
        }
    }

    fn print(&self) -> Result<String> {
        let mut output = String::new();
        let maxx = self.dots.iter().map(|d| d.x).max().context("no x")?;
        let maxy = self.dots.iter().map(|d| d.y).max().context("no y")?;
        for y in 0..maxy + 1 {
            for x in 0..maxx + 1 {
                if self.dots.contains(&Coor::new(x, y)) {
                    write!(output, "#")?;
                } else {
                    write!(output, " ")?;
                }
            }
            writeln!(output)?;
        }
        Ok(output)
    }
}

impl FromStr for Instructions {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.trim().split("\n\n");
        let raw_dots = parts.next().context("no coors found")?;
        let raw_folds = parts.next().context("no folds found")?;
        if parts.next().is_some() {
            bail!("too many parts");
        }
        let dots: HashSet<Coor> = raw_dots.lines().map(|s| s.parse()).collect::<Result<_>>()?;
        let folds = raw_folds.lines().map(parse_fold).collect::<Result<_>>()?;
        Ok(Instructions { dots, folds })
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut instructions: Instructions = input.parse()?;
    instructions.fold();
    Ok(instructions.dots.len())
}

fn part2(input: &str) -> Result<String> {
    let mut instructions: Instructions = input.parse()?;
    while !instructions.folds.is_empty() {
        instructions.fold();
    }
    instructions.print()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 17);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(TEST_INPUT)?,
            "#####
#   #
#   #
#   #
#####
"
        );
        Ok(())
    }
}
