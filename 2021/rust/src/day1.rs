pub fn part1(input: &[i32]) -> i32 {
    let mut output: i32 = 0;
    if input.len() < 2 {
        return output;
    }
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            output += 1;
        }
    }
    return output;
}

pub fn part2(input: &[i32]) -> i32 {
    let mut output: i32 = 0;
    if input.len() < 4 {
        return output;
    }
    for i in 1..input.len() {
        if i + 3 <= input.len()
            && input[i - 1..i + 2].iter().sum::<i32>() < input[i..i + 3].iter().sum::<i32>()
        {
            output += 1;
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EMPTY: [i32; 0] = [];
    const EXAMPLE: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EMPTY), 0);
        assert_eq!(part1(&EXAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EMPTY), 0);
        assert_eq!(part2(&EXAMPLE), 5);
    }
}
