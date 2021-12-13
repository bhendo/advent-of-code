use std::collections::BTreeMap;

pub fn part1<'a>(input: &'a str) -> u32 {
    let map = input.lines().fold(
        BTreeMap::new(),
        |mut paths: BTreeMap<&str, Vec<&str>>, l| {
            let (a, b) = l.split_once("-").unwrap();
            paths.entry(a).or_default().push(b);
            paths.entry(b).or_default().push(a);
            paths
        },
    );
    let mut visited = Vec::from(["start"]);
    path1(&map, "start", &mut visited)
}

fn path1<'a>(
    map: &BTreeMap<&'a str, Vec<&'a str>>,
    current: &str,
    visited: &mut Vec<&'a str>,
) -> u32 {
    map.get(current)
        .unwrap()
        .iter()
        .fold(0, |acc, &cave| match cave {
            "end" => acc + 1,
            cave if cave.to_ascii_lowercase().eq(cave) && visited.contains(&cave) => acc,
            cave => {
                let len = visited.len();
                visited.push(cave);
                let paths = path1(map, cave, visited);
                visited.truncate(len);
                acc + paths
            }
        })
}

pub fn part2<'a>(input: &'a str) -> u32 {
    let map = input.lines().fold(
        BTreeMap::new(),
        |mut paths: BTreeMap<&str, Vec<&str>>, l| {
            let (a, b) = l.split_once("-").unwrap();
            paths.entry(a).or_default().push(b);
            paths.entry(b).or_default().push(a);
            paths
        },
    );
    let mut visited = Vec::from(["start"]);
    path2(&map, "start", &mut visited, false)
}

fn path2<'a>(
    map: &BTreeMap<&'a str, Vec<&'a str>>,
    current: &str,
    visited: &mut Vec<&'a str>,
    visited_twice: bool,
) -> u32 {
    map.get(current)
        .unwrap()
        .iter()
        .fold(0, |acc, &cave| match cave {
            "start" => acc,
            "end" => acc + 1,
            cave => {
                let small_cave = cave.to_ascii_lowercase().eq(cave);
                let seen = visited.contains(&cave);

                if seen && small_cave && visited_twice {
                    return acc;
                }

                let len = visited.len();
                visited.push(cave);
                let paths = path2(map, cave, visited, visited_twice || (small_cave && seen));
                visited.truncate(len);
                acc + paths
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::day12::{part1, part2};

    const EXAMPLE1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EXAMPLE2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EXAMPLE3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 10);
        assert_eq!(part1(EXAMPLE2), 19);
        assert_eq!(part1(EXAMPLE3), 226);
        assert_eq!(part1(include_str!("../../_inputs/day12.txt")), 4749)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE1), 36);
        assert_eq!(part2(EXAMPLE2), 103);
        assert_eq!(part2(EXAMPLE3), 3509);
        assert_eq!(part2(include_str!("../../_inputs/day12.txt")), 123054)
    }
}
