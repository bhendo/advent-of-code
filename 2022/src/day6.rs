use std::collections::HashSet;

pub fn part1(signal: Vec<char>) -> i32 {
    let mut found = false;
    signal.windows(4).fold(0, |acc, g| {
        let unique = HashSet::<char>::from_iter(g.iter().map(|c| *c));
        if unique.len() >= 4 && !found {
            found = true;
            acc + 4
        } else if !found {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn part2(signal: Vec<char>) -> i32 {
    let mut found = false;
    signal.windows(14).fold(0, |acc, g| {
        let unique = HashSet::<char>::from_iter(g.iter().map(|c| *c));
        if unique.len() >= 14 && !found {
            found = true;
            acc + 14
        } else if !found {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect::<Vec<char>>()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsml";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 7);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day6.txt"))),
            1896
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 19);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day6.txt"))),
            3452
        )
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(parse_input(EXAMPLE)))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(parse_input(EXAMPLE)))
    }
}
