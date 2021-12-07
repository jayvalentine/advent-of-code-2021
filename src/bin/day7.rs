// Advent of Code
// Day 7

use std::time::{Instant};

use aoc::data;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(352997, super::part1());
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

#[cfg(test)]
mod test_positions {
    use super::*;

    #[test]
    fn position_1() {
        let positions = possible_positions(1, 5);
        assert_eq!(6, positions.len());

        assert_eq!(1, positions[0]);
        assert_eq!(0, positions[1]);
        assert_eq!(1, positions[2]);
        assert_eq!(2, positions[3]);
        assert_eq!(3, positions[4]);
        assert_eq!(4, positions[5]);
    }

    #[test]
    fn position_3() {
        let positions = possible_positions(3, 5);
        assert_eq!(6, positions.len());
        
        assert_eq!(3, positions[0]);
        assert_eq!(2, positions[1]);
        assert_eq!(1, positions[2]);
        assert_eq!(0, positions[3]);
        assert_eq!(1, positions[4]);
        assert_eq!(2, positions[5]);
    }

    #[test]
    fn position_5() {
        let positions = possible_positions(5, 5);
        assert_eq!(6, positions.len());
        
        assert_eq!(5, positions[0]);
        assert_eq!(4, positions[1]);
        assert_eq!(3, positions[2]);
        assert_eq!(2, positions[3]);
        assert_eq!(1, positions[4]);
        assert_eq!(0, positions[5]);
    }
}

#[cfg(test)]
mod test_positions_increasing {
    use super::*;

    #[test]
    fn position_1() {
        let positions = possible_positions_increasing(1, 5);
        assert_eq!(6, positions.len());

        assert_eq!(1, positions[0]);
        assert_eq!(0, positions[1]);
        assert_eq!(1, positions[2]);
        assert_eq!(3, positions[3]);
        assert_eq!(6, positions[4]);
        assert_eq!(10, positions[5]);
    }

    #[test]
    fn position_3() {
        let positions = possible_positions_increasing(3, 5);
        assert_eq!(6, positions.len());
        
        assert_eq!(6, positions[0]);
        assert_eq!(3, positions[1]);
        assert_eq!(1, positions[2]);
        assert_eq!(0, positions[3]);
        assert_eq!(1, positions[4]);
        assert_eq!(3, positions[5]);
    }

    #[test]
    fn position_5() {
        let positions = possible_positions_increasing(5, 5);
        assert_eq!(6, positions.len());
        
        assert_eq!(15, positions[0]);
        assert_eq!(10, positions[1]);
        assert_eq!(6, positions[2]);
        assert_eq!(3, positions[3]);
        assert_eq!(1, positions[4]);
        assert_eq!(0, positions[5]);
    }
}

fn possible_positions(pos: i32, max_pos: i32) -> Vec<i32> {
    let mut positions = Vec::new();
    for i in 0..(max_pos+1) {
        positions.push((pos - i).abs());
    }

    return positions;
}

fn possible_positions_increasing(pos: i32, max_pos: i32) -> Vec<i32> {
    let mut positions = Vec::new();
    for i in 0..(max_pos+1) {
        let distance = (pos - i).abs();
        
        // It's a triangular progression :)
        let cost = (distance * (distance + 1)) / 2;
        positions.push(cost);
    }

    return positions;
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
    return min_fuel_position(crabs, &possible_positions);
}

fn min_fuel_position_increasing(crabs: &[i32]) -> (i32, i32) {
    return min_fuel_position(crabs, &possible_positions_increasing);
}

fn min_fuel_position(crabs: &[i32], f_pos: &dyn Fn(i32, i32) -> Vec<i32>) -> (i32, i32) {
    let max = max_pos(crabs);

    // Get fuel cost for each crab for each position.
    let mut costs = Vec::new();
    for &c in crabs {
        costs.push(f_pos(c, max));
    }

    // Iterate over each position and find the minimum.
    let mut min = i32::MAX;
    let mut min_pos = 0;

    for p in 0..(max+1) {
        let mut sum = 0;
        for cost in &costs {
            sum += cost[p as usize];
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
    let start = Instant::now();

    let fuel = part1();
    println!("Part 1: Minimum fuel required is {}", fuel);

    let fuel = part2();
    println!("Part 2: Minimum fuel required is {}", fuel);

    let elapsed = start.elapsed().as_secs_f32();
    println!("Done in {:.4}", elapsed);
}