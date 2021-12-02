extern crate aoc2021;
use aoc2021::day2::{part1, part2, Command};
use aoc2021::shared::{read_lines};

fn main() {
    let filename = "../_inputs/day2.txt";
    match read_lines(filename) {
        Err(why) => panic!("couldn't read {}: {}", filename, why),
        Ok(lines) => {
            let mut v: Vec<Command> = Vec::new();
            for line in lines {
                if let Ok(value) = line {
                    v.push(value.into())
                }
            }
            println!("Answer Part 1: {}", part1(&v));
            println!("Answer Part 2: {}", part2(&v))
        }
    }
}
