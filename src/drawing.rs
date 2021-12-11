// Implementations relating to drawing.

use std::cmp::Ordering;
use std::hash::Hash;
use std::str::FromStr;
use std::collections::HashMap;

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

#[derive(Debug)]
pub enum PointParseError {
    MissingX,
    MissingY,
    NotNumberX,
    NotNumberY
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub y: i64,
    pub x: i64
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.split(',');

        // We expect two numbers, x and y.
        let x = s_iter.next().ok_or(PointParseError::MissingX)?;
        let y = s_iter.next().ok_or(PointParseError::MissingY)?;

        let x: i64 = match x.parse() {
            Ok(n) => n,
            Result::Err(_) => return Result::Err(PointParseError::NotNumberX)
        };

        let y: i64 = match y.parse() {
            Ok(n) => n,
            Result::Err(_) => return Result::Err(PointParseError::NotNumberY)
        };

        return Ok(Point { x, y });
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        return Point { x, y }
    }
}

pub struct Grid {
    grid: HashMap<Point, u32>
}

impl Grid {
    pub fn new(x_dim: i64, y_dim: i64) -> Grid {
        let mut grid = HashMap::new();
        for y in 0..(y_dim+1) {
            for x in 0..(x_dim+1) {
                grid.insert(Point::new(x, y), 0);
            }
        }

        return Grid { grid };
    }

    pub fn from_array(grid: Vec<Vec<u32>>) -> Grid {
        let mut g = HashMap::new();
        let mut y = 0;
        for row in grid {
            let mut x = 0;
            for v in row {
                g.insert(Point::new(x, y), v);
                x += 1;
            }
            y += 1;
        }

        return Grid { grid: g }
    }

    pub fn points(&self) -> Vec<Point> {
        let mut p: Vec<Point> = self.grid.keys().map(|p| *p).collect();
        p.sort();
        return p;
    }

    pub fn neighbours(&self, p: &Point) -> Vec<Point> {
        let x = p.x;
        let y = p.y;
        let neighbours = vec![
            Point::new(x-1, y),
            Point::new(x, y-1),
            Point::new(x+1, y),
            Point::new(x, y+1),
        ];

        return neighbours.iter().filter(|p| self.grid.contains_key(&p))
                         .map(|p| *p).collect();
    }

    pub fn set(&mut self, p: Point, val: u32) {
        self.grid.insert(p, val);
    }

    pub fn get(&self, p: &Point) -> Option<u32> {
        return match self.grid.get(p) {
            Some(v) => Some(*v),
            None => None
        }
    }

    // Counts the number of grid squares for which the predicate is true.
    pub fn count(&self, f: &dyn Fn(u32) -> bool) -> u32 {
        let mut count = 0;

        for (_, v) in &self.grid {
            if f(*v) { count += 1; }
        }
        
        return count;
    }
}
