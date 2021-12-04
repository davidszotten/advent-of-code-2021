use anyhow::{anyhow, bail, Result};
use aoc2021::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<i32>>,
    rows_seen: Vec<Vec<bool>>,
    columns_seen: Vec<Vec<bool>>,
}

impl Board {
    fn new(rows: Vec<Vec<i32>>) -> Self {
        let len = rows.len();
        Board {
            rows,
            rows_seen: vec![vec![false; len]; len],
            columns_seen: vec![vec![false; len]; len],
        }
    }

    fn from_raw(s: &str) -> Result<Self> {
        let mut rows = vec![];
        for raw_row in s.split('\n') {
            let mut row = vec![];
            for raw_number in raw_row.split_whitespace() {
                let n = raw_number.parse()?;
                row.push(n);
            }
            rows.push(row);
        }
        Ok(Board::new(rows))
    }

    fn mark(&mut self, n: i32) -> bool {
        for (row_idx, row) in self.rows.iter().enumerate() {
            for (col_idx, number) in row.iter().enumerate() {
                if *number == n {
                    self.rows_seen[row_idx][col_idx] = true;
                    self.columns_seen[col_idx][row_idx] = true;

                    if self.rows_seen[row_idx].iter().all(|&x| x)
                        || self.columns_seen[col_idx].iter().all(|&x| x)
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for row_idx in 0..self.rows.len() {
            for col_idx in 0..self.rows.len() {
                if !self.rows_seen[row_idx][col_idx] {
                    sum += self.rows[row_idx][col_idx];
                }
            }
        }
        sum
    }
}

fn parse(input: &str) -> Result<(Vec<i32>, Vec<Board>)> {
    let mut entries = input.split("\n\n");
    let raw_numbers = entries.next().ok_or(anyhow!("no numbers found"))?;
    let numbers: Vec<i32> = raw_numbers
        .split(',')
        .map(|s| {
            s.parse()
                .map_err(|e| anyhow!("invalid number `{}` ({})", s, e))
        })
        .collect::<Result<Vec<i32>>>()?;
    let mut boards = vec![];
    while let Some(raw_board) = entries.next() {
        boards.push(Board::from_raw(raw_board)?);
    }

    Ok((numbers, boards))
}

fn part1(input: &str) -> Result<i32> {
    let (numbers, mut boards) = parse(input)?;
    let mut winner = None;
    'outer: for number in numbers {
        for board_idx in 0..boards.len() {
            if boards[board_idx].mark(number) {
                winner = Some((board_idx, number));
                break 'outer;
            }
        }
    }
    if let Some((winner_idx, number)) = winner {
        return Ok(boards[winner_idx].unmarked_sum() * number);
    }
    bail!("no winner")
}

fn part2(input: &str) -> Result<i32> {
    let (numbers, mut boards) = parse(input)?;
    let mut winner = None;
    let mut won = HashSet::new();
    'outer: for number in numbers {
        for board_idx in 0..boards.len() {
            if won.contains(&board_idx) {
                continue;
            }
            if boards[board_idx].mark(number) {
                winner = Some((board_idx, number));
                won.insert(board_idx);
            }
            if won.len() == boards.len() {
                break 'outer;
            }
        }
    }
    if let Some((winner_idx, number)) = winner {
        return Ok(boards[winner_idx].unmarked_sum() * number);
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
