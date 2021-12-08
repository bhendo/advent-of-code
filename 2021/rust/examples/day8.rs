extern crate aoc2021;
use aoc2021::day8::{part1, part2};
use aoc2021::parse_day8_input;

fn main() {
    let data = parse_day8_input!(include_str!("../../_inputs/day8.txt"));

    println!("Answer Part 1: {}", part1(data.clone()));
    println!("Answer Part 2: {}", part2(data));
}
