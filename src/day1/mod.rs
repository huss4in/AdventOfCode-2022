pub use crate::Day;

#[derive(Debug)]
pub struct Day1;

impl Day for Day1 {
    fn part1(input: &str) -> u32 {
        Self::common(input).max().unwrap_or_default()
    }

    fn part2(input: &str) -> u32 {
        Self::common(input)
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum::<u32>()
    }

    fn name() -> String {
        format!("{Self:?}")
    }

    fn input() -> &'static String {
        lazy_static::lazy_static! {
            pub static ref INPUT: String = crate::read_input_day(1);
        }

        &INPUT
    }
}

use itertools::Itertools;

impl Day1 {
    fn common(input: &str) -> impl Iterator<Item = u32> + '_ {
        input.trim().split("\n\n").map(|num| {
            num.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_part1() {
        assert_eq!(Day1::part1(INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day1::part2(INPUT), 45000);
    }
}
