// Advent of Code 2021
// Day 9

use core::slice::Iter;
use aoc::drawing::Grid;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(524, super::part1());
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
        assert_eq!(1, grid.get(local_minima[0].0, local_minima[0].1));
        assert_eq!(0, grid.get(local_minima[1].0, local_minima[1].1));
        assert_eq!(5, grid.get(local_minima[2].0, local_minima[2].1));
        assert_eq!(5, grid.get(local_minima[3].0, local_minima[3].1));
    }
}

#[cfg(test)]
mod test_is_minimum {
    use super::*;

    #[test]
    fn yes() {
        let v = 4;
        let n = vec![5, 6, 7, 10];
        assert_eq!(true, is_minimum(&v, &n));
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

    return true;
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

    return Grid::from_array(grid);
}

fn local_minima(g: &Grid) -> Vec<(usize, usize)> {
    let mut minima = Vec::new();

    for (x, y) in g.points() {
        let val = g.get(x, y);
        let neighbours = g.neighbours(x, y);

        if is_minimum(&val, &neighbours) {
            minima.push((x, y));
        }
    }

    return minima;
}

fn get_data() -> Grid {
    return aoc::data::get_with_iter("data/day9.txt", &mut get_grid);
}

fn part1() -> u32 {
    let grid = get_data();
    let minima = local_minima(&grid);
    
    let mut risk_level = 0;
    for m in minima {
        risk_level += grid.get(m.0, m.1) + 1;
    }

    return risk_level;
}

fn part2() -> u32 {
    return 0;
}

fn main() {
    aoc::solution!(9, "# local minima", "");
}