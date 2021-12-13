// Advent of Code 2021
// Day 13

use std::str::FromStr;
use aoc::drawing::*;

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
    return 0;
}

fn main() {
    aoc::solution!(13, "after first fold", "");
}
