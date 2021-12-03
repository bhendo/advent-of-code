#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate aoc2021;
    extern crate test;

    use aoc2021::day2::{part1, part2, Command, Direction::Forward};
    use test::Bencher;

    const BENCHMARK: [Command; 2000] = [Command {
        direction: Forward,
        units: 0,
    }; 2000];

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(&BENCHMARK))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(&BENCHMARK))
    }
}
