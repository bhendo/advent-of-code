use std::collections::{HashMap, VecDeque};

fn count_paths<'a>(
    g: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    target: &'a str,
    mask: u8,
    memo: &mut HashMap<(&'a str, u8), usize>,
    required_bits: &HashMap<&'a str, u8>,
    required_all: u8,
) -> usize {
    if let Some(&v) = memo.get(&(node, mask)) {
        return v;
    }
    let next_mask = mask | *required_bits.get(node).unwrap_or(&0);
    let total = if node == target {
        (next_mask & required_all == required_all) as usize
    } else {
        g.get(node)
            .into_iter()
            .flatten()
            .map(|&n| count_paths(g, n, target, next_mask, memo, required_bits, required_all))
            .sum()
    };
    memo.insert((node, mask), total);
    total
}

pub fn part1(input: &str) -> usize {
    let graph = input.lines().fold(
        HashMap::new(),
        |mut graph: HashMap<&str, Vec<&str>>, line| {
            let mut devices = line.split_whitespace().collect::<VecDeque<_>>();
            let from = devices.pop_front().unwrap().strip_suffix(":").unwrap();
            for to in devices {
                graph.entry(from).or_default().push(to);
                graph.entry(to).or_default();
            }
            graph
        },
    );

    count_paths(
        &graph,
        "you",
        "out",
        0,
        &mut HashMap::new(),
        &HashMap::new(),
        0,
    )
}

pub fn part2(input: &str) -> usize {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut devices = line.split_whitespace().collect::<VecDeque<_>>();
        let from = devices.pop_front().unwrap().strip_suffix(":").unwrap();
        for to in devices {
            graph.entry(from).or_default().push(to);
            graph.entry(to).or_default();
        }
    }

    let mut required_bits: HashMap<&str, u8> = HashMap::new();
    for (idx, req) in ["fft", "dac"].iter().enumerate() {
        required_bits.insert(*req, 1u8 << idx);
    }
    let required_all: u8 = required_bits.values().fold(0, |acc, b| acc | b);
    count_paths(
        &graph,
        "svr",
        "out",
        0,
        &mut HashMap::new(),
        &required_bits,
        required_all,
    )
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 5);
        assert_eq!(part1(include_str!("../_inputs/day11.txt")), 652)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2), 2);
        assert_eq!(part2(include_str!("../_inputs/day11.txt")), 362956369749210)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE1))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE2))
    }
}
