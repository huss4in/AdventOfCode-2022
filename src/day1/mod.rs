use crate::Challenge;

use itertools::Itertools;

#[derive(Debug)]
pub struct Day1;

impl Challenge for Day1 {
    /// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    fn part1(input: &str) -> String {
        Self::common(input).max().unwrap_or_default().to_string()
    }

    /// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
    fn part2(input: &str) -> String {
        Self::common(input)
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum::<usize>()
            .to_string()
    }
}

impl Day1 {
    fn common(input: &str) -> impl Iterator<Item = usize> + '_ {
        input.trim().split("\n\n").map(|num| {
            num.lines()
                .map(|x| x.parse::<usize>().unwrap_or_default())
                .sum()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_part1() {
        assert_eq!(Day1::part1(INPUT), "24000");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day1::part2(INPUT), "45000");
    }
}
