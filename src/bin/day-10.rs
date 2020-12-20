use std::io::{self, BufRead, BufReader};
use std::{collections::HashMap, fs::File};

const MAXIMUM_DIFFERENCE: u32 = 3;

fn load_numbers_from_file(filename: &str) -> Result<Vec<u32>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines
        .filter_map(Result::ok)
        .filter_map(|line| line.parse().ok())
        .collect())
}

fn numbers_to_jolts(mut numbers: Vec<u32>) -> Vec<u32> {
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers[numbers.len() - 1] + MAXIMUM_DIFFERENCE);
    numbers
}

fn part1(jolts: &Vec<u32>) {
    let differences = jolts
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<u32>>();
    let diff_of_1 = differences.iter().filter(|&d| *d == 1).count();
    let diff_of_3 = differences.iter().filter(|&d| *d == 3).count();
    println!(
        "There {} differences of 1 jolt and {} differences of 3 jolts, the product of these is: {}",
        diff_of_1,
        diff_of_3,
        diff_of_1 * diff_of_3
    );
}

fn number_of_valid_arrangements(
    start_value: u32,
    jolts: &[u32],
    cache: &mut HashMap<u32, u64>,
) -> u64 {
    if cache.contains_key(&start_value) {
        return cache[&start_value];
    }
    let mut result = 0;
    for (i, jolt) in jolts.iter().enumerate() {
        if *jolt > start_value + MAXIMUM_DIFFERENCE {
            break;
        }
        if i == jolts.len() - 1 {
            result += 1;
        } else {
            result += number_of_valid_arrangements(*jolt, &jolts[i + 1..], cache);
        }
    }
    cache.insert(start_value, result);
    result
}

fn part2(jolts: &Vec<u32>) {
    println!(
        "Number of valid arrangements: {:#?}",
        number_of_valid_arrangements(0, &jolts[1..], &mut HashMap::new())
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = load_numbers_from_file("inputs/day-10.txt")?;
    let jolts = numbers_to_jolts(numbers);
    part1(&jolts);
    part2(&jolts);
    Ok(())
}
