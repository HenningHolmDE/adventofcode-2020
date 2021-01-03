use std::collections::HashMap;

#[derive(Clone, Debug)]
struct CupCircle {
    cups: HashMap<u32, (u32, u32)>,
    number_of_cups: usize,
}

impl CupCircle {
    fn new(mut starting_cups: Vec<u32>, number_of_cups: usize) -> Self {
        let first_cup = starting_cups.remove(0);
        let mut cup_circle = Self {
            cups: vec![(first_cup, (first_cup, first_cup))]
                .into_iter()
                .collect(),
            number_of_cups,
        };
        starting_cups.iter().for_each(|c| {
            cup_circle.insert_cup_before(first_cup, *c);
        });
        (cup_circle.len()..number_of_cups)
            .map(|c| c as u32 + 1)
            .for_each(|c| cup_circle.insert_cup_before(first_cup, c));
        cup_circle
    }

    fn len(&self) -> usize {
        self.cups.len()
    }

    fn insert_cup_before(&mut self, label: u32, cup: u32) {
        let label_tuple = self.cups.remove(&label).unwrap();
        self.cups.insert(cup, (label_tuple.0, label));
        self.cups.insert(label, (cup, label_tuple.1));
        // update pointer of previous cup
        let prev_tuple = self.cups.get(&label_tuple.0).unwrap().clone();
        self.cups.insert(label_tuple.0, (prev_tuple.0, cup));
    }

    fn insert_cup_after(&mut self, label: u32, cup: u32) {
        let label_tuple = self.cups.remove(&label).unwrap();
        self.cups.insert(cup, (label, label_tuple.1));
        self.cups.insert(label, (label_tuple.0, cup));
        // update pointer of next cup
        let next_tuple = self.cups.get(&label_tuple.1).unwrap().clone();
        self.cups.insert(label_tuple.1, (cup, next_tuple.1));
    }

    fn remove_cups_after(&mut self, label: u32, amount: usize) -> Vec<u32> {
        let mut label_tuple = self.cups.remove(&label).unwrap();
        let mut pick_up = Vec::new();
        for _ in 0..amount {
            pick_up.push(label_tuple.1);
            let cup_tuple = self.cups.remove(&label_tuple.1).unwrap();
            label_tuple.1 = cup_tuple.1;
        }
        self.cups.insert(label, label_tuple);
        // update pointer of next cup
        let next_tuple = self.cups.get(&label_tuple.1).unwrap().clone();
        self.cups.insert(label_tuple.1, (label, next_tuple.1));
        pick_up
    }

    fn find_destination_cup(&self, current_cup: u32) -> u32 {
        let mut destination = current_cup - 1;
        while self.cups.get(&destination).is_none() {
            if destination == 0 {
                destination = self.number_of_cups as u32;
            } else {
                destination -= 1;
            }
        }
        destination
    }

    fn find_next_cup(&self, label: u32) -> u32 {
        self.cups.get(&label).unwrap().1
    }

    fn to_display_string(&self, current_cup: u32) -> String {
        let mut result = String::new();
        let mut cup = current_cup;
        loop {
            result.push_str(&match cup == current_cup {
                true => format!("({})", cup),
                false => format!(" {} ", cup),
            });
            cup = self.find_next_cup(cup);
            if cup == current_cup {
                break;
            }
        }
        result
    }

    fn result_part1(&self) -> String {
        let mut result = String::new();
        let mut cup = 1;
        loop {
            cup = self.find_next_cup(cup);
            if cup == 1 {
                break;
            }
            result.push_str(&cup.to_string());
        }
        result
    }

    fn result_part2(&self) -> u64 {
        let first_star_cup = self.find_next_cup(1);
        let second_star_cup = self.find_next_cup(first_star_cup);
        first_star_cup as u64 * second_star_cup as u64
    }
}

#[derive(Clone, Debug)]
struct Game {
    cups: CupCircle,
    current_cup: u32,
    debug: bool,
}

impl Game {
    fn new(cups: Vec<u32>, number_of_cups: usize) -> Self {
        let current_cup = cups[0];
        Self {
            cups: CupCircle::new(cups, number_of_cups),
            current_cup,
            debug: false,
        }
    }

    #[allow(dead_code)]
    fn enable_debug(&mut self) {
        self.debug = true;
    }

    fn play(&mut self, moves: u32) {
        for move_ in 1..moves + 1 {
            if self.debug {
                println!("-- move {} --", move_);
                println!("cups: {}", self.cups.to_display_string(self.current_cup));
            }
            let pick_up = self.cups.remove_cups_after(self.current_cup, 3);
            let destination = self.cups.find_destination_cup(self.current_cup);
            if self.debug {
                println!(
                    "pick up: {}",
                    pick_up
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                println!("destination: {}", destination);
                println!("");
            }
            pick_up
                .iter()
                .rev()
                .for_each(|c| self.cups.insert_cup_after(destination, *c));
            self.current_cup = self.cups.find_next_cup(self.current_cup);
        }

        if self.debug {
            println!("-- final --");
            println!("cups: {}", self.cups.to_display_string(self.current_cup));
        }
    }

    fn result_part1(&self) -> String {
        self.cups.result_part1()
    }

    fn result_part2(&self) -> u64 {
        self.cups.result_part2()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let labeling = "496138527";
    let cups: Vec<_> = labeling.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // part 1
    let number_of_cups = cups.len();
    let mut game = Game::new(cups.clone(), number_of_cups);
    // game.enable_debug();
    game.play(100);
    println!(
        "Part 1: labels on the cups after cup 1: {}",
        game.result_part1()
    );

    // part 2
    let mut game = Game::new(cups.clone(), 1000000);
    // // game.enable_debug();
    game.play(10000000);
    println!(
        "Part 2: Product of star cup labels: {}",
        game.result_part2()
    );

    Ok(())
}
