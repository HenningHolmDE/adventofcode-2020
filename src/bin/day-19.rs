#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug, PartialEq)]
enum Rule {
    Letter(char),
    Sequences(Vec<Vec<u32>>),
}

impl Rule {
    fn new(input: String) -> (u32, Self) {
        lazy_static! {
            static ref RE_RULE: Regex = Regex::new(r"^(?P<id>\d+): (?P<pattern>.+)$").unwrap();
            static ref RE_LETTER: Regex = Regex::new(r#"^"(?P<letter>\w)"$"#).unwrap();
        }
        let captures = RE_RULE.captures(&input).unwrap();
        let id = captures
            .name("id")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let pattern: &str = captures.name("pattern").unwrap().as_str();
        let rule = if let Some(letter) = RE_LETTER.captures(&pattern).and_then(|c| c.name("letter"))
        {
            Self::Letter(letter.as_str().chars().next().unwrap())
        } else {
            Self::Sequences(
                pattern
                    .split('|')
                    .map(|seq| {
                        seq.trim()
                            .split(' ')
                            .map(|id| id.parse::<u32>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect(),
            )
        };
        (id, rule)
    }

    fn try_match<'a>(&self, rules: &Rules, input: &'a str) -> Option<HashSet<&'a str>> {
        match self {
            Self::Letter(letter) => {
                if Some(*letter) == input.chars().next() {
                    Some(vec![&input[1..]].into_iter().collect())
                } else {
                    None
                }
            }
            Self::Sequences(sequences) => {
                let mut result = HashSet::new();
                for sequence in sequences {
                    let mut remaining: HashSet<_> = vec![input].into_iter().collect();
                    for id in sequence {
                        let mut new_remaining = HashSet::new();
                        for remaining_str in remaining {
                            if let Some(rem) =
                                rules.0.get(id).unwrap().try_match(rules, remaining_str)
                            {
                                new_remaining.extend(rem);
                            }
                        }
                        remaining = new_remaining;
                    }
                    if remaining.contains("") {
                        // short circuit for match
                        return Some(vec![""].into_iter().collect());
                    }
                    result.extend(remaining);
                }
                match result.is_empty() {
                    false => Some(result),
                    true => None,
                }
            }
        }
    }
}

#[derive(Debug)]
struct Rules(HashMap<u32, Rule>);

impl Rules {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn insert(&mut self, k: u32, rule: Rule) {
        self.0.insert(k, rule);
    }
    fn check_messages(&self, messages: &Vec<String>) -> usize {
        messages
            .iter()
            .map(|m| self.0.get(&0).unwrap().try_match(&self, &m))
            .filter(|res| res.is_some() && res.as_ref().unwrap().contains(""))
            .count()
    }
}

fn load_data_from_file(filename: &str) -> Result<(Rules, Vec<String>), io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut rules = Rules::new();
    let mut messages = Vec::new();
    let mut rules_not_messages = true;
    let mut lines_iter = lines
        .filter_map(Result::ok)
        .map(|line| line.trim().to_owned());
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            rules_not_messages = false;
        } else {
            if rules_not_messages {
                let (id, rule) = Rule::new(line);
                rules.insert(id, rule);
            } else {
                messages.push(line);
            }
        }
    }
    Ok((rules, messages))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rules, messages) = load_data_from_file("inputs/day-19.txt")?;

    println!(
        "Part 1: Number of messages matching rule 0: {}",
        rules.check_messages(&messages)
    );

    let rules = {
        let mut rules = rules;
        let (id, rule) = Rule::new("8: 42 | 42 8".to_owned());
        rules.insert(id, rule);
        let (id, rule) = Rule::new("11: 42 31 | 42 11 31".to_owned());
        rules.insert(id, rule);
        rules
    };
    println!(
        "Part 2: Number of messages matching rule 0: {}",
        rules.check_messages(&messages)
    );

    Ok(())
}
