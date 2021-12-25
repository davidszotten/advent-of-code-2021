use anyhow::{Context, Result};
use aoc2021::coor3::{Axis, Coor3};
use aoc2021::dispatch;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Step {
    on: bool,
    min: Coor3,
    max: Coor3,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Owner {
    One(Step),
    Both,
}

impl Owner {
    fn merge(&self, other: &Owner) -> Owner {
        use Owner::*;
        match (*self, *other) {
            (One(s1), One(s2)) => {
                assert_eq!(s1, s2);
                One(s1)
            }
            (One(s), Both) => One(s),
            (Both, One(s)) => One(s),
            (Both, Both) => Both,
        }
    }
}

fn split(s: &str) -> Result<(i64, i64)> {
    let (a, b) = s.split_once("..").context("dots")?;
    Ok((
        a[2..].parse().context("a")?,
        b.parse().context(format!("b `{}`", b))?,
    ))
}
fn parse_line(s: &str) -> Result<Step> {
    let (on_raw, rest) = s.trim().split_once(' ').context("space")?;
    let on = on_raw == "on";
    let (x_raw, rest) = rest.split_once(',').context("x")?;
    let (y_raw, z_raw) = rest.split_once(',').context("y")?;
    let x = split(x_raw)?;
    let y = split(y_raw)?;
    let z = split(z_raw)?;
    Ok(Step {
        on,
        min: Coor3::new(x.0, y.0, z.0),
        max: Coor3::new(x.1, y.1, z.1),
    })
}

fn parse(s: &str) -> Result<Vec<Step>> {
    s.trim().lines().map(parse_line).collect::<Result<Vec<_>>>()
}

fn part1(input: &str) -> Result<i64> {
    let mut steps = parse(input)?;
    steps = steps
        .iter()
        .map(|s| Step {
            min: Coor3::new(
                s.min.x.clamp(-50, 51),
                s.min.y.clamp(-50, 51),
                s.min.z.clamp(-50, 51),
            ),
            max: Coor3::new(
                s.max.x.clamp(-51, 50),
                s.max.y.clamp(-51, 50),
                s.max.z.clamp(-51, 50),
            ),
            on: s.on,
        })
        .collect();
    Ok(apply(&steps))
}

fn sweep_cut(entries: &[Step], axis: Axis) -> Vec<(Owner, Coor3, Coor3)> {
    let mut sweep: Vec<_> = entries
        .iter()
        .flat_map(|s| [(s.min.axis(axis), 1, s), (s.max.axis(axis), 0, s)])
        .collect();
    sweep.sort_by_key(|&(v, start, _)| (v, -start));
    let mut res = vec![];
    let mut double = false;
    let mut sorted_entries = vec![];
    let mut sweep_peek = sweep.iter().peekable();
    while let Some(&(pos, start, step)) = sweep_peek.next() {
        if start == 1 {
            if sorted_entries.get(0) == Some(step) {
                double = true;
            }
            if sorted_entries.get(0) != Some(step) {
                sorted_entries.push(*step)
            }
        } else {
            let stack_pos = sorted_entries.iter().position(|&s| &s == step).unwrap();
            if double {
                double = false;
            } else {
                sorted_entries.remove(stack_pos);
            }
        }

        if let Some(next_pos) = sweep_peek.peek() {
            let start_adjust = if start == 0 { 1 } else { 0 };
            let end_adjust = if next_pos.1 == 1 { 1 } else { 0 };
            let left = pos + start_adjust;
            let right = next_pos.0 - end_adjust;
            if left <= right {
                let owner = if sorted_entries.len() == 1 && !double {
                    Owner::One(sorted_entries[0])
                } else {
                    if double {
                        assert_eq!(sorted_entries.len(), 1);
                    } else {
                        assert_eq!(sorted_entries.len(), 2);
                    }
                    Owner::Both
                };
                if axis == Axis::X {
                    res.push((owner.merge(&owner), axis.coor() * left, axis.coor() * right));
                } else {
                    let lower_dim = sweep_cut(&sorted_entries, axis.prev());
                    for (owner, min, max) in lower_dim {
                        res.push((
                            owner.merge(&owner),
                            min + axis.coor() * left,
                            max + axis.coor() * right,
                        ));
                    }
                }
            }
        }
    }
    res
}

/*
 aaaaaa
    bbbbbbb

 min min max max

*/
fn intersects(left: Step, right: Step) -> bool {
    (left.min.x <= right.max.x && right.min.x <= left.max.x)
        && (left.min.y <= right.max.y && right.min.y <= left.max.y)
        && (left.min.z <= right.max.z && right.min.z <= left.max.z)
}

fn apply(steps: &[Step]) -> i64 {
    let mut current: HashSet<Step> = HashSet::new();
    for step in steps {
        let mut new = HashSet::new();
        for entry in current {
            if intersects(entry, *step) {
                for (owner, min, max) in sweep_cut(&[*step, entry], Axis::Z) {
                    let on = match owner {
                        Owner::Both => step.on,
                        Owner::One(s) => s.on,
                    };
                    let sweep = Step { min, max, on };
                    if owner == Owner::One(entry) {
                        new.insert(sweep);
                    }
                }
            } else {
                new.insert(entry);
            }
        }
        if step.on {
            new.insert(*step);
        }
        current = new;
    }
    assert!(current
        .iter()
        .combinations(2)
        .all(|v| v[0] == v[1] || !intersects(*v[0], *v[1])));
    current
        .iter()
        .map(|s| (s.on, s.max - s.min + Coor3::new(1, 1, 1)))
        .map(|(on, c)| c.x * c.y * c.z * (if on { 1 } else { -0 }))
        .sum()
}

fn part2(input: &str) -> Result<i64> {
    let steps = parse(input)?;
    Ok(apply(&steps))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sweep_cut_x(entries: &[Step]) -> Vec<(Owner, i64, i64)> {
        sweep_cut(entries, Axis::X)
            .iter()
            .map(|t| (t.0, (t.1.x), (t.2.x)))
            .collect()
    }

    fn sweep_cut_y(entries: &[Step]) -> Vec<(Owner, (i64, i64), (i64, i64))> {
        sweep_cut(entries, Axis::Y)
            .iter()
            .map(|t| (t.0, (t.1.x, t.1.y), (t.2.x, t.2.y)))
            .collect()
    }

    fn sweep_cut_z(entries: &[Step]) -> Vec<(Owner, Coor3, Coor3)> {
        sweep_cut(entries, Axis::Z)
    }

    const TEST_INPUT: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const TEST_INPUT2: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    const TEST_INPUT3: &str = include_str!("../../input/day22.sample");
    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 39);
        assert_eq!(part1(TEST_INPUT2)?, 590784);
        Ok(())
    }

    /*

    ******
       xxxxxxxx
    ^  ^ ^    ^
    12345678901
    */
    #[test]
    fn test_sweep_cut_x1() -> Result<()> {
        let steps = parse(
            "on x=1..6,y=0..0,z=0..0
on x=4..11,y=0..0,z=0..0",
        )?;
        assert_eq!(
            sweep_cut_x(&steps),
            vec![
                (Owner::One(steps[0]), 1, 3),
                (Owner::Both, 4, 6),
                (Owner::One(steps[1]), 7, 11)
            ]
        );
        assert_eq!(
            sweep_cut_x(&steps),
            vec![
                (Owner::One(steps[0]), 1, 3),
                (Owner::Both, 4, 6),
                (Owner::One(steps[1]), 7, 11)
            ]
        );
        Ok(())
    }

    /*

    ******
         xxxxxx
    ^    ^    ^
    12345678901
    */
    #[test]
    fn test_sweep_cut_x2() -> Result<()> {
        let steps = parse(
            "on x=1..6,y=0..0,z=0..0
on x=6..11,y=0..0,z=0..0",
        )?;
        assert_eq!(
            sweep_cut_x(&steps),
            vec![
                (Owner::One(steps[0]), 1, 5),
                (Owner::Both, 6, 6),
                (Owner::One(steps[1]), 7, 11)
            ]
        );
        Ok(())
    }

    /*

    ******
    xxxxx
    ^   ^^
    123456
    */
    #[test]
    fn test_sweep_cut_x3() -> Result<()> {
        let steps = parse(
            "on x=1..6,y=0..0,z=0..0
on x=1..5,y=0..0,z=0..0",
        )?;
        assert_eq!(
            sweep_cut_x(&steps),
            vec![(Owner::Both, 1, 5), (Owner::One(steps[0]), 6, 6)]
        );
        Ok(())
    }

    /*

    *****
    xxxxx
    ^   ^
    12345
    */
    #[test]
    fn test_sweep_cut_x4() -> Result<()> {
        let steps = parse(
            "on x=1..5,y=0..0,z=0..0
on x=1..5,y=0..0,z=0..0",
        )?;
        assert_eq!(sweep_cut_x(&steps), vec![(Owner::Both, 1, 5)]);
        Ok(())
    }

    #[test]
    fn test_sweep_cut_y1() -> Result<()> {
        /*

        9  *****
        8  *****
        7  ***xxooo
        6  ***xxooo
        5  ***xxooo
        4  *****
        3  *****

           ^  ^^
           1  45  8

         */

        let steps = parse(
            "on x=1..5,y=3..9,z=0..0
on x=4..8,y=5..7,z=0..0",
        )?;
        assert_eq!(
            sweep_cut_y(&steps),
            vec![
                (Owner::One(steps[0]), (1, 3), (5, 4)),
                (Owner::One(steps[0]), (1, 5), (3, 7)),
                (Owner::Both, (4, 5), (5, 7)),
                (Owner::One(steps[1]), (6, 5), (8, 7)),
                (Owner::One(steps[0]), (1, 8), (5, 9)),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_sweep_cut_y2() -> Result<()> {
        /*

        9
        8
        7
        6
        5    ooo
        4  **Xoo
        3  ***

           ^ ^ ^
           1 345

         */

        let steps = parse(
            "on x=1..3,y=3..4,z=0..0
on x=3..5,y=4..5,z=0..0",
        )?;
        assert_eq!(
            sweep_cut_y(&steps),
            vec![
                (Owner::One(steps[0].clone()), (1, 3), (3, 3)),
                (Owner::One(steps[0].clone()), (1, 4), (2, 4)),
                (Owner::Both, (3, 4), (3, 4)),
                (Owner::One(steps[1].clone()), (4, 4), (5, 4)),
                (Owner::One(steps[1].clone()), (3, 5), (5, 5)),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_sweep_y4() -> Result<()> {
        /*

        1  *o

           ^^
           12

         */

        let steps = parse(
            "on x=1..1,y=0..0,z=0..0
on x=1..2,y=0..0,z=0..0",
        )?;
        assert_eq!(
            sweep_cut_y(&steps),
            vec![
                (Owner::Both, (1, 0), (1, 0)),
                (Owner::One(steps[1].clone()), (2, 0), (2, 0)),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_sweep_z1() -> Result<()> {
        /*

        1  *o

           ^^
           12

         */

        let steps = parse(
            "on x=0..0,y=0..0,z=1..1
on x=0..0,y=0..0,z=1..2",
        )?;
        assert_eq!(
            sweep_cut_z(&steps),
            vec![
                (Owner::Both, Coor3::new(0, 0, 1), Coor3::new(0, 0, 1)),
                (
                    Owner::One(steps[1].clone()),
                    Coor3::new(0, 0, 2),
                    Coor3::new(0, 0, 2)
                ),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_intersects1() -> Result<()> {
        let steps = parse(
            "on x=1..3,y=4..5,z=0..0
on x=3..5,y=5..6,z=0..0",
        )?;
        assert!(intersects(steps[0], steps[1]));

        Ok(())
    }

    #[test]
    fn test_intersects2() -> Result<()> {
        let steps = parse(
            "on x=-20..34,y=-40..6,z=-44..1
on x=-57795..-6158,y=29564..72030,z=20435..90618",
        )?;
        assert!(!intersects(steps[0], steps[1]));

        Ok(())
    }

    #[test]
    fn test_intersects3() -> Result<()> {
        let steps = parse(
            "on x=1..3,y=0..0,z=0..0
on x=3..5,y=0..0,z=0..0",
        )?;
        assert!(intersects(steps[0], steps[1]));

        Ok(())
    }

    #[test]
    fn test_intersects4() -> Result<()> {
        let steps = parse(
            "on x=2..3,y=0..0,z=0..0
on x=3..3,y=0..0,z=0..0",
        )?;
        assert!(intersects(steps[0], steps[1]));

        Ok(())
    }

    #[test]
    fn test_part2a() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 39);
        Ok(())
    }

    /*

     xxx  o*o  xxx
     xxx  o o  xxx
     xxx  o*o  xxx

    */

    #[test]
    fn test_part2c() -> Result<()> {
        assert_eq!(
            part2(
                "on x=1..3,y=1..3,z=1..3
off x=2..2,y=2..2,z=2..2"
            )?,
            26
        );
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        assert_eq!(part2(TEST_INPUT3)?, 2758514936282235);
        Ok(())
    }

    #[test]
    fn test_part2d() -> Result<()> {
        let input = "on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15";
        let input =
            itertools::Itertools::intersperse(input.lines().take(2), "\n").collect::<String>();
        assert_eq!(part1(&input)? as i64, part2(&input)?,);
        Ok(())
    }
}
