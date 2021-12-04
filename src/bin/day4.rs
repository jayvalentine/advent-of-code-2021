// Advent of Code 2021
// Day 4

use core::iter::{FromIterator, Iterator};
use std::slice::Iter;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            "8  2 23  4 24",
            "21  9 14 16  7",
            "6 10  3 18  5",
            "1 12 20 15 19",
            "",
            "3 15  0  2 22",
            "9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            "2  0 12  3  7",
        ];

        let mut input_iter = input.iter();

        let calls = parse_calls(&mut input_iter);

        let mut boards = Vec::new();
        for i in 0..3 {
            boards.push(BingoBoard::parse(&mut input_iter))
        }

        let score = play_bingo(&calls, &mut boards);

        assert_eq!(4512, score);
    }
}

#[cfg(test)]
mod test_parse_calls {
    use super::*;

    #[test]
    fn parse1() {
        let input = vec![
            "1,33,88,9,20,4",
            ""
        ];

        let mut input_iter = input.iter();

        let calls = parse_calls(&mut input_iter);

        assert_eq!(6, calls.len());
        assert_eq!(1, calls[0]);
        assert_eq!(33, calls[1]);
        assert_eq!(88, calls[2]);
        assert_eq!(9, calls[3]);
        assert_eq!(20, calls[4]);
        assert_eq!(4, calls[5]);
    }
}

#[cfg(test)]
mod test_parse_board {
    use super::*;

    #[test]
    fn parse1() {
        let input = vec![
            "99 20 40 60 22",
            " 1  4 90 63 12",
            "11 13  5  9 10",
            "34 35 36 37 38",
            "89 78 67 56 45",
        ];

        let mut input_iter = input.iter();

        let board = BingoBoard::parse(&mut input_iter);

        assert_eq!(99, board.cell(0, 0));
        assert_eq!(1, board.cell(0, 1));
        assert_eq!(11, board.cell(0, 2));
        assert_eq!(34, board.cell(0, 3));
        assert_eq!(89, board.cell(0, 4));

        assert_eq!(20, board.cell(1, 0));
        assert_eq!(4, board.cell(1, 1));
        assert_eq!(13, board.cell(1, 2));
        assert_eq!(35, board.cell(1, 3));
        assert_eq!(78, board.cell(1, 4));

        assert_eq!(40, board.cell(2, 0));
        assert_eq!(90, board.cell(2, 1));
        assert_eq!(5, board.cell(2, 2));
        assert_eq!(36, board.cell(2, 3));
        assert_eq!(67, board.cell(2, 4));

        assert_eq!(60, board.cell(3, 0));
        assert_eq!(63, board.cell(3, 1));
        assert_eq!(9, board.cell(3, 2));
        assert_eq!(37, board.cell(3, 3));
        assert_eq!(56, board.cell(3, 4));

        assert_eq!(22, board.cell(4, 0));
        assert_eq!(12, board.cell(4, 1));
        assert_eq!(10, board.cell(4, 2));
        assert_eq!(38, board.cell(4, 3));
        assert_eq!(45, board.cell(4, 4));
    }use std::fs::File;
    use std::io::{prelude::*, BufReader};
}

#[cfg(test)]
mod test_board {
    use super::*;

    #[test]
    fn mark_number() {
        let mut board = BingoBoard {
            grid: [
                [1, 2, 3, 4, 5],
                [44, 33, 22, 11, 55],
                [99, 98, 97, 96, 95],
                [12, 13, 14, 15, 16],
                [10, 9, 8, 7, 6]
            ],

            marks: [[false; 5]; 5]
        };

        board.call(22);

        assert_eq!(true, board.marks[1][2]);
        assert_eq!(false, board.marks[0][0]);
        assert_eq!(false, board.marks[2][1]);
        assert_eq!(false, board.marks[1][3]);
    }

    #[test]
    fn bingo_row() {
        let mut board = BingoBoard {
            grid: [
                [1, 2, 3, 4, 5],
                [44, 33, 22, 11, 55],
                [99, 98, 97, 96, 95],
                [12, 13, 14, 15, 16],
                [10, 9, 8, 7, 6]
            ],

            marks: [[false; 5]; 5]
        };

        board.call(22);
        assert_eq!(false, board.bingo());

        board.call(33);
        assert_eq!(false, board.bingo());

        board.call(55);
        assert_eq!(false, board.bingo());

        board.call(11);
        assert_eq!(false, board.bingo());

        board.call(44);
        assert_eq!(true, board.bingo());
    }

    #[test]
    fn bingo_col() {
        let mut board = BingoBoard {
            grid: [
                [1, 2, 3, 4, 5],
                [44, 33, 22, 11, 55],
                [99, 98, 97, 96, 95],
                [12, 13, 14, 15, 16],
                [10, 9, 8, 7, 6]
            ],

            marks: [[false; 5]; 5]
        };

        board.call(1);
        assert_eq!(false, board.bingo());

        board.call(99);
        assert_eq!(false, board.bingo());

        board.call(12);
        assert_eq!(false, board.bingo());

        board.call(44);
        assert_eq!(false, board.bingo());

        board.call(10);
        assert_eq!(true, board.bingo());
    }
}

struct BingoBoard {
    // 2D grid, row-major.
    grid: [[u32; 5]; 5],

    // Indicates which squares are marked.
    marks: [[bool; 5]; 5]
}

impl BingoBoard {
    fn parse(input: &mut Iter<&str>) -> BingoBoard {
        let mut grid = [[0; 5]; 5];

        for y in 0..5 {
            let mut row_str = input.next().expect("Missing row!");
            while row_str.trim().is_empty() {
                row_str = input.next().expect("Missing row!");
            }
            
            let mut row_iter = row_str.split_whitespace();
            for x in 0..5 {
                let n: u32 = row_iter.next().expect("Incomplete row!")
                                     .parse().expect("Incomplete row!");

                grid[y][x] = n;
            }
        }

        return BingoBoard { grid, marks: [[false; 5]; 5] }
    }

    fn cell(&self, x: usize, y: usize) -> u32 {
        return self.grid[y][x];
    }

    fn call(&mut self, n: u32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.grid[y][x] == n {
                    self.marks[y][x] = true;
                    return;
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        for y in 0..5 {
            let mut row_done = true;
            for x in 0..5 {
                if !self.marks[y][x] {
                    row_done = false;
                }
            }

            if row_done {
                return true;
            }
        }

        for x in 0..5 {
            let mut col_done = true;
            for y in 0..5 {
                if !self.marks[y][x] {
                    col_done = false;
                }
            }

            if col_done {
                return true;
            }
        }

        return false;
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.marks[y][x] {
                    score += self.grid[y][x];
                }
            }
        }

        return score;
    }
}

fn parse_calls(iter: &mut Iter<&str>) -> Vec<u32> {
    let calls_str = iter.next().expect("No call line to parse!");

    let mut calls = Vec::new();
    for call in calls_str.split(',') {
        calls.push(call.parse().expect("Non-number in call!"));
    }
    return calls;
}

fn play_bingo(calls: &[u32], boards: &mut [BingoBoard]) -> u32 {
    for call in calls {
        for board in &mut *boards {
            board.call(*call);
            if board.bingo() {
                return board.score() * call;
            }
        }
    }

    panic!("Nobody won bingo!");
}

fn part1() -> u32 {
    let file = "data/day4.txt";

    // Read test data in, iterate over each line.
    let f = File::open(file).expect(&format!("Could not open {}", file));
    let reader = BufReader::new(f);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line_str = line.expect("Invalid line in file!");

        lines.push(line_str);
    }

    let lines: Vec<&str> = lines.iter().map(|x| x.as_str()).collect();

    let mut lines_iter = lines.iter();

    let calls = parse_calls(&mut lines_iter);
    let mut boards = Vec::new();

    while lines_iter.len() > 0 {
        boards.push(BingoBoard::parse(&mut lines_iter));
    }

    let score = play_bingo(&calls, &mut boards);
    return score;
}

fn main() {
    let score = part1();
    println!("Part 1: Score is {}", score);
}
