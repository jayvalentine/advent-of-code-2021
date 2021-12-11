// Advent of Code 2021
// Day 9

use core::slice::Iter;
use std::cmp::Reverse;
use aoc::drawing::{Grid, Point};

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(524, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1235430, super::part2());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        let grid = get_grid(&mut input.iter());
        let local_minima = local_minima(&grid);

        assert_eq!(4, local_minima.len());
        assert_eq!(1, grid.get(&local_minima[0]));
        assert_eq!(0, grid.get(&local_minima[1]));
        assert_eq!(5, grid.get(&local_minima[2]));
        assert_eq!(5, grid.get(&local_minima[3]));
    }

    #[test]
    fn part2() {
        let input = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        let grid = get_grid(&mut input.iter());
        let basins = get_basins(&grid);

        assert_eq!(4, basins.len());
        assert_eq!(3, basins[0].len());
        assert_eq!(9, basins[1].len());
        assert_eq!(14, basins[2].len());
        assert_eq!(9, basins[3].len());
    }
}

#[cfg(test)]
mod test_basins {
    use super::*;

    #[test]
    fn top_left() {
        let input = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        let grid = get_grid(&mut input.iter());
        let basin = get_basin(&grid, Point::new(1, 0));

        assert_eq!(3, basin.len());
    }

    #[test]
    fn top_right() {
        let input = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        let grid = get_grid(&mut input.iter());
        let basin = get_basin(&grid, Point::new(9, 0));

        assert_eq!(9, basin.len());
    }
}

#[cfg(test)]
mod test_is_minimum {
    use super::*;

    #[test]
    fn yes() {
        let v = 4;
        let n = vec![5, 6, 7, 10];
        assert!(is_minimum(&v, &n));
    }

    #[test]
    fn no_equal() {
        let v = 4;
        let n = vec![5, 4, 7, 10];
        assert_eq!(false, is_minimum(&v, &n));
    }

    #[test]
    fn no_greater() {
        let v = 5;
        let n = vec![5, 4, 7, 10];
        assert_eq!(false, is_minimum(&v, &n));
    }
}

fn is_minimum(v: &u32, n: &[u32]) -> bool {
    for other in n {
        if other <= v {
            return false;
        }
    }

    true
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

fn local_minima(g: &Grid) -> Vec<Point> {
    let mut minima = Vec::new();

    for p in g.points() {
        let val = g.get(&p);
        let neighbours = g.neighbours(&p);
        let neighbours: Vec<u32> = neighbours.iter().map(|n| g.get(n)).collect();

        if is_minimum(&val, &neighbours) {
            minima.push(p);
        }
    }

    minima
}

fn get_basins(g: &Grid) -> Vec<Vec<Point>> {
    let minima = local_minima(g);
    let mut basins = Vec::new();
    for m in minima {
        basins.push(get_basin(g, m));
    }

    basins
}

fn get_basin(g: &Grid, starting_point: Point) -> Vec<Point> {
    let mut visited = Vec::new();
    let mut to_visit = vec![starting_point];

    while !to_visit.is_empty() {
        // Next points to visit.
        let mut next_to_visit = Vec::new();

        for p in &to_visit {
            let val = g.get(p);
            if val != 9 {
                if !visited.contains(p) {
                    visited.push(*p);
                }
                
                let neighbours = g.neighbours(p);
                for n in neighbours {
                    if !visited.contains(&n) {
                        next_to_visit.push(n);
                    }
                }
            }
        }

        to_visit.clear();
        to_visit.append(&mut next_to_visit);
    }

    visited
}

fn get_data() -> Grid {
    aoc::data::get_with_iter("data/day9.txt", &mut get_grid)
}

fn part1() -> u32 {
    let grid = get_data();
    let minima = local_minima(&grid);
    
    let mut risk_level = 0;
    for m in minima {
        risk_level += grid.get(&m) + 1;
    }

    risk_level
}

fn part2() -> u32 {
    let grid = get_data();
    let mut basins = get_basins(&grid);
    basins.sort_by_key(|b| Reverse(b.len()));

    let result = basins[0].len() * basins[1].len() * basins[2].len();
    result as u32
}

fn main() {
    aoc::solution!(9, "# local minima", "largest basins (product)");
}