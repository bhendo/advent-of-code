use std::collections::HashSet;

fn follow(front: (i32, i32), back: (i32, i32)) -> (i32, i32) {
    let mut dxdy = ((front.0 - back.0) / 2, (front.1 - back.1) / 2);
    if ((front.0 - back.0).abs() > 1 && (front.1 - back.1).abs() > 0)
        || ((front.0 - back.0).abs() > 0 && (front.1 - back.1).abs() > 1)
    {
        if (front.0 - back.0).abs() > (front.1 - back.1).abs() {
            if front.1 > back.1 {
                dxdy.1 += 1;
            } else {
                dxdy.1 -= 1;
            }
        }
        if (front.0 - back.0).abs() < (front.1 - back.1).abs() {
            if front.0 > back.0 {
                dxdy.0 += 1;
            } else {
                dxdy.0 -= 1;
            }
        }
    }
    (back.0 + dxdy.0, back.1 + dxdy.1)
}

pub fn part1(directions: Vec<(&str, i32)>) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail);

    directions
        .iter()
        .for_each(|(direction, distance)| match *direction {
            "U" => {
                for _ in 1..=*distance {
                    head.1 += 1;
                    tail = follow(head, tail);
                    visited.insert(tail);
                }
            }
            "D" => {
                for _ in 1..=*distance {
                    head.1 -= 1;
                    tail = follow(head, tail);
                    visited.insert(tail);
                }
            }
            "R" => {
                for _ in 1..=*distance {
                    head.0 += 1;
                    tail = follow(head, tail);
                    visited.insert(tail);
                }
            }
            _ => {
                for _ in 1..=*distance {
                    head.0 -= 1;
                    tail = follow(head, tail);
                    visited.insert(tail);
                }
            }
        });
    visited.len() as i32
}

pub fn part2(directions: Vec<(&str, i32)>) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots = vec![(0, 0); 10];
    visited.insert(knots[knots.len() - 1]);
    directions
        .iter()
        .for_each(|(direction, distance)| match *direction {
            "U" => {
                for _ in 1..=*distance {
                    knots[0].1 += 1;
                    for i in 1..knots.len() {
                        knots[i] = follow(knots[i - 1], knots[i]);
                    }
                    visited.insert(knots[knots.len() - 1]);
                }
            }
            "D" => {
                for _ in 1..=*distance {
                    knots[0].1 -= 1;
                    for i in 1..knots.len() {
                        knots[i] = follow(knots[i - 1], knots[i]);
                    }
                    visited.insert(knots[knots.len() - 1]);
                }
            }
            "R" => {
                for _ in 1..=*distance {
                    knots[0].0 += 1;
                    for i in 1..knots.len() {
                        knots[i] = follow(knots[i - 1], knots[i]);
                    }
                    visited.insert(knots[knots.len() - 1]);
                }
            }
            _ => {
                for _ in 1..=*distance {
                    knots[0].0 -= 1;
                    for i in 1..knots.len() {
                        knots[i] = follow(knots[i - 1], knots[i]);
                    }
                    visited.insert(knots[knots.len() - 1]);
                }
            }
        });
    visited.len() as i32
}

pub fn parse_input(input: &str) -> Vec<(&str, i32)> {
    input
        .split("\n")
        .map(|l| {
            let (dir, ammount) = l.split_once(" ").unwrap();
            (dir, ammount.parse::<i32>().unwrap())
        })
        .collect::<Vec<(&str, i32)>>()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_day1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 13);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day9.txt"))),
            6190
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE2)), 36);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day9.txt"))),
            2516
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
