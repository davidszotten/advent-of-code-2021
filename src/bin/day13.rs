use anyhow::{bail, Context, Error, Result};
use aoc2021::coor::Coor;
use aoc2021::dispatch;
use std::collections::HashSet;
use std::fmt::Write;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

enum Axis {
    X,
    Y,
}

impl FromStr for Axis {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => bail!("invalid axis `{}`", s),
        })
    }
}

struct Fold {
    axis: Axis,
    line: i64,
}

impl Fold {
    fn apply(&self, val: i64) -> i64 {
        if val < self.line {
            val
        } else {
            2 * self.line - val
        }
    }
}

impl FromStr for Fold {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        // fold along x=5";
        let (_, equation) = s.rsplit_once(' ').context("no equation")?;
        let (axis_s, raw_line) = equation.split_once('=').context("no equal sign")?;
        let line = raw_line.parse()?;
        let axis = axis_s.parse()?;
        Ok(Fold { axis, line })
    }
}

struct Instructions {
    dots: HashSet<Coor>,
    folds: Vec<Fold>,
}

impl Instructions {
    fn fold(&mut self) {
        let fold = self.folds.remove(0);
        self.dots = match fold.axis {
            Axis::X => self
                .dots
                .iter()
                .map(|d| Coor::new(fold.apply(d.x), d.y))
                .collect(),
            Axis::Y => self
                .dots
                .iter()
                .map(|d| Coor::new(d.x, fold.apply(d.y)))
                .collect(),
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
        let dots: HashSet<Coor> = raw_dots
            .lines()
            .map(Coor::from_str)
            .collect::<Result<_>>()?;
        let folds: Vec<_> = raw_folds
            .lines()
            .map(Fold::from_str)
            .collect::<Result<_>>()?;
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
