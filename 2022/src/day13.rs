use std::cmp::Ordering;
use std::vec;

#[derive(Eq)]
pub enum Packet {
    Integer(i32),
    List(Box<Vec<Packet>>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => {
                let mut i = 0;
                while i < l.len() && i < r.len() {
                    match l[i].cmp(&r[i]) {
                        Ordering::Equal => {}
                        ordering => return ordering,
                    };
                    i += 1;
                }
                l.len().cmp(&r.len())
            }
            (Packet::Integer(l), r) => Packet::List(Box::new(vec![Packet::Integer(*l)])).cmp(r),
            (l, Packet::Integer(r)) => l.cmp(&Packet::List(Box::new(vec![Packet::Integer(*r)]))),
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Integer(l), Packet::Integer(r)) => l == r,
            (Packet::List(l), Packet::List(r)) => match l.len().cmp(&r.len()) {
                Ordering::Equal => {
                    let mut i = 0;
                    while i < l.len() && i < r.len() {
                        match l[i].cmp(&r[i]) {
                            Ordering::Equal => {}
                            _ => return false,
                        };
                        i += 1;
                    }
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn part1(input: Vec<(Packet, Packet)>) -> usize {
    let mut sum = 0;
    for i in 1..=input.len() {
        let (l, r) = &input[i - 1];
        if l < r {
            sum += i;
        }
    }
    sum
}

pub fn part2(input: Vec<(Packet, Packet)>) -> usize {
    let mut packets = vec![];
    input.iter().for_each(|(l, r)| {
        packets.push(l);
        packets.push(r);
    });
    let key1 = parse_packet("[[2]]");
    let key2 = parse_packet("[[6]]");
    packets.push(&key1);
    packets.push(&key2);
    packets.sort_unstable();
    (0..packets.len())
        .filter(|&i| packets[i].eq(&key1) || packets[i].eq(&key2))
        .map(|i| i + 1)
        .product()
}

fn parse_packet(packet: &str) -> Packet {
    let mut num = vec![];
    let mut packet_parts = vec![vec![]];
    for c in packet.chars() {
        match c {
            '[' => packet_parts.push(vec![]),
            ']' => {
                let i = packet_parts.len() - 1;
                match num.iter().collect::<String>().parse::<i32>() {
                    Ok(n) => packet_parts[i].push(Packet::Integer(n)),
                    Err(_) => {}
                }
                num = vec![];
                let p = packet_parts.pop().unwrap();
                packet_parts[i - 1].push(Packet::List(Box::new(p)));
            }
            ',' => {
                let i = packet_parts.len() - 1;
                match num.iter().collect::<String>().parse::<i32>() {
                    Ok(n) => packet_parts[i].push(Packet::Integer(n)),
                    Err(_) => {}
                }
                num = vec![];
            }
            d => num.push(d),
        }
    }
    Packet::List(Box::new(packet_parts.pop().unwrap()))
}

pub fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|p| {
            let (l, r) = p.split_once("\n").unwrap();
            (parse_packet(l), parse_packet(r))
        })
        .collect::<Vec<(Packet, Packet)>>()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 13);
        assert_eq!(
            part1(parse_input(include_str!("../_inputs/day13.txt"))),
            5852
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 140);
        assert_eq!(
            part2(parse_input(include_str!("../_inputs/day13.txt"))),
            24190
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
