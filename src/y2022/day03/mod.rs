use crate::Challenge;

#[derive(Debug)]
pub struct Day03;

impl Challenge for Day03 {
    /// Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    fn part1(input: &str) -> String {
        Self::commmon(input.lines().map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            Self::get_priority(&[a, b])
        }))
    }

    // Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
    fn part2(input: &str) -> String {
        Self::commmon(
            input
                .lines()
                .array_chunks::<3>()
                .map(|x| Self::get_priority(&x)),
        )
    }
}

impl Day03 {
    fn commmon(iter: impl Iterator<Item = usize>) -> String {
        iter.sum::<usize>().to_string()
    }

    fn get_priority(lists: &[&str]) -> usize {
        let first = lists[0];
        let lists = &lists[1..];

        match first
            .chars()
            .find(|&c| lists.iter().all(|list| list.contains(c)))
        {
            Some(c @ 'a'..='z') => c as usize - 96,
            Some(c @ 'A'..='Z') => c as usize - 38,
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
        assert_eq!(Day03::part1(INPUT), "157");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03::part2(INPUT), "70");
    }
}
