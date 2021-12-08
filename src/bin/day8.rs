// Advent of Code
// Day 8

use std::{iter::FromIterator, slice::Iter};
use std::str::FromStr;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            "fdgacbe cefdb cefbgd gcbe",
            "fcgedb cgb dgebacf gc",
            "cg cg fdcagb cbg",
            "efabcd cedba gadfec cb",
            "gecf egdcabf bgf bfgea",
            "gebdcfa ecba ca fadegcb",
            "cefg dcbef fcge gbcadfe",
            "ed bcgafe cdgba cbgef",
            "gbdfcae bgc cg cgb",
            "fgae cfgab fg bagce"
        ];

        let input: Vec<String> = input.iter().map(|s| String::from_str(s).expect("Parse error!")).collect();

        let count = count_segments(&input, &[2, 4, 3, 7]);
        assert_eq!(26, count);
    }
}

#[cfg(test)]
mod test_digit {
    use super::*;

    #[test]
    fn digit_unknown() {
        let input = "ceg";
        assert_eq!(None, digit(input));
    }

    #[test]
    fn digit_0_1() {
        let input = "abcefg";
        assert_eq!('0', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_0_2() {
        let input = "ebcagf";
        assert_eq!('0', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_1_1() {
        let input = "cf";
        assert_eq!('1', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_1_2() {
        let input = "fc";
        assert_eq!('1', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_2_1() {
        let input = "acdeg";
        assert_eq!('2', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_2_2() {
        let input = "aedcg";
        assert_eq!('2', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_2_3() {
        let input = "decag";
        assert_eq!('2', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_3_1() {
        let input = "acdfg";
        assert_eq!('3', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_3_2() {
        let input = "gdcaf";
        assert_eq!('3', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_4_1() {
        let input = "bcdf";
        assert_eq!('4', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_4_2() {
        let input = "cfdb";
        assert_eq!('4', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_5_1() {
        let input = "abdfg";
        assert_eq!('5', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_5_2() {
        let input = "gfdba";
        assert_eq!('5', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_6_1() {
        let input = "gfdbae";
        assert_eq!('6', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_6_2() {
        let input = "aedbgf";
        assert_eq!('6', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_7_1() {
        let input = "acf";
        assert_eq!('7', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_7_2() {
        let input = "afc";
        assert_eq!('7', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_8_1() {
        let input = "abcdefg";
        assert_eq!('8', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_8_2() {
        let input = "gfedabc";
        assert_eq!('8', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_9_1() {
        let input = "gfdabc";
        assert_eq!('9', digit(input).expect("Expected a valid digit!"))
    }

    #[test]
    fn digit_9_2() {
        let input = "adfgbc";
        assert_eq!('9', digit(input).expect("Expected a valid digit!"))
    }
}

fn digit(s: &str) -> Option<char> {
    // First sort the characters in the string so we can match
    // against just a single pattern.
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    let s = String::from_iter(chars);

    return match s.as_str() {
        "abcefg" => Some('0'),
        "cf" => Some('1'),
        "acdeg" => Some('2'),
        "acdfg" => Some('3'),
        "bcdf" => Some('4'),
        "abdfg" => Some('5'),
        "abdefg" => Some('6'),
        "acf" => Some('7'),
        "abcdefg" => Some('8'),
        "abcdfg" => Some('9'),
        _ => None
    }
}

fn count_segments(segments: &[String], num_segments: &[u32]) -> u32 {
    let mut count = 0;

    for line in segments {
        for seg in line.split_whitespace() {
            if num_segments.contains(&(seg.len() as u32)) { count += 1; }
        }
    }

    return count;
}

fn get_segments(i: &mut Iter<&str>) -> Vec<String> {
    let mut v = Vec::new();

    for line in i {
        let s = line.split(" | ").nth(1).expect("Parse error!");
        for seg in s.split_whitespace() {
            v.push(seg.to_owned());
        }
    }

    return v;
}

fn part1() -> u32 {
    let input: Vec<String> = aoc::data::get_with_iter("data/day8.txt", &mut get_segments);

    return count_segments(&input, &[2, 4, 3, 7]);
}

fn part2() -> u32 {
    return 0;
}

fn main() {
    aoc::solution!(8, "count 1, 4, 7, 8", "");
}
