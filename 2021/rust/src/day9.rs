pub fn part1(input: Vec<Vec<u32>>) -> u32 {
    let adjacent = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut lowest_points = vec![0; 0];
    for (y, line) in input.iter().enumerate() {
        for (x, &cell) in line.iter().enumerate() {
            if adjacent.iter().all(|&(dx, dy)| {
                match input
                    .get((y as isize + dy) as usize)
                    .and_then(|l| l.get((x as isize + dx) as usize))
                    .map(|&n| cell < n)
                {
                    Some(true) => true,
                    Some(false) => false,
                    _ => true,
                }
            }) {
                lowest_points.push(cell)
            }
        }
    }
    lowest_points.iter().map(|n| n + 1).sum()
}

fn walk_basin(input: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    input[y][x] = 9;
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|&(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .fold(1, |count, (x, y)| {
            match input.get(y).and_then(|row| row.get(x)).map(|&v| v < 9) {
                Some(true) => count + walk_basin(input, x, y),
                _ => count,
            }
        })
}

pub fn part2(input: &mut Vec<Vec<u32>>) -> u32 {
    let mut basin_size = vec![0; 0];
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] < 9 {
                basin_size.push(walk_basin(input, x, y))
            }
        }
    }
    basin_size.sort_unstable();
    basin_size.iter().rev().take(3).product()
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|n| u32::from_str_radix(&n.to_string(), 10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
    use crate::day9::{parse_input, part1, part2};

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 15);
        assert_eq!(
            part1(parse_input(include_str!("../../_inputs/day9.txt"))),
            486
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut parse_input(EXAMPLE)), 1134);
        assert_eq!(
            part2(&mut parse_input(include_str!("../../_inputs/day9.txt"))),
            1059300
        )
    }
}
