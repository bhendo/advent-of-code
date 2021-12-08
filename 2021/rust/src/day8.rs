use core::panic;

pub fn part1(data: Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    data.iter()
        .map(|(_, output)| {
            output
                .iter()
                .map(|signal| match signal.len() as u32 {
                    2 => 1, // 1
                    3 => 1, // 7
                    4 => 1, // 4
                    7 => 1, // 8
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

pub fn part2(data: Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    data.iter()
        .map(|(input, output)| {
            let one = input.iter().find(|v| v.len() == 2).unwrap(); // which characters represent 1?
            let four = input.iter().find(|v| v.len() == 4).unwrap(); // which characters represent 4?
            output
                .iter()
                .map(|signal| match signal.len() {
                    2 => "1",
                    3 => "7",
                    4 => "4",
                    7 => "8",
                    len => match (
                        len,
                        signal.chars().filter(|&c| one.contains(c)).count(), // how many parts of 1?
                        signal.chars().filter(|&c| four.contains(c)).count(), // how many parts of 4?
                    ) {
                        (5, 1, 2) => "2",
                        (5, 2, 3) => "3",
                        (5, 1, 3) => "5",
                        (6, 2, 3) => "0",
                        (6, 1, 3) => "6",
                        (6, 2, 4) => "9",
                        _ => panic!("bad input"),
                    },
                })
                .collect::<String>()
        })
        .map(|v| u32::from_str_radix(&v, 10).unwrap())
        .sum()
}

#[macro_export]
macro_rules! parse_day8_input {
    ($data:expr) => {{
        $data
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| {
                let splits = line.split(" | ").collect::<Vec<_>>();
                let input = splits[0].split(" ").collect::<Vec<_>>();
                let output = splits[1].split(" ").collect::<Vec<_>>();
                (input, output)
            })
            .collect::<Vec<_>>()
    }};
}

#[cfg(test)]
mod tests {
    use crate::day8::{part1, part2};

    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_day8_input!(EXAMPLE)), 26);

        assert_eq!(
            part1(parse_day8_input!(include_str!("../../_inputs/day8.txt"))),
            284
        )
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_day8_input!(EXAMPLE)), 61229);

        assert_eq!(
            part2(parse_day8_input!(include_str!("../../_inputs/day8.txt"))),
            973499
        )
    }
}
