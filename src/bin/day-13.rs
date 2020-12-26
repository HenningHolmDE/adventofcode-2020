use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn load_lines_from_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.filter_map(Result::ok).collect())
}

fn part1(lines: &Vec<String>) {
    let timestamp: u32 = lines[0].parse().unwrap();
    let ids: Vec<u32> = lines[1]
        .split(',')
        .filter_map(|id| id.parse().ok())
        .collect();
    let (id, wait_time) = {
        let mut minimum_id = None;
        let mut minimum_wait_time = None;
        for id in ids {
            let wait_time = id - (timestamp % id);
            if let Some(old_minimum_wait_time) = minimum_wait_time {
                if wait_time < old_minimum_wait_time {
                    minimum_wait_time = Some(wait_time);
                    minimum_id = Some(id);
                }
            } else {
                minimum_wait_time = Some(wait_time);
                minimum_id = Some(id);
            }
        }
        (minimum_id.unwrap(), minimum_wait_time.unwrap())
    };
    println!(
        "Part 1: Bus ID {} departs after waiting {} minutes, product: {}",
        id,
        wait_time,
        id * wait_time
    );
}

#[derive(Debug)]
struct IndexedId {
    index: usize,
    id: u32,
}

fn find_position_timestamp(time: u64, lcm: u64, remaining_ids: &[IndexedId]) -> u64 {
    if remaining_ids.len() == 0 {
        return time;
    }
    // Valid timestamps for the previous buses repeat with the LCM of all previous IDs
    // Advance time until a valid timestamp for the current bus is found and extend LCM.
    let mut time = time;
    while (time + remaining_ids[0].index as u64) % remaining_ids[0].id as u64 != 0 {
        time += lcm;
    }
    let lcm = num_integer::lcm(lcm, remaining_ids[0].id as u64);
    return find_position_timestamp(time, lcm, &remaining_ids[1..]);
}

fn part2(lines: &Vec<String>) {
    let indexed_ids: Vec<IndexedId> = lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(index, id)| match id.parse() {
            Ok(id) => Some(IndexedId { index, id }),
            _ => None,
        })
        .collect();
    let timestamp = find_position_timestamp(0, 1, &indexed_ids);
    println!("Part 2: Timestamp with offset departure: {}", timestamp);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_lines_from_file("inputs/day-13.txt")?;
    part1(&lines);
    part2(&lines);
    Ok(())
}
