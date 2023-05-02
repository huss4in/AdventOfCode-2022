use crate::Challenge;

#[derive(Debug)]
pub struct Day6;

impl Challenge for Day6 {
    fn part1(_input: &str) -> String {
        "0".into()
    }

    fn part2<'a>(_input: &str) -> String {
        "0".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(Day6::part1(INPUT), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day6::part2(INPUT), "0");
    }
}
