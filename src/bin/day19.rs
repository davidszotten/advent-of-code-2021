use anyhow::{Context, Result};
use aoc2021::coor3::Coor3;
use aoc2021::dispatch;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

lazy_static! {
    static ref ALL_ROTATIONS: Vec<Rotation> = Rotation::_all();
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Rotation {
    rot_x: i64,
    rot_y: i64,
    rot_z: i64,
}

impl Rotation {
    fn new(x: i64, y: i64, z: i64) -> Rotation {
        Rotation {
            rot_x: x,
            rot_y: y,
            rot_z: z,
        }
    }
    fn identity() -> Rotation {
        Rotation::new(0, 0, 0)
    }
    fn all() -> &'static [Rotation] {
        &ALL_ROTATIONS
    }
    fn _all() -> Vec<Rotation> {
        let mut seen = HashSet::new();
        let mut res = vec![];
        for rot_x in 0..4 {
            for rot_y in 0..4 {
                for rot_z in 0..4 {
                    let rotation = Rotation::new(rot_x, rot_y, rot_z);
                    if seen.contains(&rotation.identity_action()) {
                        continue;
                    }
                    seen.insert(rotation.identity_action());
                    res.push(rotation);
                }
            }
        }
        res
    }

    fn apply(&self, coor: Coor3) -> Coor3 {
        let mut coor = coor;
        for _ in 0..self.rot_x {
            coor = self._rotate_x(coor);
        }
        for _ in 0..self.rot_y {
            coor = self._rotate_y(coor);
        }
        for _ in 0..self.rot_z {
            coor = self._rotate_z(coor);
        }
        coor
    }

    fn _rotate_x(&self, coor: Coor3) -> Coor3 {
        (coor.x, -coor.z, coor.y).into()
    }

    fn _rotate_y(&self, coor: Coor3) -> Coor3 {
        (coor.z, coor.y, -coor.x).into()
    }

    fn _rotate_z(&self, coor: Coor3) -> Coor3 {
        (-coor.y, coor.x, coor.z).into()
    }

    fn identity_action(&self) -> [Coor3; 3] {
        [
            self.apply((1, 0, 0).into()),
            self.apply((0, 1, 0).into()),
            self.apply((0, 0, 1).into()),
        ]
    }
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

fn offsets(coors: &[Coor3], rotation: &Rotation) -> Vec<(usize, usize, Coor3)> {
    let mut relative = vec![];
    for i in 0..coors.len() {
        for j in i + 1..coors.len() {
            relative.push((i, j, rotation.apply(coors[j]) - rotation.apply(coors[i])));
        }
    }
    relative
}

fn offset(scanner1: &[Coor3], scanner2: &[Coor3]) -> Option<(Coor3, Rotation)> {
    let relative1 = offsets(scanner1, &Rotation::identity());
    for rotation in Rotation::all() {
        let relative2 = offsets(scanner2, rotation);
        let relative_set1 = relative1.iter().map(|(_, _, c)| c).collect::<HashSet<_>>();
        let relative_set2 = relative2.iter().map(|(_, _, c)| c).collect::<HashSet<_>>();
        let intersection = relative_set1.intersection(&relative_set2);
        if intersection.count() >= 6 {
            let mut offsets = HashMap::new();
            for &(i1, _j1, c1) in &relative1 {
                for &(i2, _j2, c2) in &relative2 {
                    if c1 == c2 {
                        let offset1 = scanner1[i1] - rotation.apply(scanner2[i2]);
                        *offsets.entry(offset1).or_insert(0) += 1;
                    }
                }
            }
            let mut entries = offsets.iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
            entries.sort_unstable();
            if !entries.is_empty() && *entries[0].0 >= 12 {
                assert_eq!(entries.len(), 1);
                return Some((*entries[0].1, *rotation));
            }
        }
    }
    None
}

fn part1(input: &str) -> Result<usize> {
    let mut offsets = HashMap::new();
    offsets.insert(0, vec![(Coor3::new(0, 0, 0), Rotation::identity())]);
    let mut scanners = parse(input)?;
    let mut done = vec![0];
    let mut found = true;
    while found {
        found = false;
        for &i in &done.clone() {
            for j in 0..scanners.len() {
                if done.iter().any(|&x| x == j) {
                    continue;
                }
                if let Some((offset, rotation)) = offset(&scanners[i], &scanners[j]) {
                    scanners[j] = scanners[j]
                        .iter()
                        .map(|c| offset + rotation.apply(*c))
                        .collect();
                    done.push(j);
                    found = true;
                }
            }
        }
    }
    assert_eq!(done.len(), scanners.len());
    let mut probes = HashSet::new();
    for scanner in scanners {
        for probe in scanner {
            probes.insert(probe);
        }
    }
    Ok(probes.len())
}

fn part2(input: &str) -> Result<i64> {
    let mut offsets = HashMap::new();
    offsets.insert(0, vec![(Coor3::new(0, 0, 0), Rotation::identity())]);
    let mut scanners = parse(input)?;
    let mut done = vec![0];
    let mut found = true;
    let mut scanner_pos = vec![];
    while found {
        found = false;
        for &i in &done.clone() {
            for j in 0..scanners.len() {
                if done.iter().any(|&x| x == j) {
                    continue;
                }
                if let Some((offset, rotation)) = offset(&scanners[i], &scanners[j]) {
                    scanner_pos.push(offset);
                    scanners[j] = scanners[j]
                        .iter()
                        .map(|c| offset + rotation.apply(*c))
                        .collect();
                    done.push(j);
                    found = true;
                }
            }
        }
    }
    assert_eq!(done.len(), scanners.len());
    let manhattan = |c: Coor3| c.x.abs() + c.y.abs() + c.z.abs();
    let mut max = 0;
    for &d1 in &scanner_pos {
        for &d2 in &scanner_pos {
            max = max.max(manhattan(d2 - d1));
        }
    }
    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../input/day19.sample");

    #[test]
    fn test_offset() -> Result<()> {
        let scanners = parse(TEST_INPUT)?;
        assert_eq!(
            offset(&scanners[0], &scanners[1]).unwrap().0,
            "68,-1246,-43".parse()?
        );
        assert_ne!(
            dbg!(offset(&scanners[0], &scanners[1]).unwrap().1),
            Rotation::identity()
        );

        Ok(())
    }

    #[test]
    fn test_offset2() -> Result<()> {
        let scanners = parse(TEST_INPUT)?;
        offset(&scanners[1], &scanners[4]);
        Ok(())
    }

    #[test]
    fn test_rotation() {
        assert_eq!(
            dbg!(Rotation::all()
                .iter()
                .map(Rotation::identity_action)
                .collect::<HashSet<_>>())
            .len(),
            24
        );
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 79);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 3621);
        Ok(())
    }
}
