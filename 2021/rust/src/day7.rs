pub fn part1(mut crabs: Vec<i32>) -> i32 {
    crabs.sort_unstable();
    let median_crab = crabs[crabs.len() / 2];
    crabs
        .iter()
        .map(|crab| (crab - median_crab).abs())
        .sum::<i32>()
}

pub fn part2(crabs: Vec<i32>) -> i32 {
    let average_crab = crabs.iter().sum::<i32>() / crabs.len() as i32;

    (average_crab..average_crab + 2)
        .map(|c| {
            crabs
                .iter()
                .map(|crab| {
                    let d = (crab - c).abs();
                    d * (d + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day7::{part1, part2};

    #[test]
    fn test_part1() {
        let example = "16,1,2,0,4,2,7,1,2,14"
            .split(",")
            .map(|n| i32::from_str_radix(n, 10).unwrap())
            .collect::<Vec<i32>>();

        assert_eq!(part1(example), 37);

        let crabs = include_str!("../../_inputs/day7.txt")
            .split(",")
            .map(|v| i32::from_str_radix(v, 10).unwrap())
            .collect::<Vec<i32>>();

        assert_eq!(part1(crabs), 344138);
    }

    #[test]
    fn test_part2() {
        let example = "16,1,2,0,4,2,7,1,2,14"
            .split(",")
            .map(|n| i32::from_str_radix(n, 10).unwrap())
            .collect::<Vec<i32>>();

        assert_eq!(part2(example), 168);

        let crabs = include_str!("../../_inputs/day7.txt")
            .split(",")
            .map(|v| i32::from_str_radix(v, 10).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(part2(crabs), 94862124);
    }
}
