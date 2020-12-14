use std::fs::File;
use std::io::{self, BufRead, BufReader};

const CHAR_EMPTY: char = 'L';
const CHAR_OCCUPIED: char = '#';

#[derive(Debug, Clone, PartialEq)]
enum GridMode {
    Adjacency,
    Visibility,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}
use Direction::*;

#[derive(Debug, Clone)]
struct Grid {
    mode: GridMode,
    width: usize,
    rows: Vec<String>,
}

impl Grid {
    fn new(rows: Vec<String>) -> Self {
        let width = rows[0].len();
        Self {
            mode: GridMode::Adjacency,
            width,
            rows,
        }
    }

    fn set_mode(&mut self, mode: GridMode) {
        self.mode = mode;
    }

    fn next_round(&mut self) -> bool {
        let occupied_threshold = match self.mode {
            GridMode::Adjacency => 4,
            GridMode::Visibility => 5,
        };
        let mut new_rows = Vec::with_capacity(self.rows.len());
        for y in 0..self.rows.len() {
            let mut new_row = String::with_capacity(self.width);
            for x in 0..self.width {
                if self.is_empty(y, x) && self.number_of_occupied_adjacent(y, x) == 0 {
                    new_row.push(CHAR_OCCUPIED);
                } else if self.is_occupied(y, x)
                    && self.number_of_occupied_adjacent(y, x) >= occupied_threshold
                {
                    new_row.push(CHAR_EMPTY);
                } else {
                    new_row.push(self.get_char(y, x));
                }
            }
            new_rows.push(new_row);
        }
        let result = new_rows != self.rows;
        self.rows = new_rows;
        result
    }

    fn occupied_seats(&self) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row.chars())
            .filter(|c| *c == CHAR_OCCUPIED)
            .count()
    }

    fn get_char(&self, y: usize, x: usize) -> char {
        self.rows[y].chars().nth(x).unwrap()
    }

    fn number_of_occupied_adjacent(&self, y: usize, x: usize) -> usize {
        static DIRECTIONS: [Direction; 8] = [
            TopLeft,
            Top,
            TopRight,
            Left,
            Right,
            BottomLeft,
            Bottom,
            BottomRight,
        ];
        DIRECTIONS
            .iter()
            .filter(|dir| self.check_occupied_direction(y, x, dir))
            .count()
    }

    fn check_occupied_direction(&self, y: usize, x: usize, direction: &Direction) -> bool {
        let position = self.position_by_direction(y, x, direction);
        if let Some((y, x)) = position {
            match self.mode {
                GridMode::Adjacency => self.is_occupied(y, x),
                GridMode::Visibility => {
                    if self.is_occupied(y, x) {
                        true
                    } else if self.is_empty(y, x) {
                        false
                    } else {
                        self.check_occupied_direction(y, x, direction)
                    }
                }
            }
        } else {
            false
        }
    }

    fn position_by_direction(
        &self,
        y: usize,
        x: usize,
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            TopLeft if y == 0 || x == 0 => None,
            TopLeft => Some((y - 1, x - 1)),
            Top if y == 0 => None,
            Top => Some((y - 1, x)),
            TopRight if y == 0 || x == self.width - 1 => None,
            TopRight => Some((y - 1, x + 1)),
            Left if x == 0 => None,
            Left => Some((y, x - 1)),
            Right if x == self.width - 1 => None,
            Right => Some((y, x + 1)),
            BottomLeft if y == self.rows.len() - 1 || x == 0 => None,
            BottomLeft => Some((y + 1, x - 1)),
            Bottom if y == self.rows.len() - 1 => None,
            Bottom => Some((y + 1, x)),
            BottomRight if y == self.rows.len() - 1 || x == self.width - 1 => None,
            BottomRight => Some((y + 1, x + 1)),
        }
    }

    fn is_empty(&self, y: usize, x: usize) -> bool {
        self.get_char(y, x) == CHAR_EMPTY
    }

    fn is_occupied(&self, y: usize, x: usize) -> bool {
        self.get_char(y, x) == CHAR_OCCUPIED
    }
}

fn load_grid_from_file(filename: &str) -> Result<Grid, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(Grid::new(lines.filter_map(Result::ok).collect()))
}

fn run_simulation(grid: &mut Grid) {
    let mut round = 0;
    while grid.next_round() {
        round += 1;
    }
    println!(
        "Number of occupied seats after round {}: {}",
        round,
        grid.occupied_seats()
    );
}

fn part1(grid: &Grid) {
    println!("Part 1:");
    let mut grid = grid.clone();
    grid.set_mode(GridMode::Adjacency);
    run_simulation(&mut grid);
}

fn part2(grid: &Grid) {
    println!("Part 2:");
    let mut grid = grid.clone();
    grid.set_mode(GridMode::Visibility);
    run_simulation(&mut grid);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = load_grid_from_file("day-11/input.txt")?;
    part1(&grid);
    part2(&grid);
    Ok(())
}
