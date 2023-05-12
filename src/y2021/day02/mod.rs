use crate::Challenge;

#[derive(Debug)]
pub struct Day02;

impl Challenge for Day02 {
    fn part1(_input: &str) -> String {
        "".into()
    }

    fn part2(_input: &str) -> String {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(Day02::part1(INPUT), "");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02::part2(INPUT), "");
    }
}
