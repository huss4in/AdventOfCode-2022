use crate::Challenge;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Day2;

impl Challenge for Day2 {
    /// What would your total score be if everything goes exactly according to your strategy guide?
    fn part1(input: &str) -> u32 {
        input
            .lines()
            .map(|line| line.split_whitespace().map(Game::parse_move))
            .map(|mut plays| match (plays.next(), plays.next()) {
                (Some(Ok(op)), Some(Ok(me))) => op.play(me),
                _ => 0,
            })
            .sum()
    }

    /// Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
    fn part2(input: &str) -> u32 {
        input
            .lines()
            .map(|line| line.split_whitespace())
            .map(|mut split| {
                let op = split.next().map(Game::parse_move);
                let me = split.next().map(Game::parse_outcome);

                match (op, me) {
                    (Some(Ok(op)), Some(Ok(me))) => op.play(me),
                    _ => 0,
                }
            })
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

enum Game {
    Move(Move),
    Outcome(Outcome),
}

impl Game {
    fn parse_move(s: &str) -> Result<Self, ()> {
        match s {
            "A" | "X" => Ok(Self::Move(Move::Rock)),
            "B" | "Y" => Ok(Self::Move(Move::Paper)),
            "C" | "Z" => Ok(Self::Move(Move::Scissors)),
            _ => Err(()),
        }
    }

    fn parse_outcome(s: &str) -> Result<Self, ()> {
        match s {
            "X" => Ok(Self::Outcome(Outcome::Lose)),
            "Y" => Ok(Self::Outcome(Outcome::Draw)),
            "Z" => Ok(Self::Outcome(Outcome::Win)),
            _ => Err(()),
        }
    }

    fn play(self, other: Self) -> u32 {
        match (self, other) {
            (Self::Move(me), Self::Move(op)) => me.play_move(op),
            (Self::Move(me), Self::Outcome(op)) => me.play_outcome(op),
            _ => 0,
        }
    }
}

impl Move {
    fn play_move(self, other: Move) -> u32 {
        other as u32
            + match match (&other, &self) {
                (Move::Scissors, Move::Rock) => Ordering::Less,
                (Move::Rock, Move::Scissors) => Ordering::Greater,
                (&x, &y) => (x as u8).cmp(&(y as u8)),
            } {
                Ordering::Greater => Outcome::Win,
                Ordering::Equal => Outcome::Draw,
                Ordering::Less => Outcome::Lose,
            } as u32
    }

    fn play_outcome(self, outcome: Outcome) -> u32 {
        self.play_move(match outcome {
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
        })
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
