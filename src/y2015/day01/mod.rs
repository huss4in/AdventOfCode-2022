use crate::Challenge;

#[derive(Debug)]
pub struct Day01;

impl Challenge for Day01 {
    fn part1(input: &str) -> String {
        input
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum::<isize>()
            .to_string()
    }

    fn part2(input: &str) -> String {
        let mut sum = 0;

        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => sum += 1,
                ')' => sum -= 1,
                _ => (),
            }

            if sum == -1 {
                return (i + 1).to_string();
            }
        }

        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day01::part1("(())"), "0");
        assert_eq!(Day01::part1("()()"), "0");

        assert_eq!(Day01::part1("((("), "3");
        assert_eq!(Day01::part1("(()(()("), "3");
        assert_eq!(Day01::part1("))((((("), "3");

        assert_eq!(Day01::part1("())"), "-1");
        assert_eq!(Day01::part1("))("), "-1");

        assert_eq!(Day01::part1(")))"), "-3");
        assert_eq!(Day01::part1(")())())"), "-3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01::part2(")"), "1");
        assert_eq!(Day01::part2("()())"), "5");
    }
}
