// Advent of Code 2021
// Day 6

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let lanternfish = vec![3, 4, 3, 1, 2];

        assert_eq!(26, lanternfish_pop(&lanternfish, 18));
        assert_eq!(5934, lanternfish_pop(&lanternfish, 80));
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

fn lanternfish_pop(initial_pop: &[u32], days: u32) -> usize {
    let mut lanternfish: Vec<Lanternfish> = Vec::new();
    for n in initial_pop {
        lanternfish.push(Lanternfish { timer: *n });
    }

    for _ in 0..days {
        let mut new_lanternfish = Vec::new();
        for l in &mut lanternfish {
            let s = l.tick();
            if s.is_some() {
                new_lanternfish.push(s.unwrap());
            }
        }

        for n in new_lanternfish {
            lanternfish.push(Lanternfish { timer: n })
        }
    }

    return lanternfish.len();
}

fn main() {

}