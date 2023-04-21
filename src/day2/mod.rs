use crate::Challenge;
use std::{
    cmp::Ordering,
    str::{FromStr, SplitWhitespace},
};

#[derive(Debug)]
pub struct Day2;

impl Challenge for Day2 {
    /// What would your total score be if everything goes exactly according to your strategy guide?
    fn part1(input: &str) -> u32 {
        Self::common(input, |split| {
            let mut plays = split.map(|s| s.parse::<Move>());

            match (plays.next(), plays.next()) {
                (Some(Ok(op)), Some(Ok(me))) => op.play(me),
                _ => 0,
            }
        })
    }

    /// Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
    fn part2(input: &str) -> u32 {
        Self::common(input, |mut split| {
            let op = split.next().map(|s| s.parse::<Move>());
            let me = split.next().map(|s| s.parse::<Outcome>());

            match (op, me) {
                (Some(Ok(op)), Some(Ok(me))) => op.play_outcome(me),
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

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Move {
    fn play(self, other: Self) -> u32 {
        self.cmp(&other) as u32 + other as u32
    }

    fn cmp(&self, other: &Self) -> Outcome {
        match match (other, self) {
            (Move::Scissors, Move::Rock) => Ordering::Less,
            (Move::Rock, Move::Scissors) => Ordering::Greater,
            (&x, &y) => (x as u8).cmp(&(y as u8)),
        } {
            Ordering::Greater => Outcome::Win,
            Ordering::Equal => Outcome::Draw,
            Ordering::Less => Outcome::Lose,
        }
    }

    fn play_outcome(self, outcome: Outcome) -> u32 {
        self.play(self.from_outcome(outcome))
    }

    fn from_outcome(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => match self {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Outcome::Draw => self,
            Outcome::Lose => match self {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
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
