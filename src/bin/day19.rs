use anyhow::{Context, Result};
use aoc2021::coor3::Coor3;
use aoc2021::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<Vec<Vec<Coor3>>> {
    let mut res = vec![];
    for scanner in input.trim().split("\n\n") {
        let (_header, raw_coors) = scanner.split_once('\n').context("no newline")?;
        let coors: Vec<Coor3> = raw_coors
            .lines()
            .map(|l| l.parse())
            .collect::<Result<_>>()?;
        res.push(coors);
    }
    Ok(res)
}

fn offsets(coors: &[Coor3], rotation: &dyn Fn(Coor3) -> Coor3) -> Vec<(usize, usize, Coor3)> {
    let mut relative = vec![];
    for i in 0..coors.len() {
        for j in i + 1..coors.len() {
            relative.push((i, j, rotation(coors[j]) - rotation(coors[i])));
        }
    }
    relative
}

fn rotate(coor: Coor3, amount: usize) -> Coor3 {
    let mut coor = coor;
    for _ in 0..amount {
        coor = Coor3::new(coor.y, coor.z, coor.x);
    }
    coor
}

fn flip(coor: Coor3, x: i64, y: i64, z: i64) -> Coor3 {
    Coor3::new(x * coor.x, y * coor.y, z * coor.z)
}

fn offset(scanner1: &[Coor3], scanner2: &[Coor3]) -> Option<Coor3> {
    let relative1 = offsets(scanner1, &|c| c);
    for rot_amount in 0..2 {
        for flipx in [1, -1] {
            for flipy in [1, -1] {
                for flipz in [1, -1] {
                    let rotation = |c| rotate(flip(c, flipx, flipy, flipz), rot_amount);
                    let relative2 = offsets(scanner2, &rotation);
                    let relative_set1 = relative1.iter().map(|(_, _, c)| c).collect::<HashSet<_>>();
                    let relative_set2 = relative2.iter().map(|(_, _, c)| c).collect::<HashSet<_>>();
                    let intersection: Vec<_> = relative_set1.intersection(&relative_set2).collect();
                    if intersection.len() >= 12 {
                        for &(i1, j1, c1) in &relative1 {
                            for &(i2, j2, c2) in &relative2 {
                                if c1 == c2 {
                                    dbg!(scanner1[i1] - rotation(scanner2[i2]));
                                    dbg!(scanner1[j1] - rotation(scanner2[j2]));
                                }
                            }
                        }
                        // intersection.sort();
                        // dbg!(relative1.len());
                        // dbg!(relative2.len());
                        dbg!(rot_amount, flipx, flipy, flipz);
                    }
                }
            }
        }
    }
    todo!()
}

fn part1(input: &str) -> Result<usize> {
    let scanners = parse(input)?;
    offset(&scanners[0], &scanners[1]);
    Ok(scanners.len())
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../input/day19.sample");

    #[test]
    fn test_offset() -> Result<()> {
        let scanners = parse(TEST_INPUT)?;
        assert_eq!(
            offset(&scanners[0], &scanners[1]),
            Some("68,-1246,-43".parse()?)
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 79);
        Ok(())
    }
}
