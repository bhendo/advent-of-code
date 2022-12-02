pub fn part1(input: Vec<Vec<i32>>) -> i32 {
    input.iter().fold(0, |acc, l| match (l[0] - l[1]).abs() {
        0 => acc + (3 + l[1]),
        1 => {
            if l[0] > l[1] {
                acc + l[1]
            } else {
                acc + 6 + l[1]
            }
        }
        _ => {
            if l[0] < l[1] {
                acc + l[1]
            } else {
                acc + 6 + l[1]
            }
        }
    })
}

pub fn part2(input: Vec<Vec<i32>>) -> i32 {
    input.iter().fold(0, |acc, l| match l[1] {
        1 => {
            // lose
            acc + (0 + match l[0] {
                1 => 3,
                2 => 1,
                _ => 2,
            })
        }
        2 => acc + (3 + l[0]), // draw
        _ => {
            // win
            acc + (6 + match l[0] {
                1 => 2,
                2 => 3,
                _ => 1,
            })
        }
    })
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|l| {
            l.split(" ")
                .map(|s| match s {
                    "A" | "X" => 1,
                    "B" | "Y" => 2,
                    _ => 3,
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_day1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 15);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day2.txt"))),
            13565
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 12);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day2.txt"))),
            12424
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
