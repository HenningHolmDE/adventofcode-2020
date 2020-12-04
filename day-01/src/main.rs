use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn load_numbers_from_file(filename: &str) -> Result<Vec<u32>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines
        .filter_map(Result::ok)
        .filter_map(|line| line.parse().ok())
        .collect())
}

fn part1(numbers: &[u32]) {
    for (i, number1) in numbers.iter().enumerate() {
        for number2 in &numbers[i + 1..] {
            if number1 + number2 == 2020 {
                println!("{} * {} = {}", number1, number2, number1 * number2);
            }
        }
    }
}

fn part2(numbers: &[u32]) {
    for (i, number1) in numbers.iter().enumerate() {
        for (j, number2) in numbers[i + 1..].iter().enumerate() {
            for number3 in &numbers[i + j + 1..] {
                if number1 + number2 + number3 == 2020 {
                    println!(
                        "{} * {} * {} = {}",
                        number1,
                        number2,
                        number3,
                        number1 * number2 * number3
                    );
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = load_numbers_from_file("day-01/input.txt")?;
    part1(&numbers);
    part2(&numbers);
    Ok(())
}
