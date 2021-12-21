use anyhow::{Context, Result};
use aoc2021::dispatch;
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn take3(it: &mut dyn Iterator<Item = u64>) -> u64 {
    it.next().unwrap() + it.next().unwrap() + it.next().unwrap()
}

fn parse(input: &str) -> Result<[u64; 2]> {
    let (p1, p2) = input.trim().split_once('\n').context("no newline")?;
    Ok([
        p1.rsplit_once(' ')
            .context("no space")?
            .1
            .parse()
            .context("not a number")?,
        p2.rsplit_once(' ')
            .context("no space")?
            .1
            .parse()
            .context("not a number")?,
    ])
}

fn part1(input: &str) -> Result<u64> {
    let mut it = (1..).into_iter();
    let mut scores = [0, 0];
    let mut positions = parse(input)?;
    let mut throws = 0;
    Ok(loop {
        let points = take3(&mut it);
        let player = throws % 2;
        positions[player] = (positions[player] + points) % 10;
        scores[player] += if positions[player] == 0 {
            10
        } else {
            positions[player]
        };
        throws += 3;
        if scores[player] >= 1000 {
            let other = (player + 1) % 2;
            break throws as u64 * scores[other];
        }
    })
}

/*

111  3
112  4
113  5
121  4
122  5
123  6
131  5
132  6
133  7

3 4 4 5 5 5 6 6 7
4 5 5 6 6 6 7 7 8
5 6 6 7 7 7 8 8 9

3
4 4 4
5 5 5 5 5 5
6 6 6 6 6 6 6
7 7 7 7 7 7
8 8 8
9

     */

const DICE: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn part2(input: &str) -> Result<u64> {
    let starting_positions = parse(input)?;
    let mut positions = HashMap::new();
    positions.insert((starting_positions, [0, 0]), 1);
    let mut player = 0;
    let mut wins = [0, 0];

    loop {
        let mut new_positions = HashMap::new();
        for (throw, throw_count) in DICE {
            for (state, count) in &positions {
                let (position, scores) = state;
                let mut new_position = position.clone();
                let mut new_scores = scores.clone();
                new_position[player] = (new_position[player] + throw) % 10;
                new_scores[player] += if new_position[player] == 0 {
                    10
                } else {
                    new_position[player]
                };
                if new_scores[player] >= 21 {
                    wins[player] += count * throw_count;
                } else {
                    *new_positions.entry((new_position, new_scores)).or_insert(0) +=
                        count * throw_count;
                }
            }
        }
        if new_positions.is_empty() {
            break;
        }
        positions = new_positions;
        player = (player + 1) % 2;
    }

    Ok(wins[0].max(wins[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 739785);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 444356092776315);
        Ok(())
    }
}
