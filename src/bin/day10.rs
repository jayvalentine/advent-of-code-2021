// Advent of Code 2021
// Day 10

use aoc::data;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(240123, super::part1());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = "{([(<{}[<>[]}>{[]{[(<()>";
        let result = parse(input);
        assert_eq!(ChunkParseError::Mismatch(']', '}'), result.expect_err("error did not occur!"));

        let input = "[[<[([]))<([[{}[[()]]]";
        let result = parse(input);
        assert_eq!(ChunkParseError::Mismatch(']', ')'), result.expect_err("error did not occur!"));

        let input = "[{[{({}]{}}([{[{{{}}([]";
        let result = parse(input);
        assert_eq!(ChunkParseError::Mismatch(')', ']'), result.expect_err("error did not occur!"));

        let input = "[<(<(<(<{}))><([]([]()";
        let result = parse(input);
        assert_eq!(ChunkParseError::Mismatch('>', ')'), result.expect_err("error did not occur!"));

        let input = "<{([([[(<>()){}]>(<<{{";
        let result = parse(input);
        assert_eq!(ChunkParseError::Mismatch(']', '>'), result.expect_err("error did not occur!"));
    }
}

#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn ok_empty() {
        let input = "";
        assert_eq!(Ok(()), parse(input));
    }

    #[test]
    fn ok_chunk() {
        let input = "()";
        assert_eq!(Ok(()), parse(input));
    }

    #[test]
    fn ok_nested_chunk() {
        let input = "(<>)";
        assert_eq!(Ok(()), parse(input));
    }

    #[test]
    fn ok_two_nested_chunk() {
        let input = "(<>[])";
        assert_eq!(Ok(()), parse(input));
    }

    #[test]
    fn mismatch() {
        let input = "(]";
        assert_eq!(ChunkParseError::Mismatch(')', ']'), parse(input).expect_err("error did not occur!"));
    }

    #[test]
    fn mismatch2() {
        let input = "<}";
        assert_eq!(ChunkParseError::Mismatch('>', '}'), parse(input).expect_err("error did not occur!"));
    }

    #[test]
    fn mismatch_nested() {
        let input = "<{]>";
        assert_eq!(ChunkParseError::Mismatch('}', ']'), parse(input).expect_err("error did not occur!"));
    }

    #[test]
    fn incomplete() {
        let input = "<()";
        assert_eq!(ChunkParseError::Incomplete(vec!['<']), parse(input).expect_err("error did not occur!"));
    }

    #[test]
    fn invalid() {
        let input = "<b)";
        assert_eq!(ChunkParseError::Invalid('b'), parse(input).expect_err("error did not occur!"));
    }

    #[test]
    fn imbalance() {
        let input = "<>)";
        assert_eq!(ChunkParseError::Imbalance, parse(input).expect_err("error did not occur!"));
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ChunkParseError {
    Mismatch(char, char),
    Incomplete(Vec<char>),
    Invalid(char),
    Imbalance
}

fn parse(s: &str) -> Result<(), ChunkParseError> {
    let lhs = ['(', '[', '{', '<'];
    let rhs = [')', ']', '}', '>'];

    let mut stack = Vec::new();

    for c in s.chars() {
        // If an open chunk, push to stack.
        // If close chunk, check against TOS.
        // Otherwise, return an error.
        if lhs.contains(&c) {
            stack.push(c);
        }
        else if rhs.contains(&c) {
            let expected = stack.pop();
            let expected = match expected {
                Some(e) => match e {
                    '(' => ')',
                    '[' => ']',
                    '<' => '>',
                    '{' => '}',
                    _ => unreachable!()
                },
                None => return Err(ChunkParseError::Imbalance)
            };
            
            // Now we know what character we're expecting
            // we can check against the one in the input.
            if c != expected {
                return Err(ChunkParseError::Mismatch(expected, c));
            }
        }
        else {
            return Err(ChunkParseError::Invalid(c));
        }
    }

    // If we reach end of input and the stack is not empty,
    // we've got an incomplete chunk.
    if stack.len() > 0 {
        return Err(ChunkParseError::Incomplete(stack));
    }

    // We've not seen any errors, so it's a valid chunk!
    return Ok(());
}

fn syntax_score(e: ChunkParseError) -> u32 {
    return match e {
        ChunkParseError::Mismatch(_, c) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0
        },
        _ => 0
    }
}

fn part1() -> u32 {
    let input: Vec<String> = data::get("data/day10.txt");
    let mut score = 0;
    for i in input {
        let r = parse(&i);
        if r.is_err() {
            score += syntax_score(r.unwrap_err());
        }
    }

    return score;
}

fn part2() -> u32 {
    return 0;
}

fn main() {
    aoc::solution!(10, "syntax error score", "");
}
