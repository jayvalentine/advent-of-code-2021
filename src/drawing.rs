// Implementations relating to drawing.

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

        Ok(Point { x, y })
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

pub struct Grid {
    xsize: usize,
    ysize: usize,
    grid: HashMap<Point, u32>
}

impl Grid {
    pub fn new(xsize: usize, ysize: usize) -> Grid {
        Grid { xsize, ysize, grid: HashMap::new() }
    }

    pub fn from_array(grid: Vec<Vec<u32>>) -> Grid {
        let xsize = grid[0].len();
        let ysize = grid.len();

        let mut g = HashMap::new();
        for (y, row) in grid.into_iter().enumerate() {
            for (x, v) in row.into_iter().enumerate() {
                g.insert(Point::new(x as i64, y as i64), v);
            }
        }

        Grid { xsize, ysize, grid: g }
    }

    pub fn from_points(points: &Vec<Point>, value: u32) -> Grid {
        // Hacky, need to sort out proper grid sizing later.
        let mut grid = Grid { grid: HashMap::new(), xsize: 0, ysize: 0 };
        for p in points {
            grid.set(p, value);
        }
        grid
    }

    pub fn points(&self) -> Vec<Point> {
        let mut p: Vec<Point> = self.grid.keys().copied().collect();
        p.sort();
        p
    }

    pub fn size(&self) -> usize {
        self.xsize * self.ysize
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

        return neighbours.iter().filter(|p| self.grid.contains_key(p)).copied().collect();
    }

    pub fn neighbours_diagonal(&self, p: &Point) -> Vec<Point> {
        let x = p.x;
        let y = p.y;
        let mut neighbours = self.neighbours(p);

        neighbours.push(Point::new(x-1, y-1));
        neighbours.push(Point::new(x-1, y+1));
        neighbours.push(Point::new(x+1, y-1));
        neighbours.push(Point::new(x+1, y+1));

        return neighbours.iter().filter(|p| self.grid.contains_key(p)).copied().collect();
    }

    pub fn set(&mut self, p: &Point, val: u32) {
        if !self.grid.contains_key(p) {
            self.grid.insert(*p, val);
        }
        else
        {
            *self.grid.get_mut(p).unwrap() = val;
        }
    }

    pub fn increment(&mut self, p: &Point) {
        if !self.grid.contains_key(p) {
            self.grid.insert(*p, 0);
        }

        let p = self.grid.get_mut(p).unwrap();
        *p += 1;
    }

    pub fn get(&self, p: &Point) -> u32 {
        return match self.grid.get(p) {
            Some(v) => *v,
            None => 0
        }
    }

    pub fn do_each(&mut self, f: &dyn Fn(u32) -> u32) {
        for v in self.grid.values_mut() {
            *v = f(*v);
        }
    }

    // Counts the number of grid squares for which the predicate is true.
    pub fn count(&self, f: &dyn Fn(u32) -> bool) -> u32 {
        let mut count = 0;

        for v in self.grid.values() {
            if f(*v) { count += 1; }
        }
        
        count
    }
}
