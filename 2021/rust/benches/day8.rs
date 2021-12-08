#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate aoc2021;
    extern crate test;

    use aoc2021::day8::{part1, part2};
    use aoc2021::parse_day8_input;
    use test::Bencher;

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        let data = parse_day8_input!(include_str!("../../_inputs/day8.txt"));
        b.iter(|| part1(data.clone()))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        let data = parse_day8_input!(include_str!("../../_inputs/day8.txt"));
        b.iter(|| part2(data.clone()))
    }
}
