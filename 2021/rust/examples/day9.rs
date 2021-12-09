extern crate aoc2021;
use aoc2021::day9::{parse_input, part1, part2};

fn main() {
    let mut data = parse_input(include_str!("../../_inputs/day9.txt"));

    println!("Answer Part 1: {}", part1(data.clone()));
    println!("Answer Part 2: {}", part2(&mut data));
}
