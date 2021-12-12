// Advent of Code
// Day 12

use std::str::FromStr;


use aoc::data;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(3450, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(96528, super::part2());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1_1() {
        let input = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ];

        let input: Vec<CavePair> = input.iter().map(|i| CavePair::from_str(i).expect("Parse error!")).collect();
        let caves = CaveGraph::new(&input);

        let paths = find_paths(&caves, caves.start(), Vec::new());

        assert_eq!(10, paths.expect("expected paths").len());
    }

    #[test]
    fn part1_2() {
        let input = vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc",
        ];

        let input: Vec<CavePair> = input.iter().map(|i| CavePair::from_str(i).expect("Parse error!")).collect();
        let caves = CaveGraph::new(&input);

        let paths = find_paths(&caves, caves.start(), Vec::new());

        assert_eq!(19, paths.expect("expected paths").len());
    }

    #[test]
    fn part2_1() {
        let input = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ];

        let input: Vec<CavePair> = input.iter().map(|i| CavePair::from_str(i).expect("Parse error!")).collect();
        let caves = CaveGraph::new(&input);

        let paths = find_paths_visit_twice(&caves, caves.start(), Vec::new(), vec![0; caves.num_caves()]).expect("expected paths!");

        assert_eq!(36, paths.len());
    }

    #[test]
    fn part2_2() {
        let input = vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc",
        ];

        let input: Vec<CavePair> = input.iter().map(|i| CavePair::from_str(i).expect("Parse error!")).collect();
        let caves = CaveGraph::new(&input);

        let paths = find_paths_visit_twice(&caves, caves.start(), Vec::new(), vec![0; caves.num_caves()]);

        assert_eq!(103, paths.expect("expected paths").len());
    }
}

#[cfg(test)]
mod test_parse_cave {
    use super::*;

    #[test]
    fn start() {
        let c = Cave::from_str("start").expect("parsing failed");
        assert_eq!("start", c.name);
        assert_eq!(CaveType::Start, c.cavetype);
    }

    #[test]
    fn end() {
        let c = Cave::from_str("end").expect("parsing failed");
        assert_eq!("end", c.name);
        assert_eq!(CaveType::End, c.cavetype);
    }

    #[test]
    fn big() {
        let c = Cave::from_str("ABC").expect("parsing failed");
        assert_eq!("ABC", c.name);
        assert_eq!(CaveType::Big, c.cavetype);
    }

    #[test]
    fn small() {
        let c = Cave::from_str("fyi").expect("parsing failed");
        assert_eq!("fyi", c.name);
        assert_eq!(CaveType::Small, c.cavetype);
    }

    #[test]
    fn ambiguous() {
        let c = Cave::from_str("fYi");
        assert_eq!(Err(CaveParseError::Ambiguous), c);
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum CaveType {
    Start,
    End,
    Big,
    Small
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Cave {
    name: String,
    cavetype: CaveType,
}

#[derive(Debug, PartialEq, Eq)]
enum CaveParseError {
    Ambiguous
}

impl FromStr for Cave {
    type Err = CaveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = String::from(s);
        return if s == "start" {
            Ok(Cave { name, cavetype: CaveType::Start })
        }
        else if s == "end" {
            Ok(Cave { name, cavetype: CaveType::End })
        }
        else if s.chars().all(|c| c.is_ascii_lowercase()) {
            Ok(Cave { name, cavetype: CaveType::Small })
        }
        else if s.chars().all(|c| c.is_ascii_uppercase()) {
            Ok(Cave { name, cavetype: CaveType::Big })
        }
        else {
            Err(CaveParseError::Ambiguous)
        }
    }
}

struct CavePair {
    a: Cave,
    b: Cave
}

#[derive(Debug)]
enum CavePairParseError {
    StartInvalid(CaveParseError),
    EndInvalid(CaveParseError),
    Incomplete
}

impl FromStr for CavePair {
    type Err = CavePairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('-');
        let a = s.next().ok_or(CavePairParseError::Incomplete)?;
        let b = s.next().ok_or(CavePairParseError::Incomplete)?;

        let a = match Cave::from_str(a) {
            Ok(c) => c,
            Err(e) => return Err(CavePairParseError::StartInvalid(e))
        };

        let b = match Cave::from_str(b) {
            Ok(c) => c,
            Err(e) => return Err(CavePairParseError::EndInvalid(e))
        };

        Ok(CavePair { a, b })
    }
}

struct CaveGraph {
    caves: Vec<Cave>,
    edges: Vec<(usize, usize)>
}

impl CaveGraph {
    fn new(pairs: &[CavePair]) -> CaveGraph {
        // First get the set of caves.
        let mut caves = Vec::new();
        for pair in pairs {
            if !caves.contains(&pair.a) { caves.push(pair.a.clone() )}
            if !caves.contains(&pair.b) { caves.push(pair.b.clone() )}
        }

        // Now construct set of edges.
        let mut edges = Vec::new();
        for pair in pairs {
            // Get index in caves of each end.
            let a_index = caves.iter().position(|c| c.eq(&pair.a)).unwrap();
            let b_index = caves.iter().position(|c| c.eq(&pair.b)).unwrap();

            // Add two edges.
            edges.push((a_index, b_index));
            edges.push((b_index, a_index));
        }

        CaveGraph { caves, edges }
    }

    fn start(&self) -> usize {
        return self.caves.iter().position(|c| c.cavetype == CaveType::Start ).unwrap();
    }

    fn cave(&self, i: usize) -> &Cave {
        &self.caves[i]
    }

    fn num_caves(&self) -> usize {
        self.caves.len()
    }

    fn edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }
}

fn find_paths(caves: &CaveGraph, c: usize, mut visited: Vec<usize>) -> Option<Vec<Vec<usize>>> {
    let cave = caves.cave(c);

    if (cave.cavetype == CaveType::Small || cave.cavetype == CaveType::Start) && visited.contains(&c) {
        return None;
    }

    visited.push(c);
    if cave.cavetype == CaveType::End {
        return Some(vec![visited]);
    }

    let mut paths = Vec::new();
    for edge in caves.edges() {
        if edge.0 != c { continue; }
        let conn = edge.1;

        if let Some(mut found_paths) = find_paths(caves, conn, visited.clone()) {
            paths.append(&mut found_paths);
        }
    }
    
    Some(paths)
}

fn find_paths_visit_twice(caves: &CaveGraph, c: usize, mut visited: Vec<usize>, mut counts: Vec<u32>) -> Option<Vec<Vec<usize>>> {
    let cave = caves.cave(c);

    if cave.cavetype == CaveType::Start && visited.contains(&c) {
        return None;
    }

    if cave.cavetype == CaveType::Small {
        let visited_count = counts[c];
        
        if visited_count == 2 {
            return None;
        }
        
        if visited_count == 1 && counts.iter().any(|c| *c > 1) {
            return None;
        }

        counts[c] += 1;
    }
    
    visited.push(c);
    if cave.cavetype == CaveType::End {
        return Some(vec![visited]);
    }

    let mut paths = Vec::new();
    for edge in caves.edges() {
        if edge.0 != c { continue; }
        let conn = edge.1;
        if let Some(mut found_paths) = find_paths_visit_twice(caves, conn, visited.clone(), counts.clone()) {
            paths.append(&mut found_paths);
        }
    }
    
    Some(paths)
}

fn part1() -> u32 {
    let pairs = data::get("data/day12.txt");
    let caves = CaveGraph::new(&pairs);

    let paths = find_paths(&caves, caves.start(), Vec::new()).expect("expected at least one path!");
    paths.len() as u32
}

fn part2() -> u32 {
    let pairs = data::get("data/day12.txt");
    let caves = CaveGraph::new(&pairs);

    let paths = find_paths_visit_twice(&caves, caves.start(), Vec::new(), vec![0; caves.num_caves()]).expect("expected at least one path!");
    paths.len() as u32
}

fn main() {
    aoc::solution!(12, "# paths", "# paths (new rules)");
}
