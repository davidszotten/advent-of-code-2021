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
            // dbg!(entry);
            // dbg!(offsets.keys());
            for (known, &(offset1, rot1)) in &offsets.clone() {
                if let Some(&(offset2, rot2)) = relations.get(&(*known, entry)) {
                    // println!("{} {}", entry, known);
                    // missing.insert(entry);
                    // if entry == 1 || entry == 4 {
                    //     // 4: -20,-1133,1061
                    //     // 1: 68,-1246,-43
                    //     // dbg!(
                    //     // offset1 + offset2,
                    //     // offset1 + rot1.apply(offset2),
                    //     // offset1 + rot1._neg().apply(offset2),
                    //     // );
                    // }
                    offsets.insert(entry, (offset1 + rot1.apply(offset2), rot1.sub(rot2)));
                    // offsets.insert(entry, (offset1 + rot1.apply(offset2), rot1));
                    found = true;
                    break 'foo;
                    // dbg!(entry, known, off, rot);
                    // todo!();
                }
                if relations.get(&(entry, *known)).is_some() {
                    let (offset2b, rot2b) = offset(&scanners[*known], &scanners[entry]).unwrap();
                    // dbg!(offset1);
                    // dbg!(offset2, offset2b);
                    // offsets.insert(entry, (offset1 + rot1.neg().apply(offset2b), rot2b));
                    offsets.insert(
                        entry,
                        // (offset1 + rot1.neg().apply(offset2b), rot1.neg().sub(rot2)),
                        (offset1 + rot1.neg().apply(offset2b), rot1.sub(rot2b).neg()),
                    );
                    // dbg!(entry);
                    // dbg!(offset1, offset2b, rot1, rot2b);
                    // dbg!(offset1 - rot1.apply(offset2b));
                    // dbg!(offset1 - rot1.neg().apply(offset2b));
                    // dbg!(offset1 - rot2b.apply(offset2b));
                    // dbg!(offset1 - rot2b.neg().apply(offset2b));
                    // offsets.insert(entry, (offset1 + rot1.neg().apply(offset2), rot2));
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

    let mut probes = HashSet::new();
    // for (scanner_idx, (offset, rotation)) in offsets {
    // for probe in &scanners[scanner_idx] {
    for (idx, scanner) in scanners.iter().enumerate() {
        let (offset, rotation) = offsets[&idx];
        for probe in scanner {
            probes.insert(offset + rotation.neg().apply(*probe));
        }
    }

    let target = HashSet::from([
        Coor3::new(-892, 524, 684),
        Coor3::new(-876, 649, 763),
        Coor3::new(-838, 591, 734),
        Coor3::new(-789, 900, -551),
        Coor3::new(-739, -1745, 668),
        Coor3::new(-706, -3180, -659),
        Coor3::new(-697, -3072, -689),
        Coor3::new(-689, 845, -530),
        Coor3::new(-687, -1600, 576),
        Coor3::new(-661, -816, -575),
        Coor3::new(-654, -3158, -753),
        Coor3::new(-635, -1737, 486),
        Coor3::new(-631, -672, 1502),
        Coor3::new(-624, -1620, 1868),
        Coor3::new(-620, -3212, 371),
        Coor3::new(-618, -824, -621),
        Coor3::new(-612, -1695, 1788),
        Coor3::new(-601, -1648, -643),
        Coor3::new(-584, 868, -557),
        Coor3::new(-537, -823, -458),
        Coor3::new(-532, -1715, 1894),
        Coor3::new(-518, -1681, -600),
        Coor3::new(-499, -1607, -770),
        Coor3::new(-485, -357, 347),
        Coor3::new(-470, -3283, 303),
        Coor3::new(-456, -621, 1527),
        Coor3::new(-447, -329, 318),
        Coor3::new(-430, -3130, 366),
        Coor3::new(-413, -627, 1469),
        Coor3::new(-345, -311, 381),
        Coor3::new(-36, -1284, 1171),
        Coor3::new(-27, -1108, -65),
        Coor3::new(7, -33, -71),
        Coor3::new(12, -2351, -103),
        Coor3::new(26, -1119, 1091),
        Coor3::new(346, -2985, 342),
        Coor3::new(366, -3059, 397),
        Coor3::new(377, -2827, 367),
        Coor3::new(390, -675, -793),
        Coor3::new(396, -1931, -563),
        Coor3::new(404, -588, -901),
        Coor3::new(408, -1815, 803),
        Coor3::new(423, -701, 434),
        Coor3::new(432, -2009, 850),
        Coor3::new(443, 580, 662),
        Coor3::new(455, 729, 728),
        Coor3::new(456, -540, 1869),
        Coor3::new(459, -707, 401),
        Coor3::new(465, -695, 1988),
        Coor3::new(474, 580, 667),
        Coor3::new(496, -1584, 1900),
        Coor3::new(497, -1838, -617),
        Coor3::new(527, -524, 1933),
        Coor3::new(528, -643, 409),
        Coor3::new(534, -1912, 768),
        Coor3::new(544, -627, -890),
        Coor3::new(553, 345, -567),
        Coor3::new(564, 392, -477),
        Coor3::new(568, -2007, -577),
        Coor3::new(605, -1665, 1952),
        Coor3::new(612, -1593, 1893),
        Coor3::new(630, 319, -379),
        Coor3::new(686, -3108, -505),
        Coor3::new(776, -3184, -501),
        Coor3::new(846, -3110, -434),
        Coor3::new(1135, -1161, 1235),
        Coor3::new(1243, -1093, 1063),
        Coor3::new(1660, -552, 429),
        Coor3::new(1693, -557, 386),
        Coor3::new(1735, -437, 1738),
        Coor3::new(1749, -1800, 1813),
        Coor3::new(1772, -405, 1572),
        Coor3::new(1776, -675, 371),
        Coor3::new(1779, -442, 1789),
        Coor3::new(1780, -1548, 337),
        Coor3::new(1786, -1538, 337),
        Coor3::new(1847, -1591, 415),
        Coor3::new(1889, -1729, 1762),
        Coor3::new(1994, -1805, 1792),
    ]);

    // dbg!(probes.intersection(&target).count());
    assert_eq!(probes, target);
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
