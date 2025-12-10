use bitvec::prelude::*;
use good_lp::{Expression, ProblemVariables, Solution, SolverModel, default_solver, variable};
use itertools::Itertools;
use std::collections::VecDeque;

fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|chunk| !chunk.is_empty())
        .filter_map(|chunk| chunk.parse::<usize>().ok())
        .collect()
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace().collect::<VecDeque<_>>();
            splits.pop_back().unwrap(); // ignore joltage
            let diagram = splits
                .pop_front()
                .unwrap()
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => None,
                })
                .collect::<BitVec>();
            let diagram_size = diagram.len();
            let buttons = splits
                .iter()
                .map(|b| {
                    let mut button = bitvec![0; diagram_size];
                    for idx in parse_numbers(b) {
                        if idx < diagram_size {
                            button.set(idx, true)
                        }
                    }
                    button
                })
                .collect::<Vec<BitVec>>();
            (diagram, buttons)
        })
        .collect::<Vec<(BitVec, Vec<BitVec>)>>()
        .iter()
        .map(|(diagram, buttons)| {
            for k in 1..=buttons.len() {
                for combo in buttons.iter().combinations(k) {
                    let mut test = bitvec![0; diagram.len()];
                    for button in combo {
                        test ^= button
                    }
                    if test == *diagram {
                        return k;
                    }
                }
            }
            0
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace().collect::<VecDeque<_>>();
            splits.pop_front().unwrap(); // ignore diagram
            let joltage = parse_numbers(splits.pop_back().unwrap());
            let buttons = splits.iter().map(|b| parse_numbers(b)).collect_vec();
            (joltage, buttons)
        })
        .collect_vec()
        .iter()
        .filter_map(|(joltage, buttons)| {
            let n = joltage.len();
            let target: Vec<f64> = joltage.iter().map(|&v| v as f64).collect();

            // Build coverage matrix: inc[button][row] = times this button increases that row.
            let inc: Vec<Vec<f64>> = buttons
                .iter()
                .map(|b| {
                    let mut v = vec![0f64; n];
                    for &i in b {
                        if i < n {
                            v[i] += 1.0;
                        }
                    }
                    v
                })
                .collect();

            // Integer linear program: minimize sum(x_j) subject to A x = target, x_j >= 0 integer.
            let mut vars = ProblemVariables::new();
            let x: Vec<_> = (0..inc.len())
                .map(|_| vars.add(variable().integer().min(0)))
                .collect();

            let mut problem = vars
                .minimise(x.iter().copied().sum::<Expression>())
                .using(default_solver);
            for row in 0..n {
                let mut expr = Expression::from(0.0);
                for (col, button) in inc.iter().enumerate() {
                    let coeff = button[row];
                    if coeff != 0.0 {
                        expr += coeff * x[col];
                    }
                }
                problem = problem.with(expr.eq(target[row]));
            }

            match problem.solve() {
                Ok(solution) => {
                    Some(x.iter().map(|v| solution.value(*v)).sum::<f64>().round() as usize)
                }
                Err(_) => None,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 7);
        assert_eq!(part1(include_str!("../_inputs/day10.txt")), 461)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 33);
        assert_eq!(part2(include_str!("../_inputs/day10.txt")), 16386)
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
