// Advent of Code 2021
// Day 1

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];

        assert_eq!(7, num_increases(&input));
    }
}

#[cfg(test)]
mod test_num_decreases {
    use super::*;

    #[test]
    fn all_increases() {
        let input = vec![
            1,
            2,
            3,
            4,
            5
        ];

        assert_eq!(4, num_increases(&input));
    }

    #[test]
    fn all_same() {
        let input = vec![
            101,
            101,
            101,
            101,
            101
        ];

        assert_eq!(0, num_increases(&input));
    }

    #[test]
    fn all_decreases() {
        let input = vec![
            101,
            100,
            98,
            90,
            40
        ];

        assert_eq!(0, num_increases(&input));
    }

    #[test]
    fn up_and_down() {
        let input = vec![
            101,
            105,
            100,
            101,
            101,
            99,
            100
        ];

        assert_eq!(3, num_increases(&input));
    }
}

fn num_increases(report: &[u32]) -> u32 {
    let mut previous = u32::MAX;
    let mut increases = 0;

    for depth in report {
        if depth > &previous { increases = increases + 1; }
        previous = *depth;
    }

    return increases;
}

fn main() {

}
