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

    #[test]
    fn part2() {
        let input = "[({(<(())[]>[[{[]{<()<>>";
        let completion_string = autocomplete(input);
        assert_eq!("}}]])})]", completion_string.expect("expected completion string!"));

        let input = "[(()[<>])]({[<{<<[]>>(";
        let completion_string = autocomplete(input);
        assert_eq!(")}>]})", completion_string.expect("expected completion string!"));

        let input = "(((({<>}<{<{<>}{[]{[]{}";
        let completion_string = autocomplete(input);
        assert_eq!("}}>}>))))", completion_string.expect("expected completion string!"));

        let input = "{<[[]]>}<{[{[{[]{()[[[]";
        let completion_string = autocomplete(input);
        assert_eq!("]]}}]}]}>", completion_string.expect("expected completion string!"));

        let input = "<{([{{}}[<[[[<>{}]]]>[]]";
        let completion_string = autocomplete(input);
        assert_eq!("])}>", completion_string.expect("expected completion string!"));
    }
}

#[cfg(test)]
mod test_autocomplete {
    use super::*;

    #[test]
    fn single_char() {
        assert_eq!(")", autocomplete("(").expect("expected completion string!"));
    }

    #[test]
    fn two_open() {
        assert_eq!(">]", autocomplete("[<()").expect("expected completion string!"));
    }

    #[test]
    fn nested() {
        assert_eq!("]", autocomplete("[[]").expect("expected completion string!"));
    }

    #[test]
    fn complete() {
        assert_eq!(None, autocomplete("()"));
    }
}

#[cfg(test)]
mod test_autocomplete_score {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(294, autocomplete_score("])}>"));
    }

    #[test]
    fn b() {
        assert_eq!(288957, autocomplete_score("}}]])})]"));
    }

    #[test]
    fn c() {
        assert_eq!(5566, autocomplete_score(")}>]})"));
    }

    #[test]
    fn d() {
        assert_eq!(1480781, autocomplete_score("}}>}>))))"));
    }

    #[test]
    fn e() {
        assert_eq!(995444, autocomplete_score("]]}}]}]}>"));
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

fn autocomplete_score(s: &str) -> u64 {
    let mut score = 0;

    for c in s.chars() {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!()
        }
    }

    score
}

fn autocomplete(s: &str) -> Option<String> {
    let r = parse(s);

    // Get stack, or return none if the input is not incomplete.
    let stack = match r {
        Err(ChunkParseError::Incomplete(s)) => s,
        _ => return None
    };

    // We've got a stack - map each entry onto the expected
    // RHS character and reverse to form the completion string!
    return Some(stack.iter().map(|c| expected_rhs(*c)).rev().collect());
}

// Given a LHS character, returns the expected RHS character.
fn expected_rhs(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '<' => '>',
        '{' => '}',
        _ => unreachable!()
    }
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
                Some(e) => expected_rhs(e),
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
    if !stack.is_empty() {
        return Err(ChunkParseError::Incomplete(stack));
    }

    // We've not seen any errors, so it's a valid chunk!
    Ok(())
}

fn syntax_score(e: ChunkParseError) -> u32 {
    match e {
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

    score
}

fn part2() -> u64 {
    let input: Vec<String> = data::get("data/day10.txt");
    let mut scores = Vec::new();
    for i in input {
        let a = autocomplete(&i);
        if a.is_some() {
            scores.push(autocomplete_score(&a.unwrap()))
        }
    }

    // Sort scores, select the middle.
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    aoc::solution!(10, "syntax error score", "autocomplete score");
}
