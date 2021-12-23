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
type Row = [Option<usize>; 4];

#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Position<const LEVELS: usize> {
    rooms: [[Option<Amphipod>; ROOMS]; LEVELS],
    hallway: [Option<Amphipod>; HALLS],
}

impl<const LEVELS: usize> std::fmt::Debug for Position<LEVELS> {
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
        for r in self.rooms[0] {
            if let Some(r) = r {
                write!(f, "{}#", (b'A' + r as u8) as char)?;
            } else {
                write!(f, ".#")?;
            }
        }
        writeln!(f, "##")?;

        for level in 1..LEVELS {
            write!(f, "  #")?;
            for r in self.rooms[level] {
                if let Some(r) = r {
                    write!(f, "{}#", (b'A' + r as u8) as char)?;
                } else {
                    write!(f, ".#")?;
                }
            }
            writeln!(f, " ")?;
        }
        writeln!(f, "  #########  ")?;

        Ok(())
    }
}

fn above(room: usize) -> usize {
    assert!(room < 4);
    2 + room * 2
}

fn room_to_hall(from: usize, to: usize) -> (std::ops::RangeInclusive<usize>, usize) {
    let above = above(from);
    if to < above {
        (to..=above, above - to + 1)
    } else {
        (above..=to, to - above + 1)
    }
}

fn hall_to_room(from: usize, to: usize) -> (std::ops::RangeInclusive<usize>, usize) {
    let above = above(to);
    if from < above {
        (from..=above, above - from + 1)
    } else {
        (above..=from, from - above + 1)
    }
}

impl<const LEVELS: usize> Position<LEVELS> {
    fn num_correct(&self) -> usize {
        self.rooms
            .map(|r| r.iter().zip(0..3).filter(|(a, b)| **a == Some(*b)).count())
            .iter()
            .sum()
    }

    fn room_to_hall_len(&self, from: usize, to: usize) -> Option<usize> {
        let (path, path_len) = room_to_hall(from, to);
        if path.into_iter().all(|h| self.hallway[h].is_none()) {
            return Some(path_len);
        }
        None
    }

    fn hall_to_room_len(&self, from: usize, to: usize) -> Option<usize> {
        let (path, path_len) = hall_to_room(from, to);
        if path
            .into_iter()
            .all(|h| h == from || self.hallway[h].is_none())
        {
            return Some(path_len);
        }
        None
    }

    fn room_to_room_len(&self, from: usize, to: usize) -> Option<usize> {
        let above = above(to);
        self.room_to_hall_len(from, above)
    }

    fn room_free(&self, room: usize) -> Option<usize> {
        if self
            .rooms
            .iter()
            .filter_map(|row| row[room])
            .any(|a| a != room)
        {
            return None;
        }
        return self
            .rooms
            .iter()
            .map(|row| row[room])
            .enumerate()
            .filter(|(_, occupant)| occupant.is_none())
            .map(|(level, _)| level)
            .last();
    }

    fn top_occupant(&self, room: usize) -> Option<(usize, Amphipod)> {
        self.rooms
            .iter()
            .map(|row| row[room])
            .enumerate()
            .find_map(|(level, occupant)| occupant.map(|o| (level, o)))
    }

    fn done(&self, room: usize) -> bool {
        self.rooms
            .iter()
            .map(|row| row[room])
            .all(|a| a == Some(room))
    }

    fn next(&self) -> Vec<(Position<LEVELS>, usize)> {
        let mut res = vec![];

        for room in 0..ROOMS {
            if self.done(room) {
                continue;
            }
            if let Some((level, amphipod)) = self.top_occupant(room) {
                for hall in 0..HALLS {
                    if hall == 2 || hall == 4 || hall == 6 || hall == 8 {
                        continue;
                    }
                    if let Some(path_len) = self.room_to_hall_len(room, hall) {
                        let mut new_state = self.clone();
                        new_state.rooms[level][room] = None;
                        new_state.hallway[hall] = Some(amphipod);
                        let energy = (path_len + level) * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }

                let destination = amphipod;
                if destination == room {
                    continue;
                };
                if let Some(destination_level) = self.room_free(destination) {
                    if let Some(path_len) = self.room_to_room_len(room, destination) {
                        let mut new_state = self.clone();
                        new_state.rooms[level][room] = None;
                        new_state.rooms[destination_level][destination] = Some(amphipod);
                        let energy = (path_len + level + destination_level + 1)
                            * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }
            }
        }
        for hall in 0..HALLS {
            if let Some(amphipod) = self.hallway[hall] {
                let destination = amphipod;
                if let Some(destination_level) = self.room_free(destination) {
                    if let Some(path_len) = self.hall_to_room_len(hall, destination) {
                        let mut new_state = self.clone();
                        new_state.hallway[hall] = None;
                        new_state.rooms[destination_level][destination] = Some(amphipod);
                        let energy = (path_len + destination_level) * 10_usize.pow(amphipod as u32);
                        res.push((new_state, energy));
                    }
                }
            }
        }

        res.sort_by_key(|t| t.1);
        res
    }
}

fn parse_line(s: &str) -> [Option<usize>; 4] {
    let amphipod_index = |c| match c {
        'A'..='D' => Some(c as usize - 'A' as usize),
        _ => None,
    };
    [
        s.chars().nth(3).map(amphipod_index).unwrap(),
        s.chars().nth(5).map(amphipod_index).unwrap(),
        s.chars().nth(7).map(amphipod_index).unwrap(),
        s.chars().nth(9).map(amphipod_index).unwrap(),
    ]
}

fn parse_input(s: &str) -> Result<(Row, Row)> {
    let lines = s.trim().lines().skip(2).take(4);
    let (top, bottom) = lines.tuple_windows().next().unwrap();

    Ok((parse_line(top), parse_line(bottom)))
}

impl FromStr for Position<2> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let hallway = [None; HALLS];
        let (rooms_0, rooms_1) = parse_input(s)?;

        Ok(Position {
            hallway,
            rooms: [rooms_0, rooms_1],
        })
    }
}

impl FromStr for Position<4> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let hallway = [None; HALLS];
        let (rooms_0, rooms_3) = parse_input(s)?;
        // #D#C#B#A#
        // #D#B#A#C#
        let rooms_1 = [Some(3), Some(2), Some(1), Some(0)];
        let rooms_2 = [Some(3), Some(1), Some(0), Some(2)];

        Ok(Position {
            hallway,
            rooms: [rooms_0, rooms_1, rooms_2, rooms_3],
        })
    }
}

#[allow(dead_code)]
fn parse4(s: &str) -> Result<Position<4>> {
    let top = s.trim().lines().nth(1).unwrap();

    let amphipod_index = |c| match c {
        'A'..='D' => Some(c as usize - 'A' as usize),
        _ => None,
    };

    let hallway = [
        top.chars().nth(1).map(amphipod_index).unwrap(),
        top.chars().nth(2).map(amphipod_index).unwrap(),
        top.chars().nth(3).map(amphipod_index).unwrap(),
        top.chars().nth(4).map(amphipod_index).unwrap(),
        top.chars().nth(5).map(amphipod_index).unwrap(),
        top.chars().nth(6).map(amphipod_index).unwrap(),
        top.chars().nth(7).map(amphipod_index).unwrap(),
        top.chars().nth(8).map(amphipod_index).unwrap(),
        top.chars().nth(9).map(amphipod_index).unwrap(),
        top.chars().nth(10).map(amphipod_index).unwrap(),
        top.chars().nth(11).map(amphipod_index).unwrap(),
    ];

    let lines = s.trim().lines().skip(2).take(4);
    let (r0, r1, r2, r3) = lines.tuple_windows().next().unwrap();

    Ok(Position {
        hallway,
        rooms: [
            parse_line(r0),
            parse_line(r1),
            parse_line(r2),
            parse_line(r3),
        ],
    })
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Clone, Eq, PartialEq)]
struct State<const LEVELS: usize> {
    position: Position<LEVELS>,
    cost: usize,
}
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl<const LEVELS: usize> Ord for State<LEVELS> {
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
impl<const LEVELS: usize> PartialOrd for State<LEVELS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path<const LEVELS: usize>(
    start: Position<LEVELS>,
    goal: Position<LEVELS>,
) -> Option<usize> {
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
    let state: Position<2> = input.parse()?;
    let goal = Position {
        hallway: [None; HALLS],
        rooms: [[Some(0), Some(1), Some(2), Some(3)]; 2],
    };
    shortest_path(state, goal).context("no path")
}

fn part2(input: &str) -> Result<usize> {
    let state: Position<4> = input.parse()?;
    let goal = Position {
        hallway: [None; HALLS],
        rooms: [[Some(0), Some(1), Some(2), Some(3)]; 4],
    };

    shortest_path(state, goal).context("no path")
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
        let state: Position<2> = TEST_INPUT.parse()?;
        assert_eq!(
            state,
            Position {
                hallway: [None; HALLS],
                rooms: [
                    [Some(1), Some(2), Some(1), Some(3)],
                    [Some(0), Some(3), Some(2), Some(0)]
                ],
            }
        );
        Ok(())
    }

    #[test]
    fn test_move2() -> Result<()> {
        let mut hallway = [None; HALLS];
        hallway[3] = Some(1);
        // #############
        // #...B.......#
        // ###B#.#C#D###
        //   #A#D#C#A#
        //   #########
        let state = Position {
            hallway,
            rooms: [
                [Some(1), None, Some(2), Some(3)],
                [Some(0), Some(3), Some(2), Some(0)],
            ],
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
            rooms: [
                [Some(1), None, Some(2), Some(3)],
                [Some(0), None, Some(2), Some(0)],
            ],
        };

        if let Some((_p, cost)) = state.next().iter().find(|&p| p.0 == mid) {
            assert_eq!(*cost, 3000);
        } else {
            panic!();
        }

        // #############
        // #.....D.....#
        // ###B#.#C#D###
        //   #A#B#C#A#
        //   #########
        let mut hallway = [None; HALLS];
        hallway[5] = Some(3);
        let target = Position {
            hallway,
            rooms: [
                [Some(1), None, Some(2), Some(3)],
                [Some(0), Some(1), Some(2), Some(0)],
            ],
        };
        assert!(mid.next().iter().find(|&p| p.0 == target).is_some());
        if let Some((_p, cost)) = mid.next().iter().find(|&p| p.0 == target) {
            assert_eq!(*cost, 30);
        } else {
            panic!();
        }

        // assert!(false);

        // #############
        // #...B.......#
        // ###B#C#.#D###
        //   #A#D#C#A#
        //   #########

        let mut hallway = [None; HALLS];
        hallway[3] = Some(1);
        let before = Position {
            hallway,
            rooms: [
                [Some(1), Some(2), None, Some(3)],
                [Some(0), Some(3), Some(2), Some(0)],
            ],
        };

        // ############
        // #...B.......#
        // ###B#.#C#D###
        //   #A#D#C#A#
        //   #########
        let mut hallway = [None; HALLS];
        hallway[3] = Some(1);
        let after = Position {
            hallway,
            rooms: [
                [Some(1), None, Some(2), Some(3)],
                [Some(0), Some(3), Some(2), Some(0)],
            ],
        };
        if let Some((_p, cost)) = before.next().iter().find(|&p| p.0 == after) {
            assert_eq!(*cost, 400);
        } else {
            panic!();
        }

        Ok(())
    }

    #[test]
    fn test_room_free_for() -> Result<()> {
        let state = Position {
            hallway: [None; HALLS],
            rooms: [
                [None, None, Some(2), None],
                [None, Some(1), Some(2), Some(0)],
            ],
        };
        assert_eq!(state.room_free(0), Some(1));
        assert_eq!(state.room_free(1), Some(0));
        assert_eq!(state.room_free(2), None);
        assert_eq!(state.room_free(3), None);
        Ok(())
    }

    #[test]
    fn test_top_occupant() -> Result<()> {
        let state = Position {
            hallway: [None; HALLS],
            rooms: [
                [None, None, Some(1), None],
                [None, Some(2), Some(1), Some(4)],
            ],
        };
        assert_eq!(state.top_occupant(0), None);
        assert_eq!(state.top_occupant(1), Some((1, 2)));
        assert_eq!(state.top_occupant(2), Some((0, 1)));
        assert_eq!(state.top_occupant(3), Some((1, 4)));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 12521);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 44169);
        Ok(())
    }
}
