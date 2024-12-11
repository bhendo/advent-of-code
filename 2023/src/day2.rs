use std::{fmt::Error, str::FromStr};

type Set = [u32; 3];

pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Game {
            id: 0,
            sets: vec![],
        })
    }
}

pub fn part1(input: Vec<Game>) -> u32 {
    input
        .iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| set[0] <= 12 && set[1] <= 13 && set[2] <= 14)
        })
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| {
            game.sets
                .iter()
                .fold([0, 0, 0], |mut acc, set: &Set| {
                    if set[0] > acc[0] {
                        acc[0] = set[0];
                    }
                    if set[1] > acc[1] {
                        acc[1] = set[1];
                    }
                    if set[2] > acc[2] {
                        acc[2] = set[2];
                    }
                    acc
                })
                .into_iter()
                .product::<u32>()
        })
        .sum()
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let splits = line.split_once(":").unwrap();
            Game {
                id: splits.0.split_once(" ").unwrap().1.parse().unwrap(),
                sets: splits
                    .1
                    .split("; ")
                    .map(|cubes| {
                        let mut set = [0, 0, 0];
                        for (n, color) in cubes
                            .split(",")
                            .map(|cube| cube.trim().split_once(" ").unwrap())
                        {
                            match color {
                                "red" => set[0] = n.parse().unwrap(),
                                "green" => set[1] = n.parse().unwrap(),
                                _ => set[2] = n.parse().unwrap(),
                            }
                        }
                        set
                    })
                    .collect(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 8);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day2.txt"))),
            2237
        )
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(parse_input(EXAMPLE)))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 2286);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day2.txt"))),
            66681
        )
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(parse_input(EXAMPLE)))
    }
}
