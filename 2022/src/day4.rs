use std::{collections::HashMap, vec};

pub fn part1(input: Vec<Vec<Vec<i32>>>) -> i32 {
    input.iter().fold(0, |acc, pair| {
        let (elf1, elf2) = (pair[0].clone(), pair[1].clone());
        if (elf1[0] >= elf2[0] && elf1[1] <= elf2[1]) || (elf1[0] <= elf2[0] && elf1[1] >= elf2[1])
        {
            return acc + 1;
        }

        acc
    })
}

pub fn part2(input: Vec<Vec<Vec<i32>>>) -> i32 {
    input.iter().fold(0, |acc, pair| {
        let (elf1, elf2) = (pair[0].clone(), pair[1].clone());
        if (elf1[0] >= elf2[0] && elf1[0] <= elf2[1]) || (elf1[0] <= elf2[0] && elf1[1] >= elf2[0])
        {
            return acc + 1;
        }
        acc
    })
}

pub fn parse_input(input: &str) -> Vec<Vec<Vec<i32>>> {
    input
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|e| e.split("-").map(|n| n.parse::<i32>().unwrap()).collect())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_day1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 2);
        assert_eq!(part1(parse_input(include_str!("../_inputs/day4.txt"))), 433)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 4);
        assert_eq!(part2(parse_input(include_str!("../_inputs/day4.txt"))), 852)
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
