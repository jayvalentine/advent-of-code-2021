// Helper methods for reading puzzle data.

use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn get(file: &str) -> Vec<u32> {
    let mut input: Vec<u32> = Vec::new();

    // Read test data in, iterate over each line.
    let f = File::open(file).expect(&format!("Could not open {}", file));
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect("Invalid line in data/day1.txt");

        let n: u32 = line.trim().parse().expect("Non-number in data/day1.txt");
        input.push(n);
    }

    return input;
}
