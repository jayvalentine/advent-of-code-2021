// Advent of Code 2021
// Day 14

use std::collections::HashMap;

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

        let polymer = pair_insertion(template.to_owned(), &rules);
        assert_eq!("NCNBCHB", polymer);

        let polymer = pair_insertion(polymer, &rules);
        assert_eq!("NBCCNBBBCBHCB", polymer);

        let polymer = pair_insertion(polymer, &rules);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", polymer);

        let polymer = pair_insertion(polymer, &rules);
        assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB", polymer);
    }
}

#[cfg(test)]
mod test_insertion {
    use super::*;

    #[test]
    fn insert_one() {
        let mut rules = HashMap::new();
        rules.insert(String::from("AB"), 'C');

        let polymer = pair_insertion("AB".to_owned(), &rules);
        assert_eq!("ACB", polymer);
    }

    #[test]
    fn insert_two() {
        let mut rules = HashMap::new();
        rules.insert(String::from("AB"), 'C');
        rules.insert(String::from("BA"), 'D');

        let polymer = pair_insertion("ABA".to_owned(), &rules);
        assert_eq!("ACBDA", polymer);
    }
}

// Given a string and a set of insertion rules, return the string resulting
// from applying those rules to the input.
fn pair_insertion(input: String, rules: &HashMap<String, char>) -> String {
    let mut string = String::new();

    
    for i in 0..(input.len()-1) {
        let s = &input[i..i+2];
        let left = s.chars().nth(0).unwrap();

        let insert = *rules.get(s).unwrap();

        string.push(left);
        string.push(insert);
        
        // Don't push the RHS of the triple,
        // it will be handled by the next iteration,
        // or when we leave the loop.
    }

    // Push final character.
    string.push(input.chars().last().unwrap());

    return string;
}

fn part1() -> u32 {
    return 0;
}

fn part2() -> u32 {
    return 0;
}

fn main() {
    aoc::solution!(14, "#most - #least", "");
}

