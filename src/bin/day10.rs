use anyhow::{bail, Context, Result};
use aoc2021::dispatch;
use std::collections::VecDeque;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn pair(c: char) -> Result<char> {
    Ok(match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => bail!("invalid char for pair: `{}`", c),
    })
}

fn matches(open: char, close: char) -> Result<bool> {
    pair(open).map(|p| p == close)
}

fn invalid_score(c: char) -> Result<u64> {
    Ok(match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => bail!("invalid char for score: `{}`", c),
    })
}

fn invalid_points(line: &str) -> Result<u64> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push_front(c),
            ')' | ']' | '}' | '>' => {
                let top = stack.pop_front().context("stack empty")?;
                if !matches(top, c)? {
                    return invalid_score(c);
                }
            }
            _ => bail!("invalid char"),
        }
    }
    Ok(0)
}

fn incomplete_score(c: char) -> Result<u64> {
    Ok(match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => bail!("invalid char for score: `{}`", c),
    })
}

fn incomplete_points(line: &str) -> Result<u64> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push_front(c),
            ')' | ']' | '}' | '>' => {
                let top = stack.pop_front().context("stack empty")?;
                if !matches(top, c)? {
                    bail!("invalid line");
                }
            }
            _ => bail!("invalid char"),
        }
    }
    let mut score = 0;
    while let Some(top) = stack.pop_front() {
        score *= 5;
        score += incomplete_score(pair(top)?)?;
    }
    Ok(score)
}

fn part1(input: &str) -> Result<u64> {
    Ok(input
        .lines()
        .map(invalid_points)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum())
}

fn part2(input: &str) -> Result<u64> {
    let mut scores = input
        .lines()
        .filter(|l| matches!(invalid_points(l), Ok(0)))
        .map(incomplete_points)
        .collect::<Result<Vec<_>>>()?;
    scores.sort_unstable();
    scores
        .get(scores.len() / 2)
        .context("no score found")
        .map(|s| *s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 26397);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 288957);
        Ok(())
    }
}
