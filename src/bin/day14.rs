// Advent of Code 2021
// Day 14

use std::{collections::HashMap, slice::Iter};

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(2112, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(3243771149914, super::part2());
    }
}

#[cfg(test)]
mod test_examples {

    use super::*;

    #[test]
    fn part1() {
        let template = "NNCB";
        let rules_str = vec![
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C"
        ];

        let mut rules = HashMap::new();
        for r in rules_str {
            let mut r_iter = r.split(" -> ");
            let pair = r_iter.next().expect("pair parse error!");
            let insert = r_iter.next().expect("insert parse error!").chars().nth(0).unwrap();

            rules.insert(String::from(pair), insert);
        }

        let (pairs, counts) = pair_insertion(to_pairs(template), to_counts(template), &rules);
        assert_eq!(1, pairs["NC"]);
        assert_eq!(1, pairs["NB"]);
        assert_eq!(1, pairs["CN"]);
        assert_eq!(1, pairs["BC"]);
        assert_eq!(1, pairs["CH"]);
        assert_eq!(1, pairs["HB"]);

        let (pairs, _) = pair_insertion(pairs, counts, &rules);
        assert_eq!(2, pairs["NB"]);
        assert_eq!(2, pairs["BC"]);
        assert_eq!(1, pairs["CC"]);
        assert_eq!(1, pairs["CN"]);
        assert_eq!(2, pairs["BB"]);
        assert_eq!(2, pairs["CB"]);
        assert_eq!(1, pairs["BH"]);
        assert_eq!(1, pairs["HC"]);
    }
}

#[cfg(test)]
mod test_insertion {
    use super::*;

    #[test]
    fn insert_one() {
        let mut rules = HashMap::new();
        rules.insert(String::from("AB"), 'C');

        let (pairs, counts) = pair_insertion(to_pairs("AB"), to_counts("AB"), &rules);
        assert_eq!(1, pairs["AC"]);
        assert_eq!(1, pairs["CB"]);

        assert_eq!(1, counts[&'A']);
        assert_eq!(1, counts[&'C']);
        assert_eq!(1, counts[&'B']);
    }

    #[test]
    fn insert_two() {
        let mut rules = HashMap::new();
        rules.insert(String::from("AB"), 'C');
        rules.insert(String::from("BA"), 'D'    );

        let (pairs, counts) = pair_insertion(to_pairs("ABA"), to_counts("ABA"), &rules);
        assert_eq!(1, pairs["AC"]);
        assert_eq!(1, pairs["CB"]);
        assert_eq!(1, pairs["BD"]);
        assert_eq!(1, pairs["DA"]);

        assert_eq!(1, counts[&'C']);
        assert_eq!(2, counts[&'A']);
        assert_eq!(1, counts[&'B']);
        assert_eq!(1, counts[&'D']);
    }
}

fn to_pairs(input: &str) -> HashMap<String, u64> {
    let mut pairs = HashMap::new();

    for i in 0..(input.len()-1) {
        let s = String::from(&input[i..i+2]);
        if !pairs.contains_key(&s) {
            pairs.insert(s, 0);
        }

        *pairs.get_mut(&input[i..i+2]).unwrap() += 1;
    }

    return pairs;
}

fn to_counts(input: &str) -> HashMap<char, u64> {
    let mut counts = HashMap::new();

    // Set initial counts of characters.
    for c in input.chars() {
        if !counts.contains_key(&c) {
            counts.insert(c, 0);
        }
        *counts.get_mut(&c).unwrap() += 1;
    }

    return counts;
}

// Given a string and a set of insertion rules, return the string resulting
// from applying those rules to the input.
fn pair_insertion(pairs: HashMap<String, u64>, mut counts: HashMap<char, u64>, rules: &HashMap<String, char>) -> (HashMap<String, u64>, HashMap<char, u64>) {
    let mut new_pairs = HashMap::new();

    for (pair, count) in pairs {
        let left = pair.chars().nth(0).unwrap();
        let right = pair.chars().nth(1).unwrap();

        let insert = *rules.get(&pair).unwrap();
        if !counts.contains_key(&insert) {
            counts.insert(insert, 0);
        }

        *counts.get_mut(&insert).unwrap() += count;

        // There must be a better way to do this...
        let left_pair = format!("{}{}", left, insert);
        let right_pair = format!("{}{}", insert, right);

        if !new_pairs.contains_key(&left_pair) {
            new_pairs.insert(left_pair.clone(), 0);
        }
        if !new_pairs.contains_key(&right_pair) {
            new_pairs.insert(right_pair.clone(), 0);
        }

        *new_pairs.get_mut(&left_pair).unwrap() += count;
        *new_pairs.get_mut(&right_pair).unwrap() += count;
    }

    return (new_pairs, counts);
}

fn get_data(iter: &mut Iter<&str>) -> (String, HashMap<String, char>) {
    let input = *iter.next().expect("error parsing input!");
    let input = String::from(input);
    iter.next();

    let mut rules = HashMap::new();

    while let Some(s) = iter.next() {
        let mut split = s.split(" -> ");
        let pair = split.next().expect("error parsing pair!");
        let insert = split.next().expect("error parsing insert char!").chars().nth(0).unwrap();

        rules.insert(String::from(pair), insert);
    }

    (input, rules)
}

fn result_after_n(n: i64) -> u64 {
    let (input, rules) = aoc::data::get_with_iter("data/day14.txt", &mut get_data);

    let mut pairs = to_pairs(&input);
    let mut counts = to_counts(&input); 

    for _ in 0..n {
        let result = pair_insertion(pairs, counts, &rules);
        pairs = result.0;
        counts = result.1;
    }

    // Find the most common and the least common.
    let most = counts.values().max().unwrap();
    let least = counts.values().min().unwrap();

    return most - least;
}

fn part1() -> u64 {
    return result_after_n(10);
}

fn part2() -> u64 {
    return result_after_n(40);
}

fn main() {
    aoc::solution!(14, "#most - #least (n=10)", "#most - #least (n=40)");
}

