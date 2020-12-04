use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct PasswordPolicy {
    character: char,
    minimum_first: usize,
    maximum_second: usize,
}

#[derive(Debug)]
struct PasswordWithPolicy {
    password: String,
    policy: PasswordPolicy,
}

impl PasswordWithPolicy {
    fn new(captures: regex::Captures) -> Option<PasswordWithPolicy> {
        let password = captures.name("password")?.as_str().to_owned();
        let character = captures.name("character")?.as_str().chars().next()?;
        let minimum_first = captures.name("minimum_first")?.as_str().parse().ok()?;
        let maximum_second = captures.name("maximum_second")?.as_str().parse().ok()?;
        Some(PasswordWithPolicy {
            password,
            policy: PasswordPolicy {
                character,
                minimum_first,
                maximum_second,
            },
        })
    }

    fn is_valid1(&self) -> bool {
        let policy_character_count = self
            .password
            .chars()
            .filter(|c| c == &self.policy.character)
            .count();
        self.policy.minimum_first <= policy_character_count
            && policy_character_count <= self.policy.maximum_second
    }

    fn is_valid2(&self) -> bool {
        let chars = self.password.chars();
        let first_pos = chars.clone().nth(self.policy.minimum_first - 1);
        let second_pos = chars.clone().nth(self.policy.maximum_second - 1);
        let mut is_valid = first_pos == Some(self.policy.character);
        if second_pos == Some(self.policy.character) {
            is_valid = !is_valid;
        }
        is_valid
    }
}

fn load_passwords_from_file(filename: &str) -> Result<Vec<PasswordWithPolicy>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let re = Regex::new(
        r"^(?P<minimum_first>\d+)-(?P<maximum_second>\d+) (?P<character>[a-z]): (?P<password>[a-z]+)$",
    )
    .unwrap();
    Ok(lines
        .filter_map(Result::ok)
        .filter_map(|line| PasswordWithPolicy::new(re.captures(&line)?))
        .collect())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passwords = load_passwords_from_file("day-02/input.txt")?;
    let number_of_valid_passwords = passwords.iter().filter(|p| p.is_valid1()).count();
    println!(
        "{} of {} passwords are valid according to first policy scheme.",
        number_of_valid_passwords,
        passwords.len(),
    );
    let number_of_valid_passwords = passwords.iter().filter(|p| p.is_valid2()).count();
    println!(
        "{} of {} passwords are valid according to second policy scheme.",
        number_of_valid_passwords,
        passwords.len(),
    );
    Ok(())
}
