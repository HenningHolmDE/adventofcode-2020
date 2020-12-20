use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    fn new(input: &str) -> Self {
        let value: i32 = input[1..].parse().unwrap();
        match &input[..1] {
            "N" => Self::North(value),
            "S" => Self::South(value),
            "E" => Self::East(value),
            "W" => Self::West(value),
            "L" => Self::Left(value),
            "R" => Self::Right(value),
            "F" => Self::Forward(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Orientation {
    East,
    North,
    West,
    South,
}

#[derive(Debug)]
struct Ship {
    latitude: i32,  // positive: north
    longitude: i32, // positive: east
    orientation: Orientation,
}

impl Ship {
    fn new() -> Self {
        Self {
            latitude: 0,
            longitude: 0,
            orientation: Orientation::East,
        }
    }
    fn handle_actions(&mut self, actions: &Vec<Action>) {
        actions.iter().for_each(|action| self.handle_action(action));
    }
    fn handle_action(&mut self, action: &Action) {
        match action {
            Action::North(value) => self.latitude += value,
            Action::South(value) => self.latitude -= value,
            Action::East(value) => self.longitude += value,
            Action::West(value) => self.longitude -= value,
            Action::Left(value) => self.rotate(*value),
            Action::Right(value) => self.rotate(-value),
            Action::Forward(value) => match self.orientation {
                Orientation::East => self.longitude += value,
                Orientation::North => self.latitude += value,
                Orientation::West => self.longitude -= value,
                Orientation::South => self.latitude -= value,
            },
        }
    }
    fn rotate(&mut self, degrees: i32) {
        let mut orientation = match self.orientation {
            Orientation::East => 0,
            Orientation::North => 90,
            Orientation::West => 180,
            Orientation::South => 270,
        };
        orientation += degrees;
        orientation %= 360;
        if orientation < 0 {
            orientation += 360;
        }
        self.orientation = match orientation {
            0 => Orientation::East,
            90 => Orientation::North,
            180 => Orientation::West,
            270 => Orientation::South,
            _ => unreachable!(),
        };
    }
    fn manhattan_distance(&self) -> u32 {
        i32::abs(self.longitude) as u32 + i32::abs(self.latitude) as u32
    }
}

#[derive(Debug)]
struct Waypoint {
    latitude: i32,  // positive: north
    longitude: i32, // positive: east
}

impl Waypoint {
    fn new() -> Self {
        Self {
            latitude: 1,
            longitude: 10,
        }
    }

    fn move_north(&mut self, value: i32) {
        self.latitude += value;
    }
    fn move_south(&mut self, value: i32) {
        self.latitude -= value;
    }
    fn move_east(&mut self, value: i32) {
        self.longitude += value;
    }
    fn move_west(&mut self, value: i32) {
        self.longitude -= value;
    }

    fn rotate_left(&mut self, degrees: i32) {
        for _ in 0..(degrees / 90) {
            self.rotate_left_90();
        }
    }
    fn rotate_right(&mut self, degrees: i32) {
        for _ in 0..(degrees / 90) {
            self.rotate_right_90();
        }
    }

    fn rotate_left_90(&mut self) {
        let old_latitude = self.latitude;
        self.latitude = self.longitude;
        self.longitude = -old_latitude;
    }
    fn rotate_right_90(&mut self) {
        let old_latitude = self.latitude;
        self.latitude = -self.longitude;
        self.longitude = old_latitude;
    }
}

#[derive(Debug)]
struct ShipWithWaypoint {
    latitude: i32,  // positive: north
    longitude: i32, // positive: east
    waypoint: Waypoint,
}

impl ShipWithWaypoint {
    fn new() -> Self {
        Self {
            latitude: 0,
            longitude: 0,
            waypoint: Waypoint::new(),
        }
    }
    fn handle_actions(&mut self, actions: &Vec<Action>) {
        actions.iter().for_each(|action| self.handle_action(action));
    }
    fn handle_action(&mut self, action: &Action) {
        match action {
            Action::North(value) => self.waypoint.move_north(*value),
            Action::South(value) => self.waypoint.move_south(*value),
            Action::East(value) => self.waypoint.move_east(*value),
            Action::West(value) => self.waypoint.move_west(*value),
            Action::Left(value) => self.waypoint.rotate_left(*value),
            Action::Right(value) => self.waypoint.rotate_right(*value),
            Action::Forward(value) => {
                self.latitude += value * self.waypoint.latitude;
                self.longitude += value * self.waypoint.longitude;
            }
        }
    }
    fn manhattan_distance(&self) -> u32 {
        i32::abs(self.longitude) as u32 + i32::abs(self.latitude) as u32
    }
}

fn load_actions_from_file(filename: &str) -> Result<Vec<Action>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines
        .filter_map(Result::ok)
        .map(|code| Action::new(&code))
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let actions = load_actions_from_file("day-12/input.txt")?;

    let mut ship = Ship::new();
    ship.handle_actions(&actions);
    println!(
        "Part 1: Manhattan distance from starting position: {}",
        ship.manhattan_distance()
    );

    let mut ship = ShipWithWaypoint::new();
    ship.handle_actions(&actions);
    println!(
        "Part 2: Manhattan distance from starting position: {}",
        ship.manhattan_distance()
    );

    Ok(())
}
