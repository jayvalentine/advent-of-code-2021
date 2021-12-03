// Helper methods for reading puzzle data.

use std::str::FromStr;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn get<T: FromStr>(file: &str) -> Vec<T> {
    let mut input: Vec<T> = Vec::new();

    // Read test data in, iterate over each line.
    let f = File::open(file).expect(&format!("Could not open {}", file));
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect(&format!("Invalid line in {}", file));

        let n = match T::from_str(line.trim()) {
            Ok(v) => v,
            Err(_) => panic!(format!("Parse error in {}", file))
        };

        input.push(n);
    }

    return input;
}

pub fn get_bin(file: &str) -> Vec<u32> {
    let mut input: Vec<u32> = Vec::new();

    // Read test data in, iterate over each line.
    let f = File::open(file).expect(&format!("Could not open {}", file));
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect(&format!("Invalid line in {}", file));

        let n = match u32::from_str_radix(line.trim(), 2) {
            Ok(v) => v,
            Err(_) => panic!(format!("Parse error in {}", file))
        };

        input.push(n);
    }

    return input;
}
