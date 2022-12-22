extern crate regex;
use regex::Regex;
use std::{
    collections::BTreeSet,
    i32::{MAX, MIN},
    ops::{Add, AddAssign},
    sync::{Arc, RwLock},
    thread, vec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(i32, i32);

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Point {
    fn line(&self, other: Self) -> BTreeSet<Self> {
        let m = (other.1 - self.1) as f32 / (other.0 - self.0) as f32;
        let b = (m * self.0 as f32 - self.1 as f32) * -1f32;

        (self.0..=other.0)
            .map(|x| {
                let y = (m * x as f32 + b) as i32;
                Point(x, y)
            })
            .collect()
    }

    fn manhatten_distance(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
    fn bounded(&self, max: i32) -> bool {
        (self.0 <= max && self.0 >= 0) && (self.1 <= max && self.1 >= 0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Beacon {
    point: Point,
}

#[derive(Clone, Copy, Debug)]
pub struct Sensor {
    point: Point,
    beacon: Beacon,
    distance: i32,
}
#[derive(Debug)]
pub struct Cave {
    sensors: Vec<Sensor>,
    min: Point,
    max: Point,
}

pub fn part1(cave: Cave, y: i32) -> i32 {
    (cave.min.0..=cave.max.0).fold(0, |acc, x| {
        match cave.sensors.iter().any(|s| {
            ((s.point.0 - x).abs() + (s.point.1 - y).abs()) <= s.distance
                && s.beacon.point != Point(x, y)
        }) {
            true => acc + 1,
            false => acc,
        }
    })
}

pub fn part2(cave: Cave, max: i32) -> i64 {
    let mut just_beyond_reach = BTreeSet::new();
    cave.sensors.iter().for_each(|s| {
        let top = Point(s.point.0, s.point.1 + s.distance + 1);
        let right = Point(s.point.0 + s.distance + 1, s.point.1);
        let bottom = Point(s.point.0, s.point.1 - s.distance - 1);
        let left = Point(s.point.0 - s.distance - 1, s.point.1);

        if top.bounded(max) || right.bounded(max) || bottom.bounded(max) || left.bounded(max) {
            just_beyond_reach.append(&mut top.line(right));
            just_beyond_reach.append(&mut right.line(bottom));
            just_beyond_reach.append(&mut bottom.line(left));
            just_beyond_reach.append(&mut left.line(top));
        }
    });

    let beacon = just_beyond_reach
        .into_iter()
        .filter(|p| {
            p.bounded(max)
                && cave
                    .sensors
                    .iter()
                    .all(|s| s.point.manhatten_distance(p) > s.distance)
        })
        .collect::<BTreeSet<Point>>();

    let Point(x, y) = beacon.first().unwrap();

    *x as i64 * 4_000_000 + *y as i64
}

pub fn parse_input(input: &str) -> Cave {
    let re = Regex::new(r"Sensor at x=([\-]?[0-9]+), y=([\-]?[0-9]+): closest beacon is at x=([\-]?[0-9]+), y=([\-]?[0-9]+)").unwrap();
    let mut min = Point(MAX, MAX);
    let mut max = Point(MIN, MIN);
    let mut max_distance = 0;

    Cave {
        sensors: input
            .split("\n")
            .map(|l| {
                let caps = re.captures(l).unwrap();
                let sx: i32 = caps.get(1).map(|m| m.as_str().parse().unwrap()).unwrap();
                let sy: i32 = caps.get(2).map(|m| m.as_str().parse().unwrap()).unwrap();
                let bx: i32 = caps.get(3).map(|m| m.as_str().parse().unwrap()).unwrap();
                let by: i32 = caps.get(4).map(|m| m.as_str().parse().unwrap()).unwrap();
                let distance = (sx - bx).abs() + (sy - by).abs();
                if distance > max_distance {
                    max_distance = distance;
                }
                if sx - distance < min.0 {
                    min.0 = sx - distance;
                }
                if sy - distance < min.1 {
                    min.1 = sy - distance;
                }
                if sx + distance > max.0 {
                    max.0 = sx + distance;
                }
                if sy + distance > max.1 {
                    max.1 = sy + distance;
                }
                Sensor {
                    point: Point(sx, sy),
                    beacon: Beacon {
                        point: Point(bx, by),
                    },
                    distance,
                }
            })
            .collect(),
        min,
        max,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE), 10), 26);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day15.txt")), 2_000_000),
            5335787
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE), 20), 56_000_011);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day15.txt")), 4_000_000),
            13673971349056
        )
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(parse_input(EXAMPLE), 10))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(parse_input(EXAMPLE), 20))
    }
}
