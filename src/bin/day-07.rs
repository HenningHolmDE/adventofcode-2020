#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
struct Content {
    bag_type: String,
    amount: u32,
}

impl Content {
    fn new(content: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<amount>[0-9]+) (?P<bag_type>.*) bags?$").unwrap();
        }
        let captures = RE.captures(content).unwrap();
        Self {
            bag_type: captures.name("bag_type").unwrap().as_str().to_owned(),
            amount: captures.name("amount").unwrap().as_str().parse().unwrap(),
        }
    }
    fn bag_type(&self) -> String {
        self.bag_type.clone()
    }
    fn amount(&self) -> u32 {
        self.amount
    }
}

#[derive(Debug)]
struct Rule {
    bag_type: String,
    contents: Vec<Content>,
}

impl Rule {
    fn new(line: String) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<bag_type>.*) bags contain (?P<contents>.*).$").unwrap();
        }
        let captures = RE.captures(&line).unwrap();
        let bag_type = captures.name("bag_type").unwrap().as_str().to_owned();
        let contents = captures.name("contents").unwrap().as_str();
        let contents = if contents == "no other bags" {
            Vec::new()
        } else {
            contents.split(", ").map(Content::new).collect()
        };
        Self { bag_type, contents }
    }
    fn bag_type(&self) -> String {
        self.bag_type.clone()
    }
    fn contents(&self) -> Vec<Content> {
        self.contents.clone()
    }
    fn contains(&self, bag_type: &str) -> bool {
        self.contents
            .iter()
            .any(|content| content.bag_type == bag_type)
    }
}

fn load_rules_from_file(filename: &str) -> Result<Vec<Rule>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.filter_map(Result::ok).map(Rule::new).collect())
}

fn find_containers(rules: &Vec<Rule>, bag_type: &str) -> Vec<String> {
    rules
        .iter()
        .filter(|rule| rule.contains(bag_type))
        .flat_map(|rule| {
            vec![rule.bag_type()]
                .into_iter()
                .chain(find_containers(rules, &rule.bag_type()).into_iter())
        })
        .fold(Vec::new(), |containers, bag_type| {
            if !containers.contains(&bag_type) {
                let mut containers = containers;
                containers.push(bag_type);
                containers
            } else {
                containers
            }
        })
}

fn count_contents(rules: &Vec<Rule>, bag_type: &str) -> u32 {
    rules
        .iter()
        .filter(|rule| rule.bag_type() == bag_type)
        .flat_map(|rule| rule.contents())
        .map(|content| content.amount() * (1 + count_contents(rules, &content.bag_type())))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rules = load_rules_from_file("inputs/day-07.txt")?;
    println!(
        "Number of valid bags for a shiny gold bag: {}",
        find_containers(&rules, "shiny gold").len()
    );
    println!(
        "Number of bag inside a shiny gold bag: {}",
        count_contents(&rules, "shiny gold")
    );
    Ok(())
}
