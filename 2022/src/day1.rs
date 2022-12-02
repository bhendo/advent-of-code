pub fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut calories: Vec<i32> = input.iter().map(|v| v.iter().sum()).collect();
    calories.sort_unstable();
    calories.reverse();
    return calories[0];
}

pub fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut calories: Vec<i32> = input.iter().map(|v| v.as_slice().iter().sum()).collect();
    calories.sort_unstable();
    calories.reverse();
    return calories[0..=2].iter().sum::<i32>();
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .map(|l| {
            l.split("\n")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_day1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 24000);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day1.txt"))),
            70509
        )
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(parse_input(EXAMPLE)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 45000);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day1.txt"))),
            208567
        )
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(parse_input(EXAMPLE)))
    }
}
