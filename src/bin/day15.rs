// Advent of Code 2021
// Day 15

use std::{slice::Iter, collections::HashMap};
use std::cmp::Reverse;
use aoc::drawing::{Grid, Point};

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(652, super::part1());
    }
}

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
            (0, 0),
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

fn get_f_score(p: &Point, f_score: &HashMap<Point, i64>) -> i64 {
    match f_score.get(p) {
        Some(v) => *v,
        None => i64::MAX
    }
}

fn get_g_score(p: &Point, g_score: &HashMap<Point, i64>) -> i64 {
    match g_score.get(p) {
        Some(v) => *v,
        None => i64::MAX
    }
}

fn path(p: Point, came_from: &HashMap<Point, Point>) -> Vec<Point> {
    let mut path = vec![p];

    let mut current = p;
    while came_from.contains_key(&current) {
        current = came_from[&current];
        path.push(current);
    }

    path.iter().rev().map(|p| *p).collect()
}

// Implementation of the A* search algorithm.
// Implemented with help from: https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
fn a_star(grid: &Grid, start: Point, end: Point) -> Vec<Point> {
    // Set of nodes under consideration for search.
    let mut open_set: Vec<Point> = vec![start];

    // For a given node n, the node immediately preceeding it
    // on the cheapest path from start to n currently known.
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    // For a node n, g_score[n] is the cost of the cheapest path
    // from start to n currently known.
    let mut g_score: HashMap<Point, i64> = HashMap::new();
    g_score.insert(start, 0);

    // For a node n, f_score[n] = g_score[n] + h(n).
    // Represents current best guess as to how short a path
    // from start to end can be if it passes through n.
    let mut f_score: HashMap<Point, i64> = HashMap::new();
    f_score.insert(start, start.manhattan(&end));

    while open_set.len() > 0 {
        // Inefficient, but good enough for now!
        open_set.sort_by_key(|p| Reverse(get_f_score(p, &f_score)));

        // Lowest f_score is at the end of the set.
        let current = open_set.pop().unwrap();

        if current == end {
            return path(current, &came_from);
        }

        for n in grid.neighbours(&current) {
            // Weight of the edge is the risk level of entering the neighbour.
            let d = grid.get(&n) as i64;

            let tentative_g_score = get_g_score(&current, &g_score) + d;
            let current_g_score = get_g_score(&n, &g_score);

            if tentative_g_score < current_g_score {
                came_from.insert(n, current);
                g_score.insert(n, tentative_g_score);
                f_score.insert(n, tentative_g_score + n.manhattan(&end));

                if !open_set.contains(&n) { open_set.push(n) }
            }
        }
    }

    panic!("Path not found!");
}

fn part1() -> u64 {
    let grid = aoc::data::get_with_iter("data/day15.txt", &mut get_grid);
    let start = Point::new(0, 0);
    let end = Point::new((grid.xsize() - 1) as i64, (grid.ysize() - 1) as i64);

    let shortest_path = a_star(&grid, start, end);

    let mut risk = 0;

    // Iterate over each point in path, ignoring first (start).
    for p in &shortest_path[1..] {
        risk += grid.get(p);
    }

    return risk as u64;
}

fn part2() -> u64 {
    return 0;
}

fn main() {
    aoc::solution!(15, "lowest risk", "");
}