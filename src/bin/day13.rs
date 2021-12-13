// Advent of Code 2021
// Day 13

use std::str::FromStr;
use aoc::drawing::*;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(695, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(89, super::part2());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    fn part1() {
        let input = vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
        ];

        let input: Vec<Point> = input.iter().map(|i| Point::from_str(i).expect("parse error!")).collect();

        let grid = Grid::from_points(&input, 1);
        let grid = fold_along_x(grid, 7);
        assert_eq!(17, grid.count(&|x| x == 1));

        let grid = fold_along_y(grid, 5);
        assert_eq!(16, grid.count(&|x| x == 1));
    }
}

enum Fold {
    Vertical(i64),
    Horizontal(i64)
}

enum FoldParseError {
    InvalidValue,
    InvalidDirective,
    Syntax
}

impl FromStr for Fold {
    type Err = FoldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('=');
        let directive = s.next().ok_or(FoldParseError::Syntax)?;
        let value = s.next().ok_or(FoldParseError::Syntax)?;

        let value = value.parse().or(Err(FoldParseError::InvalidValue))?;

        match directive {
            "fold along x" => Ok(Fold::Vertical(value)),
            "fold along y" => Ok(Fold::Horizontal(value)),
            _ => Err(FoldParseError::InvalidDirective)
        }
    }
}

fn fold_along_x(grid: Grid, y: i64) -> Grid {
    let points = grid.points();

    let mut new_points = Vec::new();
    for p in points {
        if p.y < y { new_points.push(p.clone()); }
        else if p.y > y {
            // Calculate new y position of the point.
            let dist = (p.y - y).abs();
            let new_y = y - dist;
            new_points.push(Point::new(p.x, new_y));
        }
    }

    Grid::from_points(&new_points, 1)
}

fn fold_along_y(grid: Grid, x: i64) -> Grid {
    let points = grid.points();

    let mut new_points = Vec::new();
    for p in points {
        if p.x < x { new_points.push(p.clone()); }
        else if p.x > x {
            // Calculate new y position of the point.
            let dist = (p.x - x).abs();
            let new_x = x - dist;
            new_points.push(Point::new(new_x, p.y));
        }
    }

    Grid::from_points(&new_points, 1)
}

fn part1() -> u32 {
    let points = aoc::data::get("data/day13.txt");
    let grid = Grid::from_points(&points, 1);
    let grid = fold_along_y(grid, 655);
    grid.count(&|x| x == 1)
}

fn part2() -> u32 {
    let points: Vec<Point> = aoc::data::get("data/day13.txt");
    let folds: Vec<Fold> = aoc::data::get("data/day13.txt");

    let mut grid = Grid::from_points(&points, 1);

    for fold in folds {
        if let Fold::Horizontal(y) = fold {
            grid = fold_along_x(grid, y);
        }
        else if let Fold::Vertical(x) = fold {
            grid = fold_along_y(grid, x);
        }
    }

    let points = grid.points();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    for y in 0..(max_y+1) {
        for x in 0..(max_x+1) {
            let v = grid.get(&Point::new(x, y));
            if v == 1 {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }

    return grid.count(&|x| x == 1);
}

fn main() {
    aoc::solution!(13, "after first fold", "");
}
