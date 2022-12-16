use std::vec;

pub fn part1(program: Vec<(&str, i64)>) -> i64 {
    let mut cycle = 1;
    let mut register = 1;

    program.iter().fold(0, |acc, (instrucion, value)| {
        let mut s = 0;
        match *instrucion {
            "addx" => {
                if (cycle + 1 - 20) % 40 == 0 {
                    s = register * (cycle + 1);
                }

                cycle += 2;
                register += value;
            }
            _ => {
                cycle += 1;
            }
        }
        if (cycle - 20) % 40 == 0 {
            s = register * cycle;
        }
        acc + s
    })
}

pub fn part2(program: Vec<(&str, i64)>) -> Vec<Vec<char>> {
    let mut crt = vec![vec!['.'; 40]; 6];
    let mut p = program.clone();
    let mut register: i64 = 1;
    let mut working = false;
    let mut instruction = ("noop", 0);

    for cycle in 0..240 {
        let x = cycle % 40;
        if (x - register).abs() <= 1 {
            crt[(cycle / 40) as usize][x as usize] = '#';
        }

        if !working {
            instruction = p.drain(..1).last().unwrap();
            match instruction.0 {
                "addx" => {
                    working = true;
                }
                _ => instruction = ("noop", 0),
            }
        } else {
            register += instruction.1;
            working = false;
        }
    }
    crt
}

pub fn parse_input(input: &str) -> Vec<(&str, i64)> {
    input
        .split("\n")
        .map(|l| {
            let (dir, ammount) = l.split_once(" ").unwrap_or((l, "0"));
            (dir, ammount.parse::<i64>().unwrap())
        })
        .collect::<Vec<(&str, i64)>>()
}

pub fn print_output(output: Vec<Vec<char>>) {
    println!("");
    output.iter().for_each(|l| {
        println!("{}", l.iter().collect::<String>());
    });
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2, print_output};
    use test::Bencher;

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_day1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 13140);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day10.txt"))),
            12520
        )
    }

    #[test]
    fn test_part2() {
        print_output(part2(parse_input(EXAMPLE)));
        print_output(part2(parse_input(include_str!("../_inputs/day10.txt"))));
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
