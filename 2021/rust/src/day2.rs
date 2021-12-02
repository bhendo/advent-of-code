#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction{
    Forward,
    Up,
    Down,
    No,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Command {
    direction: Direction,
    units: i32,
}

impl From<String> for Command {
    fn from(item: String) -> Self {
        let mut items = item.split_ascii_whitespace();
        let direction = match items.next() {
            Some(item) => match item {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => Direction::No
            },
            None => Direction::No,
        };
        let units = match items.next() {
            Some(item) => match item.parse::<i32>(){
                Ok(value) => value,
                Err(_) => 0,
            },
            None => 0,

        };
        Command { direction: direction, units: units }
    }
}

pub fn part1(input: &[Command]) -> i32{
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    
    for command in input {
        match command.direction {
            Direction::Forward => horizontal = horizontal + command.units,
            Direction::Up => depth = depth - command.units,
            Direction::Down => depth = depth + command.units,
            Direction::No => (),
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
            Direction::No => (),
        }
    }
    return horizontal * depth
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::{Command, part1, part2};
    use super::Direction::{Forward, Up, Down, No};

    const EMPTY: [Command;0] = [];
    const EXAMPLE: [Command;6] = [
        Command{ direction: Forward, units: 5},
        Command{ direction: Down, units: 5},
        Command{ direction: Forward, units: 8},
        Command{ direction: Up, units: 3},
        Command{ direction: Down, units: 8},
        Command{ direction: Forward, units: 2},
    ];
    const BENCHMARK: [Command; 2000] = [Command{direction: Forward, units: 0}; 2000];

    #[test]
    fn test_from_string_for_command() {
        let mut c1: Command = String::from("forward 1").into();
        assert_eq!(c1, Command{direction: Forward, units: 1});

        c1 = String::from("up 2").into();
        assert_eq!(c1, Command{direction: Up, units: 2});

        c1 = String::from("down 3").into();
        assert_eq!(c1, Command{direction: Down, units: 3});

        c1 = String::from("anything else").into();
        assert_eq!(c1, Command{direction: No, units: 0});
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EMPTY), 0);        
        assert_eq!(part1(&EXAMPLE), 150);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(&EMPTY), 0);        
        assert_eq!(part2(&EXAMPLE), 900);
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| {
            part1(&BENCHMARK)
        }) 
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| {
            part2(&BENCHMARK)
        }) 
    }
}
