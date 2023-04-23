#![allow(non_snake_case)]

type Func = &'static dyn Fn(&str) -> u32;

pub trait Challenge
where
    Self: 'static,
{
    fn part1(input: &str) -> u32;
    fn part2(input: &str) -> u32;

    fn name() -> String {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("Day?")
            .to_string()
    }

    fn input() -> String {
        match Self::name().chars().last().map(|x| x.to_digit(10)) {
            Some(Some(x)) => read_input_day(x),
            _ => "".to_string(),
        }
    }

    fn run() {
        let input = Self::input();

        for (i, func) in [&Self::part1 as Func, &Self::part2].into_iter().enumerate() {
            let time = std::time::Instant::now();
            let result = func(&input);
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
    Result::expect(
        std::fs::read_to_string(format!("src/day{num}/input")),
        &format!("Error reading `day{num}` file"),
    )
}

mod day1;
mod day2;
mod day3;
mod day4;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
