extern crate regex;
use regex::Regex;
use std::i64::{MAX, MIN};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(i64, i64);

impl Point {
    fn line(&self, other: Self) -> Vec<Self> {
        let m = (other.1 - self.1) as f64 / (other.0 - self.0) as f64;
        let b = (m * self.0 as f64 - self.1 as f64) * -1f64;

        (self.0..=other.0)
            .map(|x| {
                let y = (m * x as f64 + b) as i64;
                Point(x, y)
            })
            .collect()
    }

    fn manhatten_distance(&self, other: &Self) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
    fn is_in_bounds(&self, max: i64) -> bool {
        (self.0 <= max && self.0 >= 0) && (self.1 <= max && self.1 >= 0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Beacon {
    point: Point,
}

#[derive(Clone, Debug)]
pub struct Sensor {
    point: Point,
    beacon: Beacon,
    distance: i64,
    surrounding_space: Vec<Point>,
}

impl Sensor {
    fn is_likely_in_bounds(&self, max: i64) -> bool {
        self.surrounding_space.iter().any(|p| p.is_in_bounds(max))
    }
}

#[derive(Debug)]
pub struct Cave {
    sensors: Vec<Sensor>,
    min: Point,
    max: Point,
}

fn space_just_beyond(Point(x, y): Point, distance: i64) -> Vec<Point> {
    let mut just_beyond_reach = vec![];
    let top = Point(x, y + distance + 1);
    let bottom = Point(x, y - distance - 1);
    let right = Point(x + distance + 1, y);
    let left = Point(x - distance - 1, y);
    just_beyond_reach.append(&mut top.line(right));
    just_beyond_reach.append(&mut right.line(bottom));
    just_beyond_reach.append(&mut bottom.line(left));
    just_beyond_reach.append(&mut left.line(top));
    just_beyond_reach
}

pub fn part1(cave: Cave, y: i64) -> i64 {
    (cave.min.0..=cave.max.0).fold(0, |acc, x| {
        match cave.sensors.iter().any(|s| {
            s.point.manhatten_distance(&Point(x, y)) <= s.distance && s.beacon.point != Point(x, y)
        }) {
            true => acc + 1,
            false => acc,
        }
    })
}

pub fn part2(cave: Cave, max: i64) -> i64 {
    let mut sensors_in_bounds = vec![];
    let potential_beacons = cave
        .sensors
        .iter()
        .filter_map(|s| match s.is_likely_in_bounds(max) {
            true => {
                sensors_in_bounds.push(s);
                Some(s.surrounding_space.to_owned())
            }
            false => None,
        })
        .flatten()
        .collect::<Vec<Point>>();

    for p in potential_beacons {
        if p.is_in_bounds(max)
            && sensors_in_bounds
                .iter()
                .all(|&s| s.point.manhatten_distance(&p) > s.distance)
        {
            let Point(x, y) = p;
            return x as i64 * 4_000_000 + y as i64;
        }
    }
    0
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
                let sx: i64 = caps.get(1).map(|m| m.as_str().parse().unwrap()).unwrap();
                let sy: i64 = caps.get(2).map(|m| m.as_str().parse().unwrap()).unwrap();
                let bx: i64 = caps.get(3).map(|m| m.as_str().parse().unwrap()).unwrap();
                let by: i64 = caps.get(4).map(|m| m.as_str().parse().unwrap()).unwrap();
                let sensor_point = Point(sx, sy);
                let beacon_point = Point(bx, by);
                let distance = sensor_point.manhatten_distance(&beacon_point);
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
                    point: sensor_point,
                    beacon: Beacon {
                        point: beacon_point,
                    },
                    distance,
                    surrounding_space: space_just_beyond(sensor_point, distance),
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
            5_335_787
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE), 20), 56_000_011);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day15.txt")), 4_000_000),
            13_673_971_349_056
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
