use anyhow::{bail, Context, Error, Result};
use aoc2021::coor::Coor;
use aoc2021::dispatch;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

const ENVELOPE: [Coor; 9] = [
    Coor::new(-1, -1),
    Coor::new(0, -1),
    Coor::new(1, -1),
    Coor::new(-1, 0),
    Coor::new(0, 0),
    Coor::new(1, 0),
    Coor::new(-1, 1),
    Coor::new(0, 1),
    Coor::new(1, 1),
];

#[derive(Debug)]
struct Map {
    bitmap: Vec<u8>,
    pixels: HashMap<Coor, u8>,
    background: u8,
}

fn parse_pixel(c: char) -> Result<u8> {
    Ok(match c {
        '#' => 1,
        '.' => 0,
        _ => bail!("invalid pixel value `{}`", c),
    })
}

impl Map {
    fn process(&mut self) {
        let mut next = HashMap::new();
        let min_x = self.pixels.keys().map(|c| c.x).min().unwrap() - 2;
        let max_x = self.pixels.keys().map(|c| c.x).max().unwrap() + 2;
        let min_y = self.pixels.keys().map(|c| c.y).min().unwrap() - 2;
        let max_y = self.pixels.keys().map(|c| c.y).max().unwrap() + 2;

        let fg = 1 - self.background;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let mut val: u16 = 0;
                for offset in ENVELOPE {
                    val *= 2;
                    val += *self
                        .pixels
                        .get(&(Coor::new(x, y) + offset))
                        .unwrap_or(&self.background) as u16;
                }
                next.insert(Coor::new(x, y), self.bitmap[val as usize]);
            }
        }
        if self.bitmap[0] == 1 {
            self.background = fg;
        }
        self.pixels = next;
    }

    fn _print(&self) {
        let minx = self.pixels.keys().map(|d| d.x).min().expect("no x");
        let miny = self.pixels.keys().map(|d| d.y).min().expect("no y");
        let maxx = self.pixels.keys().map(|d| d.x).max().expect("no x");
        let maxy = self.pixels.keys().map(|d| d.y).max().expect("no y");
        for y in miny..=maxy {
            for x in minx..=maxx {
                if *self.pixels.get(&Coor::new(x, y)).unwrap_or(&0) == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (bitmap_raw, pixels_raw) = s.trim().split_once("\n\n").context("no separator")?;
        let bitmap = bitmap_raw
            .chars()
            .map(parse_pixel)
            .collect::<Result<Vec<_>>>()?;
        let mut pixels = HashMap::new();
        for (y, line) in pixels_raw.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                pixels.insert(Coor::new(x as i64, y as i64), parse_pixel(c)?);
            }
        }
        Ok(Map {
            bitmap,
            pixels,
            background: 0,
        })
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    map.process();
    // map._print();
    map.process();
    // map._print();

    Ok(map.pixels.values().filter(|&v| *v == 1).count())
}

fn part2(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    for _ in 0..50 {
        map.process();
    }

    Ok(map.pixels.values().filter(|&v| *v == 1).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../input/day20.sample");

    const IT1: &str = "#

.....##.##.....
....#..#.#.....
....##.#..#....
....####..#....
.....#..##.....
......##..#....
.......#.#.....";

    const IT2: &str = "#

...............
...............
...............
..........#....
....#..#.#.....
...#.#...###...
...#...##.#....
...#.....#.#...
....#.#####....
.....#.#####...
......##.##....
.......###.....
...............
...............
...............";

    #[test]
    fn test_part1() -> Result<()> {
        let mut map: Map = TEST_INPUT.parse()?;
        let it1: Map = IT1.parse()?;
        let it2: Map = IT2.parse()?;
        map.bitmap[0] = 1;

        map.process();
        map._print();
        it1._print();
        let mut mp = map.pixels.keys().map(|&c| c).collect::<Vec<_>>();
        mp.sort_by_key(|c| (c.y, c.x));
        let mut it1p = it1
            .pixels
            .keys()
            .map(|&c| c + Coor::new(-5, -1))
            .collect::<Vec<_>>();
        it1p.sort_by_key(|c| (c.y, c.x));
        // assert_eq!(mp, it1p);

        map.process();
        map._print();
        it2._print();
        let mut mp = map.pixels.keys().map(|&c| c).collect::<Vec<_>>();
        mp.sort_by_key(|c| (c.y, c.x));
        let mut it2p = it2
            .pixels
            .keys()
            .map(|&c| c + Coor::new(-5, -5))
            .collect::<Vec<_>>();
        it2p.sort_by_key(|c| (c.y, c.x));
        // assert_eq!(mp, it2p);

        assert_eq!(part1(TEST_INPUT)?, 35);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let mut map: Map = TEST_INPUT.parse()?;
        assert_eq!(part2(TEST_INPUT)?, 3351);
        Ok(())
    }
}
