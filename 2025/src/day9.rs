use std::collections::BTreeMap;
use std::ops::Bound;

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| [x.parse().unwrap(), y.parse().unwrap()])
                .unwrap()
        })
        .collect::<Vec<[usize; 2]>>()
        .iter()
        .cloned()
        .tuple_combinations()
        .map(|([x1, y1], [x2, y2])| (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1))
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let vertices: Vec<[i64; 2]> = input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| [x.parse().unwrap(), y.parse().unwrap()])
                .unwrap()
        })
        .collect();

    let mut vertical_edges: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
    let mut horizontal_edges: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
    for ([x1, y1], [x2, y2]) in vertices.iter().cloned().tuple_windows() {
        if x1 == x2 {
            vertical_edges.entry(x1).or_default().push((y1, y2));
        } else {
            horizontal_edges.entry(y1).or_default().push((x1, x2));
        }
    }

    let mut max_area: u64 = 0;
    for ([x1, y1], [x2, y2]) in vertices.iter().cloned().tuple_combinations() {
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        let area = (max_x - min_x + 1) as u64 * (max_y - min_y + 1) as u64;

        if area <= max_area {
            continue;
        }

        let corners = [
            [min_x, min_y],
            [min_x, max_y],
            [max_x, min_y],
            [max_x, max_y],
        ];

        if !corners
            .iter()
            .all(|&p| point_in_or_on_polygon(p, &vertices))
        {
            continue;
        }

        if rectangle_crosses_boundary(
            min_x,
            max_x,
            min_y,
            max_y,
            &vertical_edges,
            &horizontal_edges,
        ) {
            continue;
        }

        max_area = area;
    }
    max_area as usize
}

fn point_in_or_on_polygon(point: [i64; 2], vertices: &[[i64; 2]]) -> bool {
    let [px, py] = point;
    let mut inside = false;

    for i in 0..vertices.len() {
        let [x1, y1] = vertices[i];
        let [x2, y2] = vertices[(i + 1) % vertices.len()];

        if point_on_segment(point, [x1, y1], [x2, y2]) {
            return true;
        }

        let intersects = ((y1 > py) != (y2 > py))
            && px < x1 + ((x2 - x1) as i128 * (py - y1) as i128 / (y2 - y1) as i128) as i64;
        if intersects {
            inside = !inside;
        }
    }

    inside
}

fn point_on_segment(p: [i64; 2], a: [i64; 2], b: [i64; 2]) -> bool {
    let [px, py] = p;
    let [x1, y1] = a;
    let [x2, y2] = b;

    if (x1 == x2 && px == x1 && py >= y1.min(y2) && py <= y1.max(y2))
        || (y1 == y2 && py == y1 && px >= x1.min(x2) && px <= x1.max(x2))
    {
        return true;
    }
    false
}

fn rectangle_crosses_boundary(
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    vertical_edges: &BTreeMap<i64, Vec<(i64, i64)>>,
    horizontal_edges: &BTreeMap<i64, Vec<(i64, i64)>>,
) -> bool {
    // Any vertical edge strictly between min_x and max_x that overlaps the open vertical span.
    if min_x < max_x {
        for spans in vertical_edges.range((Bound::Excluded(min_x), Bound::Excluded(max_x))) {
            for &(y1, y2) in spans.1 {
                let lo = y1.min(y2);
                let hi = y1.max(y2);
                if lo < max_y && hi > min_y {
                    return true;
                }
            }
        }
    }

    // Any horizontal edge strictly between min_y and max_y that overlaps the open horizontal span.
    if min_y < max_y {
        for spans in horizontal_edges.range((Bound::Excluded(min_y), Bound::Excluded(max_y))) {
            for &(x1, x2) in spans.1 {
                let lo = x1.min(x2);
                let hi = x1.max(x2);
                if lo < max_x && hi > min_x {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 50);
        assert_eq!(part1(include_str!("../_inputs/day9.txt")), 4749929916)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 24);
        assert_eq!(part2(include_str!("../_inputs/day9.txt")), 1572047142)
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
