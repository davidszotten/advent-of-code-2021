use anyhow::{Context, Result};
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn digits(mut num: i64) -> impl Iterator<Item = i64> {
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}

fn parse(s: &str) -> Vec<(i64, i64, i64)> {
    let lines = s.trim().lines();
    let a = lines
        .clone()
        .skip(4)
        .step_by(18)
        .map(|l| l.split(' ').nth(2).unwrap().parse::<i64>().unwrap());
    let b = lines
        .clone()
        .skip(5)
        .step_by(18)
        .map(|l| l.split(' ').nth(2).unwrap().parse::<i64>().unwrap());
    let c = lines
        .clone()
        .skip(15)
        .step_by(18)
        .map(|l| l.split(' ').nth(2).unwrap().parse::<i64>().unwrap());
    a.into_iter()
        .zip(b)
        .zip(c)
        .map(|((a, b), c)| (a, b, c))
        .collect()
}

fn generic(input: i64, zprev: i64, zdiv: i64, xadd: i64, yadd: i64) -> Option<i64> {
    let x = zprev % 26 + xadd;
    let z = zprev / zdiv;
    if input == x {
        Some(z)
    } else if zdiv > 1 {
        None
    } else {
        Some(z * 26 + input + yadd)
    }
}

fn generic_run(params: &[(i64, i64, i64)], n: i64) -> Option<i64> {
    let mut param_iter = params.iter();
    let inputs = digits(n);
    let mut z = 0;
    for input in inputs {
        let &(zdiv, xadd, yadd) = param_iter.next().unwrap();
        z = generic(input, z, zdiv, xadd, yadd)?;
    }
    Some(z)
}

fn find(input: &str) -> Result<Vec<i64>> {
    let params = parse(input);

    let mut found = vec![];

    'outer: for n in 10000000..100000000 {
        for d in digits(n) {
            if d == 0 {
                continue 'outer;
            }
        }
        if generic_run(&params, n).is_some() {
            found.push(n);
        }
    }
    for _ in 0..6 {
        let mut next = vec![];
        for entry in found {
            for n in 1..10 {
                let val = entry * 10 + n;
                if generic_run(&params, val).is_some() {
                    next.push(val);
                }
            }
        }
        found = next;
    }
    Ok(found)
}

fn part1(input: &str) -> Result<i64> {
    let found = find(input)?;
    found.into_iter().max().context("no max")
}

fn part2(input: &str) -> Result<i64> {
    let found = find(input)?;
    found.into_iter().min().context("no min")
}
