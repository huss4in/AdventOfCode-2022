use itertools::Itertools;

use crate::Challenge;

#[derive(Debug)]
pub struct Day3;

impl Challenge for Day3 {
    /// Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    fn part1(input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                let (a, b) = line.split_at(line.len() / 2);

                Self::int(a.chars().filter(|&c| b.contains(c)).next())
            })
            .sum()
    }

    // Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
    fn part2(input: &str) -> u32 {
        input
            .lines()
            .tuples::<(_, _, _)>()
            .map(|(a, b, c)| {
                Self::int(a.chars().filter(|&x| b.contains(x) && c.contains(x)).next())
            })
            .sum()
    }

    fn name() -> String {
        format!("{Self:?}")
    }

    fn input() -> &'static String {
        lazy_static::lazy_static! {
            pub static ref INPUT: String = crate::read_input_day(3);
        }

        &INPUT
    }
}

impl Day3 {
    fn int(char: Option<char>) -> u32 {
        match char {
            Some(c @ 'a'..='z') => c as u32 - 96,
            Some(c @ 'A'..='Z') => c as u32 - 38,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(Day3::part1(INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day3::part2(INPUT), 70);
    }
}
