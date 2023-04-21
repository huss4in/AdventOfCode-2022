#![allow(non_snake_case)]

pub mod prelude {
    pub use crate::Day;

    pub use super::day1::Day1;
    pub use super::day2::Day2;
}

type FUNC = &'static dyn Fn(&str) -> u32;

pub trait Day
where
    Self: 'static,
{
    fn name() -> String;

    fn part1(input: &str) -> u32;
    fn part2(input: &str) -> u32;

    fn parts() -> [FUNC; 2] {
        [&Self::part1, &Self::part2]
    }

    fn input() -> &'static String;

    fn run_all(num: usize) {
        for (i, func) in
            Self::parts()
                .into_iter()
                .enumerate()
                .take(if num == 0 { std::usize::MAX } else { num })
        {
            let time = std::time::Instant::now();

            let result = func(Self::input());

            let elapsed = time.elapsed().as_micros();

            println!(
                "{}::part{}() -> {:<7} in {} ms\n",
                Self::name(),
                i + 1,
                result,
                elapsed as f64 / 1000.0,
            );
        }
    }
}

fn read_input_day(num: u32) -> String {
    std::fs::read_to_string(format!("src/day{num}/input"))
        .expect(&format!("Error reading `day{num}` file"))
}

mod day1;
mod day2;
