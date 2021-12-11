#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate aoc2021;
    extern crate test;

    use aoc2021::day11::{parse_input, part1, part2};
    use test::Bencher;

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        let data = parse_input(include_str!("../../_inputs/day11.txt"));
        b.iter(|| part1(&mut data.clone()))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        let data = parse_input(include_str!("../../_inputs/day11.txt"));
        b.iter(|| part2(&mut data.clone()))
    }
}
