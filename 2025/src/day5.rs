fn parse_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_unstable_by_key(|(start, _)| *start);

    let mut merged = vec![ranges[0]];
    for (start, end) in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

pub fn part1(input: &str) -> usize {
    let (fresh_ranges, available) = input.split_once("\n\n").unwrap();
    let ranges = merge_ranges(parse_ranges(fresh_ranges));

    available
        .lines()
        .filter(|&line| {
            let ingredient: usize = line.parse().unwrap();
            match ranges.binary_search_by_key(&ingredient, |(start, _)| *start) {
                Ok(idx) => ingredient <= ranges[idx].1,
                Err(0) => false,
                Err(idx) => ingredient <= ranges[idx - 1].1,
            }
        })
        .count()
}
pub fn part2(input: &str) -> usize {
    let (fresh_ranges, _) = input.split_once("\n\n").unwrap();
    merge_ranges(parse_ranges(fresh_ranges))
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
        assert_eq!(part1(include_str!("../_inputs/day5.txt")), 758)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 14);
        assert_eq!(part2(include_str!("../_inputs/day5.txt")), 343143696885053)
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
