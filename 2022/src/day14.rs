use core::usize::MIN;
use std::{collections::BTreeSet, vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rock(usize, usize);

pub struct Cave {
    rocks: BTreeSet<Rock>,
    abyss_start: usize,
    floor: usize,
}

pub fn part1(mut cave: Cave) -> i32 {
    let moves = vec![(0isize, 1isize), (-1, 1), (1, 1)];

    let mut sand_tally = 0;
    let mut abyss = false;

    while !abyss {
        let mut sand = Rock(500, 0);
        let mut sand_falling = true;
        while sand_falling {
            let mut possible_sand = moves.iter().filter_map(|(x, y)| {
                let next_sand = Rock(
                    sand.0.wrapping_add_signed(*x),
                    sand.1.wrapping_add_signed(*y),
                );
                match cave.rocks.contains(&next_sand) {
                    true => None,
                    false => Some(next_sand),
                }
            });

            let m = possible_sand.next();
            match m {
                Some(s) => {
                    sand = s;
                    if sand.1 > cave.abyss_start {
                        sand_falling = false;
                        abyss = true;
                    }
                }
                None => {
                    sand_falling = false;
                    cave.rocks.insert(sand);
                    sand_tally += 1;
                }
            }
        }
    }
    sand_tally
}

pub fn part2(mut cave: Cave) -> i32 {
    let moves = vec![(0isize, 1isize), (-1, 1), (1, 1)];

    let mut sand_tally = 0;
    let mut blocked = false;

    while !blocked {
        let mut sand = Rock(500, 0);
        let mut sand_falling = true;
        while sand_falling {
            let mut possible_sand = moves.iter().filter_map(|(x, y)| {
                let next_sand = Rock(
                    sand.0.wrapping_add_signed(*x),
                    sand.1.wrapping_add_signed(*y),
                );
                match cave.rocks.contains(&next_sand) {
                    true => None,
                    false => Some(next_sand),
                }
            });

            let m = possible_sand.next();
            match m {
                Some(s) => {
                    sand = s;
                    if sand.1 >= cave.floor {
                        sand_falling = false;
                        cave.rocks.insert(sand);
                        sand_tally += 1;
                    }
                }
                None => {
                    let Rock(x, y) = sand;
                    if x == 500 && y == 0 {
                        blocked = true;
                    }
                    sand_falling = false;
                    cave.rocks.insert(sand);
                    sand_tally += 1;
                }
            }
        }
    }
    sand_tally
}

pub fn parse_input(input: &str) -> Cave {
    let mut abyss_start = MIN;
    let rocks = input
        .split("\n")
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    p.split_once(",")
                        .map(|(x, y)| {
                            let px = x.parse::<usize>().unwrap();
                            let py = y.parse::<usize>().unwrap();
                            if py > abyss_start {
                                abyss_start = py;
                            }
                            Rock(px, py)
                        })
                        .unwrap()
                })
                .collect::<Vec<Rock>>()
        })
        .map(|path| {
            let mut expanded_path = vec![];
            for i in 1..path.len() {
                let Rock(sx, sy) = path[i - 1];
                let Rock(ex, ey) = path[i];
                match sx.abs_diff(ex) {
                    0 => {
                        let mut range_y = ey..=sy;
                        if sy < ey {
                            range_y = sy..=ey;
                        }
                        for y in range_y {
                            expanded_path.push(Rock(sx, y));
                        }
                    }
                    _ => {
                        let mut range_x = ex..=sx;
                        if sx < ex {
                            range_x = sx..=ex;
                        }
                        for x in range_x {
                            expanded_path.push(Rock(x, sy));
                        }
                    }
                }
            }
            expanded_path
        })
        .flat_map(|p| p.into_iter())
        .collect::<BTreeSet<Rock>>();
    let floor = abyss_start + 1;
    Cave {
        rocks,
        abyss_start,
        floor,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 24);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day14.txt"))),
            674
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 93);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day14.txt"))),
            24958
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
