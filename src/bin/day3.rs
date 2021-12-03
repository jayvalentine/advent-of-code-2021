// Advent of Code 2021
// Day 3

use aoc::data;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010
        ];

        let rates = find_rates(&input, 5);

        assert_eq!(22, rates.gamma);
        assert_eq!(9, rates.epsilon);
    }
}

#[cfg(test)]
mod test_find_rates {
    use super::*;

    #[test]
    fn all_ones() {
        let input = vec![
            0b111111111,
            0b111111111,
            0b111111111,
            0b111111111,
            0b111111111,
            0b111111111
        ];

        let rates = find_rates(&input, 9);

        assert_eq!(511, rates.gamma);
        assert_eq!(0, rates.epsilon);
    }

    #[test]
    fn all_zeros() {
        let input = vec![
            0b0000000,
            0b0000000,
            0b0000000
        ];

        let rates = find_rates(&input, 7);

        assert_eq!(0, rates.gamma);
        assert_eq!(127, rates.epsilon);
    }

    #[test]
    fn mixed() {
        let input = vec![
            0b000,
            0b111,
            0b001,
            0b101,
            0b110,
            0b000,
            0b101
        ];

        let rates = find_rates(&input, 3);

        assert_eq!(5, rates.gamma);
        assert_eq!(2, rates.epsilon);
    }
}

struct Rates {
    gamma: u32,
    epsilon: u32
}

fn find_rates(input: &[u32], width: usize) -> Rates {
    let mut count_ones: Vec<u32> = vec![0; width];
    let mut count_zeros: Vec<u32> = vec![0; width];

    // Find most common bits in each position.
    for i in input {
        let mut mask = 1;
        for w in 0..width {
            if i & mask == mask {
                count_ones[w] += 1;
            }
            else {
                count_zeros[w] += 1;
            }

            mask <<= 1;
        }
    }

    // Calculate gamma.
    let mut gamma = 0;
    for w in (0..width).rev() {
        gamma <<= 1;
        if count_ones[w] > count_zeros[w] {
            gamma |= 1;
        }
    }

    // Epsilon is just gamma but NOTed,
    // truncated to width.
    let epsilon = !gamma;

    // Generate a mask to truncate epsilon.
    let mut mask = 0;
    for _ in 0..width {
        mask <<= 1;
        mask |= 1;
    }

    let epsilon = epsilon & mask;

    return Rates { gamma, epsilon }
}

fn part1() -> Rates {
    let input = data::get_bin("data/day3.txt");
    return find_rates(&input, 12);
}

fn main() {
    let rates = part1();
    let power = rates.gamma * rates.epsilon;
    println!("Gamma: {}, Epsilon: {} (Power Consumption {})", rates.gamma, rates.epsilon, power);
}
