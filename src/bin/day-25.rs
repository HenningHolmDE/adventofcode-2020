use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn load_public_keys_from_file(filename: &str) -> Result<Vec<u32>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines
        .filter_map(Result::ok)
        .map(|key| key.parse().unwrap())
        .collect())
}

fn calculate_loop_sizes(public_keys: &Vec<u32>) -> Vec<usize> {
    public_keys
        .iter()
        .map(|&key| {
            let mut loop_size = 0;
            let subject_number = 7;
            let mut value = 1u64;
            while value != key as u64 {
                value *= subject_number;
                value %= 20201227;
                loop_size += 1;
            }
            loop_size
        })
        .collect()
}

fn calculate_encryption_key(public_key: u32, loop_size: usize) -> u32 {
    let mut value = 1u64;
    for _ in 0..loop_size {
        value *= public_key as u64;
        value %= 20201227;
    }
    value as u32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let public_keys = load_public_keys_from_file("inputs/day-25.txt")?;
    let loop_sizes = calculate_loop_sizes(&public_keys);
    let encryption_key = calculate_encryption_key(public_keys[0], loop_sizes[1]);
    println!("Part 1: Encryption key {}", encryption_key);
    Ok(())
}
