use std::collections::HashMap;

fn score_illegal_char(c: &char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid input"),
    }
}
fn score_closing_char(c: &char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid input"),
    }
}

pub fn part1(input: Vec<Vec<char>>) -> u32 {
    let map_open_close: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let map_close_open: HashMap<char, char> =
        HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    input
        .iter()
        .map(|line| {
            let mut acc: Vec<&char> = vec![];
            for c in line {
                match map_open_close.get(c) {
                    Some(_) => acc.push(c),
                    None => match acc.pop() {
                        Some(l) => match map_close_open.get(c) {
                            Some(expected) => {
                                if expected != l {
                                    return score_illegal_char(c);
                                }
                            }
                            None => return score_illegal_char(c),
                        },
                        None => return score_illegal_char(c),
                    },
                }
            }
            0
        })
        .sum()
}

pub fn part2(input: Vec<Vec<char>>) -> u64 {
    let map_open_close: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let map_close_open: HashMap<char, char> =
        HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let mut scores = input
        .iter()
        .filter_map(|line| {
            let mut acc: Vec<&char> = vec![];
            for c in line {
                match map_open_close.get(c) {
                    Some(_) => acc.push(c),
                    None => match acc.pop() {
                        Some(l) => match map_close_open.get(c) {
                            Some(expected) => {
                                if expected != l {
                                    return None;
                                }
                            }
                            None => return None,
                        },
                        None => return None,
                    },
                }
            }
            Some(acc)
        })
        .map(|input| {
            input.iter().rev().fold(0u64, |mut acc, &c| {
                acc = (acc * 5) + score_closing_char(map_open_close.get(c).unwrap());
                acc
            })
        })
        .collect::<Vec<u64>>();
    let mid = (scores.len() / 2) as usize;
    *scores.select_nth_unstable(mid).1
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day10::{parse_input, part1, part2};

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 26397);
        assert_eq!(
            part1(parse_input(include_str!("../../_inputs/day10.txt"))),
            265527
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 288957);
        assert_eq!(
            part2(parse_input(include_str!("../../_inputs/day10.txt"))),
            3969823589
        )
    }
}
