use std::collections::HashSet;

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (-1, 1),
    (1, 0),
    (1, -1),
    (1, 1),
    (0, -1),
    (0, 1),
];

pub fn part1(input: &str) -> usize {
    let mut paper_rolls: HashSet<(isize, isize)> = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, val) in row.chars().enumerate() {
            if val == '@' {
                paper_rolls.insert((x as isize, y as isize));
            }
        }
    }

    paper_rolls
        .iter()
        .filter(|&&(x, y)| {
            NEIGHBORS
                .iter()
                .filter(|&(dx, dy)| {
                    if paper_rolls.contains(&(x + dx, y + dy)) {
                        return true;
                    }
                    false
                })
                .count()
                < 4
        })
        .count()
}
pub fn part2(input: &str) -> usize {
    let mut paper_rolls: HashSet<(isize, isize)> = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, val) in row.chars().enumerate() {
            if val == '@' {
                paper_rolls.insert((x as isize, y as isize));
            }
        }
    }

    let total_rolls = paper_rolls.len();

    loop {
        let to_remove: Vec<(isize, isize)> = paper_rolls
            .iter()
            .filter(|&&(x, y)| {
                NEIGHBORS
                    .iter()
                    .filter(|&(dx, dy)| {
                        if paper_rolls.contains(&(x + dx, y + dy)) {
                            return true;
                        }
                        false
                    })
                    .count()
                    < 4
            })
            .cloned()
            .collect();

        if to_remove.is_empty() {
            break;
        }
        for pos in to_remove {
            paper_rolls.remove(&pos);
        }
    }
    total_rolls - paper_rolls.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
        assert_eq!(part1(include_str!("../_inputs/day4.txt")), 1411)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 43);
        assert_eq!(part2(include_str!("../_inputs/day4.txt")), 8557)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
