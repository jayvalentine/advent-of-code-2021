use std::{str::FromStr, string::ParseError};

// Advent of Code 2021
// Day 2

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
}

// A position in terms of depth and horizontal.
struct Position {
    depth: u32,
    horizontal: u32
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
    return Position { depth: 10, horizontal: 15 }
}

fn main() {

}
