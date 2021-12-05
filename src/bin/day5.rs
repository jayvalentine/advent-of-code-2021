// Advent of Code 2021
// Day 5

use std::str::FromStr;

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

        let overlapping = overlapping_lines(&lines);
        assert_eq!(5, overlapping);
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

#[cfg(test)]
mod test_parse_point {
    use super::*;

    #[test]
    fn parse1() {
        let input = "1,2";
        let p = Point::from_str(input).expect("Line parsing failed!");
        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
    }

    #[test]
    fn parse2() {
        let input = "4,4";
        let p = Point::from_str(input).expect("Line parsing failed!");
        assert_eq!(4, p.x);
        assert_eq!(4, p.y);
    }

    #[test]
    fn parse3() {
        let input = "9,3";
        let p = Point::from_str(input).expect("Line parsing failed!");
        assert_eq!(9, p.x);
        assert_eq!(3, p.y);
    }
}

struct Point {
    x: u32,
    y: u32
}

struct Line {
    p1: Point,
    p2: Point
}

#[derive(Debug)]
enum PointParseError {
    MissingX,
    MissingY,
    NotNumberX,
    NotNumberY
}

#[derive(Debug)]
enum LineParseError {
    BadSyntax
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split(',');

        // We expect two numbers, x and y.
        let x = s_iter.next().ok_or(PointParseError::MissingX)?;
        let y = s_iter.next().ok_or(PointParseError::MissingY)?;

        let x: u32 = match x.parse() {
            Ok(n) => n,
            Result::Err(_) => return Result::Err(PointParseError::NotNumberX)
        };

        let y: u32 = match y.parse() {
            Ok(n) => n,
            Result::Err(_) => return Result::Err(PointParseError::NotNumberY)
        };

        return Ok(Point { x, y });
    }
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

fn overlapping_lines(lines: &[Line]) -> u32 {
    return 0;
}

fn main() {

}