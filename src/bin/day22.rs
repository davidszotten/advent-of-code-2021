use anyhow::{Context, Result};
use aoc2021::coor3::Coor3;
use aoc2021::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Step {
    on: bool,
    min: Coor3,
    max: Coor3,
}

fn split(s: &str) -> Result<(i64, i64)> {
    let (a, b) = s.split_once("..").context("dots")?;
    Ok((a[2..].parse().context("a")?, b.parse().context("b")?))
}
fn parse_line(s: &str) -> Result<Step> {
    let (on_raw, rest) = s.split_once(' ').context("space")?;
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

fn part1(input: &str) -> Result<usize> {
    let steps = parse(input)?;
    let mut cubes = HashSet::new();
    for (i, step) in steps.into_iter().enumerate() {
        dbg!(i);
        for x in step.min.x.max(-50)..=step.max.x.min(50) {
            for y in step.min.y.max(-50)..=step.max.y.min(50) {
                for z in step.min.z.max(-50)..=step.max.z.min(50) {
                    if step.on {
                        cubes.insert(Coor3::new(x, y, z));
                    } else {
                        cubes.remove(&Coor3::new(x, y, z));
                    }
                }
            }
        }
    }
    // dbg!(steps);
    Ok(cubes.len())
}

fn part2(input: &str) -> Result<usize> {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT3)?, 2758514936282235);
        Ok(())
    }
}
