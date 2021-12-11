// Advent of Code 2021
// Day 5

use std::str::FromStr;

use aoc::data;
use aoc::drawing::{Point, Grid};

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        let overlapping = super::part1();
        assert_eq!(5373, overlapping);
    }

    #[test]
    fn part2() {
        let overlapping = super::part2();
        assert_eq!(21514, overlapping);
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"
        ];

        let mut lines = Vec::new();
        for s in input {
            let l = Line::from_str(s).expect("Line parsing failed!");
            lines.push(l);
        }

        let overlapping = overlapping_lines_without_diagonal(&lines);
        assert_eq!(5, overlapping);
    }

    #[test]
    fn part2() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"
        ];

        let mut lines = Vec::new();
        for s in input {
            let l = Line::from_str(s).expect("Line parsing failed!");
            lines.push(l);
        }

        let overlapping = overlapping_lines(&lines);
        assert_eq!(12, overlapping);
    }
}

#[cfg(test)]
mod test_parse_line {
    use super::*;

    #[test]
    fn parse1() {
        let input = "1,4 -> 5,4";
        let line = Line::from_str(input).expect("Line parsing failed!");

        assert_eq!(1, line.p1.x);
        assert_eq!(5, line.p2.x);
        assert_eq!(4, line.p1.y);
        assert_eq!(4, line.p2.y);
    }

    #[test]
    fn parse2() {
        let input = "5,5 -> 7,7";
        let line = Line::from_str(input).expect("Line parsing failed!");

        assert_eq!(5, line.p1.x);
        assert_eq!(7, line.p2.x);
        assert_eq!(5, line.p1.y);
        assert_eq!(7, line.p2.y);
    }

    #[test]
    fn parse3() {
        let input = "9,1 -> 9,4";
        let line = Line::from_str(input).expect("Line parsing failed!");

        assert_eq!(9, line.p1.x);
        assert_eq!(9, line.p2.x);
        assert_eq!(1, line.p1.y);
        assert_eq!(4, line.p2.y);
    }
}

#[derive(Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point
}

#[derive(Debug)]
enum LineParseError {
    BadSyntax
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split(" -> ");

        // First pair of coordinates.
        let p1 = s_iter.next().ok_or(LineParseError::BadSyntax)?;

        // Second pair of coordinates.
        let p2 = s_iter.next().ok_or(LineParseError::BadSyntax)?;

        let p1 = match Point::from_str(p1) {
            Ok(p) => p,
            Err(_) => return Err(LineParseError::BadSyntax)
        };
        
        let p2 = match Point::from_str(p2) {
            Ok(p) => p,
            Err(_) => return Err(LineParseError::BadSyntax)
        };

        return Ok(Line { p1, p2 });
    }
}

fn max_x(lines: &[Line]) -> i64 {
    let mut x = 0;
    for l in lines {
        if l.p1.x > x { x = l.p1.x }
        if l.p2.x > x { x = l.p2.x }
    }

    return x;
}

fn max_y(lines: &[Line]) -> i64 {
    let mut y = 0;
    for l in lines {
        if l.p1.y > y { y = l.p1.y }
        if l.p2.y > y { y = l.p2.y }
    }

    return y;
}

fn exclude_diagonal(lines: &[Line]) -> Vec<Line> {
    let mut v = Vec::new();

    for l in lines {
        if (l.p1.x == l.p2.x) || (l.p1.y == l.p2.y) {
            v.push(*l);
        }
    }

    return v;
}

fn overlapping_lines_without_diagonal(lines: &[Line]) -> u32 {
    let lines = exclude_diagonal(&lines);

    // Return number of squares with more than one line.
    return overlapping_lines(&lines);
}

fn overlapping_lines(lines: &[Line]) -> u32 {
    // Get maximum X and Y.
    let max_x = max_x(lines);
    let max_y = max_y(lines);

    // Create the grid.
    let mut grid = Grid::new(max_x as usize, max_y as usize);

    // For each line, plot on the grid.
    for line in lines {
        // Calculate dx and dy.
        let dx = if line.p1.x > line.p2.x {
            -1
        }
        else if line.p2.x > line.p1.x {
            1
        }
        else {
            0
        };
        
        let dy = if line.p1.y > line.p2.y {
            -1
        }
        else if line.p2.y > line.p1.y {
            1
        }
        else {
            0
        };

        let mut x = line.p1.x as i32;
        let mut y = line.p1.y as i32;

        let x_end = line.p2.x as i32;
        let y_end = line.p2.y as i32;

        loop {
            grid.set(x as usize, y as usize, grid.get(x as usize, y as usize) + 1);
            x += dx;
            y += dy;

            if (x == x_end) && (y == y_end) {
                break;
            }
        }

        grid.set(x_end as usize, y_end as usize, grid.get(x_end as usize, y_end as usize) + 1);
    }

    // Return number of squares with more than one line.
    return grid.count(&|n| n > 1);
}

fn part1() -> u32 {
    let input = data::get("data/day5.txt");

    return overlapping_lines_without_diagonal(&input);
}

fn part2() -> u32 {
    let input = data::get("data/day5.txt");

    return overlapping_lines(&input);
}

fn main() {
    aoc::solution!(5, "without diagonals", "with diagonals");
}