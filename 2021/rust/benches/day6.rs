#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate aoc2021;
    extern crate test;

    use aoc2021::day6::{part1, part2};
    use test::Bencher;

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        let data = include_str!("../../_inputs/day6.txt");
        b.iter(|| part1(data))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        let data = include_str!("../../_inputs/day6.txt");
        b.iter(|| part2(data))
    }
}
