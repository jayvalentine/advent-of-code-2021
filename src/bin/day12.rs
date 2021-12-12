use std::str::FromStr;

// Advent of Code
// Day 11

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

        let paths = find_paths(&caves);

        assert_eq!(10, paths.len());
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

        let paths = find_paths(&caves);

        assert_eq!(256, paths.len());
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

#[derive(PartialEq, Eq, Debug)]
enum CaveType {
    Start,
    End,
    Big,
    Small
}

#[derive(PartialEq, Eq, Debug)]
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

}

#[derive(Debug)]
enum CavePairParseError {
    StartInvalid(CaveParseError),
    EndInvalid(CaveParseError),
}

impl FromStr for CavePair {
    type Err = CavePairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Err(CavePairParseError::StartInvalid(CaveParseError::Ambiguous))
    }
}

struct CaveGraph {

}

impl CaveGraph {
    fn new(pairs: &[CavePair]) -> CaveGraph {
        CaveGraph {}
    }
}

fn find_paths(caves: &CaveGraph) -> Vec<Vec<&Cave>> {
    return Vec::new();
}

fn part1() -> u32 {
    return 0;
}

fn part2() -> u32 {
    return 0;
}

fn main() {
    aoc::solution!(12, "# paths", "");
}
