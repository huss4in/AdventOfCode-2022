pub use crate::Day;

#[derive(Debug)]
pub struct Day2;

impl Day for Day2 {
    fn part1(input: &str) -> u32 {
        Self::common(input, |split| {
            let mut plays = split.map(|s| s.parse::<Play>());

            match (plays.next(), plays.next()) {
                (Some(Ok(op)), Some(Ok(me))) => op.play(&me),
                _ => 0,
            }
        })
    }

    fn part2(input: &str) -> u32 {
        Self::common(input, |mut split| {
            let op = split.next().map(|s| s.parse::<Play>());
            let me = split.next().map(|s| s.parse::<Outcome>());

            match (op, me) {
                (Some(Ok(op)), Some(Ok(me))) => op.play_outcome(&me),
                _ => 0,
            }
        })
    }

    fn name() -> String {
        format!("{Self:?}")
    }

    fn input() -> &'static String {
        lazy_static::lazy_static! {
            pub static ref INPUT: String = crate::read_input_day(2);
        }

        &INPUT
    }
}

impl Day2 {
    fn common(input: &str, solution: impl Fn(SplitWhitespace) -> u32) -> u32 {
        input
            .lines()
            .map(|line| line.trim().split_whitespace())
            .map(solution)
            .sum()
    }
}

use std::{cmp::Ordering, str::SplitWhitespace};

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl std::str::FromStr for Play {
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

impl<'a> std::str::FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(s.into()),
        }
    }
}

impl Play {
    fn play(&self, other: &Self) -> u32 {
        self.cmp(other) as u32 + *other as u32
    }

    fn cmp(&self, other: &Self) -> Outcome {
        match match (other, self) {
            (Play::Scissors, Play::Rock) => Ordering::Less,
            (Play::Rock, Play::Scissors) => Ordering::Greater,
            (&x, &y) => (x as u8).cmp(&(y as u8)),
        } {
            Ordering::Greater => Outcome::Win,
            Ordering::Equal => Outcome::Draw,
            Ordering::Less => Outcome::Lose,
        }
    }

    fn play_outcome(&self, outcome: &Outcome) -> u32 {
        self.play(&self.from_outcome(outcome))
    }

    fn from_outcome(&self, outcome: &Outcome) -> Self {
        use Play::{Paper, Rock, Scissors};

        match outcome {
            Outcome::Win => match self {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Outcome::Draw => *self,
            Outcome::Lose => match self {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
        }
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
    fn test_part2() {
        assert_eq!(Day2::part2(INPUT), 12);
    }
}
