use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct MapType {
    width: usize,
    rows: Vec<String>,
}

impl MapType {
    fn new(rows: Vec<String>) -> Self {
        let width = rows[0].len();
        Self { width, rows }
    }
    fn height(&self) -> usize {
        self.rows.len()
    }
    fn is_tree_at(&self, y: usize, x: usize) -> bool {
        let x = x % self.width;
        self.rows[y].chars().nth(x) == Some('#')
    }
    fn trees_for_slope(&self, slope: &Slope) -> usize {
        (0..self.height() / slope.down)
            .filter(|i| self.is_tree_at(i * slope.down, i * slope.right))
            .count()
    }
}

fn load_map_from_file(filename: &str) -> Result<MapType, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(MapType::new(lines.filter_map(Result::ok).collect()))
}

#[derive(Debug)]
struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        Self { right, down }
    }
}

impl std::fmt::Display for Slope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Right {}, down {}.", self.right, self.down)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = load_map_from_file("inputs/day-03.txt")?;
    let slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];
    let mut cumulative_product = 1;
    for slope in slopes {
        let trees_for_slope = map.trees_for_slope(&slope);
        cumulative_product *= trees_for_slope;
        println!(
            "Number of trees in path of slope \"{}\": {}",
            slope, trees_for_slope
        );
    }
    println!("Cumulative product of trees: {}", cumulative_product);
    Ok(())
}
