fn solvable(test_value: usize, current_result: usize, mut remaining_values: Vec<usize>) -> bool {
    if remaining_values.is_empty() {
        return current_result == test_value;
    }

    if current_result > test_value {
        return false;
    }

    if let Some(next_value) = remaining_values.pop() {
        return solvable(
            test_value,
            current_result + next_value,
            remaining_values.clone(),
        ) || solvable(
            test_value,
            current_result * next_value,
            remaining_values.clone(),
        );
    }
    false
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            // do work
            let (test_value, equation) = line.split_once(":").unwrap();
            let test_value = test_value.parse::<usize>().unwrap();
            let mut equation = equation
                .split(" ")
                .filter_map(|n| {
                    if n.is_empty() {
                        return None;
                    }
                    Some(n.parse::<usize>().unwrap())
                })
                .collect::<Vec<_>>();

            equation.reverse();
            if let Some(current_result) = equation.pop() {
                if solvable(test_value, current_result, equation) {
                    return test_value;
                }
            }
            0
        })
        .sum()
}

fn solvable_with_contact(
    test_value: usize,
    current_result: usize,
    mut remaining_values: Vec<usize>,
) -> bool {
    if remaining_values.is_empty() {
        return current_result == test_value;
    }

    if current_result > test_value {
        return false;
    }

    if let Some(next_value) = remaining_values.pop() {
        return solvable_with_contact(
            test_value,
            current_result + next_value,
            remaining_values.clone(),
        ) || solvable_with_contact(
            test_value,
            current_result * next_value,
            remaining_values.clone(),
        ) || solvable_with_contact(
            test_value,
            format!("{}{}", current_result, next_value)
                .parse::<usize>()
                .unwrap(),
            remaining_values,
        );
    }
    false
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            // do work
            let (test_value, equation) = line.split_once(":").unwrap();
            let test_value = test_value.parse::<usize>().unwrap();
            let mut equation = equation
                .split(" ")
                .filter_map(|n| {
                    if n.is_empty() {
                        return None;
                    }
                    Some(n.parse::<usize>().unwrap())
                })
                .collect::<Vec<_>>();

            equation.reverse();

            if let Some(current_result) = equation.pop() {
                if solvable_with_contact(test_value, current_result, equation) {
                    return test_value;
                }
            }
            0
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3749);
        assert_eq!(part1(include_str!("../_inputs/day7.txt")), 1289579105366)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 11387);
        assert_eq!(part2(include_str!("../_inputs/day7.txt")), 92148721834692)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
