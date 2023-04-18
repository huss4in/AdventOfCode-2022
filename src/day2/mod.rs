#![allow(unused_imports, dead_code, unused_variables)]

use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug)]
pub struct Day2;

impl Day2 {
    pub fn part1(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let mut chars = line.split_whitespace().map(|s| s.parse::<Game>());

                match (chars.next(), chars.next()) {
                    (Some(Ok(op)), Some(Ok(me))) => {
                        me as u32
                            + match match (me, op) {
                                (Game::Scissors, Game::Rock) => Ordering::Less,
                                (Game::Rock, Game::Scissors) => Ordering::Greater,
                                (x, y) => (x as u8).cmp(&(y as u8)),
                            } {
                                Ordering::Less => 0,
                                Ordering::Equal => 3,
                                Ordering::Greater => 6,
                            }
                    }
                    _ => 0,
                }
            })
            .sum()
    }

    pub fn part2(input: &str) -> u32 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Game {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(s.into()),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref INPUT: String = crate::read_input_day(2);
}

impl crate::Challenge for Day2 {
    type FUNC = &'static dyn Fn(&str) -> u32;

    fn parts() -> Vec<Self::FUNC> {
        vec![&Self::part1, &Self::part2]
    }

    fn input() -> &'static String {
        &INPUT
    }

    fn run(func: Self::FUNC) -> String {
        format!("{:?}", func(Self::input()))
    }

    fn name() -> String {
        format!("{Self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part1() {
        assert_eq!(Day2::part1(INPUT), 15);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(Day2::part1("A Y\nB X\nC Z\nA X"), 19);
        assert_eq!(Day2::part1("A Y\nB X\nC Z\nA X\nB Y"), 24);
        assert_eq!(Day2::part1("A Y\nB X\nC Z\nA X\nB Y\nC Y"), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day2::part2(INPUT), 0);
    }
}
