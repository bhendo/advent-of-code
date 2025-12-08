use std::ops::ControlFlow::{Break, Continue};

use itertools::Itertools;

fn squared_distance(a: &[usize], b: &[usize]) -> u64 {
    let dx = a[0] as i64 - b[0] as i64;
    let dy = a[1] as i64 - b[1] as i64;
    let dz = a[2] as i64 - b[2] as i64;
    (dx * dx + dy * dy + dz * dz) as u64
}

#[derive(Debug)]
struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

pub fn part1(input: &str) -> usize {
    let junctions: Vec<[usize; 3]> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.splitn(3, ',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let n = junctions.len();
    let to_take = if n == 20 { 10 } else { 1000 };

    let edges: Vec<(u64, usize, usize)> = (0..n)
        .flat_map(|i| {
            let junctions_ref = &junctions;
            (i + 1..n).map(move |j| (squared_distance(&junctions_ref[i], &junctions_ref[j]), i, j))
        })
        .sorted_unstable()
        .take(to_take)
        .collect();

    let mut dsu = DisjointSetUnion::new(n);
    for &(_, u, v) in edges.iter() {
        dsu.union(u, v);
    }

    (0..n)
        .filter_map(|i| {
            if dsu.find(i) == i {
                Some(dsu.size[i])
            } else {
                None
            }
        })
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

pub fn part2(input: &str) -> usize {
    let junctions: Vec<[usize; 3]> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.splitn(3, ',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let n = junctions.len();

    let edges: Vec<(u64, usize, usize)> = (0..n)
        .flat_map(|i| {
            let junctions_ref = &junctions;
            (i + 1..n).map(move |j| (squared_distance(&junctions_ref[i], &junctions_ref[j]), i, j))
        })
        .sorted_unstable()
        .collect();

    let mut dsu = DisjointSetUnion::new(n);

    match edges.into_iter().try_fold(n, |mut connections, (_, i, j)| {
        if dsu.union(i, j) {
            connections -= 1;
            if connections == 1 {
                return Break((i, j));
            }
        }
        Continue(connections)
    }) {
        Break((i, j)) => junctions[i][0] * junctions[j][0],
        Continue(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 40);
        assert_eq!(part1(include_str!("../_inputs/day8.txt")), 140008)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 25272);
        assert_eq!(part2(include_str!("../_inputs/day8.txt")), 9253260633)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }
    //
    // #[bench]
    // fn benchmark_part2(b: &mut Bencher) {
    //     b.iter(|| part2(EXAMPLE))
    // }
}
