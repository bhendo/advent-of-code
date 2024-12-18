use std::cmp::Ordering;
use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let (rule_input, instructions) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    rule_input.lines().for_each(|rule| {
        let (l, r) = rule.split_once("|").unwrap();
        let left = l.parse::<usize>().unwrap();
        let right = r.parse::<usize>().unwrap();

        rules.entry(left).or_insert_with(|| vec![right]).push(right);
    });

    instructions
        .lines()
        .map(|line| {
            line.split(",")
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter_map(|mut pages| {
            let mut needs_sorting = false;
            pages.sort_by(|a, b| {
                if let Some(rule) = rules.get(a) {
                    if rule.contains(b) {
                        needs_sorting = true;
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            });
            if needs_sorting {
                None
            } else {
                Some(pages[pages.len() / 2])
            }
        })
        .sum()
}
pub fn part2(input: &str) -> usize {
    let (rule_input, instructions) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    rule_input.lines().for_each(|rule| {
        let (l, r) = rule.split_once("|").unwrap();
        let left = l.parse::<usize>().unwrap();
        let right = r.parse::<usize>().unwrap();

        rules.entry(left).or_insert_with(|| vec![right]).push(right);
    });

    instructions
        .lines()
        .map(|line| {
            line.split(",")
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter_map(|mut pages| {
            let mut needs_sorting = false;
            pages.sort_by(|a, b| {
                if let Some(rule) = rules.get(a) {
                    if rule.contains(b) {
                        needs_sorting = true;
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            });
            if needs_sorting {
                Some(pages[pages.len() / 2])
            } else {
                None
            }
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 143);
        assert_eq!(part1(include_str!("../_inputs/day5.txt")), 6051)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 123);
        assert_eq!(part2(include_str!("../_inputs/day5.txt")), 5093)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
