use std::collections::HashMap;

pub fn part1(input: Vec<Vec<u8>>) -> i32 {
    input.iter().fold(0, |acc, l| {
        let (c1, c2) = l.split_at(l.len() / 2);
        let mut compartment1: Vec<&u8> = c1.iter().map(|c| c).collect();
        let mut compartment2: Vec<&u8> = c2.iter().map(|c| c).collect();
        compartment1.sort_unstable();
        compartment2.sort_unstable();

        let (c1d, _) = compartment1.partition_dedup();
        let (c2d, _) = compartment2.partition_dedup();

        let mut map = HashMap::new();

        for e in c1d {
            map.insert(e, true);
        }

        for e in c2d {
            match map.insert(e, true) {
                Some(_) => return acc + e.to_be() as i32,
                None => {}
            }
        }
        return acc;
    })
}

pub fn part2(input: Vec<Vec<u8>>) -> i32 {
    input.chunks(3).fold(0, |acc, b| {
        let mut b1: Vec<&u8> = b[0].iter().map(|c| c).collect();
        let mut b2: Vec<&u8> = b[1].iter().map(|c| c).collect();
        let mut b3: Vec<&u8> = b[2].iter().map(|c| c).collect();

        b1.sort_unstable();
        let (bag1, _) = b1.partition_dedup();
        b2.sort_unstable();
        let (bag2, _) = b2.partition_dedup();
        b3.sort_unstable();
        let (bag3, _) = b3.partition_dedup();

        let mut badge_group: Vec<&u8> = [bag1, bag2, bag3].concat();

        badge_group.sort_unstable();
        let (_, d1) = badge_group.partition_dedup();

        d1.sort_unstable();
        let (_, badge) = d1.partition_dedup();

        acc + badge[0].to_be() as i32
    })
}

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|c| {
                    let mut b = [0; 1];
                    c.encode_utf8(&mut b);
                    if b[0] > 96 {
                        let out = b[0] - 96;
                        out
                    } else {
                        let out = b[0] - 38;
                        out
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_day1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 157);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day3.txt"))),
            7763
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 70);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day3.txt"))),
            2569
        )
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(parse_input(EXAMPLE)))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(parse_input(EXAMPLE)))
    }
}
