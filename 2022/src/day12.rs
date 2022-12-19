extern crate pathfinding;
use pathfinding::prelude::dijkstra;
use std::vec;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

pub struct Forrest {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
    potential_starts: Vec<Pos>,
}

impl Forrest {
    fn successors(&self, Pos(x, y): Pos) -> Vec<(Pos, u32)> {
        let mut successors = vec![];
        let dxdys: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in dxdys {
            match self.is_successor(Pos(x, y), (dx, dy)) {
                Some(p) => successors.push(p),
                None => (),
            }
        }
        successors.into_iter().map(|p| (p, 1)).collect()
    }
    fn is_successor(&self, Pos(x, y): Pos, (dx, dy): (isize, isize)) -> Option<Pos> {
        let (px, py) = (x as isize + dx, y as isize + dy);
        if px >= 0 && px < self.width as isize && py >= 0 && py < self.height as isize {
            let h = get_height(self.data[y][x]);
            let ph = get_height(self.data[py as usize][px as usize]);
            if h.abs_diff(ph) <= 1 || ph < h {
                return Some(Pos(px as usize, py as usize));
            }
        }
        None
    }
}

fn get_height(ch: char) -> u8 {
    match ch {
        'S' => 0,
        'E' => 27,
        c => c as u8 - 96,
    }
}

pub fn part1(forrest: Forrest) -> u32 {
    match dijkstra(
        &forrest.start,
        |p| forrest.successors(*p),
        |p| *p == forrest.end,
    ) {
        Some(res) => res.1,
        None => 0,
    }
}

pub fn part2(forrest: Forrest) -> u32 {
    let potential_starts = forrest.potential_starts.clone();
    let mut sd = (forrest.height * forrest.width) as u32;
    for p in potential_starts {
        let d = match dijkstra(&p, |p| forrest.successors(*p), |p| *p == forrest.end) {
            Some(res) => res.1,
            None => sd,
        };
        if d < sd {
            sd = d;
        }
    }
    sd
}

pub fn parse_input(input: &str) -> Forrest {
    let data: Vec<Vec<char>> = input.split("\n").map(|l| l.chars().collect()).collect();
    let height = data.len();
    let mut width = 0;
    if height > 0 {
        width = data[0].len();
    }
    let (mut start, mut end) = (Pos(0, 0), Pos(0, 0));
    let mut potential_starts = vec![];
    for x in 0..width {
        for y in 0..height {
            if data[y][x] == 'S' {
                start = Pos(x, y);
            }
            if data[y][x] == 'E' {
                end = Pos(x, y);
            }
            if data[y][x] == 'a' {
                potential_starts.push(Pos(x, y));
            }
        }
    }
    Forrest {
        data,
        height,
        width,
        start,
        end,
        potential_starts,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 31);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day12.txt"))),
            437
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 29);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day12.txt"))),
            430
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
