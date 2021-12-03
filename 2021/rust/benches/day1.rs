#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate aoc2021;
    extern crate test;

    use aoc2021::day1::{part1, part2};
    use test::Bencher;

    const BENCHMARK: [i32; 2000] = [0; 2000];

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(&BENCHMARK))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(&BENCHMARK))
    }
}
