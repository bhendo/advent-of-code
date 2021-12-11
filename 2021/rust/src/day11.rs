fn bloom(input: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    if input[y][x] > 9 {
        return 0;
    }
    let new_value = input[y][x] + 1;
    input[y][x] = new_value;
    if input[y][x] == 10 {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, -1),
            (0, 1),
        ]
        .iter()
        .map(|&(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .fold(1, |flashes, (x, y)| {
            match input.get(y).and_then(|row| row.get(x)) {
                Some(_) => flashes + bloom(input, x, y),
                _ => flashes,
            }
        })
    } else {
        0
    }
}
pub fn part1(input: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes = 0;
    for _ in 0..100 {
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                flashes += bloom(input, x, y);
            }
        }
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] > 9 {
                    input[y][x] = 0;
                }
            }
        }
    }
    flashes
}

pub fn part2(input: &mut Vec<Vec<u32>>) -> u32 {
    for i in 0..1000 {
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                bloom(input, x, y);
            }
        }
        if input.iter().all(|l| l.iter().all(|&v| v > 9)) {
            return i + 1;
        }
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] > 9 {
                    input[y][x] = 0;
                }
            }
        }
    }
    0
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
    use crate::day11::{parse_input, part1, part2};

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&mut parse_input(EXAMPLE)), 1656);
        assert_eq!(
            part1(&mut parse_input(include_str!("../../_inputs/day11.txt"))),
            1679
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut parse_input(EXAMPLE)), 195);
        assert_eq!(
            part2(&mut parse_input(include_str!("../../_inputs/day11.txt"))),
            519
        )
    }
}
