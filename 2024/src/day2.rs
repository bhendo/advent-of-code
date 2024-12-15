pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            line.split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .map_windows(|[l, r]| (*l, *r))
                .try_fold(0, |acc, (l, r)| {
                    if acc >= 0 && l < r && l.abs_diff(r) <= 3 {
                        Ok(1)
                    } else if acc <= 0 && l > r && l.abs_diff(r) <= 3 {
                        Ok(-1)
                    } else {
                        Err(())
                    }
                })
                .is_ok()
        })
        .count()
}
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let level = line
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (0..level.len()).any(|i| {
                level[0..i]
                    .iter()
                    .chain(level[i + 1..].iter())
                    .map_windows(|[l, r]| (**l, **r))
                    .try_fold(0, |acc, (l, r)| {
                        if acc >= 0 && l < r && l.abs_diff(r) <= 3 {
                            Ok(1)
                        } else if acc <= 0 && l > r && l.abs_diff(r) <= 3 {
                            Ok(-1)
                        } else {
                            Err(())
                        }
                    })
                    .is_ok()
            })
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 2);
        assert_eq!(part1(include_str!("../_inputs/day2.txt")), 472)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4);
        assert_eq!(part2(include_str!("../_inputs/day2.txt")), 520)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
