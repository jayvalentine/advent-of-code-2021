// Advent of Code 2021
// Day 11

use std::slice::Iter;
use std::collections::HashSet;
use aoc::drawing::*;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(1652, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(220, super::part2());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];

        let mut grid = get_grid(&mut input.iter());
        assert_eq!(1656, total_flashes(&mut grid));
    }

    #[test]
    fn part2() {
        let input = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];

        let mut grid = get_grid(&mut input.iter());
        assert_eq!(195, first_synchronized_flash(&mut grid));
    }
}

#[cfg(test)]
mod test_step {
    use super::*;

    #[test]
    fn simple() {
        let input = vec![
            "000",
            "000",
            "000"
        ];

        let mut grid = get_grid(&mut input.iter());
        assert_eq!(0, step(&mut grid));
        assert_eq!(9, grid.count(&|v| v == 1));
    }

    #[test]
    fn one_flash_middle() {
        let input = vec![
            "000",
            "090",
            "000"
        ];

        let mut grid = get_grid(&mut input.iter());
        assert_eq!(1, step(&mut grid));
        assert_eq!(0, grid.get(&Point::new(1, 1)));
        assert_eq!(8, grid.count(&|v| v == 2));
    }

    #[test]
    fn one_flash_edge() {
        let input = vec![
            "090",
            "000",
            "000"
        ];

        let mut grid = get_grid(&mut input.iter());
        assert_eq!(1, step(&mut grid));
        assert_eq!(0, grid.get(&Point::new(1, 0)));
        assert_eq!(5, grid.count(&|v| v == 2));
        assert_eq!(3, grid.count(&|v| v == 1));

        assert_eq!(1, grid.get(&Point::new(0, 2)));
        assert_eq!(1, grid.get(&Point::new(1, 2)));
        assert_eq!(1, grid.get(&Point::new(2, 2)));
    }

    #[test]
    fn knockon_flash() {
        let input = vec![
            "820",
            "900",
            "000"
        ];

        let mut grid = get_grid(&mut input.iter());
        assert_eq!(2, step(&mut grid));

        assert_eq!(0, grid.get(&Point::new(0, 1)));
        assert_eq!(0, grid.get(&Point::new(0, 0)));

        assert_eq!(5, grid.get(&Point::new(1, 0)));
        assert_eq!(3, grid.get(&Point::new(1, 1)));
        assert_eq!(2, grid.get(&Point::new(0, 2)));

        assert_eq!(3, grid.count(&|v| v == 1));
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

fn step(grid: &mut Grid) -> u64 {
    // First increment each cell by 1.
    grid.do_each(&|v| v + 1);

    // Now calculate flashes.
    let mut flashes: HashSet<Point> = HashSet::new();
    for p in grid.points() {
        let v = grid.get(&p);
        if v > 9 {
            flashes.insert(p);
        }
    }

    let mut iter_flashes = flashes.clone();
    let mut count_flashes = flashes.len();

    loop {
        let mut new_flashes = HashSet::new();

        for flash in &iter_flashes {
            let neighbours = grid.neighbours_diagonal(flash);
            for n in neighbours {
                grid.increment(&n);
            }
        }

        for p in grid.points() {
            if flashes.contains(&p) { continue; }

            let v = grid.get(&p);
            if v > 9 {
                new_flashes.insert(p);
                flashes.insert(p);
            }
        }

        if new_flashes.is_empty() { break; }
        count_flashes += new_flashes.len();

        iter_flashes = new_flashes;
    }

    for f in &flashes {
        grid.set(f, 0);
    }

    count_flashes as u64
}

fn total_flashes(grid: &mut Grid) -> u64 {
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(grid);
    }
    flashes
}

fn first_synchronized_flash(grid: &mut Grid) -> u64 {
    let mut this_step = 0;
    let size = grid.size();
    loop {
        let flashes = step(grid);
        if flashes == size as u64 {
            return this_step + 1;
        }
        this_step += 1;
    }
}

fn get_data() -> Grid {
    aoc::data::get_with_iter("data/day11.txt", &mut get_grid)
}

fn part1() -> u64 {
    let mut grid = get_data();
    total_flashes(&mut grid)
}

fn part2() -> u64 {
    let mut grid = get_data();
    first_synchronized_flash(&mut grid)
}

fn main() {
    aoc::solution!(11, "total flashes", "first sync flash");
}
