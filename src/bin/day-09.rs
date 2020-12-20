use std::fs::File;
use std::io::{self, BufRead, BufReader};

const PREAMBLE_LENGTH: usize = 25;

fn load_numbers_from_file(filename: &str) -> Result<Vec<u64>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines
        .filter_map(Result::ok)
        .filter_map(|line| line.parse().ok())
        .collect())
}

fn number_valid(previous_numbers: &[u64], number: &u64) -> bool {
    previous_numbers[..previous_numbers.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, a)| previous_numbers[i + 1..].iter().map(move |b| (a, b)))
        .any(|(a, b)| a + b == *number)
}

fn part1(numbers: &Vec<u64>) -> Option<usize> {
    for (i, number) in numbers[PREAMBLE_LENGTH..numbers.len()].iter().enumerate() {
        // due to enumerate() starting with 0, i is the start index of the PREAMBLE_LENGTH slice
        let previous_numbers = &numbers[i..i + PREAMBLE_LENGTH];
        if !number_valid(previous_numbers, number) {
            println!(
                "Invalid number: {} not sum of previous {} numbers.",
                number, PREAMBLE_LENGTH
            );
            return Some(i + PREAMBLE_LENGTH);
        }
    }
    None
}

fn find_contiguous_set(numbers: &Vec<u64>, index_p1: usize) -> Option<&[u64]> {
    let invalid_number = numbers[index_p1];
    // - 1: at least two numbers
    for start in 0..index_p1 - 1 {
        let mut sum = 0;
        for current in start..index_p1 {
            sum += numbers[current];
            if sum == invalid_number {
                return Some(&numbers[start..=current]);
            }
        }
    }
    None
}

fn part2(numbers: &Vec<u64>, index_p1: usize) {
    let contiguous_set = find_contiguous_set(numbers, index_p1).unwrap();
    let mut numbers_of_set = Vec::from(contiguous_set);
    numbers_of_set.sort();
    let smallest = numbers_of_set[0];
    let largest = numbers_of_set[numbers_of_set.len() - 1];
    println!(
        "Sum of smallest ({}) and largest ({}) number of contiguous set: {}",
        smallest,
        largest,
        smallest + largest
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = load_numbers_from_file("inputs/day-09.txt")?;
    println!("Number of numbers: {}", numbers.len());
    let index_p1 = part1(&numbers).unwrap();
    part2(&numbers, index_p1);
    Ok(())
}
