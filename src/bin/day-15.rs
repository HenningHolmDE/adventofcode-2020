use std::collections::HashMap;

fn calculate_number_for_round(starting_numbers: &[u32], last_round: u32) -> u32 {
    let mut round = 0;
    let mut history = HashMap::new();
    for number in starting_numbers {
        round += 1;
        history.insert(*number, round);
    }
    let mut number = starting_numbers[starting_numbers.len() - 1];
    while round < last_round {
        number = match history.insert(number, round) {
            Some(spoken_in_round) => round - spoken_in_round,
            None => 0,
        };
        round += 1;
    }
    return number;
}

fn main() {
    let starting_numbers = vec![18, 11, 9, 0, 5, 1];
    let round = 2020;
    println!(
        "Part 1: Round {}: {}",
        round,
        calculate_number_for_round(&starting_numbers, round)
    );
    let round = 30000000;
    println!(
        "Part 2: Round {}: {}",
        round,
        calculate_number_for_round(&starting_numbers, round)
    );
}
