use anyhow::{Context, Error, Result};
use aoc2021::dispatch;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

const ROOMS: usize = 4;
const HALLS: usize = 11;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type Amphipod = usize;

#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Position {
    rooms_top: [Option<Amphipod>; 4],
    rooms_bottom: [Option<Amphipod>; 4],
    hallway: [Option<Amphipod>; HALLS],
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f)?;
        for _ in 0..(2 + HALLS) {
            write!(f, "#")?;
        }
        writeln!(f)?;
        write!(f, "#")?;
        for h in self.hallway {
            if let Some(h) = h {
                write!(f, "{}", (b'A' + h as u8) as char)?;
            } else {
                write!(f, ".")?;
            }
        }
        writeln!(f, "#")?;
        write!(f, "###")?;
        for r in self.rooms_top {
            if let Some(r) = r {
                write!(f, "{}#", (b'A' + r as u8) as char)?;
            } else {
                write!(f, ".#")?;
            }
        }
        writeln!(f, "##")?;
        write!(f, "  #")?;
        for r in self.rooms_bottom {
            if let Some(r) = r {
                write!(f, "{}#", (b'A' + r as u8) as char)?;
            } else {
                write!(f, ".#")?;
            }
        }
        writeln!(f, " ")?;
        writeln!(f, "  #########  ")?;

        Ok(())
    }
}

fn above(room: usize) -> usize {
    assert!(room < 4);
    2 + room * 2
}

fn room_to_hall(from: usize, to: usize) -> Vec<usize> {
    let above = above(from);
    if to < above {
        (to..=above).collect()
    } else {
        (above..=to).collect()
    }
}

fn room_to_room(from: usize, to: usize) -> Vec<usize> {
    let above = above(to);
    room_to_hall(from, above)
}

impl Position {
    fn num_correct(&self) -> usize {
        self.rooms_top
            .iter()
            .zip(0..3)
            .filter(|(a, b)| **a == Some(*b))
            .count()
            + self
                .rooms_bottom
                .iter()
                .zip(0..3)
                .filter(|(a, b)| **a == Some(*b))
                .count()
    }

    fn next(&self) -> Vec<(Position, usize)> {
        let mut res = vec![];

        for room in 0..ROOMS {
            if let Some(amphipod) = self.rooms_top[room] {
                if amphipod == room && self.rooms_bottom[room] == Some(amphipod) {
                    continue;
                }
                for hall in 0..HALLS {
                    if hall == above(room) {
                        continue;
                    }
                    let path = room_to_hall(room, hall);
                    let path_len = path.len();
                    if path.into_iter().all(|h| self.hallway[h].is_none()) {
                        let mut new_state = self.clone();
                        new_state.rooms_top[room] = None;
                        new_state.hallway[hall] = Some(amphipod);
                        let energy = path_len * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }

                let destination = amphipod;
                if destination == room {
                    continue;
                };
                if self.rooms_bottom[amphipod] == Some(amphipod)
                    && self.rooms_top[amphipod].is_none()
                {
                    let path = room_to_room(room, destination);
                    let path_len = path.len() + 1;
                    if path.into_iter().all(|h| self.hallway[h].is_none()) {
                        let mut new_state = self.clone();
                        new_state.rooms_top[room] = None;
                        new_state.rooms_top[destination] = Some(amphipod);
                        let energy = path_len * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }
                if self.rooms_bottom[amphipod].is_none() && self.rooms_top[amphipod].is_none() {
                    let path = room_to_room(room, destination);
                    let path_len = path.len() + 2;
                    if path.into_iter().all(|h| self.hallway[h].is_none()) {
                        let mut new_state = self.clone();
                        new_state.rooms_top[room] = None;
                        new_state.rooms_bottom[destination] = Some(amphipod);
                        let energy = path_len * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }
            }
            if let Some(amphipod) = self.rooms_bottom[room] {
                if amphipod == room {
                    continue;
                }
                if self.rooms_top[room].is_some() {
                    continue;
                }
                for hall in 0..HALLS {
                    if hall == above(room) {
                        continue;
                    }
                    let path = room_to_hall(room, hall);
                    let path_len = path.len() + 1;
                    if path.into_iter().all(|h| self.hallway[h].is_none()) {
                        let mut new_state = self.clone();
                        new_state.rooms_bottom[room] = None;
                        new_state.hallway[hall] = Some(amphipod);
                        let energy = path_len * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }

                let destination = amphipod;
                if destination == room {
                    continue;
                };
                if self.rooms_bottom[amphipod] == Some(amphipod)
                    && self.rooms_top[amphipod].is_none()
                {
                    let path = room_to_room(room, destination);
                    let path_len = path.len() + 2;
                    if path.into_iter().all(|h| self.hallway[h].is_none()) {
                        let mut new_state = self.clone();
                        new_state.rooms_bottom[room] = None;
                        new_state.rooms_top[destination] = Some(amphipod);
                        let energy = path_len * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }
                if self.rooms_bottom[amphipod].is_none() && self.rooms_top[amphipod].is_none() {
                    let path = room_to_room(room, destination);
                    let path_len = path.len() + 3;
                    if path.into_iter().all(|h| self.hallway[h].is_none()) {
                        let mut new_state = self.clone();
                        new_state.rooms_bottom[room] = None;
                        new_state.rooms_bottom[destination] = Some(amphipod);
                        let energy = path_len * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }
            }
        }
        for hall in 0..HALLS {
            if let Some(amphipod) = self.hallway[hall] {
                let destination = amphipod;
                if self.rooms_bottom[amphipod] == Some(amphipod)
                    && self.rooms_top[amphipod].is_none()
                {
                    let path = room_to_hall(destination, hall);
                    let path_len = path.len();
                    if path
                        .into_iter()
                        .all(|h| h == hall || self.hallway[h].is_none())
                    {
                        let mut new_state = self.clone();
                        new_state.hallway[hall] = None;
                        new_state.rooms_top[destination] = Some(amphipod);
                        let energy = path_len * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }
                if self.rooms_bottom[amphipod].is_none() && self.rooms_top[amphipod].is_none() {
                    let path = room_to_hall(destination, hall);
                    let path_len = path.len() + 1;
                    if path
                        .into_iter()
                        .all(|h| h == hall || self.hallway[h].is_none())
                    {
                        let mut new_state = self.clone();
                        new_state.hallway[hall] = None;
                        new_state.rooms_bottom[destination] = Some(amphipod);
                        let energy = path_len * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }
            }
        }

        res
    }
}

impl FromStr for Position {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let lines = s.trim().lines().skip(2).take(4);
        let (top, bottom) = lines.tuple_windows().next().unwrap();
        let amphipod_index = |c| match c {
            'A'..='D' => Some(c as usize - 'A' as usize),
            _ => None,
        };
        let hallway = [None; HALLS];
        let rooms_top = [
            top.chars().nth(3).map(amphipod_index).unwrap(),
            top.chars().nth(5).map(amphipod_index).unwrap(),
            top.chars().nth(7).map(amphipod_index).unwrap(),
            top.chars().nth(9).map(amphipod_index).unwrap(),
        ];
        let rooms_bottom = [
            bottom.chars().nth(3).map(amphipod_index).unwrap(),
            bottom.chars().nth(5).map(amphipod_index).unwrap(),
            bottom.chars().nth(7).map(amphipod_index).unwrap(),
            bottom.chars().nth(9).map(amphipod_index).unwrap(),
        ];

        Ok(Position {
            hallway,
            rooms_top,
            rooms_bottom,
        })
    }
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Clone, Eq, PartialEq)]
struct State {
    position: Position,
    cost: usize,
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
            (self.position.num_correct(), &self.position)
                .cmp(&(other.position.num_correct(), &other.position))
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
fn shortest_path(start: Position, goal: Position) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start.clone(), 0);
    heap.push(State {
        position: start,
        cost: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // dbg!(cost, &position);
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
        for (next, edge_cost) in position.next() {
            let next = State {
                cost: cost + edge_cost,
                position: next,
            };

            let better = match dist.get(&next.position) {
                None => true,
                Some(&dist) => next.cost < dist,
            };
            // If so, add it to the frontier and continue
            if better {
                heap.push(next.clone());
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}

fn part1(input: &str) -> Result<usize> {
    let state: Position = input.parse()?;
    let goal = Position {
        hallway: [None; HALLS],
        rooms_top: [Some(0), Some(1), Some(2), Some(3)],
        rooms_bottom: [Some(0), Some(1), Some(2), Some(3)],
    };
    shortest_path(state, goal).context("no path")
    // for next in state.next() {
    // println!("{:?}", next);
    // }
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_parse() -> Result<()> {
        let state: Position = TEST_INPUT.parse()?;
        assert_eq!(
            state,
            Position {
                hallway: [None; HALLS],
                rooms_top: [Some(1), Some(2), Some(1), Some(3)],
                rooms_bottom: [Some(0), Some(3), Some(2), Some(0)],
            }
        );
        Ok(())
    }

    #[test]
    fn test_move() -> Result<()> {
        let mut hallway = [None; HALLS];
        hallway[3] = Some(1);
        // #############
        // #...B.......#
        // ###B#.#C#D###
        //   #A#D#C#A#
        //   #########
        let state = Position {
            hallway,
            rooms_top: [Some(1), None, Some(2), Some(3)],
            rooms_bottom: [Some(0), Some(3), Some(2), Some(0)],
        };

        // #############
        // #...B.D.....#
        // ###B#.#C#D###
        //   #A#.#C#A#
        //   #########
        let mut hallway = [None; HALLS];
        hallway[3] = Some(1);
        hallway[5] = Some(3);
        let mid = Position {
            hallway,
            rooms_top: [Some(1), None, Some(2), Some(3)],
            rooms_bottom: [Some(0), None, Some(2), Some(0)],
        };

        assert!(state.next().iter().find(|&p| p.0 == mid).is_some());

        // #############
        // #.....D.....#
        // ###B#.#C#D###
        //   #A#B#C#A#
        //   #########
        let mut hallway = [None; HALLS];
        hallway[5] = Some(3);
        let target = Position {
            hallway,
            rooms_top: [Some(1), None, Some(2), Some(3)],
            rooms_bottom: [Some(0), Some(1), Some(2), Some(0)],
        };
        assert!(mid.next().iter().find(|&p| p.0 == target).is_some());

        // assert!(false);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 12521);
        Ok(())
    }
}
