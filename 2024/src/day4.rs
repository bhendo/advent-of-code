pub fn part1(input: &str) -> usize {
    let word_search = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = word_search.len();
    let width = word_search[0].len();

    (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            [
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)], // horizontal
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)], // vertical
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // diagonol right
                [(x + 3, y), (x + 2, y + 1), (x + 1, y + 2), (x, y + 3)], // diagonal left
            ]
        })
        .filter(|cords| {
            let word = cords
                .iter()
                .map(|(x, y)| {
                    word_search
                        .get(*y)
                        .and_then(|row| row.get(*x).copied())
                        .unwrap_or_default()
                })
                .collect::<String>();
            word == "XMAS" || word == "SAMX"
        })
        .count()
}
pub fn part2(input: &str) -> usize {
    let word_search = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = word_search.len();
    let width = word_search[0].len();

    (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .map(|(x, y)| {
            [
                [(x, y), (x + 1, y + 1), (x + 2, y + 2)], // diagonal right
                [(x + 2, y), (x + 1, y + 1), (x, y + 2)], // diagonal left
            ]
        })
        .filter(|cords| {
            cords
                .map(|diag| {
                    diag.iter()
                        .map(|(x, y)| {
                            word_search
                                .get(*y)
                                .and_then(|row| row.get(*x).copied())
                                .unwrap_or_default()
                        })
                        .collect::<String>()
                })
                .iter()
                .all(|word| word == "MAS" || word == "SAM")
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use test::Bencher;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 18);
        assert_eq!(part1(include_str!("../_inputs/day4.txt")), 2551)
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| part1(EXAMPLE))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 9);
        assert_eq!(part2(include_str!("../_inputs/day4.txt")), 1985)
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| part2(EXAMPLE))
    }
}
