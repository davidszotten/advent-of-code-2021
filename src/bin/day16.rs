use anyhow::{bail, Context, Error, Result};
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<usize> for Op {
    type Error = Error;
    fn try_from(n: usize) -> Result<Op> {
        Ok(match n {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Min,
            3 => Op::Max,
            5 => Op::GreaterThan,
            6 => Op::LessThan,
            7 => Op::EqualTo,
            _ => bail!("unknown op type {}", n),
        })
    }
}

#[derive(Debug)]
enum Value {
    Literal(usize),
    Op(Op, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    value: Value,
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.version
            + match &self.value {
                Value::Literal(_) => 0,
                Value::Op(_, packets) => packets.iter().map(|p| p.version_sum()).sum(),
            }
    }

    fn value(&self) -> usize {
        let bool_to_int = |b| if b { 1 } else { 0 };
        match &self.value {
            Value::Literal(val) => *val,
            Value::Op(op, packets) => {
                let values = packets.iter().map(Packet::value);
                match op {
                    Op::Sum => values.sum(),
                    Op::Product => values.product(),
                    Op::Min => values.min().expect("no packets for min"),
                    Op::Max => values.max().expect("no packets for max"),
                    Op::GreaterThan => {
                        let values: Vec<_> = values.collect();
                        assert_eq!(values.len(), 2);
                        bool_to_int(values[0] > values[1])
                    }
                    Op::LessThan => {
                        let values: Vec<_> = values.collect();
                        assert_eq!(values.len(), 2);
                        bool_to_int(values[0] < values[1])
                    }
                    Op::EqualTo => {
                        let values: Vec<_> = values.collect();
                        assert_eq!(values.len(), 2);
                        bool_to_int(values[0] == values[1])
                    }
                }
            }
        }
    }
}

fn hex_to_bytes(c: char) -> Result<Vec<u8>> {
    let decimal = u8::from_str_radix(&c.to_string(), 16).context("failed to parse")?;
    if decimal >= 16 {
        bail!("too large");
    }
    Ok(vec![
        decimal >> 3 & 1,
        decimal >> 2 & 1,
        decimal >> 1 & 1,
        decimal & 1,
    ])
}

fn bytes_to_dec<'a>(digits: impl IntoIterator<Item = &'a u8>) -> usize {
    digits.into_iter().fold(0, |acc, d| acc * 2 + *d as usize)
}

fn parse(input: &str) -> Result<Packet> {
    let digits: Vec<u8> = input
        .trim()
        .chars()
        .map(hex_to_bytes)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();
    process(&mut ByteStream::new(digits))
}

struct ByteStream {
    data: Vec<u8>,
    pos: usize,
}

impl ByteStream {
    fn new(data: Vec<u8>) -> Self {
        Self { data, pos: 0 }
    }

    fn read(&mut self, n: usize) -> &[u8] {
        let res = &self.data[self.pos..self.pos + n];
        self.pos += n;
        res
    }

    fn len(&self) -> usize {
        self.data[self.pos..].len()
    }
}

fn process(stream: &mut ByteStream) -> Result<Packet> {
    let version = bytes_to_dec(stream.read(3));
    let type_id = bytes_to_dec(stream.read(3));
    let value = match type_id {
        4 => {
            // literal
            let mut bin_parts = vec![];
            loop {
                let chunk = stream.read(5);
                bin_parts.push(chunk[1..].to_vec());
                if chunk[0] == 0 {
                    let value = bytes_to_dec(bin_parts.iter().flatten());
                    break Value::Literal(value);
                }
            }
        }
        op => {
            // operator
            let length_type_id = stream.read(1)[0];
            let packets = if length_type_id == 0 {
                let sub_bits = bytes_to_dec(stream.read(15));
                let start_len = stream.len();
                let mut packets = vec![];
                while start_len - stream.len() < sub_bits {
                    let packet = process(stream)?;
                    packets.push(packet);
                }
                packets
            } else if length_type_id == 1 {
                let sub_packets = bytes_to_dec(stream.read(11));
                let mut packets = vec![];
                while packets.len() < sub_packets {
                    let packet = process(stream)?;
                    packets.push(packet);
                }
                packets
            } else {
                bail!("invalid length_type_id");
            };
            Value::Op(Op::try_from(op)?, packets)
        }
    };
    Ok(Packet { version, value })
}

fn part1(input: &str) -> Result<usize> {
    let packet = parse(input)?;
    Ok(packet.version_sum())
}

fn part2(input: &str) -> Result<usize> {
    let packet = parse(input)?;
    Ok(packet.value())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "D2FE28";

    #[test]
    fn test_hex_to_bytes() -> Result<()> {
        assert_eq!(hex_to_bytes('0')?, vec![0, 0, 0, 0]);
        assert_eq!(hex_to_bytes('8')?, vec![1, 0, 0, 0]);
        assert_eq!(hex_to_bytes('9')?, vec![1, 0, 0, 1]);
        assert_eq!(hex_to_bytes('B')?, vec![1, 0, 1, 1]);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 6);
        assert_eq!(part1("38006F45291200")?, 9);
        assert_eq!(part1("EE00D40C823060")?, 14);
        assert_eq!(part1("8A004A801A8002F478")?, 16);
        assert_eq!(part1("620080001611562C8802118E34")?, 12);
        assert_eq!(part1("C0015000016115A2E0802F182340")?, 23);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("C200B40A82")?, 3);
        Ok(())
    }
}
