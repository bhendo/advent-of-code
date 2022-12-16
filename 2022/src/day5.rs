use std::vec;

pub fn part1((stacks, instructions): (Vec<String>, Vec<(usize, usize, usize)>)) -> String {
    let mut s = stacks.clone();
    for (_, instruction) in instructions.iter().enumerate() {
        for _ in 0..instruction.0 {
            let c = s[instruction.1 - 1].pop().unwrap();
            s[instruction.2 - 1].push(c);
        }
    }

    let mut on_top = "".to_string();
    for (_, stack) in s.iter_mut().enumerate() {
        let c = stack.pop().unwrap();
        on_top.push(c);
    }

    on_top
}

pub fn part2((stacks, instructions): (Vec<String>, Vec<(usize, usize, usize)>)) -> String {
    let mut s = stacks.clone();
    for (_, instruction) in instructions.iter().enumerate() {
        let mut crates = "".to_string();
        for _ in 0..instruction.0 {
            let c = s[instruction.1 - 1].pop().unwrap();
            crates.push(c);
        }
        crates
            .chars()
            .rev()
            .for_each(|c| s[instruction.2 - 1].push(c));
    }

    let mut on_top = "".to_string();
    for (_, stack) in s.iter_mut().enumerate() {
        let c = stack.pop().unwrap();
        on_top.push(c);
    }

    on_top
}

pub fn parse_input(input: &str) -> (Vec<String>, Vec<(usize, usize, usize)>) {
    let (c, i) = input.split_once("\n\n").unwrap();
    let crates: Vec<&str> = c
        .split("\n")
        .map(|l| l)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect();

    let numstacks = crates[0]
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .count();

    let mut stacks: Vec<String> = vec!["".to_string(); numstacks];
    for r in 1..crates.len() {
        for s in 0..numstacks {
            let c = crates[r].as_bytes()[1 + (s * 4)] as char;
            if c.is_alphabetic() {
                stacks[s].push(c);
            }
        }
    }

    let instructions: Vec<(usize, usize, usize)> = i
        .split("\n")
        .map(|l| {
            let movevec = l
                .split_whitespace()
                .filter_map(|p| p.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            (movevec[0], movevec[1], movevec[2])
        })
        .collect();

    (stacks, instructions)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), "CMZ");
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day5.txt"))),
            "CFFHVVHNC"
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), "MCD");
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day5.txt"))),
            "FSZWBPTBG"
        )
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
