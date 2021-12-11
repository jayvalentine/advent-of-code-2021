// Advent of Code 2021
// Day 10

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

#[derive(PartialEq, Eq, Debug)]
enum ChunkParseError {
    Mismatch(char, char)
}

fn parse(s: &str) -> Result<(), ChunkParseError> {
    return Ok(());
}

fn part1() -> u32 {
    return 0;
}

fn part2() -> u32 {
    return 0;
}

fn main() {
    aoc::solution!(10, "syntax error score", "");
}
