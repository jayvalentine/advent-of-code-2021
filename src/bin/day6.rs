// Advent of Code 2021
// Day 6

use aoc::data;

#[cfg(test)]
mod test_puzzles {
    #[test]
    fn part1() {
        assert_eq!(377263, super::part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1695929023803, super::part2());
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

fn lanternfish_pop(initial_pop: &[u32], days: u32) -> u64 {
    let mut population = vec![0; 9];
    for &p in initial_pop {
        population[p as usize] += 1;
    }

    for _ in 0..days {
        population = next_pop(population);
    }

    return population.iter().fold(0, |acc, x| acc + x);
}

fn get_data() -> Vec<u32> {
    let v: Vec<u32> = data::get_with_iter("data/day6.txt", &mut |iter|
        data::from_separated(iter.next().expect("Data parse error!"), ',').expect("Data parse error!")
    );

    return v;
}

fn part1() -> u64 {
    let initial_pop = get_data();
    return lanternfish_pop(&initial_pop, 80);
}

fn part2() -> u64 {
    let initial_pop = get_data();
    return lanternfish_pop(&initial_pop, 256);
}

fn main() {
    let final_pop = part1();
    println!("Final lanternfish population after 80 days is {}", final_pop);

    let final_pop = part2();
    println!("Final lanternfish population after 256 days is {}", final_pop);
}
