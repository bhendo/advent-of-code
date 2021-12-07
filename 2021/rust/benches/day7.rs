#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate aoc2021;
    extern crate test;

    use aoc2021::day7::{part1, part2};
    use test::Bencher;

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        let data = include_str!("../../_inputs/day7.txt")
            .split(",")
            .map(|v| i32::from_str_radix(v, 10).unwrap())
            .collect::<Vec<_>>();
        b.iter(|| part1(data.clone()))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        let data = include_str!("../../_inputs/day7.txt")
            .split(",")
            .map(|v| i32::from_str_radix(v, 10).unwrap())
            .collect::<Vec<_>>();
        b.iter(|| part2(data.clone()))
    }
}
