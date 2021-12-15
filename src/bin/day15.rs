// Advent of Code 2021
// Day 15

use std::slice::Iter;
use aoc::drawing::{Grid, Point};

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ];

        let grid = get_grid(&mut input.iter());

        let shortest_path = a_star(&grid, Point::new(0, 0), Point::new(9, 9));

        let expected = vec![
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (6, 3),
            (7, 3),
            (7, 4),
            (7, 5),
            (8, 5),
            (8, 6),
            (8, 7),
            (8, 8),
            (9, 8),
            (9, 9)
        ];

        let expected: Vec<Point> = expected.iter().map(|p| Point::new(p.0, p.1)).collect();

        assert_eq!(expected, shortest_path);
    }
}

fn get_grid(i: &mut Iter<&str>) -> Grid {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for s in i {
        let mut v = Vec::new();
        for c in s.chars() {
            let n = format!("{}", c).parse().expect("Parse error!");
            v.push(n);
        }
        grid.push(v);
    }

    Grid::from_array(grid)
}

fn a_star(grid: &Grid, start: Point, end: Point) -> Vec<Point> {
    return Vec::new();
}

fn part1() -> u64 {
    return 0;
}

fn part2() -> u64 {
    return 0;
}

fn main() {
    aoc::solution!(15, "lowest risk", "");
}