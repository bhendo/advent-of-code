extern crate aoc2021;
use aoc2021::day7::{part1, part2};

fn main() {
    let crabs = include_str!("../../_inputs/day7.txt")
        .split(",")
        .map(|v| i32::from_str_radix(v, 10).unwrap())
        .collect::<Vec<_>>();

    println!("Answer Part 1: {}", part1(crabs.clone()));
    println!("Answer Part 2: {}", part2(crabs));
}
