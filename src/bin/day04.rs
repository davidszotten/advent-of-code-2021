use anyhow::{anyhow, bail, Context, Error, Result};
use aoc2021::dispatch;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Board {
    size: usize,
    numbers: HashMap<i32, (usize, usize)>,
    row_counts: Vec<usize>,
    column_counts: Vec<usize>,
}

impl FromStr for Board {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut numbers = HashMap::new();
        let mut size = 0;
        for (row_idx, raw_row) in s.split('\n').enumerate() {
            for (col_idx, raw_number) in raw_row.split_whitespace().enumerate() {
                let n = raw_number.parse()?;
                numbers.insert(n, (row_idx, col_idx));
            }
            size = row_idx + 1;
        }
        Ok(Board::new(size, numbers))
    }
}

impl Board {
    fn new(size: usize, numbers: HashMap<i32, (usize, usize)>) -> Self {
        Board {
            size,
            numbers,
            row_counts: vec![0; size],
            column_counts: vec![0; size],
        }
    }

    fn mark(&mut self, n: i32) -> bool {
        if let Some((row_idx, col_idx)) = self.numbers.remove(&n) {
            self.row_counts[row_idx] += 1;
            self.column_counts[col_idx] += 1;

            if self.row_counts[row_idx] == self.size || self.column_counts[col_idx] == self.size {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> i32 {
        self.numbers.keys().sum()
    }
}

fn parse(input: &str) -> Result<(Vec<i32>, Vec<Board>)> {
    let mut entries = input.trim().split("\n\n");
    let raw_numbers = entries.next().ok_or(anyhow!("no numbers found"))?;
    let numbers: Vec<i32> = raw_numbers
        .split(',')
        .map(|s| s.parse().context(format!("invalid number `{}`", s)))
        .collect::<Result<_>>()?;
    let boards = entries.map(|s| s.parse()).collect::<Result<_>>()?;

    Ok((numbers, boards))
}

fn part1(input: &str) -> Result<i32> {
    let (numbers, mut boards) = parse(input)?;
    for number in numbers {
        for board in &mut boards {
            if board.mark(number) {
                return Ok(board.unmarked_sum() * number);
            }
        }
    }
    bail!("no winner")
}

fn part2(input: &str) -> Result<i32> {
    let (numbers, mut boards) = parse(input)?;
    let mut winner = None;
    for number in numbers {
        let mut end = boards.len();
        let mut board_idx = 0;
        while board_idx < end {
            if boards[board_idx].mark(number) {
                let board = boards.swap_remove(board_idx);
                end -= 1;
                winner = Some((board, number));
            } else {
                board_idx += 1;
            }
        }
    }
    if let Some((board, number)) = winner {
        return Ok(board.unmarked_sum() * number);
    }
    bail!("no winner")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 4512);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 1924);
        Ok(())
    }
}
