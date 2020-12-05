use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct BoardingPass {
    seat_id: u32,
}

impl BoardingPass {
    fn new(code: &str) -> Self {
        let binary = code
            .replace("B", "1")
            .replace("F", "0")
            .replace("R", "1")
            .replace("L", "0");
        Self {
            seat_id: u32::from_str_radix(&binary, 2).unwrap(),
        }
    }
    fn seat_id(&self) -> u32 {
        self.seat_id
    }
}

fn load_boarding_passes_from_file(filename: &str) -> Result<Vec<BoardingPass>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines
        .filter_map(Result::ok)
        .map(|code| BoardingPass::new(&code))
        .collect())
}

fn highest_seat_id(boarding_passes: &Vec<BoardingPass>) -> Option<u32> {
    boarding_passes.iter().map(|b| b.seat_id()).max()
}

fn seat_id_found(boarding_passes: &Vec<BoardingPass>, seat_id: u32) -> bool {
    boarding_passes.iter().any(|b| b.seat_id() == seat_id)
}

fn find_my_seat(boarding_passes: &Vec<BoardingPass>) -> Option<u32> {
    for seat_id in 1..highest_seat_id(boarding_passes).unwrap() {
        if !seat_id_found(boarding_passes, seat_id)
            && seat_id_found(boarding_passes, seat_id - 1)
            && seat_id_found(boarding_passes, seat_id + 1)
        {
            return Some(seat_id);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boarding_passes = load_boarding_passes_from_file("day-05/input.txt")?;
    println!(
        "Highest seat ID: {}",
        highest_seat_id(&boarding_passes).unwrap()
    );
    println!("My seat ID: {}", find_my_seat(&boarding_passes).unwrap());
    Ok(())
}
