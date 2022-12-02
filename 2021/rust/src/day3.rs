fn sum_at_pos(input: &[String], pos: usize) -> i32 {
    let mut tally = 0;
    input.iter().for_each(|s| match s.chars().nth(pos) {
        Some(c) => match c {
            '1' => tally += 1,
            _ => (),
        },
        None => (),
    });
    return tally;
}

pub fn part1(input: &Vec<String>) -> i32 {
    if input.len() == 0 {
        return 0;
    }
    let bits = input[0].len();
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    (0..bits).for_each(
        |pos| match sum_at_pos(input, pos) >= input.len() as i32 / 2 {
            true => {
                gamma += "1";
                epsilon += "0";
            }
            false => {
                gamma += "0";
                epsilon += "1";
            }
        },
    );

    let g = isize::from_str_radix(&gamma, 2).unwrap();
    let e = isize::from_str_radix(&epsilon, 2).unwrap();

    return (g * e) as i32;
}

pub fn part1a(input: &Vec<String>) -> i32 {
    let gamma = input
        .into_iter()
        .fold(vec![0u32; 5], |count, bits| {
            count
                .into_iter()
                .enumerate()
                .map(|(i, v)| match bits.chars().nth(i) {
                    Some('1') => v + 1,
                    _ => v,
                })
                .collect()
        })
        .into_iter()
        .map(|v| match v >= input.len() as u32 / 2 {
            true => "1",
            false => "0",
        })
        .collect::<String>();

    let g = i32::from_str_radix(&gamma, 2).unwrap();
    g * (!g & ((1 << input[0].len() as i32) - 1)) // shrug....
}

pub fn part2(input: &Vec<String>) -> i32 {
    if input.len() == 0 {
        return 0;
    }
    let bits = input[0].len();

    let ogr = (0..bits)
        .scan(input.clone(), |ogr, pos| {
            let most_is_one = sum_at_pos(ogr, pos) >= (ogr.len() as i32 + 1) / 2;
            ogr.drain_filter(|s| match s.chars().nth(pos) {
                Some('1') => !most_is_one,
                Some('0') => most_is_one,
                _ => true,
            });
            ogr.first().cloned()
        })
        .last()
        .unwrap();

    let csr = (0..bits)
        .scan(input.clone(), |csr, pos| {
            let most_is_one = sum_at_pos(csr, pos) >= (csr.len() as i32 + 1) / 2;
            csr.drain_filter(|s| match s.chars().nth(pos) {
                Some('1') => most_is_one,
                Some('0') => !most_is_one,
                _ => true,
            });
            csr.first().cloned()
        })
        .last()
        .unwrap();

    let o = isize::from_str_radix(&ogr, 2).unwrap() as i32;
    let c = isize::from_str_radix(&csr, 2).unwrap() as i32;

    return o * c;
}

#[cfg(test)]
mod tests {
    use crate::day3::sum_at_pos;

    use super::{part1, part1a, part2};

    const EMPTY: [&str; 0] = [];
    const EXAMPLE: [&'static str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_sum_at_pos() {
        let example = EXAMPLE
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();
        let total = sum_at_pos(&example, 0);

        assert_eq!(total, 7);
    }

    #[test]
    fn test_part1() {
        let empty = EMPTY
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();
        let example = EXAMPLE
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();
        assert_eq!(part1(&empty), 0);
        assert_eq!(part1a(&example), 198);
    }

    #[test]
    fn test_part2() {
        let empty = EMPTY
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();
        let example = EXAMPLE
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();
        assert_eq!(part2(&empty), 0);
        assert_eq!(part2(&example), 230);
    }

    #[test]
    fn test_bits() {
        "00100,11110,10110"
            .split(",")
            .map(|v| u32::from_str_radix(v, 2).unwrap())
            .fold(vec![0u32; 5], |count, bits| {
                count
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| v + ((bits & 1 << i) >> i)) // isolate bit at index and add it to the count at index
                    .collect()
            })
            .into_iter()
            .enumerate()
            .map(|(i, b)| ((b >= 3 / 2) as u32) << i)
            .sum::<u32>();
    }
}
