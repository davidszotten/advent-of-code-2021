use anyhow::{bail, Error, Result};
use aoc2021::dispatch;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Right,
    Down,
}

impl Cell {
    fn char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Right => '>',
            Cell::Down => 'v',
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        Ok(match c {
            '.' => Cell::Empty,
            '>' => Cell::Right,
            'v' => Cell::Down,
            _ => bail!("invalid cell `{}`", c),
        })
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        for line in &self.cells {
            println!("{}", line.iter().map(Cell::char).collect::<String>());
        }
        println!();
    }

    fn step_right(&mut self) {
        let mut next = vec![];
        for line in &self.cells {
            let mut next_line = line.clone();
            for (col, &cell) in line.iter().enumerate() {
                if cell == Cell::Right {
                    let mut dest = col + 1;
                    if dest == line.len() {
                        dest = 0
                    }
                    if line[dest] == Cell::Empty {
                        next_line.swap(col, dest);
                    }
                }
            }
            next.push(next_line);
        }
        self.cells = next;
    }

    fn step_down(&mut self) {
        let mut next = self.cells.clone();
        for (row, line) in self.cells.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if cell == Cell::Down {
                    let mut dest = row + 1;
                    if dest == next.len() {
                        dest = 0
                    }
                    if self.cells[dest][col] == Cell::Empty {
                        next[row][col] = Cell::Empty;
                        next[dest][col] = Cell::Down;
                    }
                }
            }
        }
        self.cells = next;
    }

    fn step(&mut self) -> bool {
        let before = self.cells.clone();
        self.step_right();
        self.step_down();
        self.cells != before
    }
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let cells = s
            .trim()
            .lines()
            .map(|l| l.chars().map(Cell::try_from).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<_>>>()?;
        Ok(Map { cells })
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    let mut count = 1;
    while map.step() {
        count += 1
    }
    Ok(count)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_step1() -> Result<()> {
        let mut map: Map = "....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v"
            .parse()?;
        map.step();
        map.print();
        Ok(())
    }

    #[test]
    fn test_step2() -> Result<()> {
        let mut map: Map = TEST_INPUT.parse()?;
        map.print();
        map.step();
        map.print();
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 58);
        Ok(())
    }
}
