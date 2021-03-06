
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Advent of Code 2021 Day 1 :: 1

fn main() {
    let result_part_1 = scanner_increases("input/input.txt");
    println!("Result: {}", result_part_1);
}


fn scanner_increases(filename: &str) -> i32 {
    let mut previous_depth: Option<i32> = None;
    let mut descending_counter: i32 = 0;

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth_str) = line {
                // line has been read
                if let Ok(depth) = depth_str.trim().parse() {
                    // line has been parsed to i32
                    match previous_depth {
                        None => {
                            previous_depth = Some(depth);
                        },
                        Some(n) => {
                            if depth > n {
                                descending_counter += 1;
                            }
                            previous_depth = Some(depth);
                        },
                    }
                }
            }
        }
    }
    descending_counter
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

