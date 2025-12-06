pub fn part1(input: &str) -> usize {
    let worksheet: Vec<_> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();

    let rows = worksheet.len();
    let columns = worksheet[0].len();

    (0..columns)
        .map(|c| match worksheet[rows - 1][c] {
            "+" => (0..rows - 1).fold(0, |acc, r| worksheet[r][c].parse::<usize>().unwrap() + acc),
            _ => (0..rows - 1).fold(1, |acc, r| worksheet[r][c].parse::<usize>().unwrap() * acc),
        })
        .sum()
}
pub fn part2(input: &str) -> usize {
    let mut lines: Vec<_> = input.lines().collect();
    let ops: Vec<_> = lines.pop().unwrap().split_whitespace().collect();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap();

    let reversed_rows: Vec<_> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<_> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars.reverse();
            chars
        })
        .collect();

    (0..max_len)
        .map(|idx| reversed_rows.iter().map(|row| row[idx]).collect::<String>())
        .collect::<Vec<_>>()
        .iter()
        .map(|col| col.trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split("  ")
        .map(|group| {
            group
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .zip(ops.iter().rev())
        .map(|(numbers, &op)| match op {
            "*" => numbers.iter().product::<usize>(),
            _ => numbers.iter().sum::<usize>(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
        assert_eq!(part1(include_str!("../_inputs/day6.txt")), 5381996914800)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3263827);
        assert_eq!(part2(include_str!("../_inputs/day6.txt")), 9627174150897)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
