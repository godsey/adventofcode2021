
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Advent of Code 2021 Day 1 :: 2

fn main() {
    // file_name.txt, sliding window size
    let result_part_2 = scanner_increases("input/input.txt", 3);
    println!("Result: {:#?}", result_part_2);
}


fn scanner_increases(filename: &str, window_size: usize) -> i32 {

    // sliding window of last 3 inputs
    let mut sliding_window: Vec<i32> = vec![];
    // sliding window of sliding window sums
    let mut window_sums: Vec<i32> = vec![];
    let mut descending_counter: i32 = 0;
    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth_str) = line {
                // line has been read
                if let Ok(depth) = depth_str.trim().parse() {
                    // build input sliding window
                    sliding_window.push(depth);
                    if sliding_window.len() >= window_size {
                        // if sliding window meets size requirments, sum up and append to window_sums
                        window_sums.push(sliding_window.iter().copied().sum());
                        // remove first element for next pass
                        sliding_window.drain(0..1);
                        if window_sums.len() > 1 {
                            // if sliding window of sums is > 1 check for increasing depth, bump counter
                            if window_sums[0] < window_sums[1] {
                                descending_counter+=1;
                            }
                            // remove first entry for next pass
                            window_sums.drain(0..1);
                        }
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

