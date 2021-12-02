pub enum Direction{
    Forward,
    Up,
    Down
}
pub struct Command {
    direction: Direction,
    units: i32,
}

pub fn part1(input: &[Command]) -> i32{
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    
    for command in input {
        match command.direction {
            Direction::Forward => horizontal = horizontal + command.units,
            Direction::Up => depth = depth - command.units,
            Direction::Down => depth = depth + command.units,
        }
    }
    return horizontal * depth
}

pub fn part2(input: &[Command]) -> i32{
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    
    for command in input {
        match command.direction {
            Direction::Forward => {
                horizontal = horizontal + command.units;
                depth = depth + (aim * command.units)
            },
            Direction::Up => aim = aim - command.units,
            Direction::Down => aim = aim + command.units,
        }
    }
    return horizontal * depth
}



#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::{Command, part1, part2};
    use super::Direction::{Forward, Up, Down};

    const EMPTY: &[Command] = &[];
    const EXAMPLE: &[Command] = &[
        Command{ direction: Forward, units: 5},
        Command{ direction: Down, units: 5},
        Command{ direction: Forward, units: 8},
        Command{ direction: Up, units: 3},
        Command{ direction: Down, units: 8},
        Command{ direction: Forward, units: 2},
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(EMPTY), 0);        
        assert_eq!(part1(EXAMPLE), 150);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(EMPTY), 0);        
        assert_eq!(part2(EXAMPLE), 900);
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| {
            part1(EXAMPLE)
        }) 
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| {
            part2(EXAMPLE)
        }) 
    }
}