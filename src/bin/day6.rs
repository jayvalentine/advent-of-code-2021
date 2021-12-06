// Advent of Code 2021
// Day 6

use aoc::data;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(377263, super::part1());
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let lanternfish = vec![3, 4, 3, 1, 2];

        assert_eq!(26, lanternfish_pop(&lanternfish, 18));
        assert_eq!(5934, lanternfish_pop(&lanternfish, 80));
    }

    #[test]
    fn part2() {
        let lanternfish = vec![3, 4, 3, 1, 2];

        assert_eq!(26984457539, lanternfish_pop(&lanternfish, 256));
    }
}

#[cfg(test)]
mod test_lanternfish {
    use super::*;

    #[test]
    fn test_tick() {
        let mut l = Lanternfish { timer: 3 };
        let s = l.tick();

        assert_eq!(None, s);
        assert_eq!(2, l.timer);
    }

    #[test]
    fn test_tick_0() {
        let mut l = Lanternfish { timer: 0 };
        let s = l.tick();

        assert_eq!(Some(8), s);
        assert_eq!(6, l.timer);
    }

    #[test]
    fn test_tick_1() {
        let mut l = Lanternfish { timer: 1 };
        let s = l.tick();

        assert_eq!(None, s);
        assert_eq!(0, l.timer);
    }
}

#[cfg(test)]
mod test_tick {
    use super::*;

    #[test]
    fn next_pop_1() {
        let p = vec![0, 2, 0, 0, 0, 0, 0, 0, 0];
        let p2 = next_pop(p);
        assert_eq!(9, p2.len());
        assert_eq!(2, p2[0]);
        assert_eq!(0, p2[1]);
    }

    #[test]
    fn next_pop_0() {
        let p = vec![2, 0, 0, 0, 0, 0, 0, 0, 0];
        let p2 = next_pop(p);
        assert_eq!(9, p2.len());
        assert_eq!(0, p2[0]);
        assert_eq!(2, p2[6]);
        assert_eq!(2, p2[8]);
    }

    #[test]
    fn next_pop_0_1_4() {
        let p = vec![2, 3, 0, 0, 5, 0, 0, 0, 0];
        let p2 = next_pop(p);
        assert_eq!(9, p2.len());
        assert_eq!(3, p2[0]);
        assert_eq!(0, p2[1]);
        assert_eq!(0, p2[2]);
        assert_eq!(5, p2[3]);
        assert_eq!(0, p2[4]);
        assert_eq!(0, p2[5]);
        assert_eq!(2, p2[6]);
        assert_eq!(0, p2[7]);
        assert_eq!(2, p2[8]);
    }

    #[test]
    fn next_pop_all() {
        let p = vec![2, 3, 2, 1, 4, 5, 3, 2, 1];
        let p2 = next_pop(p);
        assert_eq!(9, p2.len());
        assert_eq!(3, p2[0]);
        assert_eq!(2, p2[1]);
        assert_eq!(1, p2[2]);
        assert_eq!(4, p2[3]);
        assert_eq!(5, p2[4]);
        assert_eq!(3, p2[5]);
        assert_eq!(4, p2[6]);
        assert_eq!(1, p2[7]);
        assert_eq!(2, p2[8]);
    }
}

struct Lanternfish {
    timer: u32
}

impl Lanternfish {
    fn tick(&mut self) -> Option<u32> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(8);
        }
        else {
            self.timer -= 1;
            return None;
        }
    }
}

fn next_pop(pop: Vec<u64>) -> Vec<u64> {
    // Sanity check - shouldn't ever change vector size.
    if pop.len() != 9 {
        panic!("Vector of wrong size!");
    }

    let mut new_pop = vec![0; 9];

    let reproducing = pop[0];

    for p in 0..8 {
        new_pop[p] = pop[p+1];
    }

    // Those that have reproduced tick over to 6,
    // and produce new fish with timer 8.
    new_pop[6] += reproducing;
    new_pop[8] += reproducing;

    return new_pop;
}

fn lanternfish_pop(initial_pop: &[u32], days: u32) -> usize {
    let mut population = vec![0; 9];
    for &p in initial_pop {
        population[p as usize] += 1;
    }

    for _ in 0..days {
        population = next_pop(population);
    }

    return population.iter().fold(0, |acc, x| acc + x) as usize;
}

fn part1() -> usize {
    let initial_pop = data::get("data/day6.txt");
    return lanternfish_pop(&initial_pop, 80);
}

fn part2() -> usize {
    let initial_pop = data::get("data/day6.txt");
    return lanternfish_pop(&initial_pop, 256);
}

fn main() {
    let final_pop = part1();
    println!("Final lanternfish population after 80 days is {}", final_pop);

    let final_pop = part2();
    println!("Final lanternfish population after 256 days is {}", final_pop);
}
