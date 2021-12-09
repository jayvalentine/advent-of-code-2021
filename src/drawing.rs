// Implementations relating to drawing.

use std::str::FromStr;

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

#[derive(Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32
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


pub struct Grid {
    grid: Vec<Vec<u32>>
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Grid {
        let mut rows = Vec::new();
        for _ in 0..(y+1) {
            let row = vec![0; x+1];
            rows.push(row);
        }

        return Grid { grid: rows };
    }

    pub fn from_array(grid: Vec<Vec<u32>>) -> Grid {
        return Grid { grid }
    }

    pub fn points(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        let x_dim = self.grid[0].len();
        let y_dim = self.grid.len();
        for y in 0..y_dim {
            for x in 0..x_dim {
                v.push((x, y));
            }
        }
        return v;
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Vec<u32> {
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x-1, y));
        }
        if y > 0 {
            neighbours.push((x, y-1));
        }
        if x < (self.grid[0].len()-1) {
            neighbours.push((x+1, y));
        }
        if y < (self.grid.len()-1) {
            neighbours.push((x, y+1));
        }

        let mut v = Vec::new();
        for (x, y) in neighbours {
            v.push(self.get(x, y));
        }

        return v;
    }

    pub fn set(&mut self, x: usize, y: usize, val: u32) {
        self.grid[y][x] = val;
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        return self.grid[y][x];
    }

    // Counts the number of grid squares for which the predicate is true.
    pub fn count(&self, f: &dyn Fn(u32) -> bool) -> u32 {
        let mut count = 0;

        for row in &self.grid {
            for col in row {
                if f(*col) { count += 1; }
            }
        }
        
        return count;
    }
}
