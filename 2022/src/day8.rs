pub fn part1(trees: Vec<Vec<u32>>) -> i32 {
    let mut total = 0;
    let n = trees.len();
    for x in 0..n {
        for y in 0..n {
            if x == 0 || y == 0 || x == n - 1 || y == n - 1 {
                total += 1;
            } else {
                let t = trees[x][y];
                let mut counted = true;
                for i in 0..x {
                    if trees[i][y] >= t {
                        counted = false;
                        break;
                    }
                }
                if counted {
                    total += 1;
                    continue;
                }
                counted = true;
                for i in x + 1..n {
                    if trees[i][y] >= t {
                        counted = false;
                        break;
                    }
                }
                if counted {
                    total += 1;
                    continue;
                }
                counted = true;
                for i in 0..y {
                    if trees[x][i] >= t {
                        counted = false;
                        break;
                    }
                }
                if counted {
                    total += 1;
                    continue;
                }
                counted = true;
                for i in y + 1..n {
                    if trees[x][i] >= t {
                        counted = false;
                    }
                }
                if counted {
                    total += 1;
                    continue;
                }
            }
        }
    }
    total
}

pub fn part2(trees: Vec<Vec<u32>>) -> i32 {
    let n = trees.len();
    let mut score = 0;
    for x in 0..n {
        for y in 0..n {
            let t = trees[x][y];
            let mut tscore = 1;

            if x == 0 || y == 0 || x == n - 1 || y == n - 1 {
                tscore = 0;
            } else {
                // left
                for i in (0..x).rev() {
                    if trees[i][y] >= t || i == 0 {
                        tscore = tscore * (i as i32 - x as i32).abs();
                        break;
                    }
                }

                // right
                for i in (x + 1)..n {
                    if trees[i][y] >= t || i == n - 1 {
                        tscore = tscore * (x as i32 - i as i32).abs();
                        break;
                    }
                }

                // up
                for i in (0..y).rev() {
                    if trees[x][i] >= t || i == 0 {
                        tscore = tscore * (i as i32 - y as i32).abs();
                        break;
                    }
                }

                // down
                for i in (y + 1)..n {
                    if trees[x][i] >= t || i == n - 1 {
                        tscore = tscore * (y as i32 - i as i32).abs();
                        break;
                    }
                }
            }

            if tscore > score {
                score = tscore;
            }
        }
    }
    score
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_day1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 21);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day8.txt"))),
            1835
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 8);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day8.txt"))),
            263670
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
