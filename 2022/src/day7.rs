use std::collections::HashMap;

pub fn part1(terminal: Vec<(&str, &str)>) -> i32 {
    let mut cd: Vec<String> = vec!["".to_string(); 0];
    let mut dirs: HashMap<String, i32> = HashMap::new();

    let fs = terminal.iter().fold(0, |acc, line| match line.0 {
        "$" => match line.1.split_once(" ") {
            Some(d) => {
                let mut path = "".to_string();
                cd.iter().for_each(|dir| {
                    path.push_str("/");
                    path.push_str(dir);
                    match dirs.get(&path) {
                        Some(current_total) => dirs.insert(path.to_string(), current_total + acc),
                        None => dirs.insert(path.to_string(), acc),
                    };
                });
                if d.1 == ".." {
                    cd.pop().unwrap();
                } else {
                    cd.push(d.1.to_string());
                }
                0
            }
            None => acc,
        },
        _ => match line.0.parse::<i32>() {
            Ok(s) => acc + s,
            Err(_) => acc,
        },
    });

    let mut path = "".to_string();
    cd.iter().for_each(|dir| {
        path.push_str("/");
        path.push_str(dir);
        match dirs.get(&path) {
            Some(current_total) => dirs.insert(path.to_string(), current_total + fs),
            None => dirs.insert(path.to_string(), fs),
        };
    });

    dirs.values()
        .fold(0, |acc, &s| if s <= 100000 { acc + s } else { acc })
}

pub fn part2(terminal: Vec<(&str, &str)>) -> i32 {
    let mut cd: Vec<String> = vec!["".to_string(); 0];
    let mut dirs: HashMap<String, i32> = HashMap::new();

    let fs = terminal.iter().fold(0, |acc, line| match line.0 {
        "$" => match line.1.split_once(" ") {
            Some(d) => {
                let mut path = "".to_string();
                cd.iter().for_each(|dir| {
                    path.push_str("/");
                    path.push_str(dir);
                    match dirs.get(&path) {
                        Some(current_total) => dirs.insert(path.to_string(), current_total + acc),
                        None => dirs.insert(path.to_string(), acc),
                    };
                });
                if d.1 == ".." {
                    cd.pop().unwrap();
                } else {
                    cd.push(d.1.to_string());
                }
                0
            }
            None => acc,
        },
        _ => match line.0.parse::<i32>() {
            Ok(s) => acc + s,
            Err(_) => acc,
        },
    });

    let mut path = "".to_string();
    cd.iter().for_each(|dir| {
        path.push_str("/");
        path.push_str(dir);
        match dirs.get(&path) {
            Some(current_total) => dirs.insert(path.to_string(), current_total + fs),
            None => dirs.insert(path.to_string(), fs),
        };
    });

    let used = dirs.get("//").unwrap();
    let need = (70000000 - used - 30000000).abs();

    dirs.values().fold(
        *used,
        |acc, &s| {
            if (s >= need) && s < acc {
                s
            } else {
                acc
            }
        },
    )
}

pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .split("\n")
        .map(|l| l.split_once(" ").unwrap())
        .collect::<Vec<(&str, &str)>>()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_day1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 95437);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day7.txt"))),
            1886043
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 24933642);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day7.txt"))),
            3842121
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
