use itertools::Itertools;

use crate::Challenge;

#[derive(Debug)]
pub struct Day06;

impl Challenge for Day06 {
    fn part1(input: &str) -> String {
        const NUMBER: usize = 4;

        let mut filter = 0u32;

        input
            .chars()
            .take(NUMBER - 1)
            .for_each(|c| filter ^= 1 << (c as u32 % 32));

        match input
            .chars()
            .tuple_windows()
            .position(|(first, _, _, last)| {
                filter ^= 1 << (last as u32 % 32);
                let result = filter.count_ones() == NUMBER as u32;
                filter ^= 1 << (first as u32 % 32);
                result
            })
            .map(|x| x + NUMBER)
        {
            Some(x) => x.to_string(),
            None => "None".into(),
        }
    }

    fn part2(input: &str) -> String {
        const NUMBER: usize = 14;

        let mut idx = 0;

        let input: Vec<char> = input.chars().collect();

        while let Some(slice) = input.get(idx..idx + NUMBER) {
            let mut filter = 0u32;

            if let Some(pos) = slice.into_iter().rposition(|&char| {
                let bit_idx = char as u32 % 32;

                let res = filter & (1 << bit_idx) != 0;

                filter |= 1 << bit_idx;
                res
            }) {
                idx += pos + 1;
            } else {
                return (idx + NUMBER).to_string();
            }
        }

        "None".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part1() {
        assert_eq!(Day06::part1(INPUT), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day06::part2(INPUT), "19");
    }
}
