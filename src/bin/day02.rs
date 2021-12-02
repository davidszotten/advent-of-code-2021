use anyhow::{anyhow, bail, Error, Result};
use aoc2021::dispatch;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug, PartialEq)]
struct Command {
    direction: Direction,
    distance: i32,
}

impl Command {
    fn new(direction: Direction, distance: i32) -> Self {
        Command {
            direction,
            distance,
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => bail!("invalid direction `{}`", s),
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        let (direction_raw, distance_raw) = s
            .split_once(' ')
            .ok_or(anyhow!("no space found: `{}`", s))?;
        Ok(Command::new(
            Direction::try_from(direction_raw)?,
            distance_raw
                .parse()
                .map_err(|e| anyhow!("{} (`{}`)", e, distance_raw))?,
        ))
    }
}

fn parse(input: &str) -> Result<Vec<Command>> {
    input
        .trim()
        .split('\n')
        .map(|s| Command::try_from(s))
        .collect()
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<i32> {
    use Direction::*;
    let mut x = 0;
    let mut y = 0;
    for command in parse(input)?.iter() {
        let n = command.distance;
        match command.direction {
            Forward => x += n,
            Down => y += n,
            Up => y -= n,
        }
    }
    Ok(x * y)
}

fn part2(input: &str) -> Result<i32> {
    use Direction::*;
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for command in parse(input)?.iter() {
        let n = command.distance;
        match command.direction {
            Down => aim += n,
            Up => aim -= n,
            Forward => {
                x += n;
                y += aim * n
            }
        }
    }
    Ok(x * y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_parse() -> Result<()> {
        use Direction::*;
        assert_eq!(
            parse(TEST_INPUT)?,
            vec![
                Command::new(Forward, 5),
                Command::new(Down, 5),
                Command::new(Forward, 8),
                Command::new(Up, 3),
                Command::new(Down, 8),
                Command::new(Forward, 2),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 150);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 900);
        Ok(())
    }
}
