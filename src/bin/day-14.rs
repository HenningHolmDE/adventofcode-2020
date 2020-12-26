#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Command {
    Mask(String),
    Mem((u64, u64)),
}

impl Command {
    fn new(line: String) -> Self {
        if line.starts_with("mem") {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
            }
            let captures = RE.captures(&line).unwrap();
            let address = captures[1].parse::<u64>().unwrap();
            let value = captures[2].parse::<u64>().unwrap();
            Self::Mem((address, value))
        } else {
            debug_assert!(line.starts_with("mask = "));
            Self::Mask(line[7..].to_owned())
        }
    }
}

fn load_commands_from_file(filename: &str) -> Result<Vec<Command>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.filter_map(Result::ok).map(Command::new).collect())
}

#[derive(Debug)]
struct ValueMask {
    and_mask: u64,
    or_mask: u64,
}

impl ValueMask {
    fn default() -> Self {
        Self {
            and_mask: u64::MAX,
            or_mask: 0,
        }
    }
    fn new(mask: &str) -> Self {
        let mut and_mask = u64::MAX;
        let mut or_mask = 0;
        for (i, c) in mask.chars().rev().enumerate() {
            match c {
                '0' => and_mask &= !(1 << i),
                '1' => or_mask |= 1 << i,
                _ => (),
            }
        }
        Self { and_mask, or_mask }
    }
    fn apply(&self, value: u64) -> u64 {
        (value & self.and_mask) | self.or_mask
    }
}

fn part1(commands: &Vec<Command>) {
    let mut mask = ValueMask::default();
    let mut memory = HashMap::new();
    for command in commands {
        match command {
            Command::Mask(m) => {
                mask = ValueMask::new(m);
            }
            Command::Mem((addr, val)) => {
                memory.insert(addr, mask.apply(*val));
            }
        }
    }
    println!("Part 1: Sum of memory: {}", memory.values().sum::<u64>());
}

#[derive(Debug)]
struct AddressMask {
    and_mask: u64,
    or_mask: u64,
    offsets: Vec<u64>,
}

impl AddressMask {
    fn default() -> Self {
        Self {
            and_mask: u64::MAX,
            or_mask: 0,
            offsets: Vec::new(),
        }
    }
    fn new(mask: &str) -> Self {
        let mut and_mask = 0;
        let mut or_mask = 0;
        let mut floating_bits = Vec::new();
        for (i, c) in mask.chars().rev().enumerate() {
            match c {
                '0' => and_mask |= 1 << i,
                '1' => or_mask |= 1 << i,
                _ => floating_bits.push(i),
            }
        }
        let mut offsets = vec![0];
        for floating_bit in floating_bits {
            for i in 0..offsets.len() {
                offsets.push(offsets[i] + (1 << floating_bit));
            }
        }
        Self {
            and_mask,
            or_mask,
            offsets,
        }
    }
    fn apply(&self, address: u64) -> Vec<u64> {
        let base_address = (address & self.and_mask) | self.or_mask;
        self.offsets
            .iter()
            .map(|offset| base_address + offset)
            .collect()
    }
}

fn part2(commands: &Vec<Command>) {
    let mut mask = AddressMask::default();
    let mut memory = HashMap::new();
    for command in commands {
        match command {
            Command::Mask(m) => {
                mask = AddressMask::new(m);
            }
            Command::Mem((addr, val)) => {
                for address in mask.apply(*addr) {
                    memory.insert(address, *val);
                }
            }
        }
    }
    println!("Part 2: Sum of memory: {}", memory.values().sum::<u64>());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commands = load_commands_from_file("inputs/day-14.txt")?;
    part1(&commands);
    part2(&commands);
    Ok(())
}
