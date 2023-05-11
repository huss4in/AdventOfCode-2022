#![allow(non_snake_case)]
#![feature(iter_array_chunks)]

type Func = &'static dyn Fn(&str) -> String;

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
        let name = Self::name();

        let mut x = name.chars().rev().take(2).map(|x| x.to_digit(10));

        let day = format!(
            "day{:0>2}",
            match (x.next(), x.next()) {
                (Some(Some(x)), Some(None)) => x,
                (Some(Some(x)), Some(Some(y))) => y * 10 + x,
                _ => 0,
            }
        );

        Result::expect(
            std::fs::read_to_string(format!("src/{day}/input")),
            &format!("Error reading `{day}` file"),
        )
    }

    fn run() {
        let input = Self::input();

        for (i, func) in [&Self::part1 as Func, &Self::part2].into_iter().enumerate() {
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

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;
pub use day05::Day05;
pub use day06::Day06;
pub use day07::Day07;
pub use day08::Day08;
pub use day09::Day09;
pub use day10::Day10;
pub use day11::Day11;
pub use day12::Day12;
pub use day13::Day13;
pub use day14::Day14;
pub use day15::Day15;
pub use day16::Day16;
pub use day17::Day17;
pub use day18::Day18;
pub use day19::Day19;
pub use day20::Day20;
pub use day21::Day21;
pub use day22::Day22;
pub use day23::Day23;
pub use day24::Day24;
pub use day25::Day25;

pub const DAYS: &[fn()] = &[
    Day01::run,
    Day02::run,
    Day03::run,
    Day04::run,
    Day05::run,
    Day06::run,
    Day07::run,
    Day08::run,
    Day09::run,
    Day10::run,
    Day11::run,
    Day12::run,
    Day13::run,
    Day14::run,
    Day15::run,
    Day16::run,
    Day17::run,
    Day18::run,
    Day19::run,
    Day20::run,
    Day21::run,
    Day22::run,
    Day23::run,
    Day24::run,
    Day25::run,
];
