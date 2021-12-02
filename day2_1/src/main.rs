
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Advent of Code 2021 Day 2 :: 1

fn main() {
    // file_name.txt
    let result_part_1 = parse_input("input/input.txt");
    println!("Result: {:#?}", result_part_1);
}

fn parse_input(filename: &str) -> i32 {

    let mut horizontal_position: i32 = 0;
    let mut depth: i32 = 0;

    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(input) = line {
                // line has been read
                let instruction: Vec<&str> = input.split_whitespace().collect();
                if let Ok(distance) = instruction[1].trim().parse::<i32>() {
                    // distance has been parsed to i32
                    match instruction[0] {
                        "forward" => { horizontal_position += distance; },
                        "backward" => { horizontal_position -= distance; },
                        "up" => { depth -= distance; },
                        "down" => { depth += distance; },
                        _ => println!("Not matched"),
                    }
                }

            }
        }
    }
    horizontal_position * depth
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

