// Advent of Code
// Day 7

use aoc;
use aoc::data;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(352997, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(101571302, super::part2());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let (pos, fuel) = min_fuel_position_linear(&crabs);
        assert_eq!(2, pos);
        assert_eq!(37, fuel);
    }

    #[test]
    fn part2() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let (pos, fuel) = min_fuel_position_increasing(&crabs);
        assert_eq!(5, pos);
        assert_eq!(168, fuel);
    }
}

fn max_pos(crabs: &[i32]) -> i32 {
    let mut pos = 0;
    for &c in crabs {
        if c > pos {
            pos = c;
        }
    }

    return pos;
}

fn min_fuel_position_linear(crabs: &[i32]) -> (i32, i32) {
    return min_fuel_position(crabs, &|dist| dist);
}

fn min_fuel_position_increasing(crabs: &[i32]) -> (i32, i32) {
    return min_fuel_position(crabs, &|dist| ((dist) * (dist+1)) / 2);
}

fn min_fuel_position(crabs: &[i32], f_pos: &dyn Fn(i32) -> i32) -> (i32, i32) {
    let max = max_pos(crabs);

    // Iterate over each position and find the minimum.
    let mut min = i32::MAX;
    let mut min_pos = 0;

    for p in 0..(max+1) {
        let mut sum = 0;
        for &c in crabs {
            sum += f_pos((c - p).abs());
        }

        if sum < min {
            min = sum;
            min_pos = p;
        }
    }

    return (min_pos, min);
}

fn get_data() -> Vec<i32> {
    let v: Vec<i32> = data::get_with_iter("data/day7.txt", &mut |iter|
        data::from_separated(iter.next().expect("Data parse error!"), ',').expect("Data parse error!")
    );

    return v;
}

fn part1() -> i32 {
    let crabs = get_data();
    let (_position, fuel) = min_fuel_position_linear(&crabs);
    return fuel;
}

fn part2() -> i32 {
    let crabs = get_data();
    let (_position, fuel) = min_fuel_position_increasing(&crabs);
    return fuel;
}

fn main() {
    aoc::solution!(7, "linear", "triangular");
}
