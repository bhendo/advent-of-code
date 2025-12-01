pub fn part1(input: &str) -> usize {
    let mut dial = 50;
    input
        .lines()
        .map(move |line| {
            let (direction, distance) = line.split_at(1);
            let mut delta = distance.parse::<isize>().unwrap();
            if direction == "L" {
                delta *= -1
            }
            let end = dial + delta;
            dial = end.rem_euclid(100);
            if dial == 0 { 1 } else { 0 }
        })
        .sum()
}
pub fn part2(input: &str) -> usize {
    let mut dial = 50;
    input
        .lines()
        .map(move |line| {
            let (direction, distance) = line.split_at(1);
            let mut delta = distance.parse::<isize>().unwrap();
            if direction == "L" {
                delta *= -1
            }
            let start = dial;
            let end = dial + delta;
            let wraps = end.div_euclid(100).abs();
            dial = end.rem_euclid(100);

            let mut zero_touches = wraps as usize;

            if delta < 0 && dial == 0 {
                zero_touches += 1
            }
            if delta < 0 && start == 0 && zero_touches > 0 {
                zero_touches -= 1
            }
            zero_touches
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
        assert_eq!(part1(include_str!("../_inputs/day1.txt")), 1168)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
        assert_eq!(part2(include_str!("../_inputs/day1.txt")), 7199)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
