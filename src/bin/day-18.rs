use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(u32),
    Plus,
    Times,
    LeftParenthesis,
    RightParenthesis,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut start_of_number = None;
    let mut tokens = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if let Some(start) = start_of_number {
            match c {
                '0'..='9' => continue,
                _ => {
                    let number = input[start..i].parse::<u32>().unwrap();
                    tokens.push(Token::Number(number));
                    start_of_number = None;
                }
            }
        }
        match c {
            '0'..='9' => start_of_number = Some(i),
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Times),
            '(' => tokens.push(Token::LeftParenthesis),
            ')' => tokens.push(Token::RightParenthesis),
            _ => (),
        }
    }
    if let Some(start) = start_of_number {
        let number = input[start..].parse::<u32>().unwrap();
        tokens.push(Token::Number(number));
    }
    tokens
}

#[derive(Debug)]
struct Expression(Vec<Token>);

impl Expression {
    fn new(line: String) -> Self {
        Self(tokenize(&line))
    }

    fn evaluate1(&self) -> u64 {
        self.evaluate1_inner(&mut self.0.iter().rev().cloned().collect())
    }
    fn evaluate1_inner(&self, tokens: &mut Vec<Token>) -> u64 {
        let token = tokens.pop().expect("Unexpected end of token stream");
        let mut acc = match token {
            Token::Number(number) => number as u64,
            Token::LeftParenthesis => self.evaluate1_inner(tokens),
            token => panic!("Unexpected token: {:?}", token),
        };
        while tokens.len() > 0 {
            let operator = tokens.pop().expect("Unexpected end of token stream");
            assert!(
                operator == Token::Plus || operator == Token::Times,
                "Unexpected operator token: {:?}",
                operator
            );
            let token = tokens.pop().expect("Unexpected end of token stream");
            let operand = match token {
                Token::Number(number) => number as u64,
                Token::LeftParenthesis => self.evaluate1_inner(tokens),
                token => panic!("Unexpected token: {:?}", token),
            };
            match operator {
                Token::Plus => acc += operand as u64,
                Token::Times => acc *= operand as u64,
                _ => unreachable!(),
            }
            if tokens.last() == Some(&Token::RightParenthesis) {
                tokens.pop();
                break;
            }
        }
        acc
    }

    fn evaluate2(&self) -> u64 {
        self.evaluate2_product(&mut self.0.iter().rev().cloned().collect())
    }
    fn evaluate2_product(&self, tokens: &mut Vec<Token>) -> u64 {
        let mut acc = self.evaluate2_sum(tokens);
        while tokens.len() > 0 && tokens.last() != Some(&Token::RightParenthesis) {
            assert_eq!(
                tokens.pop().expect("Unexpected end of token stream"),
                Token::Times,
                "Expected operator token Times",
            );
            acc *= self.evaluate2_sum(tokens);
        }
        acc
    }
    fn evaluate2_sum(&self, tokens: &mut Vec<Token>) -> u64 {
        let token = tokens.pop().expect("Unexpected end of token stream");
        let mut acc = match token {
            Token::Number(number) => number as u64,
            Token::LeftParenthesis => {
                let result = self.evaluate2_product(tokens);
                assert_eq!(tokens.pop(), Some(Token::RightParenthesis));
                result
            }
            token => panic!("Unexpected token: {:?}", token),
        };
        while tokens.len() > 0
            && tokens.last() != Some(&Token::Times)
            && tokens.last() != Some(&Token::RightParenthesis)
        {
            assert_eq!(
                tokens.pop().expect("Unexpected end of token stream"),
                Token::Plus,
                "Expected operator token Plus",
            );
            let token = tokens.pop().expect("Unexpected end of token stream");
            let operand = match token {
                Token::Number(number) => number as u64,
                Token::LeftParenthesis => {
                    let result = self.evaluate2_product(tokens);
                    assert_eq!(tokens.pop(), Some(Token::RightParenthesis));
                    result
                }
                token => panic!("Unexpected token: {:?}", token),
            };
            acc += operand as u64;
        }
        acc
    }
}

fn load_expressions_from_file(filename: &str) -> Result<Vec<Expression>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.filter_map(Result::ok).map(Expression::new).collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expressions = load_expressions_from_file("inputs/day-18.txt")?;
    println!(
        "Part 1: Sum of resulting values: {}",
        expressions.iter().map(|e| e.evaluate1()).sum::<u64>()
    );
    println!(
        "Part 2: Sum of resulting values: {}",
        expressions.iter().map(|e| e.evaluate2()).sum::<u64>()
    );
    Ok(())
}
