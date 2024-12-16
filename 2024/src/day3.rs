use regex::{Captures, Regex};
fn mul(capture: Captures<'_>) -> usize {
    let (_, [a, b]) = capture.extract();
    a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
}
pub fn part1(input: &str) -> usize {
    Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .unwrap()
        .captures_iter(input)
        .map(mul)
        .sum()
}
pub fn part2(input: &str) -> usize {
    let mut enabled = true;
    Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)()()|don't\(\)()()")
        .unwrap()
        .captures_iter(input)
        .filter(|capture| {
            let (instruction, [_, _]) = capture.extract();
            match instruction {
                "do()" => {
                    enabled = true;
                    false
                }
                "don't()" => {
                    enabled = false;
                    false
                }
                _ => enabled,
            }
        })
        .map(mul)
        .sum()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 161);
        assert_eq!(part1(include_str!("../_inputs/day3.txt")), 188192787)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2), 48);
        assert_eq!(part2(include_str!("../_inputs/day3.txt")), 113965544)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE2))
    }
}
