use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;

#[derive(Clone, Debug)]
enum Dimensions {
    Three,
    Four,
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }
    fn neighbor_extent(&self) -> Extent {
        Extent {
            x: Range {
                start: self.x - 1,
                end: self.x + 2,
            },
            y: Range {
                start: self.y - 1,
                end: self.y + 2,
            },
            z: Range {
                start: self.z - 1,
                end: self.z + 2,
            },
            w: Range {
                start: self.w - 1,
                end: self.w + 2,
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Extent {
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
    w: Range<i32>,
}

impl Extent {
    fn new_empty() -> Self {
        // initialize with empty ranges
        Self {
            x: Range { start: 0, end: 0 },
            y: Range { start: 0, end: 0 },
            z: Range { start: 0, end: 0 },
            w: Range { start: 0, end: 0 },
        }
    }

    fn include_coord(&self, coord: &Coordinate) -> Extent {
        if self.x.is_empty() {
            // set range to coordinate
            Extent {
                x: Range {
                    start: coord.x,
                    end: coord.x + 1,
                },
                y: Range {
                    start: coord.y,
                    end: coord.y + 1,
                },
                z: Range {
                    start: coord.z,
                    end: coord.z + 1,
                },
                w: Range {
                    start: coord.w,
                    end: coord.w + 1,
                },
            }
        } else {
            // extend range to include coordinate
            Extent {
                x: Range {
                    start: i32::min(self.x.start, coord.x),
                    end: i32::max(self.x.end, coord.x + 1),
                },
                y: Range {
                    start: i32::min(self.y.start, coord.y),
                    end: i32::max(self.y.end, coord.y + 1),
                },
                z: Range {
                    start: i32::min(self.z.start, coord.z),
                    end: i32::max(self.z.end, coord.z + 1),
                },
                w: Range {
                    start: i32::min(self.w.start, coord.w),
                    end: i32::max(self.w.end, coord.w + 1),
                },
            }
        }
    }
    fn extend(&self, amount: i32, dimensions: &Dimensions) -> Extent {
        // extend in x,y,z and optionally w direction
        let w = match dimensions {
            Dimensions::Three => self.w.clone(),
            Dimensions::Four => Range {
                start: self.w.start - amount,
                end: self.w.end + amount,
            },
        };
        Extent {
            x: Range {
                start: self.x.start - amount,
                end: self.x.end + amount,
            },
            y: Range {
                start: self.y.start - amount,
                end: self.y.end + amount,
            },
            z: Range {
                start: self.z.start - amount,
                end: self.z.end + amount,
            },
            w,
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    active_cells: Vec<Coordinate>,
    dimensions: Dimensions,
}

impl State {
    fn new(lines: &Vec<String>) -> Self {
        let active_cells = lines
            .iter()
            .enumerate()
            .flat_map(move |(y, line)| line.match_indices("#").map(move |(x, _)| (x, y)))
            .map(|(x, y)| Coordinate::new(x as i32, y as i32, 0, 0))
            .collect();
        Self {
            active_cells,
            dimensions: Dimensions::Three,
        }
    }
    fn extent(&self) -> Extent {
        self.active_cells
            .iter()
            .fold(Extent::new_empty(), |extent, cell| {
                extent.include_coord(&cell)
            })
    }
    fn number_of_active_cells(&self) -> usize {
        self.active_cells.len()
    }
    fn run_simulation(&mut self, number_of_cycles: u32, dimensions: &Dimensions, debug: bool) {
        self.dimensions = dimensions.clone();
        if debug {
            println!("Before any cycles:\n\n{}", self);
        }
        for _cycle in 0..number_of_cycles {
            self.simulation_step();
            if debug {
                println!("After {} cycle(s):\n\n{}", _cycle + 1, self);
            }
        }
    }
    fn simulation_step(&mut self) {
        let mut next_active_cells = Vec::new();
        let possible_extent = self.extent().extend(1, &self.dimensions);
        for w in possible_extent.w {
            for z in possible_extent.z.clone() {
                for y in possible_extent.y.clone() {
                    for x in possible_extent.x.clone() {
                        let coord = Coordinate::new(x, y, z, w);
                        let num_active_neighbors = self.number_of_active_neighbors(&coord);
                        let next_active = if self.is_active(&coord) {
                            num_active_neighbors >= 2 && num_active_neighbors <= 3
                        } else {
                            num_active_neighbors == 3
                        };
                        if next_active {
                            next_active_cells.push(coord);
                        }
                    }
                }
            }
        }
        self.active_cells = next_active_cells;
    }
    fn is_active(&self, coord: &Coordinate) -> bool {
        self.active_cells.iter().position(|c| c == coord).is_some()
    }
    fn number_of_active_neighbors(&self, coord: &Coordinate) -> u8 {
        let neighbor_extent = coord.neighbor_extent();
        let mut result = 0;
        for w in neighbor_extent.w {
            for z in neighbor_extent.z.clone() {
                for y in neighbor_extent.y.clone() {
                    for x in neighbor_extent.x.clone() {
                        let neighbor = Coordinate::new(x, y, z, w);
                        if neighbor != *coord && self.is_active(&neighbor) {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let extent = self.extent();
        Ok(for w in extent.w {
            for z in extent.z.clone() {
                write!(f, "z={}, w={}\n", z, w)?;
                for y in extent.y.clone() {
                    for x in extent.x.clone() {
                        if self.is_active(&Coordinate::new(x, y, z, w)) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    write!(f, "\n")?;
                }
                write!(f, "\n")?;
            }
        })
    }
}

fn load_state_from_file(filename: &str) -> Result<State, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(State::new(&lines.filter_map(Result::ok).collect()))
}

fn run_simulation(
    initial_state: &State,
    number_of_cycles: u32,
    dimensions: &Dimensions,
    debug: bool,
) -> usize {
    let mut state = initial_state.clone();
    state.run_simulation(number_of_cycles, dimensions, debug);
    state.number_of_active_cells()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let initial_state = load_state_from_file("inputs/day-17.txt")?;
    let number_of_cycles = 6;
    let debug = false;
    println!(
        "Part 1: Number of active cells after {} cycle(s): {}",
        number_of_cycles,
        run_simulation(&initial_state, number_of_cycles, &Dimensions::Three, debug),
    );
    println!(
        "Part 2: Number of active cells after {} cycle(s): {}",
        number_of_cycles,
        run_simulation(&initial_state, number_of_cycles, &Dimensions::Four, debug),
    );
    Ok(())
}
