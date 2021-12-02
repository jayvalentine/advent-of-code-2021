// Advent of Code 2021
// Day 1

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(1162, super::part1());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];

        assert_eq!(7, num_increases(&input));
    }

    #[test]
    fn part2() {
        let input = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];

        assert_eq!(5, num_increases_sliding_window(&input));
    }
}

#[cfg(test)]
mod test_num_increases {
    use super::*;

    #[test]
    fn all_increases() {
        let input = vec![
            1,
            2,
            3,
            4,
            5
        ];

        assert_eq!(4, num_increases(&input));
    }

    #[test]
    fn all_same() {
        let input = vec![
            101,
            101,
            101,
            101,
            101
        ];

        assert_eq!(0, num_increases(&input));
    }

    #[test]
    fn all_decreases() {
        let input = vec![
            101,
            100,
            98,
            90,
            40
        ];

        assert_eq!(0, num_increases(&input));
    }

    #[test]
    fn up_and_down() {
        let input = vec![
            101,
            105,
            100,
            101,
            101,
            99,
            100
        ];

        assert_eq!(3, num_increases(&input));
    }
}

#[cfg(test)]
mod test_num_increases_sliding_window {
    use super::*;

    #[test]
    fn all_increases() {
        let input = vec![
            1,
            2,
            3, // 6
            4, // 9
            5  // 12
        ];

        assert_eq!(2, num_increases_sliding_window(&input));
    }

    #[test]
    fn all_same() {
        let input = vec![
            101,
            101,
            101,
            101,
            101
        ];

        assert_eq!(0, num_increases_sliding_window(&input));
    }

    #[test]
    fn all_decreases() {
        let input = vec![
            101,
            100,
            98,
            90,
            40
        ];

        assert_eq!(0, num_increases_sliding_window(&input));
    }

    #[test]
    fn up_and_down() {
        let input = vec![
            100,
            100,
            100, // 300
            50,  // 250
            200, // 350
            10,  // 260
            100  // 310
        ];

        assert_eq!(2, num_increases_sliding_window(&input));
    }
}

fn num_increases(report: &[u32]) -> u32 {
    let mut previous = u32::MAX;
    let mut increases = 0;

    for depth in report {
        if depth > &previous { increases = increases + 1; }
        previous = *depth;
    }

    return increases;
}

fn num_increases_sliding_window(report: &[u32]) -> u32 {
    let mut sums: Vec<u32> = Vec::new();

    for i in 2..report.len() {
        let range = &report[i-2..i+1];
        let sum = range.iter().fold(0, |acc, x| acc + x);
        sums.push(sum);
    }

    return num_increases(&sums);
}

fn part1() -> u32 {
    let mut input: Vec<u32> = Vec::new();

    // Read test data in, iterate over each line.
    let f = File::open("data/day1_part1.txt").expect("Could not open data/day1_part1.txt");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect("Invalid line in data/day1_part1.txt");

        let n: u32 = line.trim().parse().expect("Non-number in data/day1_part1.txt");
        input.push(n);
    }

    return num_increases(&input);
}

fn main() {
    println!("Part 1: The number of depth increases is {}", part1());
}