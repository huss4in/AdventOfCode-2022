use crate::Challenge;

#[derive(Debug)]
pub struct Day06;

impl Challenge for Day06 {
    fn part1(input: &str) -> String {
        Self::common(input, 4)
    }

    fn part2(input: &str) -> String {
        Self::common(input, 14)
    }
}

impl Day06 {
    fn common(input: &str, number: usize) -> String {
        let mut idx = 0;

        while let Some(slice) = input.as_bytes().get(idx..idx + number) {
            let mut filter = 0u32;

            if let Some(pos) = slice.into_iter().rposition(|byte| {
                if filter & 1 << byte % 32 == 0 {
                    filter |= 1 << byte % 32;
                    false
                } else {
                    true
                }
            }) {
                idx += pos + 1;
            } else {
                return (idx + number).to_string();
            }
        }

        0.to_string()
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
