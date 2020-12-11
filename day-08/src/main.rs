use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
enum Command {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Command {
    fn new(line: String) -> Self {
        let mut line = line.split(" ");
        let (command, argument) = (line.next().unwrap(), line.next().unwrap());
        match command {
            "acc" => Self::Acc(argument.parse().unwrap()),
            "jmp" => Self::Jmp(argument.parse().unwrap()),
            _ => Self::Nop(argument.parse().unwrap()),
        }
    }
}

type Program = Vec<Command>;

#[derive(Debug, PartialEq)]
enum ExecuteResult {
    EndlessLoop,
    Finished,
}

#[derive(Debug)]
struct Interpreter {
    program: Program,
    accumulator: i32,
    program_counter: usize,
    trace: Vec<usize>,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            accumulator: 0,
            program_counter: 0,
            trace: Vec::new(),
        }
    }
    pub fn accumulator(&self) -> i32 {
        self.accumulator
    }
    pub fn execute(&mut self) -> ExecuteResult {
        while !self.trace.contains(&self.program_counter) {
            self.trace.push(self.program_counter);
            self.step();
            if self.program_counter == self.program.len() {
                return ExecuteResult::Finished;
            }
        }
        ExecuteResult::EndlessLoop
    }

    fn step(&mut self) {
        let command = self.fetch();
        if let Command::Acc(argument) = command {
            self.accumulator += argument;
        }
        self.program_counter = match command {
            Command::Jmp(offset) => {
                if offset.is_negative() {
                    self.program_counter - offset.wrapping_abs() as u32 as usize
                } else {
                    self.program_counter + offset as usize
                }
            }
            _ => self.program_counter + 1,
        }
    }

    fn fetch(&self) -> Command {
        self.program[self.program_counter].clone()
    }
}

fn load_program_from_file(filename: &str) -> Result<Program, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.filter_map(Result::ok).map(Command::new).collect())
}

fn part1(program: Program) {
    let mut interpreter = Interpreter::new(program);
    interpreter.execute();
    println!(
        "Accumulator value when entering endless loop: {}",
        interpreter.accumulator()
    );
}

fn modify_command(command: Command) -> Command {
    match command {
        Command::Acc(_) => command,
        Command::Jmp(param) => Command::Nop(param),
        Command::Nop(param) => Command::Jmp(param),
    }
}

fn modify_program(program: Program, index: usize) -> Option<Program> {
    if let Command::Acc(_) = program[index] {
        None // there is no sense in executing an unchanged version
    } else {
        Some(
            program
                .into_iter()
                .enumerate()
                .map(|(i, cmd)| if i == index { modify_command(cmd) } else { cmd })
                .collect(),
        )
    }
}

fn part2(program: Program) {
    // try to modify every instruction of the program
    for instruction_index in 0..program.len() {
        if let Some(program) = modify_program(program.clone(), instruction_index) {
            let mut interpreter = Interpreter::new(program);
            if interpreter.execute() == ExecuteResult::Finished {
                println!(
                    "Program finished! Final accumulator value: {}",
                    interpreter.accumulator()
                );
                return;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = load_program_from_file("day-08/input.txt")?;
    part1(program.clone());
    part2(program);
    Ok(())
}
