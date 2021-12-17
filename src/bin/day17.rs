use anyhow::{Context, Error, Result};
use aoc2021::coor::Coor;
use aoc2021::dispatch;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq)]
struct Target {
    min: Coor,
    max: Coor,
}

impl FromStr for Target {
    type Err = Error;
    fn from_str(s: &str) -> Result<Target> {
        let (_, data) = s.trim().split_once(": ").context("no `: `")?;
        let (x_raw, y_raw) = data.split_once(", ").context("no `, `")?;
        let x_raw = &x_raw[2..];
        let y_raw = &y_raw[2..];
        let (x_min_raw, x_max_raw) = x_raw.split_once("..").context("x dots")?;
        let (y_min_raw, y_max_raw) = y_raw.split_once("..").context("y dots")?;

        let min = Coor::new(x_min_raw.parse()?, y_min_raw.parse()?);
        let max = Coor::new(x_max_raw.parse()?, y_max_raw.parse()?);
        Ok(Target { min, max })
    }
}

fn max_height_if_in_target(v_x: i64, v_y: i64, target: &Target) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut v_x = v_x;
    let mut v_y = v_y;
    let mut max_height: Option<i64> = None;
    let mut been_inside = false;

    let inside =
        |x, y| x >= target.min.x && x <= target.max.x && y >= target.min.y && y <= target.max.y;

    while x <= target.max.x && y >= target.min.y {
        x += v_x;
        y += v_y;
        been_inside |= inside(x, y);
        max_height = match max_height {
            None => Some(y),
            Some(max) => Some(max.max(y)),
        };

        v_x -= v_x.signum();
        v_y -= 1;
    }

    if been_inside {
        max_height
    } else {
        None
    }
}

fn part1(input: &str) -> Result<i64> {
    let target: Target = input.parse()?;
    let mut max_height = 0;
    for x in 0..target.max.x {
        for y in 0..(-target.min.y) {
            if let Some(height) = max_height_if_in_target(x, y, &target) {
                max_height = max_height.max(height);
            }
        }
    }
    Ok(max_height)
}

fn part2(input: &str) -> Result<usize> {
    let target: Target = input.parse()?;
    let mut count = 0;
    for x in 0..(target.max.x + 1) {
        for y in target.min.y..(-target.min.y + 1) {
            if max_height_if_in_target(x, y, &target).is_some() {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(
            TEST_INPUT.parse::<Target>()?,
            Target {
                min: Coor::new(20, -10),
                max: Coor::new(30, -5)
            }
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 45);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 112);
        Ok(())
    }
}
