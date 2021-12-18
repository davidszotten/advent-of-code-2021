use anyhow::{Context, Error, Result};
use aoc2021::dispatch;
use std::fmt::Write;
use std::str::FromStr;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Element {
    Number(i32),
    LParen,
    RParen,
}

impl Element {
    fn _print(&self) -> String {
        use Element::*;
        match self {
            Number(n) => format!("{}", n),
            LParen => "[".to_string(),
            RParen => "]".to_string(),
        }
    }
}

impl FromStr for Element {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        use Element::*;
        Ok(match s {
            "[" => LParen,
            "]" => RParen,
            n => Number(n.parse().context(format!("failed to parse `{}`", n))?),
        })
    }
}

fn parse(input: &str) -> Result<Vec<Element>> {
    let mut res = vec![];
    for c in input.trim().chars() {
        if c == ',' {
            continue;
        }
        res.push(c.to_string().parse()?)
    }
    Ok(res)
}

fn _print(pair: &[Element]) -> String {
    use Element::*;
    let mut output = String::new();

    for entry in pair.windows(2) {
        let comma = match (entry[0], entry[1]) {
            (Number(_), Number(_)) => true,
            (Number(_), LParen) => true,
            (Number(_), RParen) => false,

            (LParen, Number(_)) => false,
            (LParen, LParen) => false,
            (LParen, RParen) => false,

            (RParen, Number(_)) => true,
            (RParen, LParen) => true,
            (RParen, RParen) => false,
        };
        write!(
            &mut output,
            "{}{}",
            entry[0]._print(),
            if comma { "," } else { "" }
        )
        .expect("print fail");
    }
    write!(&mut output, "{}", pair[pair.len() - 1]._print()).expect("print last fail");
    output
}

fn find_explode_position(pair: &[Element]) -> Option<(usize, usize)> {
    use Element::*;
    let mut depth = 0;
    let mut start = None;
    let mut inside = false;
    for (idx, element) in pair.iter().enumerate() {
        match element {
            Number(_) => {}
            LParen => {
                if !inside && depth == 4 {
                    start = Some(idx);
                    inside = true;
                }
                depth += 1;
            }
            RParen => {
                depth -= 1;
                if inside && depth == 4 {
                    return Some((start.expect("must find start before end"), idx));
                }
            }
        }
    }
    None
}

fn index_of_last_number_before(pair: &[Element], pos: usize) -> Option<usize> {
    use Element::*;
    let mut last = None;
    for (idx, element) in pair[..pos].iter().enumerate() {
        if let Number(_) = element {
            last = Some(idx);
        }
    }
    last
}

fn index_of_first_number_after(pair: &[Element], pos: usize) -> Option<usize> {
    use Element::*;
    let start = pos + 1;
    for (idx, element) in pair[pos + 1..].iter().enumerate() {
        if let Number(_) = element {
            return Some(idx + start);
        }
    }
    None
}

fn left_value(pair: &[Element]) -> i32 {
    use Element::*;
    for element in pair {
        if let Number(n) = element {
            return *n;
        }
    }
    unreachable!();
}

fn right_value(pair: &[Element]) -> i32 {
    use Element::*;
    for element in pair.iter().rev() {
        if let Number(n) = element {
            return *n;
        }
    }
    unreachable!();
}

fn get_number(element: Element) -> i32 {
    use Element::*;
    match element {
        Number(n) => n,
        _ => unreachable!(),
    }
}

fn explode(pair: &[Element]) -> Option<Vec<Element>> {
    use Element::*;
    let mut res = vec![];
    if let Some((start, end)) = find_explode_position(pair) {
        let left_val = left_value(&pair[start..=end]);
        let right_val = right_value(&pair[start..=end]);
        if let Some(last) = index_of_last_number_before(pair, start) {
            for element in &pair[..last] {
                res.push(*element);
            }
            res.push(Number(get_number(pair[last]) + left_val));

            for element in &pair[last + 1..start] {
                res.push(*element);
            }
        } else {
            for element in &pair[..start] {
                res.push(*element);
            }
        }

        res.push(Number(0));

        if let Some(first) = index_of_first_number_after(pair, end) {
            for element in &pair[end + 1..first] {
                res.push(*element);
            }
            res.push(Number(get_number(pair[first]) + right_val));

            for element in &pair[first + 1..] {
                res.push(*element);
            }
        } else {
            for element in &pair[end + 1..] {
                res.push(*element);
            }
        }
        Some(res)
    } else {
        None
    }
}

fn split(pair: &[Element]) -> Option<Vec<Element>> {
    use Element::*;
    let mut res = vec![];
    let mut done = false;
    for element in pair {
        match element {
            Number(n) if *n >= 10 => {
                if done {
                    res.push(Number(*n))
                } else {
                    res.push(LParen);
                    res.push(Number(n / 2));
                    res.push(Number((n + 1) / 2));
                    res.push(RParen);
                    done = true;
                }
            }
            e => res.push(*e),
        }
    }
    if done {
        Some(res)
    } else {
        None
    }
}

fn reduce(pair: &[Element]) -> Vec<Element> {
    let mut pair = pair.to_vec();
    loop {
        let mut done = true;
        while let Some(next) = explode(&pair) {
            done = false;
            pair = next;
        }
        if let Some(next) = split(&pair) {
            done = false;
            pair = next;
        }
        if done {
            break pair;
        }
    }
}

fn add(left: &[Element], right: &[Element]) -> Vec<Element> {
    use Element::*;
    let mut res = vec![LParen];
    for el in left {
        res.push(*el);
    }
    for el in right {
        res.push(*el);
    }
    res.push(RParen);
    reduce(&res)
}

fn end(pair: &[Element]) -> usize {
    use Element::*;
    let mut depth = 0;
    for (idx, element) in pair.iter().enumerate() {
        match element {
            Number(_) => {}
            LParen => {
                depth += 1;
            }
            RParen => {
                depth -= 1;
                if depth == 0 {
                    return idx;
                }
            }
        }
    }
    unreachable!();
}

fn sum(pair: &[Element]) -> i32 {
    use Element::*;
    let mut total = 0;
    assert_eq!(pair[0], LParen);
    assert_eq!(pair[pair.len() - 1], RParen);
    let inside = &pair[1..pair.len() - 1];

    let mut pos = 0;
    match inside[pos] {
        Number(n) => {
            total += 3 * n;
            pos += 1;
        }
        LParen => {
            pos = end(inside) + 1;
            total += 3 * sum(&inside[..pos]);
        }
        RParen => unreachable!(),
    }

    match inside[pos] {
        Number(n) => {
            total += 2 * n;
        }
        LParen => {
            total += 2 * sum(&inside[pos..]);
        }
        RParen => unreachable!(),
    }
    total
}

fn add_list(list: &[Vec<Element>]) -> Vec<Element> {
    let mut it = list.iter();
    let mut acc: Vec<Element> = it.next().unwrap().to_vec();
    for el in it {
        acc = add(&acc, el);
    }
    acc
}

fn parse_list(input: &str) -> Result<Vec<Vec<Element>>> {
    input.trim().lines().map(parse).collect::<Result<Vec<_>>>()
}

fn part1(input: &str) -> Result<i32> {
    let list: Vec<Vec<Element>> = parse_list(input)?;
    let res = add_list(&list);
    Ok(sum(&res))
}

fn part2(input: &str) -> Result<i32> {
    let list: Vec<Vec<Element>> = parse_list(input)?;
    let mut max = 0;
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            max = max.max(sum(&add(&list[i], &list[j])));
            max = max.max(sum(&add(&list[j], &list[i])));
        }
    }
    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::_print as print;
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(
            part1(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            )?,
            4140
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            )?,
            3993
        );
        Ok(())
    }

    #[test]
    fn test_find_explode_position() -> Result<()> {
        let pair = parse("[[[[[9,8],1],2],3],4]")?;
        assert_eq!(find_explode_position(&pair), Some((4, 7)));

        let pair = parse("[7,[6,[5,[4,[3,2]]]]]")?;
        assert_eq!(find_explode_position(&pair), Some((8, 11)));
        Ok(())
    }

    #[test]
    fn test_index_of_before_after() -> Result<()> {
        let pair = parse("[[[[[9,8],1],2],3],4]")?;
        assert_eq!(index_of_last_number_before(&pair, 4), None);
        assert_eq!(index_of_first_number_after(&pair, 7), Some(8));

        let pair = parse("[7,[6,[5,[4,[3,2]]]]]")?;
        assert_eq!(index_of_last_number_before(&pair, 8), Some(7));
        assert_eq!(index_of_first_number_after(&pair, 11), None);
        Ok(())
    }

    #[test]
    fn test_print() -> Result<()> {
        let input = "[[[[[9,8],1],2],3],4]";
        let pair = parse(input)?;
        assert_eq!(print(&pair), input.to_string());
        Ok(())
    }

    #[test]
    fn test_explode() -> Result<()> {
        assert_eq!(
            print(&explode(&parse("[[[[[9,8],1],2],3],4]")?).unwrap()),
            "[[[[0,9],2],3],4]"
        );

        assert_eq!(
            print(&explode(&parse("[7,[6,[5,[4,[3,2]]]]]")?).unwrap()),
            "[7,[6,[5,[7,0]]]]"
        );

        assert_eq!(
            print(&explode(&parse("[[6,[5,[4,[3,2]]]],1]")?).unwrap()),
            "[[6,[5,[7,0]]],3]"
        );

        assert_eq!(
            print(&explode(&parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")?).unwrap()),
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
        );

        assert_eq!(
            print(&explode(&parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")?).unwrap()),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );
        Ok(())
    }

    #[test]
    fn test_add() -> Result<()> {
        assert_eq!(
            print(&add(
                &parse("[[[[4,3],4],4],[7,[[8,4],9]]]")?,
                &parse("[1,1]")?,
            )),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );

        assert_eq!(
            print(&add(
                &parse("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]")?,
                &parse("[[[[4,2],2],6],[8,7]]")?,
            )),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );

        Ok(())
    }

    #[test]
    fn test_end() -> Result<()> {
        let input = "[[[[[9,8],1],2],3],4]";
        let pair = parse(input)?;
        assert_eq!(end(&pair), 15);
        assert_eq!(end(&pair[1..]), 12);
        Ok(())
    }

    #[test]
    fn test_sum() -> Result<()> {
        assert_eq!(sum(&parse("[9,1]")?), 29);
        assert_eq!(sum(&parse("[[1,2],[[3,4],5]]")?), 143);
        assert_eq!(sum(&parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?), 1384);
        assert_eq!(sum(&parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")?), 445);
        assert_eq!(sum(&parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")?), 791);
        assert_eq!(sum(&parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")?), 1137);
        assert_eq!(
            sum(&parse(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )?),
            3488
        );
        assert_eq!(
            sum(&parse(
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            )?),
            4140
        );

        Ok(())
    }

    #[test]
    fn test_add_list() -> Result<()> {
        assert_eq!(
            print(&add_list(&parse_list(
                "[1,1]
[2,2]
[3,3]
[4,4]"
            )?)),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );

        assert_eq!(
            print(&add_list(&parse_list(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            )?)),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );

        assert_eq!(
            print(&add_list(&parse_list(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            )?)),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
        Ok(())
    }
}
