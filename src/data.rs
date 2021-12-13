// Helper methods for reading puzzle data.

use std::str::FromStr;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use core::slice::Iter;

pub fn get<T: FromStr>(file: &str) -> Vec<T> {
    let mut input: Vec<T> = Vec::new();

    // Read test data in, iterate over each line.
    let f = File::open(file).unwrap_or_else(|_| panic!("Could not open {}", file));
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| panic!("Invalid line in {}", file));

        if let Ok(v) = T::from_str(line.trim()) {
            input.push(v);
        }
    }

    input
}

// Returns a vector of the objects constructed from the given function for each line of a file.
pub fn get_with<T: FromStr>(file: &str, func: &dyn Fn(&str) -> Result<T, T::Err>) -> Result<Vec<T>, T::Err> {
    let mut input: Vec<T> = Vec::new();

    // Read test data in, iterate over each line.
    let f = File::open(file).unwrap_or_else(|_| panic!("Could not open {}", file));
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| panic!("Invalid line in {}", file));

        let n = func(line.trim())?;

        input.push(n);
    }

    Ok(input)
}

pub fn get_with_iter<T>(file: &str, f_iter: &mut dyn Fn(&mut Iter<&str>) -> T) -> T {
    let mut input = Vec::new();

    // Read test data in, iterate over each line.
    let f = File::open(file).unwrap_or_else(|_| panic!("Could not open {}", file));
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| panic!("Invalid line in {}", file));

        input.push(line);
    }

    let input: Vec<&str> = input.iter().map(|s| s as &str).collect();

    return f_iter(&mut input.iter());
}

pub fn from_separated<T: FromStr>(s: &str, sep: char) -> Result<Vec<T>, T::Err> {
    let mut v = Vec::new();
    for i in s.split(sep) {
        v.push(T::from_str(i)?)
    }

    Ok(v)
}
