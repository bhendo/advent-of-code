pub fn part1(input: &str) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = (vec![], vec![]);
    input.lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}
pub fn part2(input: &str) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = (vec![], vec![0; 100_000]);
    input.lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse().unwrap());
        right[r.parse::<usize>().unwrap()] += 1;
    });

    left.into_iter().map(|n| right[n] * n).sum()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 11);
        assert_eq!(part1(include_str!("../_inputs/day1.txt")), 765748)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 31);
        assert_eq!(part2(include_str!("../_inputs/day1.txt")), 27732508)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
