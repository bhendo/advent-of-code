pub fn part1((mut left, mut right): (Vec<usize>, Vec<usize>)) -> usize {
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (mut l, mut r) = (vec![], vec![]);
    input.lines().for_each(|line| {
        let (a, b) = line.split_once("   ").unwrap();
        l.push(a.parse().unwrap());
        r.push(b.parse().unwrap());
    });
    (l, r)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1};
    use test::Bencher;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 11);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day1.txt"))),
            765748
        )
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(parse_input(EXAMPLE)))
    }
}
