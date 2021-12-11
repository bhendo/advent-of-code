extern crate aoc2021;
use aoc2021::day11::{parse_input, part1, part2};

fn main() {
    let data = parse_input(include_str!("../../_inputs/day11.txt"));

    println!("Answer Part 1: {}", part1(&mut data.clone()));
    println!("Answer Part 2: {}", part2(&mut data.clone()));
}
