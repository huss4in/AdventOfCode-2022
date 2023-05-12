#![allow(non_snake_case)]
#![feature(iter_array_chunks)]

use itertools::Itertools;

pub trait Challenge
where
    Self: 'static,
{
    fn part1(input: &str) -> String;
    fn part2(input: &str) -> String;

    fn name() -> String {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("Day?")
            .to_string()
    }

    fn input() -> String {
        let day = std::any::type_name::<Self>()
            .split("::")
            .skip(1)
            .take(2)
            .join("/");

        std::fs::read_to_string(format!("src/{day}/input"))
            .expect(&format!("Error reading `{day}` file"))
    }

    fn run() {
        let input = Self::input();

        for (i, func) in [
            &Self::part1 as &'static dyn Fn(&str) -> String,
            &Self::part2,
        ]
        .into_iter()
        .enumerate()
        {
            let time = std::time::Instant::now();
            let result = func(&input);
            let elapsed = time.elapsed().as_micros();

            println!(
                "{}::part{}() -> {:<11} in {} ms",
                Self::name(),
                i + 1,
                format!("'{}'", result),
                elapsed as f64 / 1000.0,
            );
        }
    }
}

pub mod y2015;
pub mod y2016;
pub mod y2017;
pub mod y2018;
pub mod y2019;
pub mod y2020;
pub mod y2021;
pub mod y2022;
