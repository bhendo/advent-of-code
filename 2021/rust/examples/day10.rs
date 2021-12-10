extern crate aoc2021;
use aoc2021::day10::{parse_input, part1, part2};

fn main() {
    let data = parse_input(include_str!("../../_inputs/day10.txt"));

    println!("Answer Part 1: {}", part1(data.clone()));
    println!("Answer Part 2: {}", part2(data));
}
