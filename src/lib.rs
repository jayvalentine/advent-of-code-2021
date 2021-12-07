pub mod data;

use std::time::Instant;
use std::fmt::Display;

pub fn part<T: Display>(s: &str, descr: &str, f: &dyn Fn() -> T) {
    let start = Instant::now();

    let result = f();

    println!("{:20} {:20} {:15} {:.4}", s, descr, result, start.elapsed().as_secs_f32());
}

#[macro_export]
macro_rules! solution {
    ($day:expr, $p1:expr, $p2:expr) => {
        {
            aoc::part(&format!("day{}:part1", $day), $p1, &part1);
            aoc::part(&format!("day{}:part2", $day), $p2, &part2);
        }
    };
}
