#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate aoc2021;
    extern crate test;

    use aoc2021::day3::{part1, part1a, part2};
    use test::Bencher;

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<String>>();
        b.iter(|| part1(&data))
    }

    #[bench]
    fn benchmark_part1a(b: &mut Bencher) {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<String>>();
        b.iter(|| part1a(&data))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<String>>();
        b.iter(|| part2(&data))
    }
}
