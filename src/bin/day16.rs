use anyhow::{bail, Context, Result};
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
enum Value {
    Literal(i64),
    Op(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: i64,
    value: Value,
}

impl Packet {
    fn version_sum(&self) -> i64 {
        self.version
            + match &self.value {
                Value::Literal(_) => 0,
                Value::Op(packets) => packets.iter().map(|p| p.version_sum()).sum(),
            }
    }
}

fn hex_to_bytes(c: char) -> Result<Vec<u8>> {
    let decimal = u8::from_str_radix(&c.to_string(), 16).context("failed to parse")?;
    if decimal >= 16 {
        bail!("too large");
    }
    let mut res = vec![];
    res.push(decimal >> 3 & 1);
    res.push(decimal >> 2 & 1);
    res.push(decimal >> 1 & 1);
    res.push(decimal >> 0 & 1);
    Ok(res)
}

fn bytes_to_dec(digits: &[u8]) -> i64 {
    digits.iter().fold(0, |acc, d| acc * 2 + *d as i64)
}

fn take(it: &mut dyn Iterator<Item = u8>, n: usize) -> Result<Vec<u8>> {
    let mut res = vec![];
    for _ in 0..n {
        let n = it.next().context("not enough elements")?;
        res.push(n);
    }
    Ok(res)
}

fn parse(input: &str) -> Result<(Packet, i64)> {
    let digits: Vec<u8> = input
        .trim()
        .chars()
        .map(hex_to_bytes)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();
    let mut it = digits.into_iter();
    process(&mut it, 0)
}

fn process(it: &mut dyn Iterator<Item = u8>, depth: usize) -> Result<(Packet, i64)> {
    let mut consumed = 0;
    let version = bytes_to_dec(&take(it, 3).context("version")?);
    consumed += 3;
    let type_id = bytes_to_dec(&take(it, 3).context("type_id")?);
    consumed += 3;
    let value = match type_id {
        4 => {
            // literal
            let mut bin_parts = vec![];
            loop {
                let chunk = take(it, 5).context("chunk")?;
                consumed += 5;
                bin_parts.push(chunk[1..].to_vec());
                if chunk[0] == 0 {
                    let value = bytes_to_dec(&bin_parts.into_iter().flatten().collect::<Vec<_>>());
                    break Value::Literal(value);
                }
            }
        }
        _ => {
            // operator
            let length_type_id = it.next().context("no length_type_id")?;
            consumed += 1;
            if length_type_id == 0 {
                let sub_bits = bytes_to_dec(&take(it, 15).context("sub bits")?);
                consumed += 15;
                let mut seen_sub_bits = 0;
                let mut packets = vec![];
                for _ in 0..depth {
                    print!(" ");
                }
                println!("looking for {} sub bits", sub_bits);
                while seen_sub_bits < sub_bits {
                    let (packet, bits) = process(it, depth + 1)?;
                    packets.push(packet);
                    seen_sub_bits += bits;
                }
                for _ in 0..depth {
                    print!(" ");
                }
                println!("found {} sub bits", sub_bits);
                Value::Op(packets)
            } else if length_type_id == 1 {
                let sub_packets = bytes_to_dec(&take(it, 11).context("sub packets")?) as usize;
                consumed += 11;
                let mut packets = vec![];
                for _ in 0..depth {
                    print!(" ");
                }
                println!("looking for {} sub packets", sub_packets);
                while packets.len() < sub_packets {
                    let (packet, _) = process(it, depth + 1)?;
                    packets.push(packet);
                }
                for _ in 0..depth {
                    print!(" ");
                }
                println!("found {} sub packets", sub_packets);
                Value::Op(packets)
            } else {
                bail!("invalid length_type_id");
            }
        }
    };
    Ok((Packet { version, value }, consumed))
    // Ok(dbg!((Packet { version, value }, consumed)))
}

fn part1(input: &str) -> Result<i64> {
    let (packet, _) = parse(input)?;
    // dbg!(&packet);
    Ok(packet.version_sum())
}

fn part2(_input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "D2FE28";

    #[test]
    fn test_hex_to_bytes() -> Result<()> {
        assert_eq!(hex_to_bytes('0')?, vec![0, 0, 0, 0]);
        assert_eq!(hex_to_bytes('9')?, vec![1, 0, 0, 1]);
        assert_eq!(hex_to_bytes('B')?, vec![1, 0, 1, 1]);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        // assert_eq!(part1(TEST_INPUT)?, 6);
        // assert_eq!(part1("38006F45291200")?, 9);
        // assert_eq!(part1("8A004A801A8002F478")?, 16);
        // assert_eq!(part1("620080001611562C8802118E34")?, 12);
        assert_eq!(part1("C0015000016115A2E0802F182340")?, 23);
        Ok(())
    }
}
