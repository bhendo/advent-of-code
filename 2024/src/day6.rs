use std::{
    collections::HashSet,
    ops::Add,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
struct Vector(isize, isize);
impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
struct Lab {
    width: usize,
    height: usize,
    barriers: HashSet<Vector>,
}
impl Lab {
    fn is_vector_in_bounds(&self, vector: Vector) -> bool {
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

    while let next_location = guard.next_location()
        && lab.is_vector_in_bounds(next_location)
    {
        if lab.barriers.contains(&next_location) {
            guard.turn();
        } else {
            guard.step_forward();
        }
    }
    guard.visited.len()
}
pub fn part2(input: &str) -> usize {
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

    let starting_location = guard.location;
    let starting_velocity = guard.velocity;

    while let next_location = guard.next_location()
        && lab.is_vector_in_bounds(next_location)
    {
        if lab.barriers.contains(&next_location) {
            guard.turn();
        } else {
            guard.step_forward();
        }
    }

    // Begin concurrent approach

    let (possible_barriers_tx, possible_barriers_rx) = mpsc::channel();
    let (looped_tx, looped_rx) = mpsc::channel();
    let possible_barriers_rx = Arc::new(Mutex::new(possible_barriers_rx));

    for _ in 0..thread::available_parallelism().unwrap().get() {
        let looped_tx = looped_tx.clone();
        let lab = lab.clone();
        let possible_barriers_rx = Arc::clone(&possible_barriers_rx);
        thread::spawn(move || loop {
            let possible_barrier = match possible_barriers_rx.lock().unwrap().recv() {
                Ok(possible_barrier) => possible_barrier,
                Err(_) => return,
            };
            let mut possible_guard = Guard {
                location: starting_location,
                velocity: starting_velocity,
                visited: HashSet::new(),
            };

            let mut possible_barriers = lab.barriers.clone();
            let mut looped = false;
            let mut visited_barriers = HashSet::new();
            possible_barriers.insert(possible_barrier);

            while let next_location = possible_guard.next_location()
                && lab.is_vector_in_bounds(next_location)
            {
                if possible_barriers.contains(&next_location) {
                    if visited_barriers.insert((possible_guard.location, possible_guard.velocity)) {
                        possible_guard.turn();
                    } else {
                        looped = true;
                        break;
                    }
                } else {
                    possible_guard.step_forward();
                }
            }
            looped_tx.send(looped).unwrap()
        });
    }

    guard
        .visited
        .iter()
        .for_each(|possible_barrier| possible_barriers_tx.send(*possible_barrier).unwrap());

    let mut loops = 0;
    for _ in guard.visited.iter() {
        if let Ok(looped) = looped_rx.recv() {
            if looped {
                loops += 1
            }
        }
    }
    loops

    // End concurrent approach

    //
    // Preserved single thread approach
    //
    //guard.visited.iter().fold(0, |loops, possible_barrier| {
    //    let mut possible_guard = Guard {
    //        location: starting_location,
    //        velocity: starting_velocity,
    //        visited: HashSet::new(),
    //    };
    //    let mut looped = false;
    //    let mut visited_barriers = HashSet::new();
    //
    //    lab.barriers.insert(*possible_barrier);
    //
    //    while let next_location = possible_guard.next_location()
    //        && lab.is_vector_in_bounds(next_location)
    //    {
    //        if lab.barriers.contains(&next_location) {
    //            if visited_barriers.insert((possible_guard.location, possible_guard.velocity)) {
    //                possible_guard.turn();
    //            } else {
    //                looped = true;
    //                break;
    //            }
    //        } else {
    //            possible_guard.step_forward();
    //        }
    //    }
    //
    //    lab.barriers.remove(possible_barrier);
    //
    //    loops + looped as usize
    //})
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
        assert_eq!(part2(EXAMPLE), 6);
        assert_eq!(part2(include_str!("../_inputs/day6.txt")), 1604)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
