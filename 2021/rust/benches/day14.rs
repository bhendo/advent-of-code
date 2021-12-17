#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate aoc2021;
    extern crate test;

    use aoc2021::day14::{part1, part2};
    use test::Bencher;

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(include_str!("../../_inputs/day14.txt")))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(include_str!("../../_inputs/day14.txt")))
    }
}
