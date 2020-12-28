#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug, PartialEq)]
struct Range {
    minimum: u32,
    maximum: u32,
}

impl Range {
    fn is_valid(&self, value: u32) -> bool {
        value >= self.minimum && value <= self.maximum
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Rule {
    name: String,
    ranges: Vec<Range>,
}

impl Rule {
    fn is_valid(&self, value: u32) -> bool {
        self.ranges.iter().any(|range| range.is_valid(value))
    }
}

#[derive(Debug)]
struct Data {
    rules: Vec<Rule>,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

#[derive(Debug)]
enum LoadState {
    Rules,
    Idle,
    YourTicket,
    NearbyTickets,
}

fn parse_rule(line: &str) -> Rule {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let name = captures[1].to_owned();
    let ranges = vec![
        Range {
            minimum: captures[2].parse::<u32>().unwrap(),
            maximum: captures[3].parse::<u32>().unwrap(),
        },
        Range {
            minimum: captures[4].parse::<u32>().unwrap(),
            maximum: captures[5].parse::<u32>().unwrap(),
        },
    ];
    Rule { name, ranges }
}

fn parse_ticket(line: &str) -> Vec<u32> {
    line.split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn load_data_from_file(filename: &str) -> Result<Data, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut load_state = LoadState::Rules;
    let mut rules = Vec::new();
    let mut your_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                load_state = LoadState::Idle;
            } else {
                let line = line.trim();
                match load_state {
                    LoadState::Rules => rules.push(parse_rule(&line)),
                    LoadState::Idle => {
                        if line == "your ticket:" {
                            load_state = LoadState::YourTicket;
                        } else if line == "nearby tickets:" {
                            load_state = LoadState::NearbyTickets;
                        }
                    }
                    LoadState::YourTicket => your_ticket = parse_ticket(&line),
                    LoadState::NearbyTickets => nearby_tickets.push(parse_ticket(&line)),
                }
            }
        }
    }
    Ok(Data {
        rules,
        your_ticket,
        nearby_tickets,
    })
}

fn is_valid(rules: &Vec<Rule>, value: u32) -> bool {
    rules.iter().any(|rule| rule.is_valid(value))
}

fn part1(data: &Data) {
    let invalid_values: Vec<_> = data
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&&value| !is_valid(&data.rules, value))
        .cloned()
        .collect();
    println!(
        "Part 1: Sum of invalid values: {}",
        invalid_values.iter().sum::<u32>()
    );
}

fn part2(data: &Data) {
    let valid_tickets: Vec<_> = data
        .nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|&value| is_valid(&data.rules, value)))
        .cloned()
        .collect();

    let mut field_mapping = HashMap::new();
    let number_of_fields = valid_tickets[0].len();
    let mut remaining_rules = data.rules.clone();
    let mut remaining_fields: Vec<_> = (0..number_of_fields).collect();
    while field_mapping.len() < number_of_fields {
        for field in remaining_fields.clone() {
            let field_values: Vec<_> = valid_tickets.iter().map(|ticket| ticket[field]).collect();
            let valid_rules: Vec<_> = remaining_rules
                .iter()
                .filter(|rule| field_values.iter().all(|&value| rule.is_valid(value)))
                .collect();
            if valid_rules.len() == 1 {
                let rule = valid_rules[0].clone();
                field_mapping.insert(rule.name.to_owned(), field);
                remaining_rules.remove(remaining_rules.iter().position(|r| *r == rule).unwrap());
                remaining_fields.remove(remaining_fields.iter().position(|f| *f == field).unwrap());
            }
        }
    }

    let departure_product: u64 = field_mapping
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| v)
        .map(|&f| data.your_ticket[f] as u64)
        .product();
    println!("Part 2: Product of departure fields: {}", departure_product);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = load_data_from_file("inputs/day-16.txt")?;
    part1(&data);
    part2(&data);
    Ok(())
}
