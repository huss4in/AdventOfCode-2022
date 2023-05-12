use crate::Challenge;

#[derive(Debug)]
pub struct Day03;

impl Challenge for Day03 {
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
        assert_eq!(Day03::part1(INPUT), "");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03::part2(INPUT), "");
    }
}
