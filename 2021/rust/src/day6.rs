pub fn part1(input: &str) -> u32 {
    let mut state = input
        .split(",")
        .map(|timer| usize::from_str_radix(timer, 10).unwrap())
        .fold(vec![0; 9], |mut days, timer| {
            days[timer] += 1;
            days
        });

    for _ in 0..80 {
        state.rotate_left(1);
        state[6] += state[8];
    }

    state.into_iter().sum()
}

pub fn part2(input: &str) -> u64 {
    let mut state = input
        .split(",")
        .map(|timer| usize::from_str_radix(timer, 10).unwrap())
        .fold(vec![0; 9], |mut days, timer| {
            days[timer] += 1;
            days
        });

    for _ in 0..256 {
        state.rotate_left(1);
        state[6] += state[8];
    }

    state.into_iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::day6::{part1, part2};

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 5934);
        assert_eq!(part1(include_str!("../../_inputs/day6.txt")), 391671)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 26984457539);
        assert_eq!(part2(include_str!("../../_inputs/day6.txt")), 1754000560399)
    }
}
