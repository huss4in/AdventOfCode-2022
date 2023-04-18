#![allow(non_snake_case)]

pub mod prelude {
    pub use crate::Challenge;

    pub use super::day1::Day1;
    pub use super::day2::Day2;
}

pub trait Challenge {
    type FUNC;

    fn parts() -> Vec<Self::FUNC>;
    fn input() -> &'static String;

    fn run(func: Self::FUNC) -> String;
    fn name() -> String;

    fn run_all(num: usize) {
        for (i, func) in
            Self::parts()
                .into_iter()
                .enumerate()
                .take(if num == 0 { std::usize::MAX } else { num })
        {
            let time = std::time::Instant::now();

            let result = Self::run(func);

            let elapsed = time.elapsed().as_nanos();

            println!(
                "{}::part{}() -> {} in {elapsed:?} ns\n",
                Self::name(),
                i + 1,
                result
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
