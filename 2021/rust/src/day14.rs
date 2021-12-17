use std::collections::BTreeMap;

pub fn part1(input: &str) -> u64 {
    polymer_insertion(input, 10)
}

pub fn part2(input: &str) -> u64 {
    polymer_insertion(input, 40)
}

fn polymer_insertion(input: &str, steps: usize) -> u64 {
    let (template, rules) = parse_input(input);
    let mut pairs: BTreeMap<String, u64> = BTreeMap::new();

    template.windows(2).for_each(|v| {
        *pairs.entry(v.join("")).or_insert(0) += 1;
    });

    for _ in 0..steps {
        pairs = pairs
            .iter()
            .fold(BTreeMap::new(), |mut update_pairs, (k, v)| {
                let k1 = [&k[..1], rules[k.as_str()]].join("");
                let k2 = [rules[k.as_str()], &k[1..2]].join("");
                *update_pairs.entry(k1).or_insert(0) += *v;
                *update_pairs.entry(k2).or_insert(0) += *v;
                update_pairs
            });
    }

    let mut counts = pairs.iter().fold(BTreeMap::new(), |mut counts, (k, &v)| {
        *counts.entry(&k[..1]).or_insert(0) += v;
        counts
    });

    *counts.entry(template.last().unwrap()).or_insert(0) += 1;

    let (_, max) = counts
        .clone()
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();
    let (_, min) = counts.into_iter().min_by_key(|&(_, count)| count).unwrap();

    return max - min;
}

fn parse_input(input: &str) -> (Vec<&str>, BTreeMap<&str, &str>) {
    input
        .split_once("\n\n")
        .map(|(template, rules)| {
            let template = template
                .trim()
                .split("")
                .filter(|&s| s != "")
                .collect::<Vec<&str>>();
            let rules =
                rules
                    .lines()
                    .fold(BTreeMap::new(), |mut rules: BTreeMap<&str, &str>, s| {
                        let (l, r) = s.split_once(" -> ").unwrap();
                        rules.entry(l).or_insert(r);
                        rules
                    });
            (template, rules)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day14::{part1, part2};

    const EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1588);
        assert_eq!(part1(include_str!("../../_inputs/day14.txt")), 2899);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 2188189693529);
        assert_eq!(
            part2(include_str!("../../_inputs/day14.txt")),
            3528317079545
        );
    }
}
