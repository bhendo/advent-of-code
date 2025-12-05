pub fn part1(input: &str) -> usize {
    0
}
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
        // assert_eq!(part1(include_str!("../_inputs/day0.txt")), 0)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(EXAMPLE), 0);
    //     assert_eq!(part2(include_str!("../_inputs/day0.txt")), 0)
    // }
    //
    // #[bench]
    // fn benchmark_part1(b: &mut Bencher) {
    //     b.iter(|| part1(EXAMPLE))
    // }
    //
    // #[bench]
    // fn benchmark_part2(b: &mut Bencher) {
    //     b.iter(|| part2(EXAMPLE))
    // }
}
