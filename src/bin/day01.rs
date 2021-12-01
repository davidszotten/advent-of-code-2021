use anyhow::Result;
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    let numbers: Vec<_> = input.split('\n').filter_map(|x| x.parse::<i32>().ok()).collect();
    Ok(numbers.windows(2).filter(|win| win[0] < win[1]).count())
}


fn part2(input: &str) -> Result<usize> {
    let numbers: Vec<_> = input.split('\n').filter_map(|x| x.parse::<i32>().ok()).collect();
    let windows: Vec<i32> = numbers.windows(3).map(|win| win.iter().sum()).collect();
    Ok(windows.windows(2).filter(|win| win[0] < win[1]).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1721
199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 7);
        Ok(())
    }

    // #[test]
    // fn test_part1_empty() -> Result<()> {
    //     assert!(part1("").is_err());
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 5);
        Ok(())
    }
}
