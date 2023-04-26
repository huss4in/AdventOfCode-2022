#![allow(unused_variables, dead_code)]

use crate::Challenge;

#[derive(Debug)]
pub struct Day5;

impl Challenge for Day5 {
    /// After the rearrangement procedure completes, what crate ends up on top of each stack?
    fn part1(input: &str) -> String {
        "".to_string()
    }

    /// part2
    fn part2(input: &str) -> String {
        "".to_string()
    }
}

struct Stacks {
    len: usize,
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn parse(input: &str) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(Day5::part1(INPUT), "");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day5::part2(INPUT), "");
    }
}
