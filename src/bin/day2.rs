// Advent of Code 2021
// Day 2

use std::str::FromStr;
use aoc::data;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        let pos = super::part1();

        assert_eq!(916, pos.depth);
        assert_eq!(1970, pos.horizontal);
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ];

        let mut commands: Vec<Command> = Vec::new();
        for s in input {
            commands.push(Command::from_str(&s).expect("Could not parse command!"));
        }

        let position = final_position(&commands);

        assert_eq!(10, position.depth);
        assert_eq!(15, position.horizontal);
    }

    #[test]
    fn part2() {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ];

        let mut commands: Vec<Command> = Vec::new();
        for s in input {
            commands.push(Command::from_str(&s).expect("Could not parse command!"));
        }

        let position = final_position_with_aim(&commands);

        assert_eq!(60, position.depth);
        assert_eq!(15, position.horizontal);
    }
}

mod test_do_commands {
    use super::*;

    #[test]
    fn forward() {
        let commands = vec![
            Command { dir: Direction::Forward, val: 5 }
        ];

        let position = final_position(&commands);

        assert_eq!(0, position.depth);
        assert_eq!(5, position.horizontal);
    }

    #[test]
    fn down() {
        let commands = vec![
            Command { dir: Direction::Down, val: 19 }
        ];

        let position = final_position(&commands);

        assert_eq!(19, position.depth);
        assert_eq!(0, position.horizontal);
    }

    #[test]
    fn down_up() {
        let commands = vec![
            Command { dir: Direction::Down, val: 19 },
            Command { dir: Direction::Up, val: 4 }
        ];

        let position = final_position(&commands);

        assert_eq!(15, position.depth);
        assert_eq!(0, position.horizontal);
    }

    #[test]
    fn down_forward() {
        let commands = vec![
            Command { dir: Direction::Down, val: 6 },
            Command { dir: Direction::Forward, val: 6 }
        ];

        let position = final_position(&commands);

        assert_eq!(6, position.depth);
        assert_eq!(6, position.horizontal);
    }
}

mod test_do_commands_with_aim {
    use super::*;

    #[test]
    fn forward() {
        let commands = vec![
            Command { dir: Direction::Forward, val: 5 }
        ];

        let position = final_position_with_aim(&commands);

        assert_eq!(0, position.aim);
        assert_eq!(0, position.depth);
        assert_eq!(5, position.horizontal);
    }

    #[test]
    fn down() {
        let commands = vec![
            Command { dir: Direction::Down, val: 19 }
        ];

        let position = final_position_with_aim(&commands);

        assert_eq!(19, position.aim);
        assert_eq!(0, position.depth);
        assert_eq!(0, position.horizontal);
    }

    #[test]
    fn down_up() {
        let commands = vec![
            Command { dir: Direction::Down, val: 19 },
            Command { dir: Direction::Up, val: 4 }
        ];

        let position = final_position_with_aim(&commands);

        assert_eq!(15, position.aim);
        assert_eq!(0, position.depth);
        assert_eq!(0, position.horizontal);
    }

    #[test]
    fn down_forward() {
        let commands = vec![
            Command { dir: Direction::Down, val: 6 },
            Command { dir: Direction::Forward, val: 6 }
        ];

        let position = final_position_with_aim(&commands);

        assert_eq!(6, position.aim);
        assert_eq!(36, position.depth);
        assert_eq!(6, position.horizontal);
    }
}

// A position in terms of depth and horizontal.
struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32
}

// A direction in which the submarine can travel.
enum Direction {
    Down,
    Up,
    Forward
}

enum DirectionError {
    BadDirection
}

impl FromStr for Direction {
    type Err = DirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "down" { return Ok(Direction::Down); }
        else if s == "up" { return Ok(Direction::Up); }
        else if s == "forward" { return Ok(Direction::Forward); }
        else { return Err(DirectionError::BadDirection); }
    }
}

// A command for the submarine, like "down 5" or "forward 11".
struct Command {
    dir: Direction,
    val: u32
}

#[derive(Debug)]
enum CommandError {
    BadCommand,
    BadValue
}

// Trait implementation for Command so we can parse it from a string.
impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let dir_str = split.nth(0).ok_or(CommandError::BadCommand)?;
        let val_str = split.nth(0).ok_or(CommandError::BadValue)?;

        let dir = match Direction::from_str(&dir_str) {
            Ok(d) => d,
            Err(_) => return Err(CommandError::BadCommand)
        };

        let val: u32 = match val_str.parse() {
            Ok(v) => v,
            Err(_) => return Err(CommandError::BadValue)
        };

        let command = Command {
            dir,
            val
        };

        return Ok(command);
    }
}

fn final_position(commands: &[Command]) -> Position {
    let mut pos = Position { depth: 0, horizontal: 0, aim: 0 };

    for command in commands {
        match command.dir {
            Direction::Down => pos.depth += command.val,
            Direction::Up => pos.depth -= command.val,
            Direction::Forward => pos.horizontal += command.val
        };
    }

    return pos;
}

fn final_position_with_aim(commands: &[Command]) -> Position {
    let mut pos = Position { depth: 0, horizontal: 0, aim: 0 };

    for command in commands {
        match command.dir {
            Direction::Down => pos.aim += command.val,
            Direction::Up => pos.aim -= command.val,
            Direction::Forward => {
                pos.horizontal += command.val;
                pos.depth += pos.aim * command.val;
            }
        };
    }

    return pos;
}

fn part1() -> Position {
    let commands: Vec<Command> = data::get("data/day2.txt");

    return final_position(&commands);
}

fn main() {
    let final_pos = part1();
    let product = final_pos.depth * final_pos.horizontal;
    println!("Final position is (d{}, h{}) (product {})", final_pos.depth, final_pos.horizontal, product);
}
