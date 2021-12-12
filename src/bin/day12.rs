use anyhow::{Context, Result};
use aoc2021::dispatch;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<HashMap<&str, Vec<&str>>> {
    let mut adjacent = HashMap::new();
    for line in input.lines() {
        let mut it = line.split('-');
        let left = it.next().context("no left")?;
        let right = it.next().context("no right")?;
        assert!(it.next().is_none());
        adjacent.entry(left).or_insert_with(Vec::new).push(right);
        adjacent.entry(right).or_insert_with(Vec::new).push(left);
    }
    Ok(adjacent)
}

fn part1(input: &str) -> Result<i32> {
    let adjacent = parse(input)?;

    let mut found = 0;
    let mut queue = vec![];
    queue.push((HashSet::from(["start"]), "start"));
    while let Some((seen, pos)) = queue.pop() {
        for &next in &adjacent[pos] {
            if next == "end" {
                found += 1;
                continue;
            }
            if next.chars().all(char::is_lowercase) && seen.contains(next) {
                continue;
            }
            let mut new_seen = seen.clone();
            new_seen.insert(next);
            queue.push((new_seen, next));
        }
    }
    Ok(found)
}

fn part2(input: &str) -> Result<i32> {
    let adjacent = parse(input)?;

    let mut found = 0;
    let mut queue = vec![];
    queue.push((HashSet::new(), false, "start"));
    while let Some((seen, twice, pos)) = queue.pop() {
        for &next in &adjacent[pos] {
            if next == "start" {
                continue;
            }
            if next == "end" {
                found += 1;
                continue;
            }
            if next.chars().all(char::is_lowercase) && seen.contains(next) {
                if twice {
                    continue;
                }
                queue.push((seen.clone(), true, next));
                continue;
            }
            let mut new_seen = seen.clone();
            new_seen.insert(next);
            queue.push((new_seen, twice, next));
        }
    }
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const TEST_INPUT2: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 10);
        assert_eq!(part1(TEST_INPUT2)?, 226);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 36);
        assert_eq!(part2(TEST_INPUT2)?, 3509);
        Ok(())
    }
}
