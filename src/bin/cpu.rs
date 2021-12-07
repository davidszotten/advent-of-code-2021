use anyhow::Result;
use aoc2021::cpu::{Cpu, CpuState};
use aoc2021::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let mut cpu = Cpu::from_str(input);
    let mut output = vec![];

    loop {
        match cpu.run()? {
            CpuState::Output(value) => match value {
                c => output.push(c as u8 as char),
            },
            CpuState::Halted => break,
            s => {
                println!("State: {:?}", s);
                break;
            }
        }
    }
    Ok(output.iter().collect())
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}
