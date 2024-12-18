use std::{collections::HashSet, ops::Add};

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
struct Vector(isize, isize);
impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

struct Guard {
    location: Vector,
    velocity: Vector,
    visited: HashSet<Vector>,
}

impl Guard {
    fn next_location(&self) -> Vector {
        self.location + self.velocity
    }
    fn step_forward(&mut self) {
        self.location = self.next_location();
        self.visited.insert(self.location);
    }
    fn turn(&mut self) {
        match self.velocity {
            Vector(0, -1) => self.velocity = Vector(1, 0),
            Vector(1, 0) => self.velocity = Vector(0, 1),
            Vector(0, 1) => self.velocity = Vector(-1, 0),
            _ => self.velocity = Vector(0, -1),
        }
    }
}

struct Lab {
    width: usize,
    height: usize,
    barriers: HashSet<Vector>,
}

impl Lab {
    fn exists(&self, vector: Vector) -> bool {
        vector.0 < self.width as isize
            && vector.0 >= 0
            && vector.1 < self.height as isize
            && vector.1 >= 0
    }
}

pub fn part1(input: &str) -> usize {
    let mut lab = Lab {
        width: 0,
        height: 0,
        barriers: HashSet::new(),
    };
    let mut guard = Guard {
        location: Vector(0, 0),
        velocity: Vector(0, -1),
        visited: HashSet::new(),
    };
    lab.height = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            lab.width = line
                .chars()
                .enumerate()
                .map(|(x, pos)| {
                    match pos {
                        '#' => {
                            lab.barriers.insert(Vector(x as isize, y as isize));
                        }
                        '^' => {
                            guard.location = Vector(x as isize, y as isize);
                            guard.visited.insert(Vector(x as isize, y as isize));
                        }
                        _ => {}
                    }
                    pos
                })
                .count()
        })
        .count();

    while lab.exists(guard.next_location()) {
        if lab.barriers.get(&guard.next_location()).is_some() {
            guard.turn();
        } else {
            guard.step_forward();
        }
    }
    guard.visited.len()
}
pub fn part2(_input: &str) -> usize {
    0
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 41);
        assert_eq!(part1(include_str!("../_inputs/day6.txt")), 4559)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
        //assert_eq!(part2(include_str!("../_inputs/day6.txt")), 0)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
