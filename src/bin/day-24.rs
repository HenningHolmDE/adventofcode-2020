use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

//        north                    east
// / 2,-2 \____/ 1, 1 \____/ 0, 4
// \      /    \      /    \
//  \____/ 1,-1 \____/ 0, 2 \____/
//  /    \      /    \      /    \
// / 1,-3 \____/ 0, 0 \____/-1, 3
// \      /    \ ref  /    \
//  \____/ 0,-2 \____/-1, 1 \____/
//  /    \      /    \      /    \
// / 0,-4 \____/-1, 1 \____/-2, 2
// west              south

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Tile {
    position: (i32, i32),
}

impl Tile {
    fn new(directions: String) -> Self {
        let mut position = (0, 0);
        let mut iter = directions.chars();
        while let Some(c) = iter.next() {
            position = match c {
                'e' => (position.0, position.1 + 2),
                'w' => (position.0, position.1 - 2),
                'n' => match iter.next().expect("expected character after 'n'") {
                    'e' => (position.0 + 1, position.1 + 1),
                    'w' => (position.0 + 1, position.1 - 1),
                    _ => panic!("expected 'e' or 'w' after 'n'"),
                },
                's' => match iter.next().expect("expected character after 's'") {
                    'e' => (position.0 - 1, position.1 + 1),
                    'w' => (position.0 - 1, position.1 - 1),
                    _ => panic!("expected 'e' or 'w' after 's'"),
                },
                _ => panic!("unexpected character"),
            }
        }
        Self { position }
    }

    fn neighbors(&self) -> Vec<Tile> {
        vec![
            Self {
                position: (self.position.0 + 1, self.position.1 - 1), // north west
            },
            Self {
                position: (self.position.0 + 1, self.position.1 + 1), // north east
            },
            Self {
                position: (self.position.0, self.position.1 + 2), // east
            },
            Self {
                position: (self.position.0 - 1, self.position.1 + 1), // south east
            },
            Self {
                position: (self.position.0 - 1, self.position.1 - 1), // south west
            },
            Self {
                position: (self.position.0, self.position.1 - 2), // west
            },
        ]
    }

    fn flipped_neighbors(&self, flipped_tiles: &HashSet<Tile>) -> usize {
        self.neighbors()
            .into_iter()
            .filter(|neighbor| flipped_tiles.contains(&neighbor))
            .count()
    }
}

fn load_tiles_from_file(filename: &str) -> Result<Vec<Tile>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.filter_map(Result::ok).map(Tile::new).collect())
}

fn part1(tiles: Vec<Tile>) -> HashSet<Tile> {
    let mut flipped_tiles = HashSet::new();
    for tile in tiles.into_iter() {
        if flipped_tiles.get(&tile).is_some() {
            // tile is black
            flipped_tiles.remove(&tile);
        } else {
            // tile is white
            flipped_tiles.insert(tile);
        }
    }
    println!("Part 1: Number of flipped tiles: {}", flipped_tiles.len());
    flipped_tiles
}

fn part2(mut flipped_tiles: HashSet<Tile>) {
    let number_of_days = 100;
    for _day in 1..number_of_days + 1 {
        let mut flip_candidates = flipped_tiles.clone();
        flip_candidates.extend(flipped_tiles.iter().flat_map(|tile| tile.neighbors()));

        let mut flipped_tiles_new = flipped_tiles.clone();
        for candidate in flip_candidates.into_iter() {
            let flipped_neighbors = candidate.flipped_neighbors(&flipped_tiles);
            if flipped_tiles.contains(&candidate) {
                // tile is black
                if flipped_neighbors == 0 || flipped_neighbors > 2 {
                    flipped_tiles_new.remove(&candidate);
                }
            } else {
                // tile is white
                if flipped_neighbors == 2 {
                    flipped_tiles_new.insert(candidate);
                }
            }
        }
        flipped_tiles = flipped_tiles_new;
        // println!("Day {}: {}", _day, flipped_tiles.len());
    }
    println!(
        "Part 2: Number of flipped tiles after {} days: {}",
        number_of_days,
        flipped_tiles.len()
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tiles = load_tiles_from_file("inputs/day-24.txt")?;
    let flipped_tiles = part1(tiles);
    part2(flipped_tiles);
    Ok(())
}
