// Advent of Code 2021
// Day 3

use aoc::data;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(1816*2279, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(2031*2104, super::part2());
    }
}

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

    #[test]
    fn part2() {
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

        let rates = find_rates_life_support(&input, 5);

        assert_eq!(23, rates.oxygen);
        assert_eq!(10, rates.co2);
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

struct LifeSupportRates {
    oxygen: u32,
    co2: u32
}

fn most_common(input: &[u32], width: usize) -> (Vec<u32>, Vec<u32>) {
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

    (count_ones, count_zeros)
}

fn find_rates(input: &[u32], width: usize) -> Rates {
    let (count_ones, count_zeros) = most_common(input, width);

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

    Rates { gamma, epsilon }
}

fn find_rates_life_support(input: &[u32], width: usize) -> LifeSupportRates {
    let oxygen = find_rates_oxygen(input, width);
    let co2 = find_rates_co2(input, width);

    LifeSupportRates { oxygen, co2 }
}

fn find_rates_with_closure(input: &[u32], width: usize, f: &dyn Fn(u32, u32) -> bool) -> u32 {
    let (count_ones, count_zeros) = most_common(input, width);

    let mut selected = Vec::new();

    let w = width - 1;

    for i in input {
        if f(count_ones[w], count_zeros[w]) {
            let mask = 1 << w;
            if i & mask == mask {
                selected.push(*i);
            }
        }
        else {
            let mask = !(1 << w);
            if i | mask == mask {
                selected.push(*i)
            }
        }
    }

    if selected.len() == 1 {
        return selected[0];
    }

    find_rates_with_closure(&selected, width-1, f)
}

fn find_rates_oxygen(input: &[u32], width: usize) -> u32 {
    find_rates_with_closure(input, width, &|x, y| x >= y)
}

fn find_rates_co2(input: &[u32], width: usize) -> u32 {
    find_rates_with_closure(input, width, &|x, y| x < y)
}

fn part1() -> u32 {
    let input = data::get_with("data/day3.txt", &|s| u32::from_str_radix(s, 2)).expect("Error parsing input!");
    let rates = find_rates(&input, 12);
    rates.epsilon * rates.gamma
}

fn part2() -> u32 {
    let input = data::get_with("data/day3.txt", &|s| u32::from_str_radix(s, 2)).expect("Error parsing input!");
    let rates = find_rates_life_support(&input, 12);
    rates.oxygen * rates.co2
}

fn main() {
    aoc::solution!(3, "power consumption", "life support");
}
