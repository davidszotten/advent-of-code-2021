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
    fn eq(&self, other: Rotation) -> bool {
        self.identity_action() == other.identity_action()
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
                return *rotation;
            }
        }
        unreachable!();
    }

    fn sub(&self, other: Rotation) -> Rotation {
        // not enough brain to figure out the math. brute force instead
        for rotation in Rotation::all() {
            if rotation.add(other).eq(*self) {
                return *rotation;
            }
        }
        unreachable!();
    }

    fn neg(&self) -> Rotation {
        Rotation::identity().sub(*self)
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
            for &(i1, _j1, c1) in &relative1 {
                for &(i2, _j2, c2) in &relative2 {
                    if c1 == c2 {
                        // dbg!(scanner1[i1], scanner2[i2]);
                        let offset1 = scanner1[i1] - rotation.apply(scanner2[i2]);
                        // let offset2 = scanner1[j1] - rotation.apply(scanner2[j2]);
                        *offsets.entry(offset1).or_insert(0) += 1;
                        // *offsets.entry(offset2).or_insert(0) += 1;
                    }
                }
            }
            let mut entries = offsets.iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
            entries.sort_unstable();
            // dbg!(&entries);
            if entries.len() > 0 && *entries[0].0 >= 12 {
                assert_eq!(entries.len(), 1);
                return Some((*entries[0].1, *rotation));
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
            if let Some(res) = offset(&scanners[i], &scanners[j]) {
                relations.insert((i, j), res);
                let o1 = offset(&scanners[i], &scanners[j]).unwrap();
                let o2 = offset(&scanners[j], &scanners[i]).unwrap();
                // dbg!(o1.1.add(o2.1));
                // dbg!(o1.0 + o1.1.apply(o2.0));
                assert_eq!(o1.1, o2.1.neg());
                assert_eq!(o1.0, -o2.1.neg().apply(o2.0));

                // println!();
                // panic!();
            }
        }
    }
    // let mut missing: HashSet<_> = (1..scanners.len()).collect();
    // for entry in missing.iter().cloned() {
    // dbg!(&relations.keys());
    let mut found = true;
    while found {
        found = false;
        'foo: for entry in 1..scanners.len() {
            if offsets.contains_key(&entry) {
                continue;
            }

            let foo = offsets
                .keys()
                .filter(|k| relations.contains_key(&(**k, entry)))
                .collect::<Vec<_>>();
            if entry == 18 && foo.len() > 1 {
                println!("{:?} {}", foo, entry);
                dbg!(offsets.get(&18));
                dbg!(offsets.get(&12));
                dbg!(offset(&scanners[0], &scanners[18]));
                dbg!(offset(&scanners[12], &scanners[18]));
            }

            for (known, &(offset1, rot1)) in &offsets.clone() {
                if let Some(&(offset2, rot2)) = relations.get(&(*known, entry)) {
                    offsets.insert(entry, (offset1 + rot1.apply(offset2), rot1.sub(rot2)));
                    found = true;
                    break 'foo;
                }
                if relations.get(&(entry, *known)).is_some() {
                    let (offset2b, rot2b) = offset(&scanners[*known], &scanners[entry]).unwrap();
                    offsets.insert(
                        entry,
                        (offset1 + rot1.neg().apply(offset2b), rot1.sub(rot2b).neg()),
                    );
                    found = true;
                    break 'foo;
                }
            }
        }
    }

    // assert_eq!(offsets[&0].0, Coor3::new(0, 0, 0));
    // assert_eq!(offsets[&1].0, Coor3::new(68, -1246, -43));
    // assert_eq!(offsets[&2].0, Coor3::new(1105, -1205, 1229));
    // assert_eq!(offsets[&3].0, Coor3::new(-92, -2380, -20));
    // assert_eq!(offsets[&4].0, Coor3::new(-20, -1133, 1061));
    // dbg!(&offsets);

    // let (offset, rotation) = offsets[&1];
    // dbg!(offset, rotation);
    // let coor = Coor3::new(605, 423, 415);
    // // -537,-823,-458
    // dbg!(offset + rotation.apply(coor));

    // assert_eq!(
    //     offsets[&4],
    //     (Coor3::new(-105, 1139, -25), Rotation::new(0, 0, 3))
    // );
    // assert_eq!(
    //     offsets[&15],
    //     (Coor3::new(-52, -1077, 57), Rotation::new(1, 2, 3))
    // );
    // assert_eq!(
    //     offsets[&12],
    //     (Coor3::new(1124, 79, -1224), Rotation::new(0, 3, 2))
    // );
    // dbg!(&offsets[&12]);

    let mut probes = HashSet::new();
    for (idx, scanner) in scanners.iter().enumerate() {
        let (offset, rotation) = offsets[&idx];
        for probe in scanner {
            probes.insert(offset + rotation.neg().apply(*probe));
        }
    }
    let mut probes2 = HashSet::new();
    for (scanner_idx, (offset, rotation)) in offsets {
        for probe in &scanners[scanner_idx] {
            probes2.insert(offset + rotation.neg().apply(*probe));
        }
    }
    assert_eq!(probes, probes2);

    // dbg!(probes.intersection(&target).count());
    // dbg!(&probes);

    Ok(probes.len())
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
        // assert_eq!(Rotation::all().len(), 24);
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
    fn test_rotation_add() {
        assert!((Rotation::new(1, 0, 0).add(Rotation::new(1, 0, 0))).eq(Rotation::new(2, 0, 0)));
    }

    #[test]
    fn test_rotation_sub() {
        assert!(dbg!(Rotation::new(1, 0, 0).sub(Rotation::new(1, 0, 0))).eq(Rotation::new(0, 0, 0)));
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 79);
        Ok(())
    }
}
