use std::collections::VecDeque;

pub fn part1(input: &str) -> usize {
    let mut lines: VecDeque<&str> = input.lines().collect();
    let mut beams = vec![false; lines[0].len()];

    beams[lines.pop_front().unwrap().find('S').unwrap()] = true;

    lines.iter().fold(0, |splits, line| {
        line.char_indices()
            .filter_map(|(i, c)| {
                if c == '^' && beams[i] {
                    beams[i] = false;
                    if i > 0 {
                        beams[i - 1] = true;
                    }
                    if i < line.len() {
                        beams[i + 1] = true;
                    }
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
            .len()
            + splits
    })
}
pub fn part2(input: &str) -> usize {
    let mut lines: VecDeque<&str> = input.lines().collect();
    let mut beams = vec![0usize; lines[0].len()];

    beams[lines.pop_front().unwrap().find('S').unwrap()] = 1;

    lines
        .iter()
        .fold(beams, |current, line| {
            let mut next = vec![0usize; line.len()];

            line.char_indices().for_each(|(i, c)| match c {
                '^' => {
                    if i > 0 {
                        next[i - 1] += current[i];
                    }
                    if i + 1 < line.len() {
                        next[i + 1] += current[i];
                    }
                }
                _ => next[i] += current[i],
            });

            next
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
        assert_eq!(part1(include_str!("../_inputs/day7.txt")), 1600)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 40);
        assert_eq!(part2(include_str!("../_inputs/day7.txt")), 8632253783011)
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
