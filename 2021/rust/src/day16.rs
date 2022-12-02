use std::vec;

pub fn part1(input: &str) -> u16 {
    let bits = input
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).map(move |i| (1 & n >> (3 - i)) as u8)
        })
        .collect::<Vec<u8>>();

    version_sum(bits.as_slice()).0
}
pub fn part2(input: &str) -> u8 {
    let mut bits = input
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).map(move |i| (1 & n >> (3 - i)) as u8)
        })
        .collect::<Vec<u8>>();

    packet_value(bits.as_mut_slice())[0]
}

fn version_sum(mut bits: &[u8]) -> (u16, &[u8]) {
    let mut packet_version = from_bits(&bits[..3]);
    let packet_type = from_bits(&bits[3..6]);
    bits = &bits[6..];
    match packet_type {
        4 => {
            (0..)
                .take_while(|_| {
                    let more = bits[0] == 1;
                    bits = &bits[5..];
                    more
                })
                .count();
            (packet_version, &bits)
        }
        _ => {
            let len_type_id = bits[0];
            bits = &bits[1..];
            match len_type_id {
                0 => {
                    let len = from_bits(&bits[0..15]) as usize;
                    bits = &bits[15..];
                    let (mut sub_packets, extra) = bits.split_at(len);
                    bits = extra;
                    while !sub_packets.is_empty() {
                        let packet = version_sum(sub_packets);
                        sub_packets = packet.1;
                        packet_version += packet.0 as u16;
                    }
                    (packet_version, bits)
                }
                1 => {
                    let split = bits.split_at(11);
                    bits = split.1;
                    packet_version += (0..from_bits(split.0))
                        .map(|_| {
                            let packet = version_sum(bits);
                            bits = packet.1;
                            packet.0
                        })
                        .sum::<u16>();
                    (packet_version, bits)
                }
                _ => panic!("unreachable"),
            }
        }
    }
}

fn packet_value(mut bits: &mut [u8]) -> Vec<u8> {
    let mut values = vec![];
    let packet_type = from_bits(&bits[3..6]);
    bits = &mut bits[6..];
    match packet_type {
        4 => {
            let mut value = vec![];
            let mut new_bits = &bits[0..];
            (0..)
                .take_while(|_| {
                    let more = new_bits[0] == 1;
                    value.extend(&new_bits[1..5]);
                    new_bits = &new_bits[5..];
                    more
                })
                .count();
            return vec![from_bits(value.as_slice()) as u8];
        }
        _ => {
            let len_type_id = bits[0];
            bits = &mut bits[1..];
            match len_type_id {
                0 => {
                    let len = from_bits(&bits[0..15]) as usize;
                    bits = &mut bits[15..];
                    let (mut sub_packets, _) = bits.split_at(len);
                    while !sub_packets.is_empty() {
                        values.extend_from_slice(&packet_value(sub_packets));
                    }
                }
                1 => {
                    let split = bits.split_at(11);
                    bits = &mut split.1;
                    (0..from_bits(split.0)).for_each(|_| {
                        values.extend_from_slice(&packet_value(&mut bits));
                    });
                }
                _ => unreachable!(),
            }
        }
    }

    match packet_type {
        0 => vec![values.iter().sum()],
        1 => vec![values.iter().product()],
        2 => vec![*values.iter().min().unwrap()],
        3 => vec![*values.iter().max().unwrap()],
        5 => vec![(values[0] > values[1]) as u8],
        6 => vec![(values[0] < values[1]) as u8],
        7 => vec![(values[0] == values[1]) as u8],
        _ => unreachable!(),
    }
}

fn from_bits(bits: &[u8]) -> u16 {
    bits.iter()
        .enumerate()
        .map(|(i, &v)| (v as u16) << (bits.len() - i - 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day16::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
        assert_eq!(part1(include_str!("../../_inputs/day16.txt")), 951);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("C200B40A82"), 3);
        // assert_eq!(part2("04005AC33890"), 54);
        // assert_eq!(part2(include_str!("../../_inputs/day15.txt")), 0);
    }
}
