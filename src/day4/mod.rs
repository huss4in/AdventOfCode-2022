use crate::Challenge;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

#[derive(Debug)]
pub struct Day4;

impl Challenge for Day4 {
    /// In how many assignment pairs does one range fully contain the other?
    fn part1(input: &str) -> String {
        Self::common(input, |((a1, a2), (b1, b2))| {
            a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2
        })
    }

    /// In how many assignment pairs do the ranges overlap?
    fn part2(input: &str) -> String {
        Self::common(input, |((a1, a2), (b1, b2))| !(a1 > b2 || b1 > a2))
    }
}

impl Day4 {
    fn common(input: &str, calc: impl Fn(&((u32, u32), (u32, u32))) -> bool) -> String {
        Self::lines(input)
            .unwrap_or_default()
            .1
            .into_iter()
            .filter(calc)
            .count()
            .to_string()
    }

    fn lines(input: &str) -> IResult<&str, Vec<((u32, u32), (u32, u32))>> {
        let (input, ranges) = separated_list1(newline, Self::line)(input)?;

        Ok((input, ranges))
    }

    fn line(input: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
        let (input, (start, end)) = separated_pair(Self::pair, tag(","), Self::pair)(input)?;

        Ok((input, (start, end)))
    }

    fn pair(input: &str) -> IResult<&str, (u32, u32)> {
        let (input, (first, second)) =
            separated_pair(complete::u32, tag("-"), complete::u32)(input)?;

        Ok((input, (first, second)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(Day4::part1(INPUT), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day4::part2(INPUT), "4");
    }
}
