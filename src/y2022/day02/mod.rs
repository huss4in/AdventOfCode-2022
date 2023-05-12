use crate::Challenge;
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
pub struct Day02;

impl Challenge for Day02 {
    /// What would your total score be if everything goes exactly according to your strategy guide?
    fn part1(input: &str) -> String {
        Self::common(input, |_, me| me).to_string()
    }

    /// Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
    fn part2(input: &str) -> String {
        Self::common(input, |op, me| match me {
            Move::Rock => match op {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Move::Paper => op,
            Move::Scissors => match op {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
        })
        .to_string()
    }
}

impl Day02 {
    fn common(input: &str, parse: impl Fn(Move, Move) -> Move) -> usize {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace().map(|char| char.parse());
                match (parts.next(), parts.next()) {
                    (Some(Ok(op)), Some(Ok(me))) => {
                        let me = parse(op, me);
                        match match (op, me) {
                            (Move::Scissors, Move::Rock) => Ordering::Greater,
                            (Move::Rock, Move::Scissors) => Ordering::Less,
                            (x, y) => (y as u8).cmp(&(x as u8)),
                        } {
                            Ordering::Greater => Outcome::Win as usize + me as usize,
                            Ordering::Equal => Outcome::Draw as usize + me as usize,
                            Ordering::Less => Outcome::Lose as usize + me as usize,
                        }
                    }
                    _ => 0,
                }
            })
            .sum()
    }
}

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(Move::Rock),
            "Y" | "B" => Ok(Move::Paper),
            "Z" | "C" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part1() {
        assert_eq!(Day02::part1(INPUT), "15");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02::part2(INPUT), "12");
    }
}
