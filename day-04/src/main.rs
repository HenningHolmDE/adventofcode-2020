#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn is_valid1(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
    fn is_valid2(&self) -> bool {
        fn year_valid(year: &Option<String>, minimum: u32, maximum: u32) -> bool {
            if let Some(year) = year {
                if let Some(year) = year.parse::<u32>().ok() {
                    year >= minimum && year <= maximum
                } else {
                    false
                }
            } else {
                false
            }
        }
        let height_valid = if let Some(height) = &self.height {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(\d+)(\w+)$").unwrap();
            }
            if let Some(cap) = RE.captures(height) {
                let number = cap[1].parse::<u32>().unwrap();
                match &cap[2] {
                    "cm" => number >= 150 && number <= 193,
                    "in" => number >= 59 && number <= 76,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        };
        let hair_color_valid = if let Some(hair_color) = &self.hair_color {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }
            RE.is_match(hair_color)
        } else {
            false
        };
        let eye_color_valid = if let Some(eye_color) = &self.eye_color {
            match eye_color.as_ref() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            }
        } else {
            false
        };
        let passport_id_valid = if let Some(passport_id) = &self.passport_id {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
            }
            RE.is_match(passport_id)
        } else {
            false
        };
        year_valid(&self.birth_year, 1920, 2002)
            && year_valid(&self.issue_year, 2010, 2020)
            && year_valid(&self.expiration_year, 2020, 2030)
            && height_valid
            && hair_color_valid
            && eye_color_valid
            && passport_id_valid
    }
}

#[derive(Debug)]
struct PassportBuilder {
    passport: Passport,
}

impl PassportBuilder {
    fn new() -> Self {
        Self {
            passport: Default::default(),
        }
    }

    fn add_data(&mut self, data: &str) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]{3}):(\S+)").unwrap();
        }
        for cap in RE.captures_iter(data) {
            let string_option = Some(cap[2].to_owned());
            match &cap[1] {
                "byr" => self.passport.birth_year = string_option,
                "iyr" => self.passport.issue_year = string_option,
                "eyr" => self.passport.expiration_year = string_option,
                "hgt" => self.passport.height = string_option,
                "hcl" => self.passport.hair_color = string_option,
                "ecl" => self.passport.eye_color = string_option,
                "pid" => self.passport.passport_id = string_option,
                "cid" => self.passport.country_id = string_option,
                _ => {}
            }
        }
    }

    fn finish(self) -> Passport {
        self.passport
    }
}

fn load_passports_from_file(filename: &str) -> Result<Vec<Passport>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut passports = Vec::new();
    let mut builder = PassportBuilder::new();
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                passports.push(builder.finish());
                builder = PassportBuilder::new();
            } else {
                builder.add_data(&line);
            }
        }
    }
    passports.push(builder.finish());
    Ok(passports)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passports = load_passports_from_file("day-04/input.txt")?;
    println!(
        "{} of {} passports are valid according to first validation scheme.",
        passports.iter().filter(|p| p.is_valid1()).count(),
        passports.len(),
    );
    println!(
        "{} of {} passports are valid according to second validation scheme.",
        passports.iter().filter(|p| p.is_valid2()).count(),
        passports.len(),
    );
    Ok(())
}
