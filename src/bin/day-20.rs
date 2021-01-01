use std::fs::File;
use std::io::{self, BufRead, BufReader};

const TOP: usize = 0;
const LEFT: usize = 1;
const BOTTOM: usize = 2;
const RIGHT: usize = 3;
const MIRRORED: usize = 4;

const SEA_MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

#[derive(Clone, Debug)]
struct Image {
    image: Vec<String>,
}

impl Image {
    fn new(image: Vec<String>) -> Self {
        Self { image }
    }

    fn from_grid(grid: &Vec<Vec<Tile>>) -> Self {
        grid.iter().fold(Image::new(Vec::new()), |mut acc, row| {
            acc.append_bottom(&row.iter().fold(Image::new(Vec::new()), |mut acc, tile| {
                acc.append_right(&tile.image);
                acc
            }));
            acc
        })
    }

    fn height(&self) -> usize {
        self.image.len()
    }

    fn width(&self) -> usize {
        if self.image.len() > 0 {
            self.image[0].len()
        } else {
            0
        }
    }

    fn rotate90(&mut self) {
        self.image = (0..self.width())
            .rev()
            .map(|col| {
                self.image
                    .iter()
                    .map(|row| row.chars().nth(col).unwrap())
                    .collect::<String>()
            })
            .collect();
    }

    fn mirror_vert(&mut self) {
        self.image = self.image.iter().rev().cloned().collect();
    }

    fn mirror_hor(&mut self) {
        self.image = self
            .image
            .iter()
            .map(|row| row.chars().rev().collect())
            .collect();
    }

    fn append_right(&mut self, other: &Self) {
        for _ in 0..other.image.len() - self.image.len() {
            self.image.push(String::new())
        }
        for (i, row) in other.image.iter().enumerate() {
            self.image[i].push_str(row);
        }
    }

    fn append_bottom(&mut self, other: &Self) {
        for row in other.image.iter() {
            self.image.push(row.clone());
        }
    }

    fn set(&mut self, x: usize, y: usize, new_c: char) {
        self.image[y] = self.image[y]
            .chars()
            .enumerate()
            .map(|(i, c)| if i == x { new_c } else { c })
            .collect();
    }

    fn get(&mut self, x: usize, y: usize) -> char {
        self.image[y].chars().nth(x).unwrap()
    }

    fn count(&mut self, needle: char) -> usize {
        self.image
            .iter()
            .map(|row| row.matches(&needle.to_string()).count())
            .sum()
    }
}

#[derive(Clone, Debug)]
struct Tile {
    id: u32,
    edges: Vec<u32>,
    image: Image,
}

impl Tile {
    fn new(lines: &Vec<String>) -> Self {
        let id = lines[0][5..lines[0].len() - 1].parse().unwrap();
        let edges = vec![
            lines[1].clone(), // top
            lines[1..lines.len()]
                .iter()
                .map(|line| line.chars().nth(0).unwrap())
                .collect(), // left
            lines[lines.len() - 1].clone(), // bottom
            lines[1..lines.len()]
                .iter()
                .map(|line| line.chars().nth(line.len() - 1).unwrap())
                .collect(), // right
            lines[1].chars().rev().collect(), // top mirrored
            lines[1..lines.len()]
                .iter()
                .rev()
                .map(|line| line.chars().nth(0).unwrap())
                .collect(), // left mirrored
            lines[lines.len() - 1].chars().rev().collect(), // bottom mirrored
            lines[1..lines.len()]
                .iter()
                .rev()
                .map(|line| line.chars().nth(line.len() - 1).unwrap())
                .collect(), // right mirrored
        ];
        // replace edge strings by binary integer representations
        let edges = edges
            .iter()
            .map(|e| e.replace(".", "0").replace("#", "1"))
            .map(|e| u32::from_str_radix(&e, 2).unwrap())
            .collect();
        let image = Image::new(
            lines[2..lines.len() - 1]
                .iter()
                .map(|line| line[1..line.len() - 1].to_owned())
                .collect(),
        );
        Self { id, edges, image }
    }

    fn align_top_left(&mut self, edges: &Vec<u32>) {
        // as first tile will always fit without mirroring, only rotation has to be aligned
        let top = edges.contains(&self.edges[TOP]);
        let left = edges.contains(&self.edges[LEFT]);
        let bottom = edges.contains(&self.edges[BOTTOM]);
        let right = edges.contains(&self.edges[RIGHT]);
        match (top, left, bottom, right) {
            (true, true, false, false) => (),
            (true, false, false, true) => self.rotate90(1),
            (false, false, true, true) => self.rotate90(2),
            (false, true, true, false) => self.rotate90(3),
            _ => unreachable!(),
        }
    }

    fn align_left(&mut self, edge: u32) {
        let edge_index = self.edges.iter().position(|&id| id == edge).unwrap();
        match edge_index % 4 {
            TOP => self.rotate90(1),
            LEFT => (),
            BOTTOM => self.rotate90(3),
            RIGHT => self.rotate90(2),
            _ => unreachable!(),
        };
        if self.edges[LEFT] != edge {
            self.mirror_vert();
        }
    }

    fn align_top(&mut self, edge: u32) {
        let edge_index = self.edges.iter().position(|&id| id == edge).unwrap();
        match edge_index % MIRRORED {
            TOP => (),
            LEFT => self.rotate90(3),
            BOTTOM => self.rotate90(2),
            RIGHT => self.rotate90(1),
            _ => unreachable!(),
        };
        if self.edges[TOP] != edge {
            self.mirror_hor();
        }
    }

    fn rotate90(&mut self, amount: u32) {
        for _ in 0..amount {
            self.edges = vec![
                self.edges[RIGHT],             // top
                self.edges[MIRRORED + TOP],    // left
                self.edges[LEFT],              // bottom
                self.edges[MIRRORED + BOTTOM], // right
                self.edges[MIRRORED + RIGHT],  // top (mirrored)
                self.edges[TOP],               // left (mirrored)
                self.edges[MIRRORED + LEFT],   // bottom (mirrored)
                self.edges[BOTTOM],            // right (mirrored)
            ];
            self.image.rotate90();
        }
    }

    fn mirror_vert(&mut self) {
        self.edges = vec![
            self.edges[BOTTOM],            // top
            self.edges[MIRRORED + LEFT],   // left
            self.edges[TOP],               // bottom
            self.edges[MIRRORED + RIGHT],  // right
            self.edges[MIRRORED + BOTTOM], // top (mirrored)
            self.edges[LEFT],              // left (mirrored)
            self.edges[MIRRORED + TOP],    // bottom (mirrored)
            self.edges[RIGHT],             // right (mirrored)
        ];
        self.image.mirror_vert();
    }

    fn mirror_hor(&mut self) {
        self.edges = vec![
            self.edges[MIRRORED + TOP],    // top
            self.edges[RIGHT],             // left
            self.edges[MIRRORED + BOTTOM], // bottom
            self.edges[LEFT],              // right
            self.edges[TOP],               // top (mirrored)
            self.edges[MIRRORED + RIGHT],  // left (mirrored)
            self.edges[BOTTOM],            // bottom (mirrored)
            self.edges[MIRRORED + LEFT],   // right (mirrored)
        ];
        self.image.mirror_hor();
    }

    fn right_edge(&self) -> u32 {
        self.edges[RIGHT]
    }
    fn bottom_edge(&self) -> u32 {
        self.edges[BOTTOM]
    }
}

fn load_tiles_from_file(filename: &str) -> Result<Vec<Tile>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut tiles = Vec::new();
    let mut tile_lines = Vec::new();
    for line in lines
        .filter_map(Result::ok)
        .map(|line| line.trim().to_owned())
    {
        if line.is_empty() {
            tiles.push(Tile::new(&tile_lines));
            tile_lines.clear();
        } else {
            tile_lines.push(line);
        }
    }
    if !tile_lines.is_empty() {
        tiles.push(Tile::new(&tile_lines));
    }
    Ok(tiles)
}

fn find_top_left_tile(tiles: &mut Vec<Tile>) -> Tile {
    let all_edge_ids: Vec<_> = tiles.iter().flat_map(|t| t.edges.iter().cloned()).collect();
    let mut all_edge_ids = all_edge_ids;
    all_edge_ids.sort();
    let mut tile_index = 0;
    let mut unique_edges = Vec::new();
    for (index, tile) in tiles.iter().enumerate() {
        unique_edges = tile
            .edges
            .iter()
            .map(|edge_id| {
                (
                    *edge_id,
                    all_edge_ids.iter().filter(|id| *id == edge_id).count(),
                )
            })
            .filter(|(_, c)| *c == 1)
            .map(|(id, _)| id)
            .collect();
        if unique_edges.len() == 4 {
            // 4: two unique edges with their mirrored versions
            tile_index = index;
            break;
        }
    }
    let mut tile = tiles.remove(tile_index);
    tile.align_top_left(&unique_edges);
    tile
}

fn try_find_right_tile(tiles: &mut Vec<Tile>, grid: &Vec<Vec<Tile>>) -> Option<Tile> {
    let left_edge = grid.iter().last()?.iter().last()?.right_edge();
    let tile_index = tiles
        .iter()
        .enumerate()
        .find(|(_, tile)| tile.edges.contains(&left_edge))
        .map(|(index, _)| index);
    let tile = tile_index.map(|index| tiles.remove(index));
    match tile {
        Some(mut tile) => {
            tile.align_left(left_edge);
            Some(tile)
        }
        None => None,
    }
}

fn find_bottom_tile(tiles: &mut Vec<Tile>, grid: &Vec<Vec<Tile>>) -> Tile {
    let top_edge = grid
        .iter()
        .last()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .bottom_edge();
    let tile_index = tiles
        .iter()
        .enumerate()
        .find(|(_, tile)| tile.edges.contains(&top_edge))
        .map(|(index, _)| index);
    let tile_index = tile_index.unwrap();
    let mut tile = tiles.remove(tile_index);
    tile.align_top(top_edge);
    tile
}

fn replace_sea_monsters(image: &mut Image) -> u32 {
    let sea_monster_iter = SEA_MONSTER.iter().enumerate().flat_map(|(y, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(x, _)| (x, y))
    });
    let mut number_of_sea_monsters = 0;
    for y_offset in 0..image.height() - SEA_MONSTER.len() + 1 {
        for x_offset in 0..image.width() - SEA_MONSTER[0].len() + 1 {
            let sea_monster_offset_iter = sea_monster_iter
                .clone()
                .map(|(x, y)| (x + x_offset, y + y_offset));
            if sea_monster_offset_iter
                .clone()
                .all(|(x, y)| image.get(x, y) != '.')
            {
                sea_monster_offset_iter.for_each(|(x, y)| image.set(x, y, 'O'));
                number_of_sea_monsters += 1;
            }
        }
    }
    number_of_sea_monsters
}

fn find_sea_monsters(image: &mut Image) {
    for _ in 0..2 {
        for _ in 0..4 {
            let number_of_sea_monsters = replace_sea_monsters(image);
            if number_of_sea_monsters > 0 {
                println!("Found {} sea monster(s)!", number_of_sea_monsters);
                return;
            }
            image.rotate90();
        }
        image.mirror_hor();
    }
}

fn display_grid(grid: &Vec<Vec<Tile>>) {
    println!("Grid:");
    for row in grid.iter() {
        for tile in row.iter() {
            print!(
                "    {:>3} {:>3}    ",
                tile.edges[TOP],
                tile.edges[MIRRORED + TOP]
            );
        }
        print!("\n");
        for tile in row.iter() {
            print!(
                " {:>3} [{}] {:>3}",
                tile.edges[LEFT], tile.id, tile.edges[RIGHT]
            );
        }
        print!("\n");
        for tile in row.iter() {
            print!(
                " {:>3}        {:>3}",
                tile.edges[MIRRORED + LEFT],
                tile.edges[MIRRORED + RIGHT]
            );
        }
        print!("\n");
        for tile in row.iter() {
            print!(
                "    {:>3} {:>3}    ",
                tile.edges[BOTTOM],
                tile.edges[MIRRORED + BOTTOM]
            );
        }
        print!("\n");
    }
}

fn display_image(image: &Image) {
    println!("Image:");
    for line in &image.image {
        println!("{}", line);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tiles = load_tiles_from_file("inputs/day-20.txt")?;

    // create grid from tiles
    let mut remaining_tiles = tiles.clone();
    let mut grid = Vec::new();
    while !remaining_tiles.is_empty() {
        if grid.is_empty() {
            grid.push(vec![find_top_left_tile(&mut remaining_tiles)]);
        } else {
            grid.push(vec![find_bottom_tile(&mut remaining_tiles, &grid)]);
        }
        while let Some(tile) = try_find_right_tile(&mut remaining_tiles, &grid) {
            grid.iter_mut().last().unwrap().push(tile);
        }
    }
    display_grid(&grid);

    // part 1
    let corner_product = grid[0][0].id as u64
        * grid[0][grid[0].len() - 1].id as u64
        * grid[grid.len() - 1][0].id as u64
        * grid[grid.len() - 1][grid[grid.len() - 1].len() - 1].id as u64;
    println!("Part 1: Product of corner tile IDs: {}", corner_product);

    // part 2
    let mut image = Image::from_grid(&grid);
    find_sea_monsters(&mut image);
    display_image(&image);
    println!("Part 2: Habitat's water roughness: {}", image.count('#'));

    Ok(())
}
