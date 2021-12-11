// Advent of Code
// Day 8

use std::collections::{HashSet, HashMap};
use std::{iter::FromIterator, slice::Iter};

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(375, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1019355, super::part2());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;
    use std::str::FromStr;

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

// Possible positions for a segment.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum SegmentPosition {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom
}

impl SegmentPosition {
    fn all() -> [SegmentPosition; 7] {
        return [
            SegmentPosition::Top,
            SegmentPosition::TopLeft,
            SegmentPosition::TopRight,
            SegmentPosition::Middle,
            SegmentPosition::BottomLeft,
            SegmentPosition::BottomRight,
            SegmentPosition::Bottom
        ];
    }
}

#[cfg(test)]
mod test_segment {
    use super::*;

    #[test]
    fn test_must() {
        let seg = Segment::new("abc");
        let seg2 = seg.must_be("ab");
        assert_eq!(2, seg2.len());
        assert_eq!(true, seg2.possibles.contains(&'a'));
        assert_eq!(true, seg2.possibles.contains(&'b'));
        assert_eq!(false, seg2.possibles.contains(&'c'));
    }

    #[test]
    fn test_cannot() {
        let seg = Segment::new("abc");
        let seg2 = seg.cannot_be("ab");
        assert_eq!(1, seg2.len());
        assert_eq!(false, seg2.possibles.contains(&'a'));
        assert_eq!(false, seg2.possibles.contains(&'b'));
        assert_eq!(true, seg2.possibles.contains(&'c'));
    }
}

#[derive(Clone)]
struct Segment {
    possibles: HashSet<char>
}

impl Segment {
    fn new(s: &str) -> Segment {
        return Segment { possibles: HashSet::from_iter(s.chars()) }
    }

    fn len(&self) -> usize {
        return self.possibles.len();
    }

    fn must_be(&self, s: &str) -> Segment {
        let iter = s.chars();
        let other = HashSet::from_iter(iter);
        let intersect = self.possibles.intersection(&other);
        return Segment { possibles: HashSet::from_iter(intersect.map(|c| *c)) };
    }

    fn cannot_be(&self, s: &str) -> Segment {
        let iter = s.chars();
        let other = HashSet::from_iter(iter);
        let diff = self.possibles.difference(&other);
        return Segment { possibles: HashSet::from_iter(diff.map(|c| *c)) }
    }
}

// A segment display, encoding the possible signals in each segment.
#[derive(Clone)]
struct SegmentDisplay {
    segments: HashMap<SegmentPosition, Segment>
}

impl SegmentDisplay {
    // Create a new SegmentDisplay where all signals are possible in all positions.
    fn init() -> SegmentDisplay {
        let signals = "abcdefg";
        let positions = SegmentPosition::all();

        let mut segments = HashMap::new();
        for &pos in positions.iter() {
            segments.insert(pos, Segment::new(signals));
        }
        
        return SegmentDisplay { segments }
    }

    // Excludes the possible signals given a number and its pattern.
    fn exclude(&self, pattern: &str) -> SegmentDisplay {
        let n = match pattern.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => 0
        };

        let mut new_segments = HashMap::new();

        let must_positions = if n == 1 {
            vec![SegmentPosition::TopRight, SegmentPosition::BottomRight]
        }
        else if n == 7 {
            vec![SegmentPosition::Top, SegmentPosition::TopRight, SegmentPosition::BottomRight]
        }
        else if n == 4 {
            vec![SegmentPosition::TopLeft, SegmentPosition::TopRight, SegmentPosition::Middle, SegmentPosition::BottomRight]
        }
        else {
            return self.clone();
        };

        for &pos in SegmentPosition::all().iter() {
            if must_positions.contains(&pos) {
                new_segments.insert(pos, self.segments[&pos].must_be(pattern));
            }
            else {
                new_segments.insert(pos, self.segments[&pos].cannot_be(pattern));
            }
        }

        return SegmentDisplay { segments: new_segments };
    }

    fn permutations(&self) -> Vec<SegmentDisplay> {
        // Find first segment with more than one possibility.
        let mut first_pos = None;
        let positions = SegmentPosition::all();
        for pos in positions.iter() {
            if self.segments[&pos].len() > 1 {
                first_pos = Some(pos);
                break;
            }
        }

        // Only one permutation - this one.
        // Otherwise recurse.
        if first_pos.is_none() {
            return vec![self.clone()];
        }
        else {
            let pos = first_pos.unwrap();
            let mut perms = Vec::new();

            // Create a new SegmentDisplay for each possibility of this segment.
            // Concatenate the permutations of that segment display onto this one.
            for c in &self.segments[&pos].possibles {
                let mut new = self.clone();
                new.segments.insert(*pos, self.segments[&pos].must_be(&format!("{}", c)));

                // All other positions cannot be this one.
                for other_pos in positions.iter() {
                    if other_pos != pos {
                        new.segments.insert(*other_pos, self.segments[&other_pos].cannot_be(&format!("{}", c)));
                    }
                }

                perms.append(&mut new.permutations());
            }

            return perms;
        }
    }

    fn valid_combinations(&self) -> Vec<HashSet<SegmentPosition>> {
        let mut v = Vec::new();

        // 0
        let mut h = HashSet::new();
        h.insert(SegmentPosition::Top);
        h.insert(SegmentPosition::TopLeft);
        h.insert(SegmentPosition::TopRight);
        h.insert(SegmentPosition::BottomLeft);
        h.insert(SegmentPosition::BottomRight);
        h.insert(SegmentPosition::Bottom);
        v.push(h);

        // 1
        let mut h = HashSet::new();
        h.insert(SegmentPosition::TopRight);
        h.insert(SegmentPosition::BottomRight);
        v.push(h);

        // 2
        let mut h = HashSet::new();
        h.insert(SegmentPosition::Top);
        h.insert(SegmentPosition::TopRight);
        h.insert(SegmentPosition::Middle);
        h.insert(SegmentPosition::BottomLeft);
        h.insert(SegmentPosition::Bottom);
        v.push(h);

        // 3
        let mut h = HashSet::new();
        h.insert(SegmentPosition::Top);
        h.insert(SegmentPosition::TopRight);
        h.insert(SegmentPosition::Middle);
        h.insert(SegmentPosition::BottomRight);
        h.insert(SegmentPosition::Bottom);
        v.push(h);

        // 4
        let mut h = HashSet::new();
        h.insert(SegmentPosition::TopLeft);
        h.insert(SegmentPosition::TopRight);
        h.insert(SegmentPosition::Middle);
        h.insert(SegmentPosition::BottomRight);
        v.push(h);

        // 5
        let mut h = HashSet::new();
        h.insert(SegmentPosition::Top);
        h.insert(SegmentPosition::TopLeft);
        h.insert(SegmentPosition::Middle);
        h.insert(SegmentPosition::BottomRight);
        h.insert(SegmentPosition::Bottom);
        v.push(h);

        // 6
        let mut h = HashSet::new();
        h.insert(SegmentPosition::Top);
        h.insert(SegmentPosition::TopLeft);
        h.insert(SegmentPosition::Middle);
        h.insert(SegmentPosition::BottomRight);
        h.insert(SegmentPosition::BottomLeft);
        h.insert(SegmentPosition::Bottom);
        v.push(h);

        // 7
        let mut h = HashSet::new();
        h.insert(SegmentPosition::Top);
        h.insert(SegmentPosition::TopRight);
        h.insert(SegmentPosition::BottomRight);
        v.push(h);

        // 8
        let mut h = HashSet::new();
        h.insert(SegmentPosition::Top);
        h.insert(SegmentPosition::TopLeft);
        h.insert(SegmentPosition::TopRight);
        h.insert(SegmentPosition::Middle);
        h.insert(SegmentPosition::BottomLeft);
        h.insert(SegmentPosition::BottomRight);
        h.insert(SegmentPosition::Bottom);
        v.push(h);

        // 9
        let mut h = HashSet::new();
        h.insert(SegmentPosition::Top);
        h.insert(SegmentPosition::TopLeft);
        h.insert(SegmentPosition::TopRight);
        h.insert(SegmentPosition::Middle);
        h.insert(SegmentPosition::BottomRight);
        h.insert(SegmentPosition::Bottom);
        v.push(h);

        return v;
    }

    fn digit(&self, pattern: &str) -> Option<u32> {
        // Get the lit segments.
        let mut lit = HashSet::new();
        for c in pattern.chars() {
            for (k, v) in &self.segments {
                if v.len() != 1 {
                    panic!("More than one possibility!");
                }
                let c_seg = v.possibles.iter().last().expect("No value!");
                if c == *c_seg {
                    lit.insert(*k);
                    break;
                }
            }
        }

        let valid_combos = self.valid_combinations();

        let mut val = 0;
        for comb in valid_combos {
            if comb == lit {
                return Some(val);
            }
            val += 1;
        }
        return None;
    }

    fn valid(&self, pattern: &str) -> bool {
        return self.digit(pattern).is_some();
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

fn get_segment_config(patterns: &[String]) -> SegmentDisplay {
    // First exclude down to the minimal.
    
    let mut seg = SegmentDisplay::init();

    for i in patterns {
        seg = seg.exclude(i);
    }

    for d in seg.permutations() {
        let mut found = true;
        for i in patterns {
            if !d.valid(i) {
                found = false;
            }
        }

        if found {
            return d;
        }
    }

    panic!("Didn't find a configuration!");
}

fn get_output(patterns: &[String], output_patterns: &[String]) -> u32 {
    // Get the segment display configuration.
    let seg = get_segment_config(patterns);

    // Get each output digit.
    let mut val = 0;
    for pattern in output_patterns {
        val = val * 10;
        val = val + seg.digit(pattern).expect("Didn't decode!");
    }

    return val;
}

fn get_segments(i: &mut Iter<&str>) -> Vec<(Vec<String>, Vec<String>)> {
    let mut v = Vec::new();

    for line in i {
        let mut iter = line.split(" | ");
        let patterns = iter.next().expect("Parse ereor!");
        let patterns = patterns.split_whitespace().map(|s| String::from(s)).collect();
        let output = iter.next().expect("Parse error!");
        let output = output.split_whitespace().map(|s| String::from(s)).collect();

        v.push((patterns, output));
    }

    return v;
}

fn get_data() -> Vec<(Vec<String>, Vec<String>)> {
    return aoc::data::get_with_iter("data/day8.txt", &mut get_segments);
}

fn part1() -> u32 {
    let input = get_data();
    let mut outputs = Vec::new();
    for mut i in input {
        outputs.append(&mut i.1);
    }

    return count_segments(&outputs, &[2, 4, 3, 7]);
}

fn part2() -> u32 {
    let input = get_data();
    let mut result = 0;
    for (patterns, output) in input {
        result += get_output(&patterns, &output);
    }

    return result;
}

fn main() {
    aoc::solution!(8, "count 1, 4, 7, 8", "sum of output values");
}
