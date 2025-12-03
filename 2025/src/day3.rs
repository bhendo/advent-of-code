pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut battery_one = "0".as_bytes()[0];
            let mut battery_two = "0".as_bytes()[0];
            let mut greatest_index = 0;
            let batteries = line.as_bytes();

            for (i, &b) in batteries[..batteries.len() - 1].iter().enumerate() {
                if b > battery_one {
                    battery_one = b;
                    greatest_index = i;
                }
            }

            for &b in batteries[greatest_index + 1..batteries.len()].iter() {
                if b > battery_two {
                    battery_two = b
                }
            }

            [battery_one, battery_two]
                .iter()
                .fold(0, |acc, &d| acc * 10 + (d - b'0') as usize)
        })
        .sum()
}
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let batteries = line.as_bytes();
            let mut to_remove = batteries.len() - 12;
            let mut battery_stack: Vec<u8> = Vec::with_capacity(batteries.len());

            for &b in batteries {
                while to_remove > 0
                    && !battery_stack.is_empty()
                    && *battery_stack.last().unwrap() < b
                {
                    battery_stack.pop();
                    to_remove -= 1;
                }
                battery_stack.push(b);
            }
            battery_stack.truncate(12);
            battery_stack
                .iter()
                .fold(0, |acc, &d| acc * 10 + (d - b'0') as usize)
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
        assert_eq!(part1(include_str!("../_inputs/day3.txt")), 17107)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
        assert_eq!(part2(include_str!("../_inputs/day3.txt")), 169349762274117)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
