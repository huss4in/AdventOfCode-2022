use crate::Challenge;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Day05;

impl Challenge for Day05 {
    /// After the rearrangement procedure completes, what crate ends up on top of each stack?
    fn part1(input: &str) -> String {
        Self::common(input, |stack, len| stack.drain(len..).rev().collect())
    }

    /// After the rearrangement procedure completes, what crate ends up on top of each stack?
    fn part2<'a>(input: &str) -> String {
        Self::common(input, |stack, len| stack.drain(len..).collect())
    }
}

#[derive(Debug)]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

impl Day05 {
    fn common<'a>(input: &'a str, drained: impl Fn(&mut Vec<char>, usize) -> Vec<char>) -> String {
        let (_, (mut stacks, moves)) = Self::stacks(input).unwrap_or_default();

        for Move { number, from, to } in moves.into_iter() {
            let len = stacks[from].len() - number;

            for c in drained(&mut stacks[from], len) {
                stacks[to].push(c);
            }
        }

        stacks.into_iter().map(|v| v[v.len() - 1]).collect()
    }

    fn stacks(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Move>)> {
        let (input, crates_horizontal) =
            separated_list1(newline, separated_list1(tag(" "), Self::crates))(input)?;

        let (input, (_, _, _)) =
            tuple((newline, many1(preceded(space1, digit1)), multispace1))(input)?;

        let (input, moves) = separated_list1(newline, Self::moves)(input)?;

        let mut crates_vertical = vec![];

        for _ in 0..crates_horizontal[0].len() {
            crates_vertical.push(vec![]);
        }

        for vec in crates_horizontal.into_iter().rev() {
            for (i, c) in vec.into_iter().enumerate() {
                crates_vertical[i].push(c)
            }
        }

        let final_crates: Vec<Vec<char>> = crates_vertical
            .into_iter()
            .map(|vec| vec.into_iter().filter_map(|v| v).collect())
            .collect();

        Ok((input, (final_crates, moves)))
    }

    fn crates(input: &str) -> IResult<&str, Option<char>> {
        let (input, c) = alt((
            tag("   "),
            delimited(complete::char('['), alpha1, complete::char(']')),
        ))(input)?;

        let result = match c {
            "   " => None,
            value => match value.chars().next() {
                None => None,
                Some(c) => Some(c),
            },
        };

        Ok((input, result))
    }

    fn moves(input: &str) -> IResult<&str, Move> {
        let (input, (number, from, to)) = tuple((
            preceded(tag("move "), complete::u8),
            preceded(tag(" from "), complete::u8),
            preceded(tag(" to "), complete::u8),
        ))(input)?;

        Ok((
            input,
            Move {
                number: number as usize,
                from: from as usize - 1,
                to: to as usize - 1,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(Day05::part1(INPUT), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day05::part2(INPUT), "MCD");
    }
}
