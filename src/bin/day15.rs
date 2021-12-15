use anyhow::{Context, Error, Result};
use aoc2021::coor::Coor;
use aoc2021::dispatch;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

const NEIGHBOURS: [Coor; 4] = [
    Coor::new(-1, 0),
    Coor::new(1, 0),
    Coor::new(0, -1),
    Coor::new(0, 1),
];

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn wrap_add(val: usize, addition: usize) -> usize {
    let mut res = val + addition;
    while res > 9 {
        res -= 9
    }
    res
}

struct Map {
    levels: HashMap<Coor, usize>,
    size: i64,
    large: bool,
}

impl Map {
    fn get(&self, coor: &Coor) -> Option<usize> {
        if !self.large {
            return self.levels.get(coor).cloned();
        }
        if coor.x >= self.size * 5 || coor.y >= self.size * 5 {
            return None;
        }
        let addition = coor.x / self.size + coor.y / self.size;
        let scaled_coor = Coor::new(coor.x % self.size, coor.y % self.size);
        self.levels
            .get(&scaled_coor)
            .map(|v| wrap_add(*v, addition as usize))
    }
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut levels = HashMap::new();
        let mut size = 0;
        for (y, line) in s.trim().lines().enumerate() {
            size += 1; // square
            for (x, c) in line.chars().enumerate() {
                levels.insert(Coor::new(x as i64, y as i64), c as usize - '0' as usize);
            }
        }

        Ok(Self {
            levels,
            size,
            large: false,
        })
    }
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coor,
}
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost).then_with(|| {
            (self.position.x, self.position.y).cmp(&(other.position.x, other.position.y))
        })
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(map: &Map, start: Coor, goal: Coor) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if let Some(&dist) = dist.get(&position) {
            if cost > dist {
                continue;
            }
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for &offset in &NEIGHBOURS {
            let neighbour = position + offset;
            let edge_cost = match map.get(&neighbour) {
                Some(cost) => cost,
                None => continue,
            };
            let next = State {
                cost: cost + edge_cost,
                position: neighbour,
            };

            let better = match dist.get(&next.position) {
                None => true,
                Some(&dist) => next.cost < dist,
            };
            // If so, add it to the frontier and continue
            if better {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}

fn part1(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    let mut coors: Vec<_> = map.levels.keys().collect();
    coors.sort_by_key(|&c| (-c.x, -c.y));
    let &end = coors[0];
    shortest_path(&map, Coor::new(0, 0), end).context("no path found")
}

fn part2(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    map.large = true;
    let mut coors: Vec<_> = map.levels.keys().collect();
    coors.sort_by_key(|&c| (-c.x, -c.y));
    let end = (*coors[0] + Coor::new(1, 1)) * 5 - Coor::new(1, 1);
    shortest_path(&map, Coor::new(0, 0), end).context("no path found")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 40);
        Ok(())
    }

    #[test]
    fn test_wrap_add() {
        assert_eq!(wrap_add(8, 0), 8);
        assert_eq!(wrap_add(8, 1), 9);
        assert_eq!(wrap_add(8, 2), 1);
        assert_eq!(wrap_add(8, 3), 2);
        assert_eq!(wrap_add(8, 4), 3);
        assert_eq!(wrap_add(8, 5), 4);
        assert_eq!(wrap_add(8, 6), 5);
        assert_eq!(wrap_add(8, 7), 6);
        assert_eq!(wrap_add(8, 8), 7);
    }

    #[test]
    fn test_large() -> Result<()> {
        let mut map: Map = TEST_INPUT.parse()?;
        map.large = true;
        assert!(map.get(&Coor::new(50, 0)).is_none());

        for x in 0..map.size * 5 {
            print!("{}", map.get(&Coor::new(x, 49)).expect("missing"));
        }
        println!();
        assert_eq!(map.get(&Coor::new(1, 1)).context("missing")?, 3);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 315);
        Ok(())
    }
}
