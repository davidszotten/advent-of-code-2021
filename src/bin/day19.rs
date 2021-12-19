use anyhow::{Context, Result};
use aoc2021::coor3::Coor3;
use aoc2021::dispatch;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Rotation {
    rot_x: i64,
    rot_y: i64,
    rot_z: i64,
}

impl Rotation {
    fn identity() -> Rotation {
        Rotation {
            rot_x: 0,
            rot_y: 0,
            rot_z: 0,
        }
    }
    fn all() -> Vec<Rotation> {
        let mut res = vec![];
        for rot_x in 0..4 {
            for rot_y in 0..4 {
                for rot_z in 0..4 {
                    res.push(Rotation {
                        rot_x,
                        rot_y,
                        rot_z,
                    });
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

    fn add(&self, other: Rotation) -> Rotation {
        // not enough brain to figure out the math. brute force instead
        let one = self.identity_action();
        let both = [
            other.apply(one[0]),
            other.apply(one[1]),
            other.apply(one[2]),
        ];
        for rotation in Rotation::all() {
            if rotation.identity_action() == both {
                return rotation;
            }
        }
        unreachable!();
    }

    // fn _rotate(&self, coor: Coor3) -> Coor3 {
    //     let mut coor = coor;
    //     for _ in 0..self.rotation {
    //         coor = Coor3::new(coor.y, coor.z, coor.x);
    //     }
    //     coor
    // }

    // fn _flip(&self, coor: Coor3) -> Coor3 {
    //     Coor3::new(
    //         self.flip_x * coor.x,
    //         self.flip_y * coor.y,
    //         self.flip_z * coor.z,
    //     )
    // }
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
        let relative2 = offsets(scanner2, &rotation);
        let relative_set1 = relative1.iter().map(|(_, _, c)| c).collect::<HashSet<_>>();
        let relative_set2 = relative2.iter().map(|(_, _, c)| c).collect::<HashSet<_>>();
        let intersection: Vec<_> = relative_set1.intersection(&relative_set2).collect();
        // dbg!(intersection.len());

        // if intersection.len() >= 12 {
        if intersection.len() >= 6 {
            // if true {
            let mut offsets = HashMap::new();
            for &(i1, j1, c1) in &relative1 {
                for &(i2, j2, c2) in &relative2 {
                    if c1 == c2 {
                        let offset1 = scanner1[i1] - rotation.apply(scanner2[i2]);
                        let offset2 = scanner1[j1] - rotation.apply(scanner2[j2]);
                        *offsets.entry(offset1).or_insert(0) += 1;
                        *offsets.entry(offset2).or_insert(0) += 1;
                    }
                }
            }
            let mut entries = offsets.iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
            entries.sort_unstable();
            if entries.len() > 0 && *entries[0].0 >= 12 {
                assert_eq!(entries.len(), 1);
                return Some((*entries[0].1, rotation));
            }
        }
    }
    None
}

fn part1(input: &str) -> Result<usize> {
    let mut relations = HashMap::new();
    let mut offsets = HashMap::new();
    offsets.insert(0, (Coor3::new(0, 0, 0), Rotation::identity()));
    let scanners = parse(input)?;
    for i in 0..scanners.len() {
        for j in i + 1..scanners.len() {
            // dbg!(i, j, offset(&scanners[i], &scanners[j]));
            if let Some(res) = offset(&scanners[i], &scanners[j]) {
                relations.insert((i, j), res);
            }
        }
    }
    // let mut missing: HashSet<_> = (1..scanners.len()).collect();
    // for entry in missing.iter().cloned() {
    dbg!(&relations);
    let mut found = true;
    while found {
        found = false;
        for entry in 1..scanners.len() {
            if offsets.contains_key(&entry) {
                continue;
            }
            for (known, &(offset1, rot1)) in &offsets.clone() {
                if let Some(&(offset2, rot2)) = relations.get(&(*known, entry)) {
                    println!("{} {}", entry, known);
                    // missing.insert(entry);
                    if entry == 4 {
                        dbg!(offset1, offset2);
                    }
                    offsets.insert(entry, (offset1 + offset2, rot1.add(rot2)));
                    found = true;
                    break;
                    // dbg!(entry, known, off, rot);
                    // todo!();
                }
            }
        }
    }
    // dbg!(offsets);
    // while offsets.len() < scanners.len() - 1 {
    //     for i in 0..scanners.len() {
    //         if offsets.contains(&i) {continue}
    //         // i is unset. who can we connect it to
    //         for (rel, offset) in relations {
    //             if rel.0 == i
    //         }
    //     }

    // }
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
            dbg!(offset(&scanners[0], &scanners[1]).unwrap()).0,
            "68,-1246,-43".parse()?
        );
        let cmp = offset(&scanners[1], &scanners[0]).unwrap();
        dbg!(cmp.0.apply(cmp.1));
        // dbg!(&scanners[2]);
        // for i in [0, 1, 3, 4] {
        // dbg!(&scanners[i].len(), &scanners[2].len());
        // offset(&scanners[i], &scanners[2]);
        // }
        // assert!(false);
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
}
