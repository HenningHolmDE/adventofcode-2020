use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Group {
    answers: Vec<String>,
}

impl Group {
    fn new() -> Self {
        Group {
            answers: Vec::new(),
        }
    }
    fn add_answers(&mut self, answers: &str) {
        self.answers.push(answers.to_string());
    }
    fn number_of_questions_anyone(&self) -> usize {
        let mut questions = Vec::new();
        for answer in self.answers.iter() {
            for question in answer.chars() {
                if !questions.contains(&question) {
                    questions.push(question);
                }
            }
        }
        questions.len()
    }
    fn number_of_questions_everyone(&self) -> usize {
        let mut questions = Vec::new();
        for (i, answer) in self.answers.iter().enumerate() {
            let mut questions_new = Vec::new();
            for question in answer.chars() {
                if i == 0 || questions.contains(&question) {
                    questions_new.push(question);
                }
            }
            questions = questions_new;
        }
        questions.len()
    }
}

fn load_groups_from_file(filename: &str) -> Result<Vec<Group>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut groups = Vec::new();
    let mut group = Group::new();
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                groups.push(group);
                group = Group::new();
            } else {
                group.add_answers(&line);
            }
        }
    }
    groups.push(group);
    Ok(groups)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let groups = load_groups_from_file("day-06/input.txt")?;
    let number_of_questions: usize = groups.iter().map(|g| g.number_of_questions_anyone()).sum();
    println!(
        "Number of questions ANYONE in a group answered: {}",
        number_of_questions
    );
    let number_of_questions: usize = groups
        .iter()
        .map(|g| g.number_of_questions_everyone())
        .sum();
    println!(
        "Number of questions EVERYONE in a group answered: {}",
        number_of_questions
    );
    Ok(())
}
