use itertools::Itertools;

use crate::Challenge;

#[derive(Debug)]
pub struct Day3;

impl Challenge for Day3 {
    /// Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    fn part1(input: &str) -> u32 {
        input
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(a, b)| Self::priority(Self::common([a, b])))
            .sum()
    }

    // Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
    fn part2(input: &str) -> u32 {
        input
            .lines()
            .tuples()
            .map(|(a, b, c)| Self::priority(Self::common([a, b, c])))
            .sum()
    }
}

impl Day3 {
    fn priority(char: Option<char>) -> u32 {
        match char {
            Some(c @ 'a'..='z') => c as u32 - 96,
            Some(c @ 'A'..='Z') => c as u32 - 38,
            _ => 0,
        }
    }

    fn common<'a>(lists: impl IntoIterator<Item = &'a str> + Copy) -> Option<char> {
        match lists.into_iter().next() {
            Some(first) => first
                .chars()
                .filter(|&c| lists.into_iter().skip(1).all(|list| list.contains(c)))
                .next(),
            _ => None,
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
