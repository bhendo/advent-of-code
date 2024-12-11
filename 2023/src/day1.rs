pub fn part1(input: Vec<&str>) -> u32 {
    input.iter().fold(0, |acc, line| {
        let vec = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        acc + 10 * vec.first().unwrap() + vec.last().unwrap()
    })
}

pub fn part2(input: Vec<&str>) -> u32 {
    input.iter().fold(0, |acc, line| {
        let vec = line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        acc + 10 * vec.first().unwrap() + vec.last().unwrap()
    })
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 142);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day1.txt"))),
            53921
        )
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(parse_input(EXAMPLE)))
    }

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE2)), 281);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day1.txt"))),
            54676
        )
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(parse_input(EXAMPLE)))
    }
}
