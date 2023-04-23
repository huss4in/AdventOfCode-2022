use crate::Challenge;

#[derive(Debug)]
pub struct Day4;

impl Challenge for Day4 {
    /// In how many assignment pairs does one range fully contain the other?
    fn part1(input: &str) -> u32 {
        input
            .lines()
            .filter(|line| {
                let mut parts = line
                    .split(',')
                    .map(|x| x.split('-').map(|x| x.parse::<u32>()))
                    .map(|mut n| (n.next(), n.next()));

                match (parts.next(), parts.next()) {
                    (Some((Some(Ok(a1)), Some(Ok(a2)))), Some((Some(Ok(b1)), Some(Ok(b2))))) => {
                        a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2
                    }
                    _ => false,
                }
            })
            .count() as u32
    }

    /// In how many assignment pairs do the ranges overlap?
    fn part2(input: &str) -> u32 {
        input
            .lines()
            .filter(|line| {
                let mut parts = line
                    .split(',')
                    .map(|x| x.split('-').map(|x| x.parse::<u32>()))
                    .map(|mut n| (n.next(), n.next()));

                match (parts.next(), parts.next()) {
                    (Some((Some(Ok(a1)), Some(Ok(a2)))), Some((Some(Ok(b1)), Some(Ok(b2))))) => {
                        !(a1 > b2 || b1 > a2)
                    }
                    _ => false,
                }
            })
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(Day4::part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day4::part2(INPUT), 4);
    }
}
